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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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

#[derive(Clone)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: Option<Vec<libc::c_char>>,
    pub data: *mut libc::c_char,
}

impl Default for buffer_t {
    fn default() -> Self {
        Self {
            len: Default::default(),
            alloc: Default::default(),
            data: std::ptr::null_mut(),
        }
    }
}

pub fn buffer_new() -> Box<buffer_t> {
    return buffer_new_with_size(64u64);
}

pub fn buffer_new_with_size(mut n: size_t) -> Box<buffer_t> {
    let mut self_0 = Box::new(buffer_t::default());
    self_0.len = n;
    self_0.alloc = Some(vec![0; n.wrapping_add(1) as usize]);
    self_0.data = self_0
        .alloc
        .as_mut()
        .map_or(std::ptr::null_mut(), |v| v.as_mut_ptr());
    self_0
}

pub fn buffer_new_with_string(str: Box<[libc::c_char]>) -> Box<buffer_t> {
    let ptr = str.as_ptr();
    return buffer_new_with_string_length(str, unsafe { strlen(ptr) });
}

pub fn buffer_new_with_string_length(str: Box<[libc::c_char]>, len: size_t) -> Box<buffer_t> {
    let mut self_0 = Box::new(buffer_t::default());
    self_0.len = len;
    self_0.alloc = Some(str.into());
    self_0.data = self_0
        .alloc
        .as_mut()
        .map_or(std::ptr::null_mut(), |v| v.as_mut_ptr());
    return self_0;
}

pub fn buffer_new_with_copy(str: &[libc::c_char]) -> Box<buffer_t> {
    let mut len = unsafe { strlen(str.as_ptr()) };
    let mut self_0 = buffer_new_with_size(len);
    self_0
        .alloc
        .as_mut()
        .map(|v| v[..len as usize].copy_from_slice(str))
        .unwrap();
    self_0.data = self_0
        .alloc
        .as_mut()
        .map_or(std::ptr::null_mut(), |v| v.as_mut_ptr());
    return self_0;
}

pub fn buffer_compact(self_0: &mut buffer_t) -> ssize_t {
    let len = buffer_length(self_0);
    let rem = self_0.len - len;
    let mut buf = vec![0i8; len as usize + 1];
    buf.copy_from_slice(self_0.alloc.as_ref().map(|slice| &slice[..]).unwrap());
    let _ = std::mem::take(&mut self_0.alloc);
    self_0.len = len;
    self_0.alloc = Some(buf);
    self_0.data = self_0
        .alloc
        .as_mut()
        .map_or(std::ptr::null_mut(), |v| v.as_mut_ptr());
    return rem as ssize_t;
}

pub fn buffer_free(_self_0: Box<buffer_t>) {}

pub fn buffer_size(self_0: &buffer_t) -> size_t {
    return self_0.len;
}

pub fn buffer_length(self_0: &buffer_t) -> size_t {
    return unsafe { strlen(self_0.data) };
}

pub fn buffer_resize(self_0: &mut buffer_t, mut n: size_t) -> i32 {
    n = n.wrapping_add((1024 as i32 - 1 as i32) as u64) & !(1024 as i32 - 1 as i32) as u64;
    self_0.len = n;
    // !!!! FIXED
    self_0
        .alloc
        .as_mut()
        .map(|v| v.resize(n.wrapping_add(1u64) as usize, 0));
    self_0.data = self_0
        .alloc
        .as_mut()
        .map_or(std::ptr::null_mut(), |v| v.as_mut_ptr());
    if self_0.alloc.is_none() {
        return -1i32;
    }
    self_0.alloc.as_mut().unwrap()[n as usize] = '\u{0}' as i32 as libc::c_char;
    return 0i32;
}

pub fn buffer_append(self_0: &mut buffer_t, str: *mut libc::c_char) -> i32 {
    return buffer_append_n(self_0, str, unsafe { strlen(str) });
}

pub fn buffer_append_n(self_0: &mut buffer_t, str: *mut libc::c_char, len: size_t) -> i32 {
    let prev = unsafe { strlen(self_0.data) };
    let needed = len.wrapping_add(prev);
    if self_0.len > needed {
        unsafe {
            strncat(self_0.data, str, len);
            return 0i32;
        }
    }
    let mut ret = buffer_resize(self_0, needed);
    if -1i32 == ret {
        return -1i32;
    }
    return 0i32;
}

pub fn buffer_prepend(self_0: &mut buffer_t, str: *mut libc::c_char) -> i32 {
    let mut ret = 0i32;
    let len = unsafe { strlen(str) };
    let prev = unsafe { strlen(self_0.data) };
    let needed = len.wrapping_add(prev);
    if !self_0.len > needed {
        ret = buffer_resize(self_0, needed);
        if -1i32 == ret {
            return -1i32;
        }
    }
    unsafe {
        std::ptr::copy(
            self_0.data,
            self_0.data.offset(len as isize),
            len.wrapping_add(1) as usize,
        )
    };
    unsafe { std::ptr::copy_nonoverlapping(str, self_0.data, len as usize) };
    return 0i32;
}

pub fn buffer_slice(buf: &buffer_t, from: size_t, mut to: ssize_t) -> Option<Box<buffer_t>> {
    let len = unsafe { strlen(buf.data) };
    if (to as u64) < from {
        return None;
    }
    if to < 0 as libc::c_long {
        to = len.wrapping_sub(!to as u64) as ssize_t
    }
    if to as u64 > len {
        to = len as ssize_t
    }
    let n = (to as u64).wrapping_sub(from);
    let self_0 = buffer_new_with_size(n);
    unsafe {
        std::ptr::copy_nonoverlapping(buf.data.offset(from as isize), self_0.data, n as usize)
    };
    return Some(self_0);
}

pub fn buffer_equals(self_0: &buffer_t, other: &buffer_t) -> i32 {
    return (0i32 == unsafe { strcmp(self_0.data, other.data) }) as i32;
}

pub fn buffer_indexof(self_0: &buffer_t, str: *mut libc::c_char) -> ssize_t {
    let sub = unsafe { strstr(self_0.data, str) };
    if sub.is_null() {
        return -1i32 as ssize_t;
    }
    return unsafe { sub.offset_from(self_0.data) } as libc::c_long;
}
