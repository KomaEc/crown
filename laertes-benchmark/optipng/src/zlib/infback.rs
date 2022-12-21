
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn zcfree(opaque: voidpf, ptr: voidpf);
    #[no_mangle]
    fn zcalloc(opaque: voidpf, items: std::os::raw::c_uint, size: std::os::raw::c_uint)
     -> voidpf;
    #[no_mangle]
    fn inflate_table(type_0: codetype, lens: *mut std::os::raw::c_ushort,
                     codes: std::os::raw::c_uint, table: *mut *mut code,
                     bits: *mut std::os::raw::c_uint, work: *mut std::os::raw::c_ushort)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn inflate_fast(strm: z_streamp, start: std::os::raw::c_uint);
}
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
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: std::os::raw::c_int,
    pub time: uLong,
    pub xflags: std::os::raw::c_int,
    pub os: std::os::raw::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: std::os::raw::c_int,
    pub done: std::os::raw::c_int,
}
pub type gz_header = gz_header_s;
pub type gz_headerp = *mut gz_header;
pub type in_func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                _: *mut *mut std::os::raw::c_uchar) -> std::os::raw::c_uint>;
pub type out_func
    =
    Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: *mut std::os::raw::c_uchar,
                                _: std::os::raw::c_uint) -> std::os::raw::c_int>;
pub const BAD: inflate_mode = 16209;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub strm: z_streamp,
    pub mode: inflate_mode,
    pub last: std::os::raw::c_int,
    pub wrap: std::os::raw::c_int,
    pub havedict: std::os::raw::c_int,
    pub flags: std::os::raw::c_int,
    pub dmax: std::os::raw::c_uint,
    pub check: std::os::raw::c_ulong,
    pub total: std::os::raw::c_ulong,
    pub head: gz_headerp,
    pub wbits: std::os::raw::c_uint,
    pub wsize: std::os::raw::c_uint,
    pub whave: std::os::raw::c_uint,
    pub wnext: std::os::raw::c_uint,
    pub window: *mut std::os::raw::c_uchar,
    pub hold: std::os::raw::c_ulong,
    pub bits: std::os::raw::c_uint,
    pub length: std::os::raw::c_uint,
    pub offset: std::os::raw::c_uint,
    pub extra: std::os::raw::c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: std::os::raw::c_uint,
    pub distbits: std::os::raw::c_uint,
    pub ncode: std::os::raw::c_uint,
    pub nlen: std::os::raw::c_uint,
    pub ndist: std::os::raw::c_uint,
    pub have: std::os::raw::c_uint,
    pub next: *mut code,
    pub lens: [std::os::raw::c_ushort; 320],
    pub work: [std::os::raw::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: std::os::raw::c_int,
    pub back: std::os::raw::c_int,
    pub was: std::os::raw::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: std::os::raw::c_uchar,
    pub bits: std::os::raw::c_uchar,
    pub val: std::os::raw::c_ushort,
}
pub type inflate_mode = std::os::raw::c_uint;
pub const SYNC: inflate_mode = 16211;
pub const MEM: inflate_mode = 16210;
pub const DONE: inflate_mode = 16208;
pub const LENGTH: inflate_mode = 16207;
pub const CHECK: inflate_mode = 16206;
pub const LIT: inflate_mode = 16205;
pub const MATCH: inflate_mode = 16204;
pub const DISTEXT: inflate_mode = 16203;
pub const DIST: inflate_mode = 16202;
pub const LENEXT: inflate_mode = 16201;
pub const LEN: inflate_mode = 16200;
pub const LEN_: inflate_mode = 16199;
pub const CODELENS: inflate_mode = 16198;
pub const LENLENS: inflate_mode = 16197;
pub const TABLE: inflate_mode = 16196;
pub const COPY: inflate_mode = 16195;
pub const COPY_: inflate_mode = 16194;
pub const STORED: inflate_mode = 16193;
pub const TYPEDO: inflate_mode = 16192;
pub const TYPE: inflate_mode = 16191;
pub const DICT: inflate_mode = 16190;
pub const DICTID: inflate_mode = 16189;
pub const HCRC: inflate_mode = 16188;
pub const COMMENT: inflate_mode = 16187;
pub const NAME: inflate_mode = 16186;
pub const EXTRA: inflate_mode = 16185;
pub const EXLEN: inflate_mode = 16184;
pub const OS: inflate_mode = 16183;
pub const TIME: inflate_mode = 16182;
pub const FLAGS: inflate_mode = 16181;
pub const HEAD: inflate_mode = 16180;
pub type codetype = std::os::raw::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
/*
   strm provides memory allocation functions in zalloc and zfree, or
   Z_NULL to use the library memory allocation functions.

   windowBits is in the range 8..15, and window is a user-supplied
   window and output buffer that is 2**windowBits bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn inflateBackInit_(mut strm: z_streamp,
                                          mut windowBits: std::os::raw::c_int,
                                          mut window: *mut std::os::raw::c_uchar,
                                          mut version: *const std::os::raw::c_char,
                                          mut stream_size: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state =
        0 as *mut inflate_state; /* in case we return an error */
    if version.is_null() ||
           *version.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
               (*::std::mem::transmute::<&[u8; 15],
                                         &[std::os::raw::c_char; 15]>(b"1.2.11-optipng\x00"))[0
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          usize]
                   as std::os::raw::c_int ||
           stream_size !=
               ::std::mem::size_of::<z_stream>() as std::os::raw::c_ulong as
                   std::os::raw::c_int {
        return -(6 as std::os::raw::c_int)
    }
    if strm.is_null() || window.is_null() || windowBits < 8 as std::os::raw::c_int ||
           windowBits > 15 as std::os::raw::c_int {
        return -(2 as std::os::raw::c_int)
    }
    (*strm).msg = 0 as *mut std::os::raw::c_char;
    if (*strm).zalloc.is_none() {
        (*strm).zalloc =
            Some(zcalloc as
                     unsafe extern "C" fn(_: voidpf, _: std::os::raw::c_uint,
                                          _: std::os::raw::c_uint) -> voidpf);
        (*strm).opaque = 0 as voidpf
    }
    if (*strm).zfree.is_none() {
        (*strm).zfree =
            Some(zcfree as unsafe extern "C" fn(_: voidpf, _: voidpf) -> ())
    }
    state =
        Some((*strm).zalloc.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                     1
                                                                                                         as
                                                                                                         std::os::raw::c_int
                                                                                                         as
                                                                                                         uInt,
                                                                                                     ::std::mem::size_of::<inflate_state>()
                                                                                                         as
                                                                                                         std::os::raw::c_ulong
                                                                                                         as
                                                                                                         uInt)
            as *mut inflate_state;
    if state.is_null() { return -(4 as std::os::raw::c_int) }
    (*strm).state = state as *mut internal_state;
    (*state).dmax = 32768 as std::os::raw::c_uint;
    (*state).wbits = windowBits as uInt;
    (*state).wsize = (1 as std::os::raw::c_uint) << windowBits;
    (*state).window = window;
    (*state).wnext = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).whave = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    return 0 as std::os::raw::c_int;
}
/* infback.c -- inflate using a call-back interface
 * Copyright (C) 1995-2016 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/*
   This code is largely copied from inflate.c.  Normally either infback.o or
   inflate.o would be linked into an application--not both.  The interface
   with inffast.c is retained so that optimized assembler-coded versions of
   inflate_fast() can be used with either inflate.c or infback.c.
 */
/* function prototypes */
/*
   Return state with length and distance decoding tables and index sizes set to
   fixed code decoding.  Normally this returns fixed tables from inffixed.h.
   If BUILDFIXED is defined, then instead this routine builds the tables the
   first time it's called, and returns those tables the first time and
   thereafter.  This reduces the size of the code by about 2K bytes, in
   exchange for a little execution time.  However, BUILDFIXED should not be
   used for threaded applications, since the rewriting of the tables and virgin
   may not be thread-safe.
 */
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    static mut lenfix: [code; 512] =
        [{
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 80 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 112 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 48 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 192 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 96 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 32 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 160 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 128 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 64 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 224 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 88 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 144 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 120 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 56 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 208 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 104 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 40 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 176 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 136 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 72 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 240 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 84 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 20 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 116 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 52 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 200 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 100 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 36 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 168 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 132 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 68 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 232 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 92 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 28 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 152 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 124 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 60 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 216 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 108 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 44 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 184 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 140 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 76 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 248 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 82 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 18 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 114 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 50 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 196 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 98 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 34 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 164 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 130 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 66 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 228 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 90 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 26 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 148 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 122 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 58 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 212 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 106 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 42 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 180 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 138 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 74 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 244 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 86 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 22 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 118 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 54 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 204 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 102 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 38 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 172 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 134 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 70 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 236 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 94 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 30 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 156 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 126 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 62 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 220 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 110 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 46 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 188 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 14 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 142 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 78 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 252 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 81 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 113 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 194 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 162 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 226 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 89 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 146 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 121 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 57 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 210 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 105 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 41 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 178 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 137 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 73 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 242 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 85 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 21 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 258 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 117 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 53 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 202 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 101 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 37 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 170 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 133 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 69 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 234 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 93 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 29 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 154 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 125 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 61 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 218 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 109 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 45 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 186 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 141 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 77 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 250 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 198 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 166 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 230 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 91 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 150 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 123 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 214 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 107 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 182 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 139 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 75 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 246 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 87 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 119 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 55 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 206 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 103 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 39 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 174 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 135 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 71 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 238 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 95 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 158 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 127 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 63 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 222 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 111 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 47 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 190 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 143 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 79 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 254 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 80 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 112 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 48 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 96 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 32 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 161 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 128 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 64 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 225 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 88 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 145 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 120 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 56 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 209 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 104 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 40 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 177 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 136 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 72 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 241 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 84 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 20 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 116 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 52 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 201 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 100 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 36 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 169 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 132 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 68 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 233 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 92 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 28 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 153 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 124 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 60 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 217 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 108 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 44 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 185 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 140 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 76 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 249 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 82 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 18 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 114 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 50 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 197 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 98 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 34 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 165 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 130 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 66 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 229 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 90 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 26 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 149 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 122 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 58 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 213 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 106 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 42 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 181 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 138 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 74 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 245 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 86 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 22 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 118 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 54 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 205 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 102 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 38 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 173 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 134 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 70 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 237 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 94 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 30 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 157 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 126 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 62 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 221 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 110 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 46 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 189 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 14 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 142 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 78 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 253 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 96 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 81 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 113 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 10 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 163 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 227 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 89 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 147 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 121 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 57 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 211 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 105 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 41 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 179 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 137 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 73 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 243 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 85 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 21 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 258 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 117 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 53 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 203 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 101 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 37 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 171 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 133 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 69 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 235 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 93 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 29 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 155 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 125 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 61 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 219 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 109 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 45 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 187 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 141 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 77 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 251 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 83 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 195 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 115 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 199 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 35 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 167 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 131 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 231 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 91 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 151 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 67 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 123 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 59 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 215 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 19 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 107 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 43 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 183 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 11 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 139 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 75 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 247 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 87 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 23 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 51 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 119 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 55 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 207 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 103 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 39 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 175 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 135 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 71 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 239 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 95 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 31 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 159 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 99 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 127 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 63 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 223 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 7 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 27 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 111 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 47 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 191 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 15 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 143 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 8 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 79 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 0 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 9 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 255 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         }];
    static mut distfix: [code; 32] =
        [{
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 23 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 257 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 17 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 27 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4097 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 5 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 25 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1025 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 65 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 29 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 16385 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 24 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 513 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 33 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 28 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 8193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 9 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 26 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2049 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 22 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 129 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 2 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 23 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 385 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 19 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 25 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 27 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 6145 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 17 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 7 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 25 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 1537 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 21 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 97 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 29 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 24577 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 16 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 4 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 24 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 769 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 20 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 49 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 28 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 12289 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 18 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 13 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 26 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 3073 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 22 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 193 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         },
         {
             let mut init =
                 code{op: 64 as std::os::raw::c_int as std::os::raw::c_uchar,
                      bits: 5 as std::os::raw::c_int as std::os::raw::c_uchar,
                      val: 0 as std::os::raw::c_int as std::os::raw::c_ushort,};
             init
         }];
    /* !BUILDFIXED */
    /* BUILDFIXED */
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as std::os::raw::c_int as std::os::raw::c_uint;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as std::os::raw::c_int as std::os::raw::c_uint;
}
/* Remove n bits from the bit accumulator */
/* Remove zero to seven bits as needed to go to a byte boundary */
/* Assure that some output space is available, by writing out the window
   if it's full.  If the write fails, return from inflateBack() with a
   Z_BUF_ERROR. */
/*
   strm provides the memory allocation functions and window buffer on input,
   and provides information on the unused input on return.  For Z_DATA_ERROR
   returns, strm will also provide an error message.

   in() and out() are the call-back input and output functions.  When
   inflateBack() needs more input, it calls in().  When inflateBack() has
   filled the window with output, or when it completes with data in the
   window, it calls out() to write out the data.  The application must not
   change the provided input until in() is called again or inflateBack()
   returns.  The application must not change the window/output buffer until
   inflateBack() returns.

   in() and out() are called with a descriptor parameter provided in the
   inflateBack() call.  This parameter can be a structure that provides the
   information required to do the read or write, as well as accumulated
   information on the input and output such as totals and check values.

   in() should return zero on failure.  out() should return non-zero on
   failure.  If either in() or out() fails, than inflateBack() returns a
   Z_BUF_ERROR.  strm->next_in can be checked for Z_NULL to see whether it
   was in() or out() that caused in the error.  Otherwise,  inflateBack()
   returns Z_STREAM_END on success, Z_DATA_ERROR for an deflate format
   error, or Z_MEM_ERROR if it could not allocate memory for the state.
   inflateBack() can also return Z_STREAM_ERROR if the input parameters
   are not correct, i.e. strm is Z_NULL or the state was not initialized.
 */
#[no_mangle]
pub unsafe extern "C" fn inflateBack(mut strm: z_streamp, mut in_0: in_func,
                                     mut in_desc: *mut std::os::raw::c_void,
                                     mut out: out_func,
                                     mut out_desc: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    let mut state: *mut inflate_state =
        0 as *mut inflate_state; /* next input */
    let mut next: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* next output */
    let mut put: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* available input and output */
    let mut have: std::os::raw::c_uint = 0; /* bit buffer */
    let mut left: std::os::raw::c_uint = 0; /* bits in bit buffer */
    let mut hold: std::os::raw::c_ulong =
        0; /* number of stored or match bytes to copy */
    let mut bits: std::os::raw::c_uint = 0; /* where to copy match bytes from */
    let mut copy: std::os::raw::c_uint = 0; /* current decoding table entry */
    let mut from: *mut std::os::raw::c_uchar =
        0 as *mut std::os::raw::c_uchar; /* parent table entry */
    let mut here: code =
        code{op: 0,
             bits: 0,
             val: 0,}; /* length to copy for repeats, bits to drop */
    let mut last: code = code{op: 0, bits: 0, val: 0,}; /* return code */
    let mut len: std::os::raw::c_uint = 0;
    let mut ret: std::os::raw::c_int = 0;
    static mut order: [std::os::raw::c_ushort; 19] =
        [16 as std::os::raw::c_int as std::os::raw::c_ushort,
         17 as std::os::raw::c_int as std::os::raw::c_ushort,
         18 as std::os::raw::c_int as std::os::raw::c_ushort,
         0 as std::os::raw::c_int as std::os::raw::c_ushort,
         8 as std::os::raw::c_int as std::os::raw::c_ushort,
         7 as std::os::raw::c_int as std::os::raw::c_ushort,
         9 as std::os::raw::c_int as std::os::raw::c_ushort,
         6 as std::os::raw::c_int as std::os::raw::c_ushort,
         10 as std::os::raw::c_int as std::os::raw::c_ushort,
         5 as std::os::raw::c_int as std::os::raw::c_ushort,
         11 as std::os::raw::c_int as std::os::raw::c_ushort,
         4 as std::os::raw::c_int as std::os::raw::c_ushort,
         12 as std::os::raw::c_int as std::os::raw::c_ushort,
         3 as std::os::raw::c_int as std::os::raw::c_ushort,
         13 as std::os::raw::c_int as std::os::raw::c_ushort,
         2 as std::os::raw::c_int as std::os::raw::c_ushort,
         14 as std::os::raw::c_int as std::os::raw::c_ushort,
         1 as std::os::raw::c_int as std::os::raw::c_ushort,
         15 as std::os::raw::c_int as std::os::raw::c_ushort];
    /* Check that the strm exists and that the state was initialized */
    if strm.is_null() || (*strm).state.is_null() {
        return -(2 as std::os::raw::c_int)
    }
    state = (*strm).state as *mut inflate_state;
    /* Reset the state */
    (*strm).msg = 0 as *mut std::os::raw::c_char;
    (*state).mode = TYPE;
    (*state).last = 0 as std::os::raw::c_int;
    (*state).whave = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    next = (*strm).next_in;
    have =
        if !next.is_null() {
            (*strm).avail_in
        } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
    hold = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    bits = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    put = (*state).window;
    left = (*state).wsize;
    's_95:
        loop 
             /* Inflate until end of block marked as last */
             {
            match (*state).mode as std::os::raw::c_uint {
                16191 => {
                    /* determine and dispatch block type */
                    if (*state).last != 0 {
                        hold >>= bits & 7 as std::os::raw::c_int as std::os::raw::c_uint;
                        bits =
                            bits.wrapping_sub(bits &
                                                  7 as std::os::raw::c_int as
                                                      std::os::raw::c_uint);
                        (*state).mode = DONE;
                        continue ;
                    } else {
                        while bits < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                have =
                                    in_0.expect("non-null function pointer")(in_desc,
                                                                             &mut next);
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    next = 0 as *mut std::os::raw::c_uchar;
                                    ret = -(5 as std::os::raw::c_int);
                                    break 's_95 ;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh0 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh0 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        (*state).last =
                            (hold as std::os::raw::c_uint &
                                 ((1 as std::os::raw::c_uint) <<
                                      1 as
                                          std::os::raw::c_int).wrapping_sub(1 as
                                                                        std::os::raw::c_int
                                                                        as
                                                                        std::os::raw::c_uint))
                                as std::os::raw::c_int;
                        hold >>= 1 as std::os::raw::c_int;
                        bits =
                            bits.wrapping_sub(1 as std::os::raw::c_int as
                                                  std::os::raw::c_uint);
                        match hold as std::os::raw::c_uint &
                                  ((1 as std::os::raw::c_uint) <<
                                       2 as
                                           std::os::raw::c_int).wrapping_sub(1 as
                                                                         std::os::raw::c_int
                                                                         as
                                                                         std::os::raw::c_uint)
                            {
                            0 => {
                                /* stored block */
                                (*state).mode = STORED
                            }
                            1 => {
                                /* fixed block */
                                fixedtables(state); /* decode codes */
                                (*state).mode = LEN
                            }
                            2 => {
                                /* dynamic block */
                                (*state).mode = TABLE
                            }
                            3 => {
                                (*strm).msg =
                                    b"invalid block type\x00" as *const u8 as
                                        *const std::os::raw::c_char as
                                        *mut std::os::raw::c_char; /* go to byte boundary */
                                (*state).mode = BAD
                            }
                            _ => { }
                        }
                        hold >>= 2 as std::os::raw::c_int;
                        bits =
                            bits.wrapping_sub(2 as std::os::raw::c_int as
                                                  std::os::raw::c_uint);
                        continue ;
                    }
                }
                16193 => {
                    /* get and verify stored block length */
                    hold >>= bits & 7 as std::os::raw::c_int as std::os::raw::c_uint;
                    bits =
                        bits.wrapping_sub(bits &
                                              7 as std::os::raw::c_int as
                                                  std::os::raw::c_uint);
                    while bits < 32 as std::os::raw::c_int as std::os::raw::c_uint {
                        if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            have =
                                in_0.expect("non-null function pointer")(in_desc,
                                                                         &mut next);
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                next = 0 as *mut std::os::raw::c_uchar;
                                ret = -(5 as std::os::raw::c_int);
                                break 's_95 ;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh1 = next;
                        next = next.offset(1);
                        hold =
                            hold.wrapping_add((*fresh1 as std::os::raw::c_ulong) <<
                                                  bits);
                        bits =
                            bits.wrapping_add(8 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    if hold & 0xffff as std::os::raw::c_int as std::os::raw::c_ulong !=
                           hold >> 16 as std::os::raw::c_int ^
                               0xffff as std::os::raw::c_int as std::os::raw::c_ulong {
                        (*strm).msg =
                            b"invalid stored block lengths\x00" as *const u8
                                as *const std::os::raw::c_char as *mut std::os::raw::c_char;
                        (*state).mode = BAD;
                        continue ;
                    } else {
                        (*state).length =
                            hold as std::os::raw::c_uint &
                                0xffff as std::os::raw::c_int as std::os::raw::c_uint;
                        hold = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
                        bits = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        /* copy stored block from input to output */
                        while (*state).length !=
                                  0 as std::os::raw::c_int as std::os::raw::c_uint {
                            copy = (*state).length;
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                have =
                                    in_0.expect("non-null function pointer")(in_desc,
                                                                             &mut next);
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    next = 0 as *mut std::os::raw::c_uchar;
                                    ret = -(5 as std::os::raw::c_int);
                                    break 's_95 ;
                                }
                            }
                            if left == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                put = (*state).window;
                                left = (*state).wsize;
                                (*state).whave = left;
                                if out.expect("non-null function pointer")(out_desc,
                                                                           put,
                                                                           left)
                                       != 0 {
                                    ret = -(5 as std::os::raw::c_int);
                                    break 's_95 ;
                                }
                            }
                            if copy > have { copy = have }
                            if copy > left { copy = left }
                            memcpy(put as *mut std::os::raw::c_void,
                                   next as *const std::os::raw::c_void,
                                   copy as std::os::raw::c_ulong);
                            have = have.wrapping_sub(copy);
                            next = next.offset(copy as isize);
                            left = left.wrapping_sub(copy);
                            put = put.offset(copy as isize);
                            (*state).length =
                                (*state).length.wrapping_sub(copy)
                        }
                        (*state).mode = TYPE;
                        continue ;
                    }
                }
                16196 => {
                    /* get dynamic table entries descriptor */
                    while bits < 14 as std::os::raw::c_int as std::os::raw::c_uint {
                        if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            have =
                                in_0.expect("non-null function pointer")(in_desc,
                                                                         &mut next);
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                next = 0 as *mut std::os::raw::c_uchar;
                                ret = -(5 as std::os::raw::c_int);
                                break 's_95 ;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh2 = next;
                        next = next.offset(1);
                        hold =
                            hold.wrapping_add((*fresh2 as std::os::raw::c_ulong) <<
                                                  bits);
                        bits =
                            bits.wrapping_add(8 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    (*state).nlen =
                        (hold as std::os::raw::c_uint &
                             ((1 as std::os::raw::c_uint) <<
                                  5 as
                                      std::os::raw::c_int).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint)).wrapping_add(257
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint);
                    hold >>= 5 as std::os::raw::c_int;
                    bits =
                        bits.wrapping_sub(5 as std::os::raw::c_int as std::os::raw::c_uint);
                    (*state).ndist =
                        (hold as std::os::raw::c_uint &
                             ((1 as std::os::raw::c_uint) <<
                                  5 as
                                      std::os::raw::c_int).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint)).wrapping_add(1
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint);
                    hold >>= 5 as std::os::raw::c_int;
                    bits =
                        bits.wrapping_sub(5 as std::os::raw::c_int as std::os::raw::c_uint);
                    (*state).ncode =
                        (hold as std::os::raw::c_uint &
                             ((1 as std::os::raw::c_uint) <<
                                  4 as
                                      std::os::raw::c_int).wrapping_sub(1 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint)).wrapping_add(4
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint);
                    hold >>= 4 as std::os::raw::c_int;
                    bits =
                        bits.wrapping_sub(4 as std::os::raw::c_int as std::os::raw::c_uint);
                    if (*state).nlen > 286 as std::os::raw::c_int as std::os::raw::c_uint ||
                           (*state).ndist > 30 as std::os::raw::c_int as std::os::raw::c_uint
                       {
                        (*strm).msg =
                            b"too many length or distance symbols\x00" as
                                *const u8 as *const std::os::raw::c_char as
                                *mut std::os::raw::c_char;
                        (*state).mode = BAD;
                        continue ;
                    } else {
                        /* get code length code lengths (not a typo) */
                        (*state).have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                        while (*state).have < (*state).ncode {
                            while bits < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    have =
                                        in_0.expect("non-null function pointer")(in_desc,
                                                                                 &mut next);
                                    if have ==
                                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                                        next = 0 as *mut std::os::raw::c_uchar;
                                        ret = -(5 as std::os::raw::c_int);
                                        break 's_95 ;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let fresh3 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh3 as
                                                           std::os::raw::c_ulong) <<
                                                          bits);
                                bits =
                                    bits.wrapping_add(8 as std::os::raw::c_int as
                                                          std::os::raw::c_uint)
                            }
                            let fresh4 = (*state).have;
                            (*state).have = (*state).have.wrapping_add(1);
                            (*state).lens[order[fresh4 as usize] as usize] =
                                (hold as std::os::raw::c_uint &
                                     ((1 as std::os::raw::c_uint) <<
                                          3 as
                                              std::os::raw::c_int).wrapping_sub(1 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_uint))
                                    as std::os::raw::c_ushort;
                            hold >>= 3 as std::os::raw::c_int;
                            bits =
                                bits.wrapping_sub(3 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        while (*state).have <
                                  19 as std::os::raw::c_int as std::os::raw::c_uint {
                            let fresh5 = (*state).have;
                            (*state).have = (*state).have.wrapping_add(1);
                            (*state).lens[order[fresh5 as usize] as usize] =
                                0 as std::os::raw::c_int as std::os::raw::c_ushort
                        }
                        (*state).next = (*state).codes.as_mut_ptr();
                        (*state).lencode = (*state).next as *const code;
                        (*state).lenbits = 7 as std::os::raw::c_int as std::os::raw::c_uint;
                        ret =
                            inflate_table(CODES, (*state).lens.as_mut_ptr(),
                                          19 as std::os::raw::c_int as std::os::raw::c_uint,
                                          &mut (*state).next,
                                          &mut (*state).lenbits,
                                          (*state).work.as_mut_ptr());
                        if ret != 0 {
                            (*strm).msg =
                                b"invalid code lengths set\x00" as *const u8
                                    as *const std::os::raw::c_char as
                                    *mut std::os::raw::c_char;
                            (*state).mode = BAD;
                            continue ;
                        } else {
                            /* get length and distance code code lengths */
                            (*state).have = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                            while (*state).have <
                                      (*state).nlen.wrapping_add((*state).ndist)
                                  {
                                loop  {
                                    here =
                                        *(*state).lencode.offset((hold as
                                                                      std::os::raw::c_uint
                                                                      &
                                                                      ((1 as
                                                                            std::os::raw::c_uint)
                                                                           <<
                                                                           (*state).lenbits).wrapping_sub(1
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              std::os::raw::c_uint))
                                                                     as
                                                                     isize);
                                    if here.bits as std::os::raw::c_uint <= bits {
                                        break ;
                                    }
                                    if have ==
                                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                                        have =
                                            in_0.expect("non-null function pointer")(in_desc,
                                                                                     &mut next);
                                        if have ==
                                               0 as std::os::raw::c_int as
                                                   std::os::raw::c_uint {
                                            next = 0 as *mut std::os::raw::c_uchar;
                                            ret = -(5 as std::os::raw::c_int);
                                            break 's_95 ;
                                        }
                                    }
                                    have = have.wrapping_sub(1);
                                    let fresh6 = next;
                                    next = next.offset(1);
                                    hold =
                                        hold.wrapping_add((*fresh6 as
                                                               std::os::raw::c_ulong)
                                                              << bits);
                                    bits =
                                        bits.wrapping_add(8 as std::os::raw::c_int as
                                                              std::os::raw::c_uint)
                                }
                                if (here.val as std::os::raw::c_int) <
                                       16 as std::os::raw::c_int {
                                    hold >>= here.bits as std::os::raw::c_int;
                                    bits =
                                        bits.wrapping_sub(here.bits as
                                                              std::os::raw::c_uint);
                                    let fresh7 = (*state).have;
                                    (*state).have =
                                        (*state).have.wrapping_add(1);
                                    (*state).lens[fresh7 as usize] = here.val
                                } else {
                                    if here.val as std::os::raw::c_int ==
                                           16 as std::os::raw::c_int {
                                        while bits <
                                                  (here.bits as std::os::raw::c_int +
                                                       2 as std::os::raw::c_int) as
                                                      std::os::raw::c_uint {
                                            if have ==
                                                   0 as std::os::raw::c_int as
                                                       std::os::raw::c_uint {
                                                have =
                                                    in_0.expect("non-null function pointer")(in_desc,
                                                                                             &mut next);
                                                if have ==
                                                       0 as std::os::raw::c_int as
                                                           std::os::raw::c_uint {
                                                    next =
                                                        0 as
                                                            *mut std::os::raw::c_uchar;
                                                    ret = -(5 as std::os::raw::c_int);
                                                    break 's_95 ;
                                                }
                                            }
                                            have = have.wrapping_sub(1);
                                            let fresh8 = next;
                                            next = next.offset(1);
                                            hold =
                                                hold.wrapping_add((*fresh8 as
                                                                       std::os::raw::c_ulong)
                                                                      <<
                                                                      bits);
                                            bits =
                                                bits.wrapping_add(8 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_uint)
                                        }
                                        hold >>= here.bits as std::os::raw::c_int;
                                        bits =
                                            bits.wrapping_sub(here.bits as
                                                                  std::os::raw::c_uint);
                                        if (*state).have ==
                                               0 as std::os::raw::c_int as
                                                   std::os::raw::c_uint {
                                            (*strm).msg =
                                                b"invalid bit length repeat\x00"
                                                    as *const u8 as
                                                    *const std::os::raw::c_char as
                                                    *mut std::os::raw::c_char;
                                            (*state).mode = BAD;
                                            break ;
                                        } else {
                                            len =
                                                (*state).lens[(*state).have.wrapping_sub(1
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             std::os::raw::c_uint)
                                                                  as usize] as
                                                    std::os::raw::c_uint;
                                            copy =
                                                (3 as std::os::raw::c_int as
                                                     std::os::raw::c_uint).wrapping_add(hold
                                                                                    as
                                                                                    std::os::raw::c_uint
                                                                                    &
                                                                                    ((1
                                                                                          as
                                                                                          std::os::raw::c_uint)
                                                                                         <<
                                                                                         2
                                                                                             as
                                                                                             std::os::raw::c_int).wrapping_sub(1
                                                                                                                           as
                                                                                                                           std::os::raw::c_int
                                                                                                                           as
                                                                                                                           std::os::raw::c_uint));
                                            hold >>= 2 as std::os::raw::c_int;
                                            bits =
                                                bits.wrapping_sub(2 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_uint)
                                        }
                                    } else if here.val as std::os::raw::c_int ==
                                                  17 as std::os::raw::c_int {
                                        while bits <
                                                  (here.bits as std::os::raw::c_int +
                                                       3 as std::os::raw::c_int) as
                                                      std::os::raw::c_uint {
                                            if have ==
                                                   0 as std::os::raw::c_int as
                                                       std::os::raw::c_uint {
                                                have =
                                                    in_0.expect("non-null function pointer")(in_desc,
                                                                                             &mut next);
                                                if have ==
                                                       0 as std::os::raw::c_int as
                                                           std::os::raw::c_uint {
                                                    next =
                                                        0 as
                                                            *mut std::os::raw::c_uchar;
                                                    ret = -(5 as std::os::raw::c_int);
                                                    break 's_95 ;
                                                }
                                            }
                                            have = have.wrapping_sub(1);
                                            let fresh9 = next;
                                            next = next.offset(1);
                                            hold =
                                                hold.wrapping_add((*fresh9 as
                                                                       std::os::raw::c_ulong)
                                                                      <<
                                                                      bits);
                                            bits =
                                                bits.wrapping_add(8 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_uint)
                                        }
                                        hold >>= here.bits as std::os::raw::c_int;
                                        bits =
                                            bits.wrapping_sub(here.bits as
                                                                  std::os::raw::c_uint);
                                        len =
                                            0 as std::os::raw::c_int as std::os::raw::c_uint;
                                        copy =
                                            (3 as std::os::raw::c_int as
                                                 std::os::raw::c_uint).wrapping_add(hold
                                                                                as
                                                                                std::os::raw::c_uint
                                                                                &
                                                                                ((1
                                                                                      as
                                                                                      std::os::raw::c_uint)
                                                                                     <<
                                                                                     3
                                                                                         as
                                                                                         std::os::raw::c_int).wrapping_sub(1
                                                                                                                       as
                                                                                                                       std::os::raw::c_int
                                                                                                                       as
                                                                                                                       std::os::raw::c_uint));
                                        hold >>= 3 as std::os::raw::c_int;
                                        bits =
                                            bits.wrapping_sub(3 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_uint)
                                    } else {
                                        while bits <
                                                  (here.bits as std::os::raw::c_int +
                                                       7 as std::os::raw::c_int) as
                                                      std::os::raw::c_uint {
                                            if have ==
                                                   0 as std::os::raw::c_int as
                                                       std::os::raw::c_uint {
                                                have =
                                                    in_0.expect("non-null function pointer")(in_desc,
                                                                                             &mut next);
                                                if have ==
                                                       0 as std::os::raw::c_int as
                                                           std::os::raw::c_uint {
                                                    next =
                                                        0 as
                                                            *mut std::os::raw::c_uchar;
                                                    ret = -(5 as std::os::raw::c_int);
                                                    break 's_95 ;
                                                }
                                            }
                                            have = have.wrapping_sub(1);
                                            let fresh10 = next;
                                            next = next.offset(1);
                                            hold =
                                                hold.wrapping_add((*fresh10 as
                                                                       std::os::raw::c_ulong)
                                                                      <<
                                                                      bits);
                                            bits =
                                                bits.wrapping_add(8 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_uint)
                                        }
                                        hold >>= here.bits as std::os::raw::c_int;
                                        bits =
                                            bits.wrapping_sub(here.bits as
                                                                  std::os::raw::c_uint);
                                        len =
                                            0 as std::os::raw::c_int as std::os::raw::c_uint;
                                        copy =
                                            (11 as std::os::raw::c_int as
                                                 std::os::raw::c_uint).wrapping_add(hold
                                                                                as
                                                                                std::os::raw::c_uint
                                                                                &
                                                                                ((1
                                                                                      as
                                                                                      std::os::raw::c_uint)
                                                                                     <<
                                                                                     7
                                                                                         as
                                                                                         std::os::raw::c_int).wrapping_sub(1
                                                                                                                       as
                                                                                                                       std::os::raw::c_int
                                                                                                                       as
                                                                                                                       std::os::raw::c_uint));
                                        hold >>= 7 as std::os::raw::c_int;
                                        bits =
                                            bits.wrapping_sub(7 as std::os::raw::c_int
                                                                  as
                                                                  std::os::raw::c_uint)
                                    }
                                    if (*state).have.wrapping_add(copy) >
                                           (*state).nlen.wrapping_add((*state).ndist)
                                       {
                                        (*strm).msg =
                                            b"invalid bit length repeat\x00"
                                                as *const u8 as
                                                *const std::os::raw::c_char as
                                                *mut std::os::raw::c_char;
                                        (*state).mode = BAD;
                                        break ;
                                    } else {
                                        loop  {
                                            let fresh11 = copy;
                                            copy = copy.wrapping_sub(1);
                                            if !(fresh11 != 0) { break ; }
                                            let fresh12 = (*state).have;
                                            (*state).have =
                                                (*state).have.wrapping_add(1);
                                            (*state).lens[fresh12 as usize] =
                                                len as std::os::raw::c_ushort
                                        }
                                    }
                                }
                            }
                            /* handle error breaks in while */
                            if (*state).mode as std::os::raw::c_uint ==
                                   BAD as std::os::raw::c_int as std::os::raw::c_uint {
                                continue ;
                            }
                            /* check for end-of-block code (better have one) */
                            if (*state).lens[256 as std::os::raw::c_int as usize] as
                                   std::os::raw::c_int == 0 as std::os::raw::c_int {
                                (*strm).msg =
                                    b"invalid code -- missing end-of-block\x00"
                                        as *const u8 as *const std::os::raw::c_char as
                                        *mut std::os::raw::c_char;
                                (*state).mode = BAD;
                                continue ;
                            } else {
                                /* build code tables -- note: do not change the lenbits or distbits
               values here (9 and 6) without reading the comments in inftrees.h
               concerning the ENOUGH constants, which depend on those values */
                                (*state).next = (*state).codes.as_mut_ptr();
                                (*state).lencode =
                                    (*state).next as *const code;
                                (*state).lenbits =
                                    9 as std::os::raw::c_int as std::os::raw::c_uint;
                                ret =
                                    inflate_table(LENS,
                                                  (*state).lens.as_mut_ptr(),
                                                  (*state).nlen,
                                                  &mut (*state).next,
                                                  &mut (*state).lenbits,
                                                  (*state).work.as_mut_ptr());
                                if ret != 0 {
                                    (*strm).msg =
                                        b"invalid literal/lengths set\x00" as
                                            *const u8 as *const std::os::raw::c_char
                                            as *mut std::os::raw::c_char;
                                    (*state).mode = BAD;
                                    continue ;
                                } else {
                                    (*state).distcode =
                                        (*state).next as *const code;
                                    (*state).distbits =
                                        6 as std::os::raw::c_int as std::os::raw::c_uint;
                                    ret =
                                        inflate_table(DISTS,
                                                      (*state).lens.as_mut_ptr().offset((*state).nlen
                                                                                            as
                                                                                            isize),
                                                      (*state).ndist,
                                                      &mut (*state).next,
                                                      &mut (*state).distbits,
                                                      (*state).work.as_mut_ptr());
                                    if ret != 0 {
                                        (*strm).msg =
                                            b"invalid distances set\x00" as
                                                *const u8 as
                                                *const std::os::raw::c_char as
                                                *mut std::os::raw::c_char;
                                        (*state).mode = BAD;
                                        continue ;
                                    } else { (*state).mode = LEN }
                                }
                            }
                        }
                    }
                }
                16200 => { }
                16208 => {
                    /* inflate stream terminated properly -- write leftover output */
                    ret = 1 as std::os::raw::c_int;
                    if left < (*state).wsize {
                        if out.expect("non-null function pointer")(out_desc,
                                                                   (*state).window,
                                                                   (*state).wsize.wrapping_sub(left))
                               != 0 {
                            ret = -(5 as std::os::raw::c_int)
                        }
                    }
                    break ;
                }
                16209 => { ret = -(3 as std::os::raw::c_int); break ; }
                _ => {
                    /* can't happen, but makes compilers happy */
                    ret = -(2 as std::os::raw::c_int);
                    break ;
                }
            }
            /* use inflate_fast() if we have enough input and output */
            if have >= 6 as std::os::raw::c_int as std::os::raw::c_uint &&
                   left >= 258 as std::os::raw::c_int as std::os::raw::c_uint {
                (*strm).next_out = put;
                (*strm).avail_out = left;
                (*strm).next_in = next;
                (*strm).avail_in = have;
                (*state).hold = hold;
                (*state).bits = bits;
                if (*state).whave < (*state).wsize {
                    (*state).whave = (*state).wsize.wrapping_sub(left)
                }
                inflate_fast(strm, (*state).wsize);
                put = (*strm).next_out;
                left = (*strm).avail_out;
                next = (*strm).next_in;
                have = (*strm).avail_in;
                hold = (*state).hold;
                bits = (*state).bits
            } else {
                loop 
                     /* get a literal, length, or end-of-block code */
                     {
                    here =
                        *(*state).lencode.offset((hold as std::os::raw::c_uint &
                                                      ((1 as std::os::raw::c_uint) <<
                                                           (*state).lenbits).wrapping_sub(1
                                                                                              as
                                                                                              std::os::raw::c_int
                                                                                              as
                                                                                              std::os::raw::c_uint))
                                                     as isize);
                    if here.bits as std::os::raw::c_uint <= bits { break ; }
                    if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        have =
                            in_0.expect("non-null function pointer")(in_desc,
                                                                     &mut next);
                        if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            next = 0 as *mut std::os::raw::c_uchar;
                            ret = -(5 as std::os::raw::c_int);
                            break 's_95 ;
                        }
                    }
                    have = have.wrapping_sub(1);
                    let fresh13 = next;
                    next = next.offset(1);
                    hold =
                        hold.wrapping_add((*fresh13 as std::os::raw::c_ulong) <<
                                              bits);
                    bits = bits.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_uint)
                }
                if here.op as std::os::raw::c_int != 0 &&
                       here.op as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                           0 as std::os::raw::c_int {
                    last = here;
                    loop  {
                        here =
                            *(*state).lencode.offset((last.val as
                                                          std::os::raw::c_uint).wrapping_add((hold
                                                                                          as
                                                                                          std::os::raw::c_uint
                                                                                          &
                                                                                          ((1
                                                                                                as
                                                                                                std::os::raw::c_uint)
                                                                                               <<
                                                                                               last.bits
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   +
                                                                                                   last.op
                                                                                                       as
                                                                                                       std::os::raw::c_int).wrapping_sub(1
                                                                                                                                     as
                                                                                                                                     std::os::raw::c_int
                                                                                                                                     as
                                                                                                                                     std::os::raw::c_uint))
                                                                                         >>
                                                                                         last.bits
                                                                                             as
                                                                                             std::os::raw::c_int)
                                                         as isize);
                        if (last.bits as std::os::raw::c_int +
                                here.bits as std::os::raw::c_int) as std::os::raw::c_uint <=
                               bits {
                            break ;
                        }
                        if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            have =
                                in_0.expect("non-null function pointer")(in_desc,
                                                                         &mut next);
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                next = 0 as *mut std::os::raw::c_uchar;
                                ret = -(5 as std::os::raw::c_int);
                                break 's_95 ;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh14 = next;
                        next = next.offset(1);
                        hold =
                            hold.wrapping_add((*fresh14 as std::os::raw::c_ulong) <<
                                                  bits);
                        bits =
                            bits.wrapping_add(8 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    hold >>= last.bits as std::os::raw::c_int;
                    bits = bits.wrapping_sub(last.bits as std::os::raw::c_uint)
                }
                hold >>= here.bits as std::os::raw::c_int;
                bits = bits.wrapping_sub(here.bits as std::os::raw::c_uint);
                (*state).length = here.val as std::os::raw::c_uint;
                /* process literal */
                if here.op as std::os::raw::c_int == 0 as std::os::raw::c_int {
                    if left == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        put = (*state).window;
                        left = (*state).wsize;
                        (*state).whave = left;
                        if out.expect("non-null function pointer")(out_desc,
                                                                   put, left)
                               != 0 {
                            ret = -(5 as std::os::raw::c_int);
                            break ;
                        }
                    }
                    let fresh15 = put;
                    put = put.offset(1);
                    *fresh15 = (*state).length as std::os::raw::c_uchar;
                    left = left.wrapping_sub(1);
                    (*state).mode = LEN
                } else if here.op as std::os::raw::c_int & 32 as std::os::raw::c_int != 0 {
                    (*state).mode = TYPE
                } else if here.op as std::os::raw::c_int & 64 as std::os::raw::c_int != 0 {
                    (*strm).msg =
                        b"invalid literal/length code\x00" as *const u8 as
                            *const std::os::raw::c_char as *mut std::os::raw::c_char;
                    (*state).mode = BAD
                } else {
                    /* process end of block */
                    /* invalid code */
                    /* length code -- get extra bits, if any */
                    (*state).extra =
                        here.op as std::os::raw::c_uint &
                            15 as std::os::raw::c_int as std::os::raw::c_uint;
                    if (*state).extra != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                        while bits < (*state).extra {
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                have =
                                    in_0.expect("non-null function pointer")(in_desc,
                                                                             &mut next);
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    next = 0 as *mut std::os::raw::c_uchar;
                                    ret = -(5 as std::os::raw::c_int);
                                    break 's_95 ;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh16 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh16 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        (*state).length =
                            (*state).length.wrapping_add(hold as std::os::raw::c_uint
                                                             &
                                                             ((1 as
                                                                   std::os::raw::c_uint)
                                                                  <<
                                                                  (*state).extra).wrapping_sub(1
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   std::os::raw::c_uint));
                        hold >>= (*state).extra;
                        bits = bits.wrapping_sub((*state).extra)
                    }
                    loop 
                         /* get distance code */
                         {
                        here =
                            *(*state).distcode.offset((hold as std::os::raw::c_uint &
                                                           ((1 as
                                                                 std::os::raw::c_uint)
                                                                <<
                                                                (*state).distbits).wrapping_sub(1
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint))
                                                          as isize);
                        if here.bits as std::os::raw::c_uint <= bits { break ; }
                        if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                            have =
                                in_0.expect("non-null function pointer")(in_desc,
                                                                         &mut next);
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                next = 0 as *mut std::os::raw::c_uchar;
                                ret = -(5 as std::os::raw::c_int);
                                break 's_95 ;
                            }
                        }
                        have = have.wrapping_sub(1);
                        let fresh17 = next;
                        next = next.offset(1);
                        hold =
                            hold.wrapping_add((*fresh17 as std::os::raw::c_ulong) <<
                                                  bits);
                        bits =
                            bits.wrapping_add(8 as std::os::raw::c_int as
                                                  std::os::raw::c_uint)
                    }
                    if here.op as std::os::raw::c_int & 0xf0 as std::os::raw::c_int ==
                           0 as std::os::raw::c_int {
                        last = here;
                        loop  {
                            here =
                                *(*state).distcode.offset((last.val as
                                                               std::os::raw::c_uint).wrapping_add((hold
                                                                                               as
                                                                                               std::os::raw::c_uint
                                                                                               &
                                                                                               ((1
                                                                                                     as
                                                                                                     std::os::raw::c_uint)
                                                                                                    <<
                                                                                                    last.bits
                                                                                                        as
                                                                                                        std::os::raw::c_int
                                                                                                        +
                                                                                                        last.op
                                                                                                            as
                                                                                                            std::os::raw::c_int).wrapping_sub(1
                                                                                                                                          as
                                                                                                                                          std::os::raw::c_int
                                                                                                                                          as
                                                                                                                                          std::os::raw::c_uint))
                                                                                              >>
                                                                                              last.bits
                                                                                                  as
                                                                                                  std::os::raw::c_int)
                                                              as isize);
                            if (last.bits as std::os::raw::c_int +
                                    here.bits as std::os::raw::c_int) as std::os::raw::c_uint
                                   <= bits {
                                break ;
                            }
                            if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                have =
                                    in_0.expect("non-null function pointer")(in_desc,
                                                                             &mut next);
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    next = 0 as *mut std::os::raw::c_uchar;
                                    ret = -(5 as std::os::raw::c_int);
                                    break 's_95 ;
                                }
                            }
                            have = have.wrapping_sub(1);
                            let fresh18 = next;
                            next = next.offset(1);
                            hold =
                                hold.wrapping_add((*fresh18 as std::os::raw::c_ulong)
                                                      << bits);
                            bits =
                                bits.wrapping_add(8 as std::os::raw::c_int as
                                                      std::os::raw::c_uint)
                        }
                        hold >>= last.bits as std::os::raw::c_int;
                        bits = bits.wrapping_sub(last.bits as std::os::raw::c_uint)
                    }
                    hold >>= here.bits as std::os::raw::c_int;
                    bits = bits.wrapping_sub(here.bits as std::os::raw::c_uint);
                    if here.op as std::os::raw::c_int & 64 as std::os::raw::c_int != 0 {
                        (*strm).msg =
                            b"invalid distance code\x00" as *const u8 as
                                *const std::os::raw::c_char as *mut std::os::raw::c_char;
                        (*state).mode = BAD
                    } else {
                        (*state).offset = here.val as std::os::raw::c_uint;
                        /* get distance extra bits, if any */
                        (*state).extra =
                            here.op as std::os::raw::c_uint &
                                15 as std::os::raw::c_int as std::os::raw::c_uint;
                        if (*state).extra != 0 as std::os::raw::c_int as std::os::raw::c_uint
                           {
                            while bits < (*state).extra {
                                if have == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    have =
                                        in_0.expect("non-null function pointer")(in_desc,
                                                                                 &mut next);
                                    if have ==
                                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                                        next = 0 as *mut std::os::raw::c_uchar;
                                        ret = -(5 as std::os::raw::c_int);
                                        break 's_95 ;
                                    }
                                }
                                have = have.wrapping_sub(1);
                                let fresh19 = next;
                                next = next.offset(1);
                                hold =
                                    hold.wrapping_add((*fresh19 as
                                                           std::os::raw::c_ulong) <<
                                                          bits);
                                bits =
                                    bits.wrapping_add(8 as std::os::raw::c_int as
                                                          std::os::raw::c_uint)
                            }
                            (*state).offset =
                                (*state).offset.wrapping_add(hold as
                                                                 std::os::raw::c_uint
                                                                 &
                                                                 ((1 as
                                                                       std::os::raw::c_uint)
                                                                      <<
                                                                      (*state).extra).wrapping_sub(1
                                                                                                       as
                                                                                                       std::os::raw::c_int
                                                                                                       as
                                                                                                       std::os::raw::c_uint));
                            hold >>= (*state).extra;
                            bits = bits.wrapping_sub((*state).extra)
                        }
                        if (*state).offset >
                               (*state).wsize.wrapping_sub((if (*state).whave
                                                                   <
                                                                   (*state).wsize
                                                               {
                                                                left
                                                            } else {
                                                                0 as
                                                                    std::os::raw::c_int
                                                                    as
                                                                    std::os::raw::c_uint
                                                            })) {
                            (*strm).msg =
                                b"invalid distance too far back\x00" as
                                    *const u8 as *const std::os::raw::c_char as
                                    *mut std::os::raw::c_char;
                            (*state).mode = BAD
                        } else {
                            loop 
                                 /* copy match from window to output */
                                 {
                                if left == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    put = (*state).window;
                                    left = (*state).wsize;
                                    (*state).whave = left;
                                    if out.expect("non-null function pointer")(out_desc,
                                                                               put,
                                                                               left)
                                           != 0 {
                                        ret = -(5 as std::os::raw::c_int);
                                        break 's_95 ;
                                    }
                                }
                                copy =
                                    (*state).wsize.wrapping_sub((*state).offset);
                                if copy < left {
                                    from = put.offset(copy as isize);
                                    copy = left.wrapping_sub(copy)
                                } else {
                                    from =
                                        put.offset(-((*state).offset as
                                                         isize));
                                    copy = left
                                }
                                if copy > (*state).length {
                                    copy = (*state).length
                                }
                                (*state).length =
                                    (*state).length.wrapping_sub(copy);
                                left = left.wrapping_sub(copy);
                                loop  {
                                    let fresh20 = from;
                                    from = from.offset(1);
                                    let fresh21 = put;
                                    put = put.offset(1);
                                    *fresh21 = *fresh20;
                                    copy = copy.wrapping_sub(1);
                                    if !(copy != 0) { break ; }
                                }
                                if !((*state).length !=
                                         0 as std::os::raw::c_int as std::os::raw::c_uint) {
                                    break ;
                                }
                            }
                        }
                    }
                }
            }
        }
    /* Return unused input */
    (*strm).next_in = next;
    (*strm).avail_in = have;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn inflateBackEnd(mut strm: z_streamp) -> std::os::raw::c_int {
    if strm.is_null() || (*strm).state.is_null() || (*strm).zfree.is_none() {
        return -(2 as std::os::raw::c_int)
    }
    Some((*strm).zfree.expect("non-null function pointer")).expect("non-null function pointer")((*strm).opaque,
                                                                                                (*strm).state
                                                                                                    as
                                                                                                    voidpf);
    (*strm).state = 0 as *mut internal_state;
    return 0 as std::os::raw::c_int;
}
