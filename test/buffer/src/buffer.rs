use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
impl Default for __va_list_tag {fn default() -> Self {Self {
gp_offset: Default::default(),
fp_offset: Default::default(),
overflow_arg_area: std::ptr::null_mut(),
reg_save_area: std::ptr::null_mut(),
}}}

pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut /* owning */ libc::c_char,
    pub data: *mut libc::c_char,
}
impl Default for buffer_t {fn default() -> Self {Self {
len: Default::default(),
alloc: std::ptr::null_mut(),
data: std::ptr::null_mut(),
}}}
impl buffer_t {pub fn take(&mut self) -> Self {core::mem::take(self)}}

#[no_mangle]
pub unsafe extern "C" fn buffer_new() -> Option<Box<buffer_t>> {
    return buffer_new_with_size(64 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_size(mut n: size_t) -> Option<Box<buffer_t>> {
    let mut self_0 = Some(Box::new(<crate::src::buffer::buffer_t as Default>::default()));
    if self_0.as_deref().is_none() {();
        return None;
    }
    (*self_0.as_deref_mut().unwrap()).len= n;
    (*self_0.as_deref_mut().unwrap()).alloc= calloc(
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char; (*self_0.as_deref_mut().unwrap()).data= (*self_0.as_deref().unwrap()).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string(
    mut str: *mut libc::c_char,
) -> Option<Box<buffer_t>> {
    return {let crown_promoted_local_0 = strlen(str);buffer_new_with_string_length(str, crown_promoted_local_0)};
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_string_length(
    mut str: *mut libc::c_char,
    mut len: size_t,
) -> Option<Box<buffer_t>> {
    let mut self_0 = Some(Box::new(<crate::src::buffer::buffer_t as Default>::default()));
    if self_0.as_deref().is_none() {();
        return None;
    }
    (*self_0.as_deref_mut().unwrap()).len= len;
    (*self_0.as_deref_mut().unwrap()).alloc= str; (*self_0.as_deref_mut().unwrap()).data= (*self_0.as_deref().unwrap()).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new_with_copy(
    mut str: *mut libc::c_char,
) -> Option<Box<buffer_t>> {
    let mut len = strlen(str);
    let mut self_0 = buffer_new_with_size(len);
    if self_0.as_deref().is_none() {();
        return None;
    }
    memcpy((*self_0.as_deref().unwrap()).alloc as *mut libc::c_void, str as *const libc::c_void, len);
    (*self_0.as_deref_mut().unwrap()).data= (*self_0.as_deref().unwrap()).alloc;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_compact(mut self_0: Option<&mut buffer_t>) -> ssize_t {
    let mut len = buffer_length(self_0.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    let mut rem = (*self_0.as_deref().unwrap()).len.wrapping_sub(len);
    let mut buf = calloc(
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        1 as libc::c_int as libc::c_ulong,
    ) as *mut libc::c_char;
    if buf.is_null() {();
        return -(1 as libc::c_int) as ssize_t;
    }
    memcpy(buf as *mut libc::c_void, (*self_0.as_deref().unwrap()).data as *const libc::c_void, len);
    free((*self_0.as_deref().unwrap()).alloc as *mut libc::c_void);
    (*self_0.as_deref_mut().unwrap()).len= len;
    (*self_0.as_deref_mut().unwrap()).alloc= buf; (*self_0.as_deref_mut().unwrap()).data= (*self_0.as_deref().unwrap()).alloc;
    return rem as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_free(mut self_0: Option<Box<buffer_t>>) {
    free((*self_0.as_deref().unwrap()).alloc as *mut libc::c_void);
    ();
}
#[no_mangle]
pub unsafe extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> size_t {
    return (*self_0).len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> size_t {
    return strlen((*self_0).data);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_resize(
    mut self_0: Option<&mut buffer_t>,
    mut n: size_t,
) -> libc::c_int {
    n= n.wrapping_add((1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        & !(1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    (*self_0.as_deref_mut().unwrap()).len= n;
    (*self_0.as_deref_mut().unwrap()).data= realloc(
        (*self_0.as_deref().unwrap()).alloc as *mut libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char; (*self_0.as_deref_mut().unwrap()).alloc= (*self_0.as_deref().unwrap()).data;
    if (*self_0.as_deref().unwrap()).alloc.is_null() {();
        return -(1 as libc::c_int);
    }
    *(*self_0.as_deref().unwrap()).alloc.offset(n as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
// #[no_mangle]
// pub unsafe extern "C" fn buffer_appendf(
//     mut self_0: *mut buffer_t,
//     mut format: *const libc::c_char,
//     mut args: ...
// ) -> libc::c_int {
//     let mut ap: ::std::ffi::VaListImpl;
//     let mut tmpa: ::std::ffi::VaListImpl;
//     let mut dst = 0 as *mut libc::c_char;
//     let mut length = 0 as libc::c_int;
//     let mut required = 0 as libc::c_int;
//     let mut bytes = 0 as libc::c_int;
//     ap = args.clone();
//     length = buffer_length(self_0) as libc::c_int;
//     tmpa = ap.clone();
//     required = vsnprintf(
//         0 as *mut libc::c_char,
//         0 as libc::c_int as libc::c_ulong,
//         format,
//         tmpa.as_va_list(),
//     );
//     if -(1 as libc::c_int) == buffer_resize(self_0, (length + required) as size_t) {
//         return -(1 as libc::c_int);
//     }
//     dst = ((*self_0).data).offset(length as isize);
//     bytes = vsnprintf(
//         dst,
//         (1 as libc::c_int + required) as libc::c_ulong,
//         format,
//         ap.as_va_list(),
//     );
//     return if bytes < 0 as libc::c_int { -(1 as libc::c_int) } else { 0 as libc::c_int };
// }
#[no_mangle]
pub unsafe extern "C" fn buffer_append(
    mut self_0: Option<&mut buffer_t>,
    mut str: *const libc::c_char,
) -> libc::c_int {
    return {let crown_promoted_local_0 = strlen(str);buffer_append_n(self_0.as_deref_mut(), str, crown_promoted_local_0)};
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append_n(
    mut self_0: Option<&mut buffer_t>,
    mut str: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut prev = strlen((*self_0.as_deref().unwrap()).data);
    let mut needed = len.wrapping_add(prev);
    if (*self_0.as_deref().unwrap()).len > needed {
        strncat((*self_0.as_deref().unwrap()).data, str, len);
        return 0 as libc::c_int;
    }
    let mut ret = buffer_resize(self_0.as_deref_mut(), needed);
    if -(1 as libc::c_int) == ret {
        return -(1 as libc::c_int);
    }
    strncat((*self_0.as_deref().unwrap()).data, str, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_prepend(
    mut self_0: Option<&mut buffer_t>,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len = strlen(str);
    let mut prev = strlen((*self_0.as_deref().unwrap()).data);
    let mut needed = len.wrapping_add(prev);
    if !((*self_0.as_deref().unwrap()).len > needed) {
        ret= buffer_resize(self_0.as_deref_mut(), needed);
        if -(1 as libc::c_int) == ret {
            return -(1 as libc::c_int);
        }
    }
    memmove(
        (*self_0.as_deref().unwrap()).data.offset(len as isize) as *mut libc::c_void,
        (*self_0.as_deref().unwrap()).data as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    memcpy((*self_0.as_deref().unwrap()).data as *mut libc::c_void, str as *const libc::c_void, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_slice(
    mut buf: *mut buffer_t,
    mut from: size_t,
    mut to: ssize_t,
) -> Option<Box<buffer_t>> {
    let mut len = strlen((*buf).data);
    if (to as libc::c_ulong) < from {
        return None;
    }
    if to < 0 as libc::c_int as libc::c_long {
        to= len.wrapping_sub(!to as libc::c_ulong) as ssize_t;
    }
    if to as libc::c_ulong > len {
        to= len as ssize_t;
    }
    let mut n = (to as libc::c_ulong).wrapping_sub(from);
    let mut self_0 = buffer_new_with_size(n);
    memcpy(
        (*self_0.as_deref().unwrap()).data as *mut libc::c_void,
        (*buf).data.offset(from as isize) as *const libc::c_void,
        n,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_equals(
    mut self_0: *mut buffer_t,
    mut other: *mut buffer_t,
) -> libc::c_int {
    return (0 as libc::c_int == strcmp((*self_0).data, (*other).data)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_indexof(
    mut self_0: *mut buffer_t,
    mut str: *mut libc::c_char,
) -> ssize_t {
    let mut sub = strstr((*self_0).data, str);
    if sub.is_null() {();
        return -(1 as libc::c_int) as ssize_t;
    }
    return sub.offset_from((*self_0).data) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_left(mut self_0: Option<&mut buffer_t>) {
    let mut c: libc::c_int = 0;
    loop {
        c= (*(*self_0.as_deref().unwrap()).data) as libc::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
        (*self_0.as_deref_mut().unwrap()).data= (*self_0.as_deref().unwrap()).data.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim_right(mut self_0: Option<&mut buffer_t>) {
    let mut c: libc::c_int = 0;
    let mut i = (buffer_length(self_0.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()))).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        c= *(*self_0.as_deref().unwrap()).data.offset(i as isize) as libc::c_int;
        if !(c != 0
            && *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
        let fresh10 = i;
        i= i.wrapping_sub(1);
        *(*self_0.as_deref().unwrap()).data.offset(fresh10 as isize) = 0 as libc::c_int as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn buffer_trim(mut self_0: Option<&mut buffer_t>) {
    buffer_trim_left(self_0.as_deref_mut());
    buffer_trim_right(self_0.as_deref_mut());
}
#[no_mangle]
pub unsafe extern "C" fn buffer_fill(mut self_0: Option<&mut buffer_t>, mut c: libc::c_int) {
    memset((*self_0.as_deref().unwrap()).data as *mut libc::c_void, c, (*self_0.as_deref().unwrap()).len);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_clear(mut self_0: Option<&mut buffer_t>) {
    buffer_fill(self_0.as_deref_mut(), 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_print(mut self_0: *mut buffer_t) {
    let mut len = (*self_0).len;
    printf(b"\n \0" as *const u8 as *const libc::c_char);
    let mut i = 0 as libc::c_int;
    while (i as libc::c_ulong) < len {
        printf(
            b" %02x\0" as *const u8 as *const libc::c_char,
            *(*self_0).alloc.offset(i as isize) as libc::c_int,
        );
        if (i + 1 as libc::c_int) % 8 as libc::c_int == 0 as libc::c_int {
            printf(b"\n \0" as *const u8 as *const libc::c_char);
        }
        i+= 1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
