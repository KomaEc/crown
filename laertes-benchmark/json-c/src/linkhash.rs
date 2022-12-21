#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, ptr_offset_from, register_tool)]
extern "C" {
    #[no_mangle]
    fn vprintf(_: *const std::os::raw::c_char, _: ::std::ffi::VaList) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;
    #[no_mangle]
    fn json_c_get_random_seed() -> std::os::raw::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: std::os::raw::c_uint,
    pub fp_offset: std::os::raw::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __darwin_ptrdiff_t = std::os::raw::c_long;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type uintptr_t = std::os::raw::c_ulong;
pub type va_list = __darwin_va_list;
pub type size_t = __darwin_size_t;
pub type uint8_t = std::os::raw::c_uchar;
pub type uint16_t = std::os::raw::c_ushort;
pub type uint32_t = std::os::raw::c_uint;
pub type ptrdiff_t = __darwin_ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const libc::c_void,
    pub k_is_constant: std::os::raw::c_int,
    pub v: *const libc::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
}
pub type json_bool = std::os::raw::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_table {
    pub size: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub head: *mut lh_entry,
    pub tail: *mut lh_entry,
    pub table: *mut lh_entry,
    pub free_fn: Option<lh_entry_free_fn>,
    pub hash_fn: Option<lh_hash_fn>,
    pub equal_fn: Option<lh_equal_fn>,
}
pub type lh_equal_fn
    =
    unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void)
        -> std::os::raw::c_int;
pub type lh_hash_fn
    =
    unsafe extern "C" fn(_: *const libc::c_void) -> std::os::raw::c_ulong;
pub type lh_entry_free_fn = unsafe extern "C" fn(_: *mut lh_entry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *const libc::c_void,
    pub i: size_t,
}
#[inline]
unsafe extern "C" fn lh_get_hash(mut t: *const lh_table,
                                 mut k: *const libc::c_void)
 -> std::os::raw::c_ulong {
    return (*t).hash_fn.expect("non-null function pointer")(k);
}
static mut char_hash_fn: Option<lh_hash_fn> =
    unsafe {
        Some(lh_char_hash as
                 unsafe extern "C" fn(_: *const libc::c_void)
                     -> std::os::raw::c_ulong)
    };
#[no_mangle]
pub unsafe extern "C" fn json_global_set_string_hash(h: std::os::raw::c_int)
 -> std::os::raw::c_int {
    match h {
        0 => {
            char_hash_fn =
                Some(lh_char_hash as
                         unsafe extern "C" fn(_: *const libc::c_void)
                             -> std::os::raw::c_ulong)
        }
        1 => {
            char_hash_fn =
                Some(lh_perllike_str_hash as
                         unsafe extern "C" fn(_: *const libc::c_void)
                             -> std::os::raw::c_ulong)
        }
        _ => { return -(1 as std::os::raw::c_int) }
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_abort(mut msg: *const std::os::raw::c_char,
                                  mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    vprintf(msg, ap.as_va_list());
    exit(1 as std::os::raw::c_int);
}
unsafe extern "C" fn lh_ptr_hash(mut k: *const libc::c_void)
 -> std::os::raw::c_ulong {
    /* CAW: refactored to be 64bit nice */
    return (k as ptrdiff_t as
                std::os::raw::c_ulong).wrapping_mul(0x9e370001 as std::os::raw::c_ulong) >>
               4 as std::os::raw::c_int & 0xffffffffffffffff as std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn lh_ptr_equal(mut k1: *const libc::c_void,
                                      mut k2: *const libc::c_void)
 -> std::os::raw::c_int {
    return (k1 == k2) as std::os::raw::c_int;
}
/*
-------------------------------------------------------------------------------
mix -- mix 3 32-bit values reversibly.

This is reversible, so any information in (a,b,c) before mix() is
still in (a,b,c) after mix().

If four pairs of (a,b,c) inputs are run through mix(), or through
mix() in reverse, there are at least 32 bits of the output that
are sometimes the same for one pair and different for another pair.
This was tested for:
* pairs that differed by one bit, by two bits, in any combination
  of top bits of (a,b,c), or in any combination of bottom bits of
  (a,b,c).
* "differ" is defined as +, -, ^, or ~^.  For + and -, I transformed
  the output delta to a Gray code (a^(a>>1)) so a string of 1's (as
  is commonly produced by subtraction) look like a single 1-bit
  difference.
* the base values were pseudorandom, all zero but one bit set, or
  all zero plus a counter that starts at zero.

Some k values for my "a-=c; a^=rot(c,k); c+=b;" arrangement that
satisfy this are
    4  6  8 16 19  4
    9 15  3 18 27 15
   14  9  3  7 17  3
Well, "9 15 3 18 27 15" didn't quite get 32 bits diffing
for "differ" defined as + with a one-bit base and a two-bit delta.  I
used http://burtleburtle.net/bob/hash/avalanche.html to choose
the operations, constants, and arrangements of the variables.

This does not achieve avalanche.  There are input bits of (a,b,c)
that fail to affect some output bits of (a,b,c), especially of a.  The
most thoroughly mixed value is c, but it doesn't really even achieve
avalanche in c.

This allows some parallelism.  Read-after-writes are good at doubling
the number of bits affected, so the goal of mixing pulls in the opposite
direction as the goal of parallelism.  I did what I could.  Rotates
seem to cost as much as shifts on every machine I could lay my hands
on, and rotates are much kinder to the top and bottom bits, so I used
rotates.
-------------------------------------------------------------------------------
*/
/*
-------------------------------------------------------------------------------
final -- final mixing of 3 32-bit values (a,b,c) into c

Pairs of (a,b,c) values differing in only a few bits will usually
produce values of c that look totally different.  This was tested for
* pairs that differed by one bit, by two bits, in any combination
  of top bits of (a,b,c), or in any combination of bottom bits of
  (a,b,c).
* "differ" is defined as +, -, ^, or ~^.  For + and -, I transformed
  the output delta to a Gray code (a^(a>>1)) so a string of 1's (as
  is commonly produced by subtraction) look like a single 1-bit
  difference.
* the base values were pseudorandom, all zero but one bit set, or
  all zero plus a counter that starts at zero.

These constants passed:
 14 11 25 16 4 14 24
 12 14 25 16 4 14 24
and these came close:
  4  8 15 26 3 22 24
 10  8 15 26 3 22 24
 11  8 15 26 3 22 24
-------------------------------------------------------------------------------
*/
/*
-------------------------------------------------------------------------------
hashlittle() -- hash a variable-length key into a 32-bit value
  k       : the key (the unaligned variable-length array of bytes)
  length  : the length of the key, counting by bytes
  initval : can be any 4-byte value
Returns a 32-bit value.  Every bit of the key affects every bit of
the return value.  Two keys differing by one or two bits will have
totally different hash values.

The best hash table sizes are powers of 2.  There is no need to do
mod a prime (mod is sooo slow!).  If you need less than 32 bits,
use a bitmask.  For example, if you need only 10 bits, do
  h = (h & hashmask(10));
In which case, the hash table should have hashsize(10) elements.

If you are hashing n strings (uint8_t **)k, do it like this:
  for (i=0, h=0; i<n; ++i) h = hashlittle( k[i], len[i], h);

By Bob Jenkins, 2006.  bob_jenkins@burtleburtle.net.  You may use this
code any way you wish, private, educational, or commercial.  It's free.

Use for hash table lookup, or anything where one collision in 2^^32 is
acceptable.  Do NOT use for cryptographic purposes.
-------------------------------------------------------------------------------
*/
unsafe extern "C" fn hashlittle(mut key: *const libc::c_void,
                                mut length: size_t, mut initval: uint32_t)
 -> uint32_t {
    let mut a: uint32_t = 0; /* internal state */
    let mut b: uint32_t = 0; /* needed for Mac Powerbook G4 */
    let mut c: uint32_t = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed{ptr: 0 as *const libc::c_void,};
    /* Set up the internal state */
    c =
        (0xdeadbeef as
             std::os::raw::c_uint).wrapping_add(length as
                                            uint32_t).wrapping_add(initval); /* read 32-bit chunks */
    b = c;
    a = b;
    u.ptr = key;
    if 0 as std::os::raw::c_int != 0 &&
           u.i & 0x3 as std::os::raw::c_int as std::os::raw::c_ulong ==
               0 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut k: *const uint32_t = key as *const uint32_t;
        /*------ all but last block: aligned reads and affect 32 bits of (a,b,c) */
        while length > 12 as std::os::raw::c_int as std::os::raw::c_ulong {
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int as
                                                              isize)) as
                    uint32_t as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int as
                                                              isize)) as
                    uint32_t as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add(*k.offset(2 as std::os::raw::c_int as
                                                              isize)) as
                    uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 4 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 6 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 6 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 8 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 8 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 16 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 16 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 19 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 19 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 4 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length =
                (length as
                     std::os::raw::c_ulong).wrapping_sub(12 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t;
            k = k.offset(3 as std::os::raw::c_int as isize)
        }
        /*----------------------------- handle the last (probably partial) block */
    /*
     * "k[2]&0xffffff" actually reads beyond the end of the string, but
     * then masks off the part it's not allowed to read.  Because the
     * string is aligned, the masked-off tail is in the same word as the
     * rest of the string.  Every machine with memory protection I've seen
     * does it on word boundaries, so is OK with this.  But VALGRIND will
     * still catch it and complain.  The masking trick does make the hash
     * noticably faster for short strings (like English words).
     * AddressSanitizer is similarly picky about overrunning
	 * the buffer. (http://clang.llvm.org/docs/AddressSanitizer.html
     */
        /* GCC's ASAN */
        /* Clang's ASAN */
        match length {
            12 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k.offset(2 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t;
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
                /* zero length strings require no mixing */
            }
            11 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k.offset(2 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffffff as
                                                            std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as
                        uint32_t; /* need to read the key one byte at a time */
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t; /* read 16-bit chunks */
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            10 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k.offset(2 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffff as std::os::raw::c_int
                                                            as std::os::raw::c_uint)
                        as uint32_t as uint32_t;
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            9 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k.offset(2 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xff as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            8 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            7 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffffff as
                                                            std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            6 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffff as std::os::raw::c_int
                                                            as std::os::raw::c_uint)
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            5 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k.offset(1 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xff as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            4 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize))
                        as uint32_t as uint32_t
            }
            3 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffffff as
                                                            std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as uint32_t
            }
            2 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xffff as std::os::raw::c_int
                                                            as std::os::raw::c_uint)
                        as uint32_t as uint32_t
            }
            1 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k.offset(0 as std::os::raw::c_int
                                                                  as isize) &
                                                        0xff as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                        uint32_t as uint32_t
            }
            0 => { return c }
            _ => { }
        }
    } else if 0 as std::os::raw::c_int != 0 &&
                  u.i & 0x1 as std::os::raw::c_int as std::os::raw::c_ulong ==
                      0 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut k_0: *const uint16_t = key as *const uint16_t;
        let mut k8: *const uint8_t = 0 as *const uint8_t;
        while length > 12 as std::os::raw::c_int as std::os::raw::c_ulong
              /*--------------- all but last block: aligned reads and different mixing */
              {
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 isize)
                                                                                     as
                                                                                     uint32_t)
                                                                                    <<
                                                                                    16
                                                                                        as
                                                                                        std::os::raw::c_int))
                    as uint32_t as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add((*k_0.offset(2 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_uint).wrapping_add((*k_0.offset(3
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 isize)
                                                                                     as
                                                                                     uint32_t)
                                                                                    <<
                                                                                    16
                                                                                        as
                                                                                        std::os::raw::c_int))
                    as uint32_t as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add((*k_0.offset(4 as std::os::raw::c_int
                                                                 as isize) as
                                                     std::os::raw::c_uint).wrapping_add((*k_0.offset(5
                                                                                                 as
                                                                                                 std::os::raw::c_int
                                                                                                 as
                                                                                                 isize)
                                                                                     as
                                                                                     uint32_t)
                                                                                    <<
                                                                                    16
                                                                                        as
                                                                                        std::os::raw::c_int))
                    as uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 4 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 6 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 6 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 8 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 8 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 16 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 16 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 19 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 19 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 4 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length =
                (length as
                     std::os::raw::c_ulong).wrapping_sub(12 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t;
            k_0 = k_0.offset(6 as std::os::raw::c_int as isize)
        }
        k8 = k_0 as *const uint8_t;
        let mut current_block_102: u64;
        match length {
            12 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(4 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(5
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t;
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(2 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(3
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t;
                current_block_102 = 3812947724376655173;
                /*----------------------------- handle the last (probably partial) block */
                /* zero length requires no mixing */
            }
            11 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add((*k8.offset(10 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t; /* fall through */
                current_block_102 = 4519966451224754431; /* fall through */
            }
            10 => {
                current_block_102 = 4519966451224754431; /* fall through */
            }
            9 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k8.offset(8 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t; /* fall through */
                current_block_102 = 17974925641258900466; /* fall through */
            }
            8 => { current_block_102 = 17974925641258900466; }
            7 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k8.offset(6 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_102 = 2776114520721993823;
            }
            6 => { current_block_102 = 2776114520721993823; }
            5 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k8.offset(4 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                current_block_102 = 10086923867428078017;
            }
            4 => { current_block_102 = 10086923867428078017; }
            3 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k8.offset(2 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_102 = 270789961462444427;
            }
            2 => { current_block_102 = 270789961462444427; }
            1 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k8.offset(0 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                current_block_102 = 3812947724376655173;
            }
            0 => { return c }
            _ => { current_block_102 = 3812947724376655173; }
        }
        match current_block_102 {
            2776114520721993823 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k_0.offset(2 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t
            }
            17974925641258900466 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(2 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(3
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t
            }
            4519966451224754431 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k_0.offset(4 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(2 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(3
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t;
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t
            }
            10086923867428078017 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_0.offset(0 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as
                                                         std::os::raw::c_uint).wrapping_add((*k_0.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                         as
                                                                                         uint32_t)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int))
                        as uint32_t as uint32_t
            }
            270789961462444427 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k_0.offset(0 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t
            }
            _ => { }
        }
    } else {
        let mut k_1: *const uint8_t = key as *const uint8_t;
        /*--------------- all but the last block: affect some 32 bits of (a,b,c) */
        while length > 12 as std::os::raw::c_int as std::os::raw::c_ulong {
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add(*k_1.offset(0 as std::os::raw::c_int
                                                                as isize) as
                                                    std::os::raw::c_uint) as uint32_t
                    as uint32_t;
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(1 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    8 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(2 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    16 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            a =
                (a as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(3 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    24 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add(*k_1.offset(4 as std::os::raw::c_int
                                                                as isize) as
                                                    std::os::raw::c_uint) as uint32_t
                    as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(5 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    8 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(6 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    16 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            b =
                (b as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(7 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    24 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add(*k_1.offset(8 as std::os::raw::c_int
                                                                as isize) as
                                                    std::os::raw::c_uint) as uint32_t
                    as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(9 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    8 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(10 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    16 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            c =
                (c as
                     std::os::raw::c_uint).wrapping_add((*k_1.offset(11 as std::os::raw::c_int
                                                                 as isize) as
                                                     uint32_t) <<
                                                    24 as std::os::raw::c_int) as
                    uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 4 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 6 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 6 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 8 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 8 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as std::os::raw::c_uint).wrapping_sub(c) as uint32_t as uint32_t;
            a ^=
                c << 16 as std::os::raw::c_int |
                    c >> 32 as std::os::raw::c_int - 16 as std::os::raw::c_int;
            c = (c as std::os::raw::c_uint).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as std::os::raw::c_uint).wrapping_sub(a) as uint32_t as uint32_t;
            b ^=
                a << 19 as std::os::raw::c_int |
                    a >> 32 as std::os::raw::c_int - 19 as std::os::raw::c_int;
            a = (a as std::os::raw::c_uint).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as std::os::raw::c_uint).wrapping_sub(b) as uint32_t as uint32_t;
            c ^=
                b << 4 as std::os::raw::c_int |
                    b >> 32 as std::os::raw::c_int - 4 as std::os::raw::c_int;
            b = (b as std::os::raw::c_uint).wrapping_add(a) as uint32_t as uint32_t;
            length =
                (length as
                     std::os::raw::c_ulong).wrapping_sub(12 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as size_t
                    as size_t;
            k_1 = k_1.offset(12 as std::os::raw::c_int as isize)
        }
        let mut current_block_153: u64;
        /*-------------------------------- last block: affect all 32 bits of (c) */
        match length {
            12 => {
                /* all the case statements fall through */
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(11 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        24 as std::os::raw::c_int) as
                        uint32_t as uint32_t; /* FALLTHRU */
                current_block_153 = 1912075236522443459; /* FALLTHRU */
            }
            11 => {
                current_block_153 = 1912075236522443459; /* FALLTHRU */
            }
            10 => {
                current_block_153 = 4175524987153788437; /* FALLTHRU */
            }
            9 => {
                current_block_153 = 12414557862826760723; /* FALLTHRU */
            }
            8 => {
                current_block_153 = 3676371612434101161; /* FALLTHRU */
            }
            7 => {
                current_block_153 = 15611831060393419491; /* FALLTHRU */
            }
            6 => {
                current_block_153 = 4716274144113630571; /* FALLTHRU */
            }
            5 => {
                current_block_153 = 17902300423635264959; /* FALLTHRU */
            }
            4 => {
                current_block_153 = 16683488423647041184; /* FALLTHRU */
            }
            3 => { current_block_153 = 722990626647897627; }
            2 => { current_block_153 = 17894648329654382942; }
            1 => { current_block_153 = 14062854596027233563; }
            0 => { return c }
            _ => { current_block_153 = 2704538829018177290; }
        }
        match current_block_153 {
            1912075236522443459 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(10 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 4175524987153788437;
            }
            _ => { }
        }
        match current_block_153 {
            4175524987153788437 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(9 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        8 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 12414557862826760723;
            }
            _ => { }
        }
        match current_block_153 {
            12414557862826760723 => {
                c =
                    (c as
                         std::os::raw::c_uint).wrapping_add(*k_1.offset(8 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                current_block_153 = 3676371612434101161;
            }
            _ => { }
        }
        match current_block_153 {
            3676371612434101161 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(7 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        24 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 15611831060393419491;
            }
            _ => { }
        }
        match current_block_153 {
            15611831060393419491 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(6 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 4716274144113630571;
            }
            _ => { }
        }
        match current_block_153 {
            4716274144113630571 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(5 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        8 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 17902300423635264959;
            }
            _ => { }
        }
        match current_block_153 {
            17902300423635264959 => {
                b =
                    (b as
                         std::os::raw::c_uint).wrapping_add(*k_1.offset(4 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t;
                current_block_153 = 16683488423647041184;
            }
            _ => { }
        }
        match current_block_153 {
            16683488423647041184 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(3 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        24 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 722990626647897627;
            }
            _ => { }
        }
        match current_block_153 {
            722990626647897627 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(2 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        16 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 17894648329654382942;
            }
            _ => { }
        }
        match current_block_153 {
            17894648329654382942 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add((*k_1.offset(1 as
                                                                     std::os::raw::c_int
                                                                     as isize)
                                                         as uint32_t) <<
                                                        8 as std::os::raw::c_int) as
                        uint32_t as uint32_t;
                current_block_153 = 14062854596027233563;
            }
            _ => { }
        }
        match current_block_153 {
            14062854596027233563 => {
                a =
                    (a as
                         std::os::raw::c_uint).wrapping_add(*k_1.offset(0 as
                                                                    std::os::raw::c_int
                                                                    as isize)
                                                        as std::os::raw::c_uint) as
                        uint32_t as uint32_t
            }
            _ => { }
        }
    }
    c ^= b;
    c =
        (c as
             std::os::raw::c_uint).wrapping_sub(b << 14 as std::os::raw::c_int |
                                            b >>
                                                32 as std::os::raw::c_int -
                                                    14 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    a ^= c;
    a =
        (a as
             std::os::raw::c_uint).wrapping_sub(c << 11 as std::os::raw::c_int |
                                            c >>
                                                32 as std::os::raw::c_int -
                                                    11 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    b ^= a;
    b =
        (b as
             std::os::raw::c_uint).wrapping_sub(a << 25 as std::os::raw::c_int |
                                            a >>
                                                32 as std::os::raw::c_int -
                                                    25 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    c ^= b;
    c =
        (c as
             std::os::raw::c_uint).wrapping_sub(b << 16 as std::os::raw::c_int |
                                            b >>
                                                32 as std::os::raw::c_int -
                                                    16 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    a ^= c;
    a =
        (a as
             std::os::raw::c_uint).wrapping_sub(c << 4 as std::os::raw::c_int |
                                            c >>
                                                32 as std::os::raw::c_int -
                                                    4 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    b ^= a;
    b =
        (b as
             std::os::raw::c_uint).wrapping_sub(a << 14 as std::os::raw::c_int |
                                            a >>
                                                32 as std::os::raw::c_int -
                                                    14 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    c ^= b;
    c =
        (c as
             std::os::raw::c_uint).wrapping_sub(b << 24 as std::os::raw::c_int |
                                            b >>
                                                32 as std::os::raw::c_int -
                                                    24 as std::os::raw::c_int) as
            uint32_t as uint32_t;
    return c;
}
/* a simple hash function similiar to what perl does for strings.
 * for good results, the string should not be excessivly large.
 */
unsafe extern "C" fn lh_perllike_str_hash(mut k: *const libc::c_void)
 -> std::os::raw::c_ulong {
    let mut rkey: *const std::os::raw::c_char = k as *const std::os::raw::c_char;
    let mut hashval: std::os::raw::c_uint = 1 as std::os::raw::c_int as std::os::raw::c_uint;
    while *rkey != 0 {
        let fresh0 = rkey;
        rkey = rkey.offset(1);
        hashval =
            hashval.wrapping_mul(33 as std::os::raw::c_int as
                                     std::os::raw::c_uint).wrapping_add(*fresh0 as
                                                                    std::os::raw::c_uint)
    }
    return hashval as std::os::raw::c_ulong;
}
/*
 * $Id: linkhash.c,v 1.4 2006/01/26 02:16:28 mclark Exp $
 *
 * Copyright (c) 2004, 2005 Metaparadigm Pte. Ltd.
 * Michael Clark <michael@metaparadigm.com>
 * Copyright (c) 2009 Hewlett-Packard Development Company, L.P.
 *
 * This library is free software; you can redistribute it and/or modify
 * it under the terms of the MIT license. See COPYING for details.
 *
 */
/* hash functions */
unsafe extern "C" fn lh_char_hash(mut k: *const libc::c_void)
 -> std::os::raw::c_ulong {
    static mut random_seed: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    if random_seed == -(1 as std::os::raw::c_int) {
        let mut seed: std::os::raw::c_int = 0;
        loop 
             /* potentially racy */
             /* we can't use -1 as it is the unitialized sentinel */
             {
            seed = json_c_get_random_seed();
            if !(seed == -(1 as std::os::raw::c_int)) { break ; }
        }
        ::std::ptr::write_volatile(&mut random_seed as *mut std::os::raw::c_int, seed)
    }
    return hashlittle(k as *const std::os::raw::c_char as *const libc::c_void,
                      strlen(k as *const std::os::raw::c_char),
                      random_seed as uint32_t) as std::os::raw::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn lh_char_equal(mut k1: *const libc::c_void,
                                       mut k2: *const libc::c_void)
 -> std::os::raw::c_int {
    return (strcmp(k1 as *const std::os::raw::c_char, k2 as *const std::os::raw::c_char) ==
                0 as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_new(mut size: std::os::raw::c_int,
                                      mut free_fn: Option<lh_entry_free_fn>,
                                      mut hash_fn: Option<lh_hash_fn>,
                                      mut equal_fn: Option<lh_equal_fn>)
 -> *mut lh_table {
    let mut i: std::os::raw::c_int = 0;
    let mut t: *mut lh_table = 0 as *mut lh_table;
    t =
        calloc(1 as std::os::raw::c_int as std::os::raw::c_ulong,
               ::std::mem::size_of::<lh_table>() as std::os::raw::c_ulong) as
            *mut lh_table;
    if t.is_null() { return 0 as *mut lh_table }
    (*t).count = 0 as std::os::raw::c_int;
    (*t).size = size;
    (*t).table =
        calloc(size as std::os::raw::c_ulong,
               ::std::mem::size_of::<lh_entry>() as std::os::raw::c_ulong) as
            *mut lh_entry;
    if (*t).table.is_null() {
        free(t as *mut libc::c_void);
        return 0 as *mut lh_table
    }
    (*t).free_fn = free_fn;
    (*t).hash_fn = hash_fn;
    (*t).equal_fn = equal_fn;
    i = 0 as std::os::raw::c_int;
    while i < size {
        let ref mut fresh1 = (*(*t).table.offset(i as isize)).k;
        *fresh1 = -(1 as std::os::raw::c_int) as *mut libc::c_void;
        i += 1
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn lh_kchar_table_new(mut size: std::os::raw::c_int,
                                            mut free_fn:
                                                Option<lh_entry_free_fn>)
 -> *mut lh_table {
    return lh_table_new(size, free_fn, char_hash_fn,
                        Some(lh_char_equal as
                                 unsafe extern "C" fn(_: *const libc::c_void,
                                                      _: *const libc::c_void)
                                     -> std::os::raw::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn lh_kptr_table_new(mut size: std::os::raw::c_int,
                                           mut free_fn:
                                               Option<lh_entry_free_fn>)
 -> *mut lh_table {
    return lh_table_new(size, free_fn,
                        Some(lh_ptr_hash as
                                 unsafe extern "C" fn(_: *const libc::c_void)
                                     -> std::os::raw::c_ulong),
                        Some(lh_ptr_equal as
                                 unsafe extern "C" fn(_: *const libc::c_void,
                                                      _: *const libc::c_void)
                                     -> std::os::raw::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_resize(mut t: *mut lh_table,
                                         mut new_size: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut new_t: *mut lh_table = 0 as *mut lh_table;
    let mut ent: *mut lh_entry = 0 as *mut lh_entry;
    new_t = lh_table_new(new_size, None, (*t).hash_fn, (*t).equal_fn);
    if new_t.is_null() { return -(1 as std::os::raw::c_int) }
    ent = (*t).head;
    while !ent.is_null() {
        let mut h: std::os::raw::c_ulong = lh_get_hash(new_t, (*ent).k);
        let mut opts: std::os::raw::c_uint = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        if (*ent).k_is_constant != 0 {
            opts = ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as std::os::raw::c_uint
        }
        if lh_table_insert_w_hash(new_t, (*ent).k, (*ent).v, h, opts) !=
               0 as std::os::raw::c_int {
            lh_table_free(new_t);
            return -(1 as std::os::raw::c_int)
        }
        ent = (*ent).next
    }
    free((*t).table as *mut libc::c_void);
    (*t).table = (*new_t).table;
    (*t).size = new_size;
    (*t).head = (*new_t).head;
    (*t).tail = (*new_t).tail;
    free(new_t as *mut libc::c_void);
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_free(mut t: *mut lh_table) {
    let mut c: *mut lh_entry = 0 as *mut lh_entry;
    if (*t).free_fn.is_some() {
        c = (*t).head;
        while !c.is_null() {
            (*t).free_fn.expect("non-null function pointer")(c);
            c = (*c).next
        }
    }
    free((*t).table as *mut libc::c_void);
    free(t as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_insert_w_hash(mut t: *mut lh_table,
                                                mut k: *const libc::c_void,
                                                mut v: *const libc::c_void,
                                                h: std::os::raw::c_ulong,
                                                opts: std::os::raw::c_uint)
 -> std::os::raw::c_int {
    let mut n: std::os::raw::c_ulong = 0;
    if (*t).count as std::os::raw::c_double >= (*t).size as std::os::raw::c_double * 0.66f64 {
        if lh_table_resize(t, (*t).size * 2 as std::os::raw::c_int) !=
               0 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
    }
    n = h.wrapping_rem((*t).size as std::os::raw::c_ulong);
    while !((*(*t).table.offset(n as isize)).k ==
                -(1 as std::os::raw::c_int) as *mut libc::c_void as
                    *const libc::c_void ||
                (*(*t).table.offset(n as isize)).k ==
                    -(2 as std::os::raw::c_int) as *mut libc::c_void as
                        *const libc::c_void) {
        n = n.wrapping_add(1);
        if n as std::os::raw::c_int == (*t).size {
            n = 0 as std::os::raw::c_int as std::os::raw::c_ulong
        }
    }
    let ref mut fresh2 = (*(*t).table.offset(n as isize)).k;
    *fresh2 = k;
    (*(*t).table.offset(n as isize)).k_is_constant =
        (opts & ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int) as std::os::raw::c_uint) as
            std::os::raw::c_int;
    let ref mut fresh3 = (*(*t).table.offset(n as isize)).v;
    *fresh3 = v;
    (*t).count += 1;
    if (*t).head.is_null() {
        (*t).tail = &mut *(*t).table.offset(n as isize) as *mut lh_entry;
        (*t).head = (*t).tail;
        let ref mut fresh4 = (*(*t).table.offset(n as isize)).prev;
        *fresh4 = 0 as *mut lh_entry;
        let ref mut fresh5 = (*(*t).table.offset(n as isize)).next;
        *fresh5 = *fresh4
    } else {
        (*(*t).tail).next =
            &mut *(*t).table.offset(n as isize) as *mut lh_entry;
        let ref mut fresh6 = (*(*t).table.offset(n as isize)).prev;
        *fresh6 = (*t).tail;
        let ref mut fresh7 = (*(*t).table.offset(n as isize)).next;
        *fresh7 = 0 as *mut lh_entry;
        (*t).tail = &mut *(*t).table.offset(n as isize) as *mut lh_entry
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_insert(mut t: *mut lh_table,
                                         mut k: *const libc::c_void,
                                         mut v: *const libc::c_void)
 -> std::os::raw::c_int {
    return lh_table_insert_w_hash(t, k, v, lh_get_hash(t, k),
                                  0 as std::os::raw::c_int as std::os::raw::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_entry_w_hash(mut t: *mut lh_table,
                                                      mut k:
                                                          *const libc::c_void,
                                                      h: std::os::raw::c_ulong)
 -> *mut lh_entry {
    let mut n: std::os::raw::c_ulong = h.wrapping_rem((*t).size as std::os::raw::c_ulong);
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    while count < (*t).size {
        if (*(*t).table.offset(n as isize)).k ==
               -(1 as std::os::raw::c_int) as *mut libc::c_void as *const libc::c_void
           {
            return 0 as *mut lh_entry
        }
        if (*(*t).table.offset(n as isize)).k !=
               -(2 as std::os::raw::c_int) as *mut libc::c_void as *const libc::c_void
               &&
               (*t).equal_fn.expect("non-null function pointer")((*(*t).table.offset(n
                                                                                         as
                                                                                         isize)).k,
                                                                 k) != 0 {
            return &mut *(*t).table.offset(n as isize) as *mut lh_entry
        }
        n = n.wrapping_add(1);
        if n as std::os::raw::c_int == (*t).size {
            n = 0 as std::os::raw::c_int as std::os::raw::c_ulong
        }
        count += 1
    }
    return 0 as *mut lh_entry;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_entry(mut t: *mut lh_table,
                                               mut k: *const libc::c_void)
 -> *mut lh_entry {
    return lh_table_lookup_entry_w_hash(t, k, lh_get_hash(t, k));
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup(mut t: *mut lh_table,
                                         mut k: *const libc::c_void)
 -> *const libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    lh_table_lookup_ex(t, k, &mut result);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_lookup_ex(mut t: *mut lh_table,
                                            mut k: *const libc::c_void,
                                            mut v: *mut *mut libc::c_void)
 -> json_bool {
    let mut e: *mut lh_entry = lh_table_lookup_entry(t, k);
    if !e.is_null() {
        if !v.is_null() { *v = (*e).v as uintptr_t as *mut libc::c_void }
        return 1 as std::os::raw::c_int
        //#warning "racy random seed initializtion if used by multiple threads"
        /* key found */
    }
    if !v.is_null() { *v = 0 as *mut libc::c_void }
    return 0 as std::os::raw::c_int;
    /* key not found */
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_delete_entry(mut t: *mut lh_table,
                                               mut e: *mut lh_entry)
 -> std::os::raw::c_int {
    let mut n: ptrdiff_t =
        e.offset_from((*t).table) as
            std::os::raw::c_long; /* CAW: fixed to be 64bit nice, still need the crazy negative case... */
    /* CAW: this is bad, really bad, maybe stack goes other direction on this machine... */
    if n < 0 as std::os::raw::c_int as std::os::raw::c_long { return -(2 as std::os::raw::c_int) }
    if (*(*t).table.offset(n as isize)).k ==
           -(1 as std::os::raw::c_int) as *mut libc::c_void as *const libc::c_void ||
           (*(*t).table.offset(n as isize)).k ==
               -(2 as std::os::raw::c_int) as *mut libc::c_void as *const libc::c_void
       {
        return -(1 as std::os::raw::c_int)
    }
    (*t).count -= 1;
    if (*t).free_fn.is_some() {
        (*t).free_fn.expect("non-null function pointer")(e);
    }
    let ref mut fresh8 = (*(*t).table.offset(n as isize)).v;
    *fresh8 = 0 as *const libc::c_void;
    let ref mut fresh9 = (*(*t).table.offset(n as isize)).k;
    *fresh9 = -(2 as std::os::raw::c_int) as *mut libc::c_void;
    if (*t).tail == &mut *(*t).table.offset(n as isize) as *mut lh_entry &&
           (*t).head == &mut *(*t).table.offset(n as isize) as *mut lh_entry {
        (*t).tail = 0 as *mut lh_entry;
        (*t).head = (*t).tail
    } else if (*t).head ==
                  &mut *(*t).table.offset(n as isize) as *mut lh_entry {
        (*(*(*t).head).next).prev = 0 as *mut lh_entry;
        (*t).head = (*(*t).head).next
    } else if (*t).tail ==
                  &mut *(*t).table.offset(n as isize) as *mut lh_entry {
        (*(*(*t).tail).prev).next = 0 as *mut lh_entry;
        (*t).tail = (*(*t).tail).prev
    } else {
        let ref mut fresh10 = (*(*(*t).table.offset(n as isize)).prev).next;
        *fresh10 = (*(*t).table.offset(n as isize)).next;
        let ref mut fresh11 = (*(*(*t).table.offset(n as isize)).next).prev;
        *fresh11 = (*(*t).table.offset(n as isize)).prev
    }
    let ref mut fresh12 = (*(*t).table.offset(n as isize)).prev;
    *fresh12 = 0 as *mut lh_entry;
    let ref mut fresh13 = (*(*t).table.offset(n as isize)).next;
    *fresh13 = *fresh12;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_delete(mut t: *mut lh_table,
                                         mut k: *const libc::c_void)
 -> std::os::raw::c_int {
    let mut e: *mut lh_entry = lh_table_lookup_entry(t, k);
    if e.is_null() { return -(1 as std::os::raw::c_int) }
    return lh_table_delete_entry(t, e);
}
#[no_mangle]
pub unsafe extern "C" fn lh_table_length(mut t: *mut lh_table)
 -> std::os::raw::c_int {
    return (*t).count;
}
