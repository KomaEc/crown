
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn realloc(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn fputc(_: std::os::raw::c_int, _: *mut FILE) -> std::os::raw::c_int;
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *const std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
impl Default for __sbuf {fn default() -> Self {Self {
_base: std::ptr::null_mut(),
_size: Default::default(),
}}}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *const std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *const std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *const __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl Default for __sFILE {fn default() -> Self {Self {
_p: std::ptr::null_mut(),
_r: Default::default(),
_w: Default::default(),
_flags: Default::default(),
_file: Default::default(),
_bf: Default::default(),
_lbfsize: Default::default(),
_cookie: std::ptr::null_mut(),
_close: Default::default(),
_read: Default::default(),
_seek: Default::default(),
_write: Default::default(),
_ub: Default::default(),
_extra: std::ptr::null_mut(),
_ur: Default::default(),
_ubuf: Default::default(),
_nbuf: Default::default(),
_lb: Default::default(),
_blksize: Default::default(),
_offset: Default::default(),
}}}

pub type FILE = __sFILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: std::os::raw::c_int,
    pub quoted: std::os::raw::c_int,
    pub spaces: size_t,
    pub entry_buf: *mut std::os::raw::c_uchar,
    pub entry_pos: size_t,
    pub entry_size: size_t,
    pub status: std::os::raw::c_int,
    pub options: std::os::raw::c_uchar,
    pub quote_char: std::os::raw::c_uchar,
    pub delim_char: std::os::raw::c_uchar,
    pub is_space: Option<unsafe extern "C" fn(_: std::os::raw::c_uchar)
                             -> std::os::raw::c_int>,
    pub is_term: Option<unsafe extern "C" fn(_: std::os::raw::c_uchar)
                            -> std::os::raw::c_int>,
    pub blk_size: size_t,
    pub malloc_func: Option<unsafe extern "C" fn(_: size_t)
                                -> *mut std::os::raw::c_void>,
    pub realloc_func: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                                  _: size_t)
                                 -> *mut std::os::raw::c_void>,
    pub free_func: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ()>,
}
impl Default for csv_parser {fn default() -> Self {Self {
pstate: Default::default(),
quoted: Default::default(),
spaces: Default::default(),
entry_buf: std::ptr::null_mut(),
entry_pos: Default::default(),
entry_size: Default::default(),
status: Default::default(),
options: Default::default(),
quote_char: Default::default(),
delim_char: Default::default(),
is_space: Default::default(),
is_term: Default::default(),
blk_size: Default::default(),
malloc_func: Default::default(),
realloc_func: Default::default(),
free_func: Default::default(),
}}}

static mut csv_errors: [*const std::os::raw::c_char; 5] =
    [b"success\x00" as *const u8 as *const std::os::raw::c_char,
     b"error parsing data while strict checking enabled\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"memory exhausted while increasing buffer size\x00" as *const u8 as
         *const std::os::raw::c_char,
     b"data size too large\x00" as *const u8 as *const std::os::raw::c_char,
     b"invalid status code\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn csv_error(mut p: *const csv_parser) -> std::os::raw::c_int {
    if !(!p.is_null() &&
             !(b"received null csv_parser\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null()) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn(b"csv_error\x00" as *const u8 as *const i8,
                     b"libcsv.c\x00" as *const u8 as *const std::os::raw::c_char,
                     82 as std::os::raw::c_int,
                     b"p && \"received null csv_parser\"\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    /* Return the current status of the parser */
    return (*p).status;
}
#[no_mangle]
pub unsafe extern "C" fn csv_strerror(mut status: std::os::raw::c_int)
 -> *const std::os::raw::c_char {
    /* Return a textual description of status */
    if status >= 4 as std::os::raw::c_int || status < 0 as std::os::raw::c_int {
        return csv_errors[4 as std::os::raw::c_int as usize]
    } else { return csv_errors[status as usize] };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_opts(mut p: *const csv_parser)
 -> std::os::raw::c_int {
    /* Return the currently set options of parser */
    if p.is_null() {(); return -(1 as std::os::raw::c_int) }
    return (*p).options as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_opts(mut p: Option<&mut csv_parser>,
                                      mut options: std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    /* Set the options */
    if p.as_deref().is_none() {(); return -(1 as std::os::raw::c_int) }
    (*p.as_deref_mut().unwrap()).options= options;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_init(mut p: Option<&mut csv_parser>,
                                  mut options: std::os::raw::c_uchar) -> std::os::raw::c_int {
    /* Initialize a csv_parser object returns 0 on success, -1 on error */
    if p.as_deref().is_none() {(); return -(1 as std::os::raw::c_int) }
    (*p.as_deref_mut().unwrap()).entry_buf= 0 as *mut std::os::raw::c_uchar;
    (*p.as_deref_mut().unwrap()).pstate= 0 as std::os::raw::c_int;
    (*p.as_deref_mut().unwrap()).quoted= 0 as std::os::raw::c_int;
    (*p.as_deref_mut().unwrap()).spaces= 0 as std::os::raw::c_int as size_t;
    (*p.as_deref_mut().unwrap()).entry_pos= 0 as std::os::raw::c_int as size_t;
    (*p.as_deref_mut().unwrap()).entry_size= 0 as std::os::raw::c_int as size_t;
    (*p.as_deref_mut().unwrap()).status= 0 as std::os::raw::c_int;
    (*p.as_deref_mut().unwrap()).options= options;
    (*p.as_deref_mut().unwrap()).quote_char= 0x22 as std::os::raw::c_int as std::os::raw::c_uchar;
    (*p.as_deref_mut().unwrap()).delim_char= 0x2c as std::os::raw::c_int as std::os::raw::c_uchar;
    (*p.as_deref_mut().unwrap()).is_space= None;
    (*p.as_deref_mut().unwrap()).is_term= None;
    (*p.as_deref_mut().unwrap()).blk_size= 128 as std::os::raw::c_int as size_t;
    (*p.as_deref_mut().unwrap()).malloc_func= None;
    (*p.as_deref_mut().unwrap()).realloc_func=
        Some(realloc as
                 unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: std::os::raw::c_ulong)
                     -> *mut std::os::raw::c_void);
    (*p.as_deref_mut().unwrap()).free_func=
        Some(free as unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> ());
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_free(mut p: *mut csv_parser) {
    /* Free the entry_buffer of csv_parser object */
    if p.is_null() {(); return }
    if !(*p).entry_buf.is_null() && (*p).free_func.is_some() {
        (*p).free_func.expect("non-null function pointer")((*p).entry_buf as
                                                               *mut std::os::raw::c_void);
    }
    (*p).entry_buf= 0 as *mut std::os::raw::c_uchar;
    (*p).entry_size= 0 as std::os::raw::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fini(mut p: Option<&mut csv_parser>,
                                  mut cb1:
                                      Option<unsafe extern "C" fn(_:
                                                                      *mut std::os::raw::c_void,
                                                                  _: size_t,
                                                                  _:
                                                                      *mut std::os::raw::c_void)
                                                 -> ()>,
                                  mut cb2:
                                      Option<unsafe extern "C" fn(_:
                                                                      std::os::raw::c_int,
                                                                  _:
                                                                      *mut std::os::raw::c_void)
                                                 -> ()>,
                                  mut data: *mut std::os::raw::c_void)
 -> std::os::raw::c_int {
    if p.as_deref().is_none() {(); return -(1 as std::os::raw::c_int) }
    /* Finalize parsing.  Needed, for example, when file does not end in a newline */
    let mut quoted: std::os::raw::c_int = (*p.as_deref().unwrap()).quoted;
    let mut pstate: std::os::raw::c_int = (*p.as_deref().unwrap()).pstate;
    let mut spaces: size_t = (*p.as_deref().unwrap()).spaces;
    let mut entry_pos: size_t = (*p.as_deref().unwrap()).entry_pos;
    if pstate == 2 as std::os::raw::c_int && (*p.as_deref().unwrap()).quoted != 0 &&
           (*p.as_deref().unwrap()).options as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 &&
           (*p.as_deref().unwrap()).options as std::os::raw::c_int & 4 as std::os::raw::c_int != 0 {
        /* Current field is quoted, no end-quote was seen, and CSV_STRICT_FINI is set */
        (*p.as_deref_mut().unwrap()).status=
            1 as std::os::raw::c_int; /* get rid of spaces and original quote */
        return -(1 as std::os::raw::c_int)
    }
    let mut current_block_26: u64;
    match pstate {
        3 => {
            (*p.as_deref_mut().unwrap()).entry_pos=
                ((*p.as_deref().unwrap()).entry_pos as
                     std::os::raw::c_ulong).wrapping_sub((*p.as_deref().unwrap()).spaces.wrapping_add(1 as
                                                                              std::os::raw::c_int
                                                                              as
                                                                              std::os::raw::c_ulong))
                    as size_t as size_t;
            entry_pos= (*p.as_deref().unwrap()).entry_pos;
            current_block_26= 4830382299733546467;
        }
        1 | 2 => { current_block_26= 4830382299733546467; }
        0 | _ => { current_block_26= 4761528863920922185; }
    }
    match current_block_26 {
        4830382299733546467 =>
        /*lint -fallthrough */
        /* Unnecessary:
      quoted = p->quoted, pstate = p->pstate;
      spaces = p->spaces, entry_pos = p->entry_pos;
      */
        {
            if quoted == 0 {
                entry_pos=
                    (entry_pos as std::os::raw::c_ulong).wrapping_sub(spaces) as
                        size_t as size_t
            }
            if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                    '\u{0}' as i32 as std::os::raw::c_uchar
            }
            if cb1.is_some() &&
                   (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int != 0 &&
                   quoted == 0 &&
                   entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                cb1.expect("non-null function pointer")(0 as
                                                            *mut std::os::raw::c_void,
                                                        entry_pos, data);
            } else if cb1.is_some() {
                cb1.expect("non-null function pointer")((*p).entry_buf as
                                                            *mut std::os::raw::c_void,
                                                        entry_pos, data);
            }
            pstate= 1 as std::os::raw::c_int;
            spaces= 0 as std::os::raw::c_int as size_t;
            quoted= spaces as std::os::raw::c_int;
            entry_pos= quoted as size_t;
            if cb2.is_some() {
                cb2.expect("non-null function pointer")(-(1 as std::os::raw::c_int),
                                                        data);
            }
            pstate= 0 as std::os::raw::c_int;
            spaces= 0 as std::os::raw::c_int as size_t;
            quoted= spaces as std::os::raw::c_int;
            entry_pos= quoted as size_t
        }
        _ => { }
    }
    /* Reset parser */
    (*p.as_deref_mut().unwrap()).status= 0 as std::os::raw::c_int;
    (*p.as_deref_mut().unwrap()).entry_pos= (*p.as_deref().unwrap()).status as size_t;
    (*p.as_deref_mut().unwrap()).quoted= (*p.as_deref().unwrap()).entry_pos as std::os::raw::c_int;
    (*p.as_deref_mut().unwrap()).spaces= (*p.as_deref().unwrap()).quoted as size_t;
    (*p.as_deref_mut().unwrap()).pstate= 0 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_delim(mut p: Option<&mut csv_parser>,
                                       mut c: std::os::raw::c_uchar) {
    /* Set the delimiter */
    if !p.as_deref().is_none() { (*p.as_deref_mut().unwrap()).delim_char= c }else { (); };
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_quote(mut p: Option<&mut csv_parser>,
                                       mut c: std::os::raw::c_uchar) {
    /* Set the quote character */
    if !p.as_deref().is_none() { (*p.as_deref_mut().unwrap()).quote_char= c }else { (); };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_delim(mut p: *const csv_parser)
 -> std::os::raw::c_uchar {
    if !(!p.is_null() &&
             !(b"received null csv_parser\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null()) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn(b"csv_get_delim\x00" as *const u8 as *const i8,
                     b"libcsv.c\x00" as *const u8 as *const std::os::raw::c_char,
                     222 as std::os::raw::c_int,
                     b"p && \"received null csv_parser\"\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    /* Get the delimiter */
    return (*p).delim_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_quote(mut p: *const csv_parser)
 -> std::os::raw::c_uchar {
    if !(!p.is_null() &&
             !(b"received null csv_parser\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null()) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn(b"csv_get_quote\x00" as *const u8 as *const i8,
                     b"libcsv.c\x00" as *const u8 as *const std::os::raw::c_char,
                     231 as std::os::raw::c_int,
                     b"p && \"received null csv_parser\"\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    /* Get the quote character */
    return (*p).quote_char;
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_space_func(mut p: Option<&mut csv_parser>,
                                            mut f:
                                                Option<unsafe extern "C" fn(_:
                                                                                std::os::raw::c_uchar)
                                                           -> std::os::raw::c_int>) {
    /* Set the space function */
    if !p.as_deref().is_none() { (*p.as_deref_mut().unwrap()).is_space= f }else { (); };
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_term_func(mut p: Option<&mut csv_parser>,
                                           mut f:
                                               Option<unsafe extern "C" fn(_:
                                                                               std::os::raw::c_uchar)
                                                          -> std::os::raw::c_int>) {
    /* Set the term function */
    if !p.as_deref().is_none() { (*p.as_deref_mut().unwrap()).is_term= f }else { (); };
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_realloc_func(mut p: Option<&mut csv_parser>,
                                              mut f:
                                                  Option<unsafe extern "C" fn(_:
                                                                                  *mut std::os::raw::c_void,
                                                                              _:
                                                                                  size_t)
                                                             ->
                                                                 *mut std::os::raw::c_void>) {
    /* Set the realloc function used to increase buffer size */
    if !p.as_deref().is_none() && f.is_some() { (*p.as_deref_mut().unwrap()).realloc_func= f };
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_free_func(mut p: Option<&mut csv_parser>,
                                           mut f:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut std::os::raw::c_void)
                                                          -> ()>) {
    /* Set the free function used to free the buffer */
    if !p.as_deref().is_none() && f.is_some() { (*p.as_deref_mut().unwrap()).free_func= f };
}
#[no_mangle]
pub unsafe extern "C" fn csv_set_blk_size(mut p: Option<&mut csv_parser>,
                                          mut size: size_t) {
    /* Set the block size used to increment buffer size */
    if !p.as_deref().is_none() { (*p.as_deref_mut().unwrap()).blk_size= size }else { (); };
}
#[no_mangle]
pub unsafe extern "C" fn csv_get_buffer_size(mut p: *const csv_parser)
 -> size_t {
    /* Get the size of the entry buffer */
    if !p.is_null() { return (*p).entry_size }else { (); }
    return 0 as std::os::raw::c_int as size_t;
}
unsafe extern "C" fn csv_increase_buffer(mut p: *mut csv_parser)
 -> std::os::raw::c_int {
    if p.is_null() {(); return 0 as std::os::raw::c_int }
    if (*p).realloc_func.is_none() { return 0 as std::os::raw::c_int }
    /* Increase the size of the entry buffer.  Attempt to increase size by 
   * p->blk_size, if this is larger than SIZE_MAX try to increase current
   * buffer size to SIZE_MAX.  If allocation fails, try to allocate halve 
   * the size and try again until successful or increment size is zero.
   */
    let mut to_add: size_t = (*p).blk_size;
    let mut vp: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    if (*p).entry_size >=
           (18446744073709551615 as std::os::raw::c_ulong).wrapping_sub(to_add) {
        to_add=
            (18446744073709551615 as
                 std::os::raw::c_ulong).wrapping_sub((*p).entry_size)
    }
    if to_add == 0 {
        (*p).status= 3 as std::os::raw::c_int;
        return -(1 as std::os::raw::c_int)
    }
    loop  {
        vp=
            (*p).realloc_func.expect("non-null function pointer")((*p).entry_buf
                                                                      as
                                                                      *mut std::os::raw::c_void,
                                                                  (*p).entry_size.wrapping_add(to_add));
        if !vp.is_null() { break ; }else { (); }
        to_add=
            (to_add as
                 std::os::raw::c_ulong).wrapping_div(2 as std::os::raw::c_int as
                                                 std::os::raw::c_ulong) as size_t as
                size_t;
        if to_add == 0 {
            (*p).status= 2 as std::os::raw::c_int;
            return -(1 as std::os::raw::c_int)
        }
    }
    /* Update entry buffer pointer and entry_size if successful */
    (*p).entry_buf=
        vp as
            *mut std::os::raw::c_uchar; /* Access input data as array of unsigned char */
    (*p).entry_size=
        ((*p).entry_size as std::os::raw::c_ulong).wrapping_add(to_add) as size_t as
            size_t; /* The character we are currently processing */
    return 0 as
               std::os::raw::c_int; /* The number of characters we have processed in this call */
}
#[no_mangle]
pub unsafe extern "C" fn csv_parse(mut p: Option<&mut csv_parser>,
                                   mut s: *const std::os::raw::c_void,
                                   mut len: size_t,
                                   mut cb1:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut std::os::raw::c_void,
                                                                   _: size_t,
                                                                   _:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()>,
                                   mut cb2:
                                       Option<unsafe extern "C" fn(_:
                                                                       std::os::raw::c_int,
                                                                   _:
                                                                       *mut std::os::raw::c_void)
                                                  -> ()>,
                                   mut data: *mut std::os::raw::c_void) -> size_t {
    if !(!p.as_deref().is_none() &&
             !(b"received null csv_parser\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null()) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn(b"csv_parse\x00" as *const u8 as *const i8,
                     b"libcsv.c\x00" as *const u8 as *const std::os::raw::c_char,
                     321 as std::os::raw::c_int,
                     b"p && \"received null csv_parser\"\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    if s.is_null() {(); return 0 as std::os::raw::c_int as size_t }
    let mut us: *const std::os::raw::c_uchar = s as *const std::os::raw::c_uchar;
    let mut c: std::os::raw::c_uchar = 0;
    let mut pos: size_t = 0 as std::os::raw::c_int as size_t;
    /* Store key fields into local variables for performance */
    let mut delim: std::os::raw::c_uchar = (*p.as_deref().unwrap()).delim_char;
    let mut quote: std::os::raw::c_uchar = (*p.as_deref().unwrap()).quote_char;
    let mut is_space:
            Option<unsafe extern "C" fn(_: std::os::raw::c_uchar) -> std::os::raw::c_int> =
        (*p.as_deref().unwrap()).is_space;
    let mut is_term:
            Option<unsafe extern "C" fn(_: std::os::raw::c_uchar) -> std::os::raw::c_int> =
        (*p.as_deref().unwrap()).is_term;
    let mut quoted: std::os::raw::c_int = (*p.as_deref().unwrap()).quoted;
    let mut pstate: std::os::raw::c_int = (*p.as_deref().unwrap()).pstate;
    let mut spaces: size_t = (*p.as_deref().unwrap()).spaces;
    let mut entry_pos: size_t = (*p.as_deref().unwrap()).entry_pos;
    if (*p.as_deref().unwrap()).entry_buf.is_null() && pos < len {
        /* Buffer hasn't been allocated yet and len > 0 */
        if csv_increase_buffer(p.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 as std::os::raw::c_int {
            (*p.as_deref_mut().unwrap()).quoted= quoted;
            (*p.as_deref_mut().unwrap()).pstate= pstate;
            (*p.as_deref_mut().unwrap()).spaces= spaces;
            (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
            return pos
        }
    }
    /* Space or Tab */
    while pos < len {
        /* Check memory usage, increase buffer if necessary */
        if entry_pos ==
               (if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                    (*p.as_deref().unwrap()).entry_size.wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong)
                } else { (*p).entry_size }) {
            if csv_increase_buffer(p.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 as std::os::raw::c_int {
                (*p.as_deref_mut().unwrap()).quoted= quoted;
                (*p.as_deref_mut().unwrap()).pstate= pstate;
                (*p.as_deref_mut().unwrap()).spaces= spaces;
                (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
                return pos
            }
        }
        let fresh0 = pos;
        pos= pos.wrapping_add(1);
        c= *us.offset(fresh0 as isize);
        match pstate {
            0 | 1 => {
                if (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                             c as std::os::raw::c_int == 0x9 as std::os::raw::c_int) as
                            std::os::raw::c_int
                    }) != 0 && c as std::os::raw::c_int != delim as std::os::raw::c_int {
                    continue ;
                }
                if if is_term.is_some() {
                       is_term.expect("non-null function pointer")(c)
                   } else {
                       (c as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                            c as std::os::raw::c_int == 0xa as std::os::raw::c_int) as
                           std::os::raw::c_int
                   } != 0 {
                    /* Carriage Return or Line Feed */
                    if pstate == 1 as std::os::raw::c_int {
                        if quoted == 0 {
                            entry_pos=
                                (entry_pos as
                                     std::os::raw::c_ulong).wrapping_sub(spaces) as
                                    size_t as size_t
                        } /* ROW_NOT_BEGUN */
                        if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0
                           {
                            *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                                '\u{0}' as i32 as std::os::raw::c_uchar
                        }
                        if cb1.is_some() &&
                               (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int
                                   != 0 && quoted == 0 &&
                               entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                           {
                            cb1.expect("non-null function pointer")(0 as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")((*p).entry_buf
                                                                        as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        }
                        pstate= 1 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as
                                                                        std::os::raw::c_int,
                                                                    data);
                        }
                        pstate= 0 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t
                    } else if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 2 as std::os::raw::c_int
                                  != 0 {
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as
                                                                        std::os::raw::c_int,
                                                                    data);
                        }
                        pstate= 0 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t
                    }
                } else if c as std::os::raw::c_int == delim as std::os::raw::c_int {
                    /* Don't submit empty rows by default */
                    /* Comma */
                    if quoted == 0 {
                        entry_pos=
                            (entry_pos as std::os::raw::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t
                    } /* Anything else */
                    if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                        *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_uchar
                    }
                    if cb1.is_some() &&
                           (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int !=
                               0 && quoted == 0 &&
                           entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        cb1.expect("non-null function pointer")(0 as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")((*p).entry_buf
                                                                    as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    }
                    pstate= 1 as std::os::raw::c_int;
                    spaces= 0 as std::os::raw::c_int as size_t;
                    quoted= spaces as std::os::raw::c_int;
                    entry_pos= quoted as size_t
                } else if c as std::os::raw::c_int == quote as std::os::raw::c_int {
                    /* Quote */
                    pstate= 2 as std::os::raw::c_int;
                    quoted= 1 as std::os::raw::c_int
                } else {
                    pstate= 2 as std::os::raw::c_int;
                    quoted= 0 as std::os::raw::c_int;
                    let fresh1 = entry_pos;
                    entry_pos= entry_pos.wrapping_add(1);
                    *(*p.as_deref().unwrap()).entry_buf.offset(fresh1 as isize) = c
                }
            }
            2 => {
                if c as std::os::raw::c_int == quote as std::os::raw::c_int {
                    /* Quote */
                    if quoted != 0 {
                        let fresh2 = entry_pos;
                        entry_pos= entry_pos.wrapping_add(1);
                        *(*p.as_deref().unwrap()).entry_buf.offset(fresh2 as isize) = c;
                        pstate= 3 as std::os::raw::c_int
                    } else {
                        /* STRICT ERROR - double quote inside non-quoted field */
                        if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 1 as std::os::raw::c_int != 0
                           {
                            (*p.as_deref_mut().unwrap()).status= 1 as std::os::raw::c_int;
                            (*p.as_deref_mut().unwrap()).quoted= quoted;
                            (*p.as_deref_mut().unwrap()).pstate= pstate;
                            (*p.as_deref_mut().unwrap()).spaces= spaces;
                            (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
                            return pos.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)
                        }
                        let fresh3 = entry_pos;
                        entry_pos= entry_pos.wrapping_add(1);
                        *(*p.as_deref().unwrap()).entry_buf.offset(fresh3 as isize) = c;
                        spaces= 0 as std::os::raw::c_int as size_t
                    }
                } else if c as std::os::raw::c_int == delim as std::os::raw::c_int {
                    /* Comma */
                    if quoted != 0 {
                        let fresh4 = entry_pos;
                        entry_pos= entry_pos.wrapping_add(1);
                        *(*p.as_deref().unwrap()).entry_buf.offset(fresh4 as isize) = c
                    } else {
                        if quoted == 0 {
                            entry_pos=
                                (entry_pos as
                                     std::os::raw::c_ulong).wrapping_sub(spaces) as
                                    size_t as size_t
                        }
                        if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0
                           {
                            *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                                '\u{0}' as i32 as std::os::raw::c_uchar
                        }
                        if cb1.is_some() &&
                               (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int
                                   != 0 && quoted == 0 &&
                               entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                           {
                            cb1.expect("non-null function pointer")(0 as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")((*p).entry_buf
                                                                        as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        }
                        pstate= 1 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t
                    }
                } else if if is_term.is_some() {
                              is_term.expect("non-null function pointer")(c)
                          } else {
                              (c as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                                   c as std::os::raw::c_int == 0xa as std::os::raw::c_int) as
                                  std::os::raw::c_int
                          } != 0 {
                    /* Carriage Return or Line Feed */
                    if quoted == 0 {
                        if quoted == 0 {
                            entry_pos=
                                (entry_pos as
                                     std::os::raw::c_ulong).wrapping_sub(spaces) as
                                    size_t as size_t
                        } /* Anything else */
                        if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0
                           {
                            *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                                '\u{0}' as i32 as std::os::raw::c_uchar
                        }
                        if cb1.is_some() &&
                               (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int
                                   != 0 && quoted == 0 &&
                               entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong
                           {
                            cb1.expect("non-null function pointer")(0 as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")((*p).entry_buf
                                                                        as
                                                                        *mut std::os::raw::c_void,
                                                                    entry_pos,
                                                                    data);
                        }
                        pstate= 1 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as
                                                                        std::os::raw::c_int,
                                                                    data);
                        }
                        pstate= 0 as std::os::raw::c_int;
                        spaces= 0 as std::os::raw::c_int as size_t;
                        quoted= spaces as std::os::raw::c_int;
                        entry_pos= quoted as size_t
                    } else {
                        let fresh5 = entry_pos;
                        entry_pos= entry_pos.wrapping_add(1);
                        *(*p.as_deref().unwrap()).entry_buf.offset(fresh5 as isize) = c
                    }
                } else if quoted == 0 &&
                              (if is_space.is_some() {
                                   is_space.expect("non-null function pointer")(c)
                               } else {
                                   (c as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                        c as std::os::raw::c_int ==
                                            0x9 as std::os::raw::c_int) as std::os::raw::c_int
                               }) != 0 {
                    /* Tab or space for non-quoted field */
                    let fresh6 = entry_pos;
                    entry_pos= entry_pos.wrapping_add(1);
                    *(*p.as_deref().unwrap()).entry_buf.offset(fresh6 as isize) = c;
                    spaces= spaces.wrapping_add(1)
                } else {
                    let fresh7 = entry_pos;
                    entry_pos= entry_pos.wrapping_add(1);
                    *(*p.as_deref().unwrap()).entry_buf.offset(fresh7 as isize) = c;
                    spaces= 0 as std::os::raw::c_int as size_t
                }
            }
            3 => {
                /* This only happens when a quote character is encountered in a quoted field */
                if c as std::os::raw::c_int == delim as std::os::raw::c_int {
                    /* Comma */
                    entry_pos=
                        (entry_pos as
                             std::os::raw::c_ulong).wrapping_sub(spaces.wrapping_add(1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 std::os::raw::c_ulong))
                            as size_t as
                            size_t; /* get rid of spaces and original quote */
                    if quoted == 0 {
                        entry_pos=
                            (entry_pos as std::os::raw::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t
                    }
                    if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                        *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_uchar
                    }
                    if cb1.is_some() &&
                           (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int !=
                               0 && quoted == 0 &&
                           entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        cb1.expect("non-null function pointer")(0 as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")((*p).entry_buf
                                                                    as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    }
                    pstate= 1 as std::os::raw::c_int;
                    spaces= 0 as std::os::raw::c_int as size_t;
                    quoted= spaces as std::os::raw::c_int;
                    entry_pos= quoted as size_t
                } else if if is_term.is_some() {
                              is_term.expect("non-null function pointer")(c)
                          } else {
                              (c as std::os::raw::c_int == 0xd as std::os::raw::c_int ||
                                   c as std::os::raw::c_int == 0xa as std::os::raw::c_int) as
                                  std::os::raw::c_int
                          } != 0 {
                    /* Carriage Return or Line Feed */
                    entry_pos=
                        (entry_pos as
                             std::os::raw::c_ulong).wrapping_sub(spaces.wrapping_add(1
                                                                                 as
                                                                                 std::os::raw::c_int
                                                                                 as
                                                                                 std::os::raw::c_ulong))
                            as size_t as
                            size_t; /* get rid of spaces and original quote */
                    if quoted == 0 {
                        entry_pos=
                            (entry_pos as std::os::raw::c_ulong).wrapping_sub(spaces)
                                as size_t as size_t
                    }
                    if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 8 as std::os::raw::c_int != 0 {
                        *(*p.as_deref().unwrap()).entry_buf.offset(entry_pos as isize) =
                            '\u{0}' as i32 as std::os::raw::c_uchar
                    }
                    if cb1.is_some() &&
                           (*p.as_deref().unwrap()).options as std::os::raw::c_int & 16 as std::os::raw::c_int !=
                               0 && quoted == 0 &&
                           entry_pos == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        cb1.expect("non-null function pointer")(0 as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    } else if cb1.is_some() {
                        cb1.expect("non-null function pointer")((*p).entry_buf
                                                                    as
                                                                    *mut std::os::raw::c_void,
                                                                entry_pos,
                                                                data);
                    }
                    pstate= 1 as std::os::raw::c_int;
                    spaces= 0 as std::os::raw::c_int as size_t;
                    quoted= spaces as std::os::raw::c_int;
                    entry_pos= quoted as size_t;
                    if cb2.is_some() {
                        cb2.expect("non-null function pointer")(c as
                                                                    std::os::raw::c_int,
                                                                data);
                    }
                    pstate= 0 as std::os::raw::c_int;
                    spaces= 0 as std::os::raw::c_int as size_t;
                    quoted= spaces as std::os::raw::c_int;
                    entry_pos= quoted as size_t
                } else if if is_space.is_some() {
                              is_space.expect("non-null function pointer")(c)
                          } else {
                              (c as std::os::raw::c_int == 0x20 as std::os::raw::c_int ||
                                   c as std::os::raw::c_int == 0x9 as std::os::raw::c_int) as
                                  std::os::raw::c_int
                          } != 0 {
                    /* Space or Tab */
                    let fresh8 = entry_pos; /* Anything else */
                    entry_pos= entry_pos.wrapping_add(1);
                    *(*p.as_deref().unwrap()).entry_buf.offset(fresh8 as isize) = c;
                    spaces= spaces.wrapping_add(1)
                } else if c as std::os::raw::c_int == quote as std::os::raw::c_int {
                    /* Quote */
                    if spaces != 0 {
                        /* STRICT ERROR - unescaped double quote */
                        if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 1 as std::os::raw::c_int != 0
                           {
                            (*p.as_deref_mut().unwrap()).status= 1 as std::os::raw::c_int;
                            (*p.as_deref_mut().unwrap()).quoted= quoted;
                            (*p.as_deref_mut().unwrap()).pstate= pstate;
                            (*p.as_deref_mut().unwrap()).spaces= spaces;
                            (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
                            return pos.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)
                        }
                        spaces= 0 as std::os::raw::c_int as size_t;
                        let fresh9 = entry_pos;
                        entry_pos= entry_pos.wrapping_add(1);
                        *(*p.as_deref().unwrap()).entry_buf.offset(fresh9 as isize) = c
                    } else {
                        /* Two quotes in a row */
                        pstate= 2 as std::os::raw::c_int
                    }
                } else {
                    /* STRICT ERROR - unescaped double quote */
                    if (*p.as_deref().unwrap()).options as std::os::raw::c_int & 1 as std::os::raw::c_int != 0 {
                        (*p.as_deref_mut().unwrap()).status= 1 as std::os::raw::c_int;
                        (*p.as_deref_mut().unwrap()).quoted= quoted;
                        (*p.as_deref_mut().unwrap()).pstate= pstate;
                        (*p.as_deref_mut().unwrap()).spaces= spaces;
                        (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
                        return pos.wrapping_sub(1 as std::os::raw::c_int as
                                                    std::os::raw::c_ulong)
                    }
                    pstate= 2 as std::os::raw::c_int;
                    spaces= 0 as std::os::raw::c_int as size_t;
                    let fresh10 = entry_pos;
                    entry_pos= entry_pos.wrapping_add(1);
                    *(*p.as_deref().unwrap()).entry_buf.offset(fresh10 as isize) = c
                }
            }
            _ => { }
        }
    }
    (*p.as_deref_mut().unwrap()).quoted= quoted;
    (*p.as_deref_mut().unwrap()).pstate= pstate;
    (*p.as_deref_mut().unwrap()).spaces= spaces;
    (*p.as_deref_mut().unwrap()).entry_pos= entry_pos;
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn csv_write(mut dest: *mut /* owning */ std::os::raw::c_void,
                                   mut dest_size: size_t,
                                   mut src: *const std::os::raw::c_void,
                                   mut src_size: size_t) -> size_t {
    return csv_write2(dest, dest_size, src, src_size,
                      0x22 as std::os::raw::c_int as std::os::raw::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite(mut fp: Option<&mut FILE>,
                                    mut src: *const std::os::raw::c_void,
                                    mut src_size: size_t) -> std::os::raw::c_int {
    return csv_fwrite2(fp.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), src, src_size,
                       0x22 as std::os::raw::c_int as std::os::raw::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn csv_write2(mut dest: *mut std::os::raw::c_void,
                                    mut dest_size: size_t,
                                    mut src: *const std::os::raw::c_void,
                                    mut src_size: size_t,
                                    mut quote: std::os::raw::c_uchar) -> size_t {
    let mut cdest: *mut std::os::raw::c_uchar = dest as *mut std::os::raw::c_uchar;
    let mut csrc: *const std::os::raw::c_uchar = src as *const std::os::raw::c_uchar;
    let mut chars: size_t = 0 as std::os::raw::c_int as size_t;
    if src.is_null() {(); return 0 as std::os::raw::c_int as size_t }
    if dest.is_null() {(); dest_size= 0 as std::os::raw::c_int as size_t }
    if dest_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        let fresh11 = cdest;
        cdest= cdest.offset(1);
        *fresh11= quote
    }
    chars= chars.wrapping_add(1);
    while src_size != 0 {
        if (*csrc) as std::os::raw::c_int == quote as std::os::raw::c_int {
            if dest_size > chars {
                let fresh12 = cdest;
                cdest= cdest.offset(1);
                *fresh12= quote
            }
            if chars < 18446744073709551615 as std::os::raw::c_ulong {
                chars= chars.wrapping_add(1)
            }
        }
        if dest_size > chars {
            let fresh13 = cdest;
            cdest= cdest.offset(1);
            *fresh13= (*csrc)
        }
        if chars < 18446744073709551615 as std::os::raw::c_ulong {
            chars= chars.wrapping_add(1)
        }
        src_size= src_size.wrapping_sub(1);
        csrc= csrc.offset(1)
    }
    if dest_size > chars { *cdest= quote }
    if chars < 18446744073709551615 as std::os::raw::c_ulong {
        chars= chars.wrapping_add(1)
    }
    return chars;
}
#[no_mangle]
pub unsafe extern "C" fn csv_fwrite2(mut fp: *mut FILE,
                                     mut src: *const std::os::raw::c_void,
                                     mut src_size: size_t,
                                     mut quote: std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    let mut csrc: *const std::os::raw::c_uchar = src as *const std::os::raw::c_uchar;
    if fp.is_null() || src.is_null() { return 0 as std::os::raw::c_int }
    if fputc(quote as std::os::raw::c_int, fp) == -(1 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    while src_size != 0 {
        if (*csrc) as std::os::raw::c_int == quote as std::os::raw::c_int {
            if fputc(quote as std::os::raw::c_int, fp) == -(1 as std::os::raw::c_int) {
                return -(1 as std::os::raw::c_int)
            }
        }
        if fputc((*csrc) as std::os::raw::c_int, fp) == -(1 as std::os::raw::c_int) {
            return -(1 as std::os::raw::c_int)
        }
        src_size= src_size.wrapping_sub(1);
        csrc= csrc.offset(1)
    }
    if fputc(quote as std::os::raw::c_int, fp) == -(1 as std::os::raw::c_int) {
        return -(1 as std::os::raw::c_int)
    }
    return 0 as std::os::raw::c_int;
}
