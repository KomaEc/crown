use ::libc;
extern "C" {
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: u64) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u64;
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
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

pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut libc::c_char,
    pub data: *mut libc::c_char,
}
impl Default for buffer_t {
    fn default() -> Self {
        Self {
            len: Default::default(),
            alloc: std::ptr::null_mut(),
            data: std::ptr::null_mut(),
        }
    }
}

// prototypes
/*
 * Allocate a new buffer with BUFFER_DEFAULT_SIZE.
 */
pub fn buffer_new() -> Option<Box<buffer_t>> {
    return buffer_new_with_size(64 as i32 as size_t);
}
/*
 * Allocate a new buffer with `n` bytes.
 */
pub fn buffer_new_with_size(mut n: size_t) -> Option<Box<buffer_t>> {
    let mut self_0 = Some(Box::new(<buffer_t as Default>::default()));
    if self_0.as_deref().is_none() {
        return None;
    }
    (*self_0.as_deref_mut().unwrap()).len = n;
    (*self_0.as_deref_mut().unwrap()).alloc =
        unsafe { calloc(n.wrapping_add(1 as i32 as u64), 1 as i32 as u64) as *mut libc::c_char };
    (*self_0.as_deref_mut().unwrap()).data = (*self_0.as_deref_mut().unwrap()).alloc;
    return self_0;
}
/*
 * Allocate a new buffer with `str`.
 */
pub fn buffer_new_with_string(mut str: *mut libc::c_char) -> Option<Box<buffer_t>> {
    return buffer_new_with_string_length(str, unsafe { strlen(str) });
}
/*
 * Allocate a new buffer with `str` and `len`.
 */
pub fn buffer_new_with_string_length(
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> Option<Box<buffer_t>> {
    let mut self_0 = Some(Box::new(<buffer_t as Default>::default()));
    if self_0.as_deref().is_none() {
        return None;
    }
    (*self_0.as_deref_mut().unwrap()).len = len;
    (*self_0.as_deref_mut().unwrap()).alloc = str;
    (*self_0.as_deref_mut().unwrap()).data = (*self_0.as_deref_mut().unwrap()).alloc;
    return self_0;
}
/*
 * Allocate a new buffer with a copy of `str`.
 */
pub fn buffer_new_with_copy(mut str: *mut libc::c_char) -> Option<Box<buffer_t>> {
    let mut len = unsafe { strlen(str) };
    let mut self_0 = buffer_new_with_size(len);
    if self_0.as_deref().is_none() {
        return None;
    }
    unsafe {
        memcpy(
            (*self_0.as_deref_mut().unwrap()).alloc as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
    }
    (*self_0.as_deref_mut().unwrap()).data = (*self_0.as_deref_mut().unwrap()).alloc;
    return self_0;
}
/*
 * Deallocate excess memory, the number
 * of bytes removed or -1.
 */
pub fn buffer_compact(mut self_0: Option<&mut buffer_t>) -> ssize_t {
    let mut len = buffer_length(self_0.as_deref());
    let mut rem = (*self_0.as_deref().unwrap()).len.wrapping_sub(len);
    let mut buf =
        unsafe { calloc(len.wrapping_add(1 as i32 as u64), 1 as i32 as u64) as *mut libc::c_char };
    if buf.is_null() {
        return -(1 as i32) as ssize_t;
    }
    unsafe {
        memcpy(
            buf as *mut libc::c_void,
            (*self_0.as_deref().unwrap()).data as *const libc::c_void,
            len,
        );
        free((*self_0.as_deref_mut().unwrap()).alloc as *mut libc::c_void);
    }
    (*self_0.as_deref_mut().unwrap()).len = len;
    (*self_0.as_deref_mut().unwrap()).alloc = buf;
    (*self_0.as_deref_mut().unwrap()).data = (*self_0.as_deref_mut().unwrap()).alloc;
    return rem as ssize_t;
}
/*
 * Free the buffer.
 */
pub fn buffer_free(mut self_0: Option<Box<buffer_t>>) {
    unsafe {
        free((*self_0.as_deref_mut().unwrap()).alloc as *mut libc::c_void);
    }
    // free(self_0 as *mut libc::c_void);
    let _ = self_0;
}
/*
 * Return buffer size.
 */
pub fn buffer_size(mut self_0: Option<&buffer_t>) -> size_t {
    return (*self_0.clone().unwrap()).len;
}
/*
 * Return string length.
 */
pub fn buffer_length(mut self_0: Option<&buffer_t>) -> size_t {
    return unsafe { strlen((*self_0.clone().unwrap()).data) };
}
/*
 * Resize to hold `n` bytes.
 */
pub fn buffer_resize(mut self_0: Option<&mut buffer_t>, mut n: size_t) -> i32 {
    n = n.wrapping_add((1024 as i32 - 1 as i32) as u64) & !(1024 as i32 - 1 as i32) as u64;
    (*self_0.as_deref_mut().unwrap()).len = n;
    (*self_0.as_deref_mut().unwrap()).data = unsafe {
        realloc(
            (*self_0.as_deref_mut().unwrap()).alloc as *mut libc::c_void,
            n.wrapping_add(1 as i32 as u64),
        ) as *mut libc::c_char
    };
    (*self_0.as_deref_mut().unwrap()).alloc = (*self_0.as_deref_mut().unwrap()).data;
    if (*self_0.as_deref_mut().unwrap()).alloc.is_null() {
        return -(1 as i32);
    }
    unsafe {
        *(*self_0.as_deref_mut().unwrap()).alloc.offset(n as isize) = '\u{0}' as i32 as libc::c_char
    };
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
pub fn buffer_append(mut self_0: Option<&mut buffer_t>, mut str: *const libc::c_char) -> i32 {
    return buffer_append_n(self_0.as_deref_mut(), str, unsafe { strlen(str) });
}
/*
 * Append the first `len` bytes from `str` to `self` and
 * return 0 on success, -1 on failure.
 */
pub fn buffer_append_n(
    mut self_0: Option<&mut buffer_t>,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> i32 {
    let mut prev = unsafe { strlen((*self_0.as_deref().unwrap()).data) };
    let mut needed = len.wrapping_add(prev);
    // enough space
    if (*self_0.as_deref().unwrap()).len > needed {
        unsafe { strncat((*self_0.as_deref_mut().unwrap()).data, str, len) };
        return 0 as i32;
    }
    // resize
    let mut ret = buffer_resize(self_0.as_deref_mut(), needed);
    if -(1 as i32) == ret {
        return -(1 as i32);
    }
    unsafe { strncat((*self_0.as_deref_mut().unwrap()).data, str, len) };
    return 0 as i32;
}
/*
 * Prepend `str` to `self` and return 0 on success, -1 on failure.
 */
pub fn buffer_prepend(mut self_0: Option<&mut buffer_t>, mut str: *mut libc::c_char) -> i32 {
    let mut ret: i32 = 0;
    let mut len = unsafe { strlen(str) };
    let mut prev = unsafe { strlen((*self_0.as_deref().unwrap()).data) };
    let mut needed = len.wrapping_add(prev);
    // enough space
    if !((*self_0.as_deref().unwrap()).len > needed) {
        // resize
        ret = buffer_resize(self_0.as_deref_mut(), needed);
        if -(1 as i32) == ret {
            return -(1 as i32);
        }
    }
    // move
    unsafe {
        memmove(
            (*self_0.as_deref_mut().unwrap()).data.offset(len as isize) as *mut libc::c_void,
            (*self_0.as_deref().unwrap()).data as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        memcpy(
            (*self_0.as_deref_mut().unwrap()).data as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
    }
    return 0 as i32;
}
/*
 * Return a new buffer based on the `from..to` slice of `buf`,
 * or NULL on error.
 */
pub fn buffer_slice(
    mut buf: Option<&buffer_t>,
    mut from: size_t,
    mut to: ssize_t,
) -> Option<Box<buffer_t>> {
    let mut len = unsafe { strlen((*buf.clone().unwrap()).data) };
    // bad range
    if (to as u64) < from {
        return None;
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
    unsafe {
        memcpy(
            (*self_0.as_deref_mut().unwrap()).data as *mut libc::c_void,
            (*buf.clone().unwrap()).data.offset(from as isize) as *const libc::c_void,
            n,
        );
    }
    return self_0;
}
/*
 * Return 1 if the buffers contain equivalent data.
 */
pub fn buffer_equals(mut self_0: Option<&buffer_t>, mut other: Option<&buffer_t>) -> i32 {
    return (0 as i32
        == unsafe {
            strcmp(
                (*self_0.clone().unwrap()).data,
                (*other.clone().unwrap()).data,
            )
        }) as i32;
}
/*
 * Return the index of the substring `str`, or -1 on failure.
 */
pub fn buffer_indexof(mut self_0: Option<&buffer_t>, mut str: *mut libc::c_char) -> ssize_t {
    let mut sub = unsafe { strstr((*self_0.clone().unwrap()).data, str) };
    if sub.is_null() {
        return -(1 as i32) as ssize_t;
    }
    return unsafe { sub.offset_from((*self_0.clone().unwrap()).data) as libc::c_long };
}
/*
 * Trim leading whitespace.
 */
pub fn buffer_trim_left(mut self_0: Option<&mut buffer_t>) {
    let mut c: i32 = 0;
    loop {
        c = unsafe { *(*self_0.as_deref_mut().unwrap()).data as i32 };
        if !(c != 0 && char::from_u32(c as u32).unwrap().is_whitespace())
        /* *(*__ctype_b_loc()).offset(c as isize) as i32
        & _ISspace as i32 as libc::c_ushort as i32
        != 0)
        */
        {
            break;
        }
        (*self_0.as_deref_mut().unwrap()).data =
            unsafe { (*self_0.as_deref_mut().unwrap()).data.offset(1) }
    }
}
/*
 * Trim trailing whitespace.
 */
pub fn buffer_trim_right(mut self_0: Option<&mut buffer_t>) {
    let mut c: i32 = 0;
    let mut i = buffer_length(self_0.as_deref()).wrapping_sub(1 as i32 as u64);
    loop {
        c = unsafe { *(*self_0.as_deref_mut().unwrap()).data.offset(i as isize) as i32 };
        if !(c != 0 && char::from_u32(c as u32).unwrap().is_whitespace()) {
            break;
        }
        let fresh0 = i;
        i = i.wrapping_sub(1);
        unsafe {
            *(*self_0.as_deref_mut().unwrap())
                .data
                .offset(fresh0 as isize) = 0 as i32 as libc::c_char
        }
    }
}
/*
 * Trim trailing and leading whitespace.
 */
pub fn buffer_trim(mut self_0: Option<&mut buffer_t>) {
    buffer_trim_left(self_0.as_deref_mut());
    buffer_trim_right(self_0.as_deref_mut());
}
/*
 * Fill the buffer with `c`.
 */
pub fn buffer_fill(mut self_0: Option<&mut buffer_t>, mut c: i32) {
    unsafe {
        memset(
            (*self_0.as_deref_mut().unwrap()).data as *mut libc::c_void,
            c,
            (*self_0.as_deref().unwrap()).len,
        );
    }
}
/*
 * Fill the buffer with 0.
 */
pub fn buffer_clear(mut self_0: Option<&mut buffer_t>) {
    // buffer_fill(&raw mut *self_0, 0 as i32);
    buffer_fill(self_0.as_deref_mut(), 0 as i32);
}
/*
 * Print a hex dump of the buffer.
 */
pub fn buffer_print(mut self_0: Option<&buffer_t>) {
    let mut i: i32 = 0;
    let mut len = (*self_0.clone().unwrap()).len;
    unsafe { printf(b"\n \x00" as *const u8 as *const libc::c_char) };
    // hex
    i = 0 as i32;
    while (i as u64) < len {
        unsafe {
            printf(
                b" %02x\x00" as *const u8 as *const libc::c_char,
                *(*self_0.clone().unwrap()).alloc.offset(i as isize) as i32,
            )
        };
        if (i + 1 as i32) % 8 as i32 == 0 as i32 {
            unsafe { printf(b"\n \x00" as *const u8 as *const libc::c_char) };
        }
        i += 1
    }
    unsafe { printf(b"\n\x00" as *const u8 as *const libc::c_char) };
}
