use ::libc;
extern "C" {
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: u64) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    #[no_mangle]
    fn malloc(_: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = u64;
pub type __ssize_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type ssize_t = __ssize_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut libc::c_char,
    pub data: *mut libc::c_char,
}
// prototypes
/*
 * Allocate a new buffer with BUFFER_DEFAULT_SIZE.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(64 as i32 as size_t);
}
/*
 * Allocate a new buffer with `n` bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_size(mut n: size_t) -> *mut buffer_t {
    let mut self_0 = malloc(::std::mem::size_of::<buffer_t>() as u64) as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = n;
    (*self_0).alloc = calloc(n.wrapping_add(1 as i32 as u64), 1 as i32 as u64) as *mut libc::c_char;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
/*
 * Allocate a new buffer with `str`.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string(mut str: *mut libc::c_char) -> *mut buffer_t {
    return buffer_new_with_string_length(str, strlen(str));
}
/*
 * Allocate a new buffer with `str` and `len`.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string_length(
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> *mut buffer_t {
    let mut self_0 = malloc(::std::mem::size_of::<buffer_t>() as u64) as *mut buffer_t;
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    (*self_0).len = len;
    (*self_0).alloc = str;
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
/*
 * Allocate a new buffer with a copy of `str`.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_copy(mut str: *mut libc::c_char) -> *mut buffer_t {
    let mut len = strlen(str);
    let mut self_0 = buffer_new_with_size(len);
    if self_0.is_null() {
        return 0 as *mut buffer_t;
    }
    memcpy(
        (*self_0).alloc as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    (*self_0).data = (*self_0).alloc;
    return self_0;
}
/*
 * Deallocate excess memory, the number
 * of bytes removed or -1.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_compact(mut self_0: *mut buffer_t) -> ssize_t {
    let mut len = buffer_length(self_0);
    let mut rem = (*self_0).len.wrapping_sub(len);
    let mut buf = calloc(len.wrapping_add(1 as i32 as u64), 1 as i32 as u64) as *mut libc::c_char;
    if buf.is_null() {
        return -(1 as i32) as ssize_t;
    }
    memcpy(
        buf as *mut libc::c_void,
        (*self_0).data as *const libc::c_void,
        len,
    );
    free((*self_0).alloc as *mut libc::c_void);
    (*self_0).len = len;
    (*self_0).alloc = buf;
    (*self_0).data = (*self_0).alloc;
    return rem as ssize_t;
}
/*
 * Free the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_free(mut self_0: *mut buffer_t) {
    free((*self_0).alloc as *mut libc::c_void);
    free(self_0 as *mut libc::c_void);
}
/*
 * Return buffer size.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> size_t {
    return (*self_0).len;
}
/*
 * Return string length.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> size_t {
    return strlen((*self_0).data);
}
/*
 * Resize to hold `n` bytes.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_resize(mut self_0: *mut buffer_t, mut n: size_t) -> i32 {
    n = n.wrapping_add((1024 as i32 - 1 as i32) as u64) & !(1024 as i32 - 1 as i32) as u64;
    (*self_0).len = n;

    (*self_0).alloc = realloc(
        (*self_0).alloc as *mut libc::c_void,
        n.wrapping_add(1 as i32 as u64),
    ) as *mut libc::c_char;
    (*self_0).data = (*self_0).alloc;
    /* 
    (*self_0).data = realloc(
        (*self_0).alloc as *mut libc::c_void,
        n.wrapping_add(1 as i32 as u64),
    ) as *mut libc::c_char;
    (*self_0).alloc = (*self_0).data;
    */
    if (*self_0).alloc.is_null() {
        return -(1 as i32);
    }
    *(*self_0).alloc.offset(n as isize) = '\u{0}' as i32 as libc::c_char;
    return 0 as i32;
}
/*
 * Append a printf-style formatted string to the buffer.
 */
/*
int buffer_appendf(buffer_t *self, const char *format, ...) {
  va_list ap;
  va_list tmpa;
  char *dst = NULL;
  int length = 0;
  int required = 0;
  int bytes = 0;

  va_start(ap, format);

  length = buffer_length(self);

  // First, we compute how many bytes are needed
  // for the formatted string and allocate that
  // much more space in the buffer.
  va_copy(tmpa, ap);
  required = vsnprintf(NULL, 0, format, tmpa);
  va_end(tmpa);
  if (-1 == buffer_resize(self, length + required)) {
    va_end(ap);
    return -1;
  }

  // Next format the string into the space that we
  // have made room for.
  dst = self->data + length;
  bytes = vsnprintf(dst, 1 + required, format, ap);
  va_end(ap);

  return bytes < 0
    ? -1
    : 0;
}
*/
/*
 * Append `str` to `self` and return 0 on success, -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_append(
    mut self_0: *mut buffer_t,
    mut str: *const libc::c_char,
) -> i32 {
    return buffer_append_n(self_0, str, strlen(str));
}
/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_append_n(
    mut self_0: *mut buffer_t,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> i32 {
    let mut prev = strlen((*self_0).data);
    let mut needed = len.wrapping_add(prev);
    // enough space
    if (*self_0).len > needed {
        strncat((*self_0).data, str, len);
        return 0 as i32;
    }
    // resize
    let mut ret = buffer_resize(self_0, needed);
    if -(1 as i32) == ret {
        return -(1 as i32);
    }
    strncat((*self_0).data, str, len);
    return 0 as i32;
}
/*
 * Prepend `str` to `self` and return 0 on success, -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_prepend(
    mut self_0: *mut buffer_t,
    mut str: *mut libc::c_char,
) -> i32 {
    let mut ret: i32 = 0;
    let mut len = strlen(str);
    let mut prev = strlen((*self_0).data);
    let mut needed = len.wrapping_add(prev);
    // enough space
    if !((*self_0).len > needed) {
        // resize
        ret = buffer_resize(self_0, needed);
        if -(1 as i32) == ret {
            return -(1 as i32);
        }
    }
    // move
    memmove(
        (*self_0).data.offset(len as isize) as *mut libc::c_void,
        (*self_0).data as *const libc::c_void,
        len.wrapping_add(1 as i32 as u64),
    );
    memcpy(
        (*self_0).data as *mut libc::c_void,
        str as *const libc::c_void,
        len,
    );
    return 0 as i32;
}
/*
 * Return a new buffer based on the `from..to` slice of `buf`,
 * or NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_slice(
    mut buf: *mut buffer_t,
    mut from: size_t,
    mut to: ssize_t,
) -> *mut buffer_t {
    let mut len = strlen((*buf).data);
    // bad range
    if (to as u64) < from {
        return 0 as *mut buffer_t;
    }
    // relative to end
    if to < 0 as i32 as libc::c_long {
        to = len.wrapping_sub(!to as u64) as ssize_t
    }
    // cap end
    if to as u64 > len {
        to = len as ssize_t
    }
    let mut n = (to as u64).wrapping_sub(from);
    let mut self_0 = buffer_new_with_size(n);
    memcpy(
        (*self_0).data as *mut libc::c_void,
        (*buf).data.offset(from as isize) as *const libc::c_void,
        n,
    );
    return self_0;
}
/*
 * Return 1 if the buffers contain equivalent data.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_equals(mut self_0: *mut buffer_t, mut other: *mut buffer_t) -> i32 {
    return (0 as i32 == strcmp((*self_0).data, (*other).data)) as i32;
}
/*
 * Return the index of the substring `str`, or -1 on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_indexof(
    mut self_0: *mut buffer_t,
    mut str: *mut libc::c_char,
) -> ssize_t {
    let mut sub = strstr((*self_0).data, str);
    if sub.is_null() {
        return -(1 as i32) as ssize_t;
    }
    return sub.offset_from((*self_0).data) as libc::c_long;
}
/*
 * Trim leading whitespace.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_left(mut self_0: *mut buffer_t) {
    let mut c: i32 = 0;
    loop {
        c = *(*self_0).data as i32;
        if !(c != 0
            && char::from_u32(c as u32).unwrap().is_whitespace())
            /* *(*__ctype_b_loc()).offset(c as isize) as i32
                & _ISspace as i32 as libc::c_ushort as i32
                != 0)
                */
        {
            break;
        }
        (*self_0).data = (*self_0).data.offset(1)
    }
}
/*
 * Trim trailing whitespace.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_right(mut self_0: *mut buffer_t) {
    let mut c: i32 = 0;
    let mut i = buffer_length(self_0).wrapping_sub(1 as i32 as u64);
    loop {
        c = *(*self_0).data.offset(i as isize) as i32;
        if !(c != 0
            && char::from_u32(c as u32).unwrap().is_whitespace())
        {
            break;
        }
        let fresh0 = i;
        i = i.wrapping_sub(1);
        *(*self_0).data.offset(fresh0 as isize) = 0 as i32 as libc::c_char
    }
}
/*
 * Trim trailing and leading whitespace.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_trim(mut self_0: *mut buffer_t) {
    buffer_trim_left(self_0);
    buffer_trim_right(self_0);
}
/*
 * Fill the buffer with `c`.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_fill(mut self_0: *mut buffer_t, mut c: i32) {
    memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
}
/*
 * Fill the buffer with 0.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_clear(mut self_0: *mut buffer_t) {
    buffer_fill(self_0, 0 as i32);
}
/*
 * Print a hex dump of the buffer.
 */
#[no_mangle]
pub unsafe extern "C" fn buffer_print(mut self_0: *mut buffer_t) {
    let mut i: i32 = 0;
    let mut len = (*self_0).len;
    printf(b"\n \x00" as *const u8 as *const libc::c_char);
    // hex
    i = 0 as i32;
    while (i as u64) < len {
        printf(
            b" %02x\x00" as *const u8 as *const libc::c_char,
            *(*self_0).alloc.offset(i as isize) as i32,
        );
        if (i + 1 as i32) % 8 as i32 == 0 as i32 {
            printf(b"\n \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
