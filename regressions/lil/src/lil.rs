use ::libc;
extern "C" {
    
    
    pub type _IO_marker;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoll(__nptr: *const libc::c_char) -> libc::c_longlong;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: libc::c_long,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: libc::c_long,
    pub _codecvt: *mut crate::src::main::_IO_codecvt,
    pub _wide_data: *mut crate::src::main::_IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: libc::c_ulong,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type int64_t = __int64_t;
pub type lilint_t = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct _lil_value_t {
    pub l: libc::c_ulong,
    pub d: *mut /* owning */ libc::c_char,
}
impl Default for _lil_value_t {fn default() -> Self {Self {
l: Default::default(),
d: std::ptr::null_mut(),
}}}
impl _lil_value_t {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type lil_value_t = *mut _lil_value_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_func_t {
    pub name: *mut libc::c_char,
    pub code: *mut _lil_value_t,
    pub argnames: *mut _lil_list_t,
    pub proc_0: Option::<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>,
}
pub type lil_func_proc_t = Option::<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>;
pub type lil_t = *mut _lil_t;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer1;
#[repr(C)]
pub struct _lil_t {
    pub code: *const libc::c_char,
    pub rootcode: *const libc::c_char,
    pub clen: libc::c_ulong,
    pub head: libc::c_ulong,
    pub ignoreeol: libc::c_int,
    pub cmd: *mut *mut _lil_func_t,
    pub cmds: libc::c_ulong,
    pub syscmds: libc::c_ulong,
    pub catcher: *mut libc::c_char,
    pub in_catcher: libc::c_int,
    pub dollarprefix: *mut /* owning */ libc::c_char,
    pub env: *mut /* owning */ _lil_env_t,
    pub rootenv: *mut _lil_env_t,
    pub downenv: *mut _lil_env_t,
    pub empty: *mut /* owning */ _lil_value_t,
    pub error: libc::c_int,
    pub err_head: libc::c_ulong,
    pub err_msg: *mut libc::c_char,
    pub callback: [Option::<unsafe extern "C" fn() -> ()>; 8],
    pub parse_depth: libc::c_ulong,
    pub data: *mut libc::c_void,
}
impl Default for _lil_t {fn default() -> Self {Self {
code: std::ptr::null(),
rootcode: std::ptr::null(),
clen: Default::default(),
head: Default::default(),
ignoreeol: Default::default(),
cmd: std::ptr::null_mut(),
cmds: Default::default(),
syscmds: Default::default(),
catcher: std::ptr::null_mut(),
in_catcher: Default::default(),
dollarprefix: std::ptr::null_mut(),
env: std::ptr::null_mut(),
rootenv: std::ptr::null_mut(),
downenv: std::ptr::null_mut(),
empty: std::ptr::null_mut(),
error: Default::default(),
err_head: Default::default(),
err_msg: std::ptr::null_mut(),
callback: [Default::default(); 8],
parse_depth: Default::default(),
data: std::ptr::null_mut(),
}}}
impl _lil_t {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type lil_callback_proc_t = Option::<unsafe extern "C" fn() -> ()>;
pub type lil_env_t = *mut _lil_env_t;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer2;
#[repr(C)]
pub struct _lil_env_t {
    pub parent: *mut /* owning */ _lil_env_t,
    pub func: *mut _lil_func_t,
    pub catcher_for: *mut _lil_value_t,
    pub var: *mut *mut _lil_var_t,
    pub vars: libc::c_ulong,
    pub retval: *mut _lil_value_t,
    pub retval_set: libc::c_int,
    pub breakrun: libc::c_int,
}
impl Default for _lil_env_t {fn default() -> Self {Self {
parent: std::ptr::null_mut(),
func: std::ptr::null_mut(),
catcher_for: std::ptr::null_mut(),
var: std::ptr::null_mut(),
vars: Default::default(),
retval: std::ptr::null_mut(),
retval_set: Default::default(),
breakrun: Default::default(),
}}}
impl _lil_env_t {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type lil_var_t = *mut _lil_var_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_var_t {
    pub n: *mut libc::c_char,
    pub env: *mut _lil_env_t,
    pub v: *mut _lil_value_t,
}
pub type lil_func_t = *mut _lil_func_t;
pub type lil_list_t = *mut _lil_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_list_t {
    pub v: *mut *mut _lil_value_t,
    pub c: libc::c_ulong,
}
pub type lil_exit_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, lil_value_t) -> (),
>;
pub type lil_write_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char) -> (),
>;
pub type lil_read_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char) -> *mut libc::c_char,
>;
pub type lil_source_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char) -> *mut libc::c_char,
>;
pub type lil_store_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char, *const libc::c_char) -> (),
>;
pub type lil_error_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, size_t, *const libc::c_char) -> (),
>;
pub type lil_setvar_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char, *mut lil_value_t) -> libc::c_int,
>;
pub type lil_getvar_callback_proc_t = Option::<
    unsafe extern "C" fn(lil_t, *const libc::c_char, *mut lil_value_t) -> libc::c_int,
>;
pub type expreval_t = _expreval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _expreval_t {
    pub code: *const libc::c_char,
    pub len: libc::c_ulong,
    pub head: libc::c_ulong,
    pub ival: libc::c_long,
    pub dval: libc::c_double,
    pub type_0: libc::c_int,
    pub error: libc::c_int,
}
unsafe extern "C" fn strclone(mut s: *const libc::c_char) -> *mut /* owning */ libc::c_char {
    let mut len = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut ns = malloc(len) as *mut libc::c_char;
    if ns.is_null() {();
        return 0 as *mut libc::c_char;
    }
    memcpy(ns as *mut libc::c_void, s as *const libc::c_void, len);
    return ns;
}
unsafe extern "C" fn alloc_value(mut str: *const libc::c_char) -> *mut /* owning */ _lil_value_t {
    let mut val = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_value_t>() as libc::c_ulong,
    ) as lil_value_t;
    if val.is_null() {();
        return 0 as lil_value_t;
    }
    if !str.is_null() {
        (*val).l= strlen(str);
        (*val).d= malloc((*val).l.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if (*val).d.is_null() {();
            free(val as *mut libc::c_void);
            return 0 as lil_value_t;
        }
        memcpy(
            (*val).d as *mut libc::c_void,
            str as *const libc::c_void,
            (*val).l.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {();
        (*val).l= 0 as libc::c_int as size_t;
        (*val).d= 0 as *mut libc::c_char;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_clone_value(mut src: *mut _lil_value_t) -> *mut /* owning */ _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    if src.is_null() {();
        return 0 as lil_value_t;
    }
    val= calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_value_t>() as libc::c_ulong,
    ) as lil_value_t;
    if val.is_null() {();
        return 0 as lil_value_t;
    }
    (*val).l= (*src).l;
    if (*src).l != 0 {
        (*val).d= malloc((*val).l.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if (*val).d.is_null() {();
            free(val as *mut libc::c_void);
            return 0 as lil_value_t;
        }
        memcpy(
            (*val).d as *mut libc::c_void,
            (*src).d as *const i8 as *const libc::c_void,
            (*val).l.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        (*val).d= 0 as *mut libc::c_char;
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_char(
    mut val: *mut _lil_value_t,
    mut ch: libc::c_char,
) -> libc::c_int {
    let mut new = realloc(
        (*val).d as *mut libc::c_void,
        (*val).l.wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if new.is_null() {();
        return 0 as libc::c_int;
    }
    let fresh5 = (*val).l;(*val).l= (*val).l.wrapping_add(1);
    *new.offset(fresh5 as isize) = ch;
    *new.offset((*val).l as isize) = 0 as libc::c_int as libc::c_char;
    (*val).d= new;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_string(
    mut val: Option<&mut _lil_value_t>,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut new = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if s.is_null() || *s.offset(0 as libc::c_int as isize) == 0 {
        return 1 as libc::c_int;
    }
    len= strlen(s);
    new= realloc(
        (*val.as_deref().unwrap()).d as *mut libc::c_void,
        (*val.as_deref().unwrap()).l.wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if new.is_null() {();
        return 0 as libc::c_int;
    }
    memcpy(
        new.offset((*val.as_deref().unwrap()).l as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*val.as_deref_mut().unwrap()).l= ((*val.as_deref().unwrap()).l as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    (*val.as_deref_mut().unwrap()).d= new;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_append_val(
    mut val: *mut _lil_value_t,
    mut v: *mut _lil_value_t,
) -> libc::c_int {
    let mut new = 0 as *mut libc::c_char;
    if v.is_null() || (*v).l == 0 {
        return 1 as libc::c_int;
    }
    new= realloc(
        (*val).d as *mut libc::c_void,
        (*val).l.wrapping_add((*v).l).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if new.is_null() {();
        return 0 as libc::c_int;
    }
    memcpy(
        new.offset((*val).l as isize) as *mut libc::c_void,
        (*v).d as *const i8 as *const libc::c_void,
        (*v).l.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*val).l= ((*val).l as libc::c_ulong).wrapping_add((*v).l) as size_t as size_t;
    (*val).d= new;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_value(mut val: *mut /* owning */ _lil_value_t) {
    if val.is_null() {();
        return;
    }
    free((*val).d as *mut libc::c_void);
    free(val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_list() -> *mut /* owning */ _lil_list_t {
    let mut list = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_list_t>() as libc::c_ulong,
    ) as lil_list_t;
    (*list).v= 0 as *mut lil_value_t;
    (*list).c= 0 as libc::c_int as size_t;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_list(mut list: *mut /* owning */ _lil_list_t) {
    let mut i: size_t = 0;
    if list.is_null() {();
        return;
    }
    i= 0 as libc::c_int as size_t;
    while i < (*list).c {
        lil_free_value(*(*list).v.offset(i as isize));
        i= i.wrapping_add(1);
    }
    free((*list).v as *mut libc::c_void);
    free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_append(mut list: *mut _lil_list_t, mut val: *mut _lil_value_t) {
    let mut nv = realloc(
        (*list).v as *mut libc::c_void,
        (::std::mem::size_of::<lil_value_t>() as libc::c_ulong)
            .wrapping_mul((*list).c.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut lil_value_t;
    if nv.is_null() {();
        return;
    }
    (*list).v= nv;
    let fresh14 = (*list).c;(*list).c= (*list).c.wrapping_add(1);
    *nv.offset(fresh14 as isize) = val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_size(mut list: *mut _lil_list_t) -> libc::c_ulong {
    return (*list).c;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_get(
    mut list: *mut _lil_list_t,
    mut index: libc::c_ulong,
) -> *mut _lil_value_t {
    return if index >= (*list).c {
        0 as lil_value_t
    } else {
        *(*list).v.offset(index as isize)
    };
}
unsafe extern "C" fn needs_escape(mut str: *const libc::c_char) -> libc::c_int {
    let mut i: size_t = 0;
    if str.is_null() || *str.offset(0 as libc::c_int as isize) == 0 {
        return 1 as libc::c_int;
    }
    i= 0 as libc::c_int as size_t;
    while *str.offset(i as isize) != 0 {
        if *(*__ctype_b_loc()).offset(*str.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
            != 0
            || *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            return 1 as libc::c_int;
        }
        i= i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_list_to_value(
    mut list: *mut _lil_list_t,
    mut do_escape: libc::c_int,
) -> *mut /* owning */ _lil_value_t {
    let mut val = alloc_value(0 as *const libc::c_char);
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < (*list).c {
        let mut escape = if do_escape != 0 {
            needs_escape(lil_to_string(*(*list).v.offset(i as isize)))
        } else {
            0 as libc::c_int
        };
        if i != 0 {
            lil_append_char(val, ' ' as i32 as libc::c_char);
        }
        if escape != 0 {
            lil_append_char(val, '{' as i32 as libc::c_char);
        }
        lil_append_val(val, *(*list).v.offset(i as isize));
        if escape != 0 {
            lil_append_char(val, '}' as i32 as libc::c_char);
        }
        i= i.wrapping_add(1);
    }
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_env(mut parent: *mut /* owning */ _lil_env_t) -> *mut /* owning */ _lil_env_t {
    let mut env = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_env_t>() as libc::c_ulong,
    ) as lil_env_t;
    (*env).parent= parent;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_free_env(mut env: *mut /* owning */ _lil_env_t) {
    let mut i: size_t = 0;
    if env.is_null() {();
        return;
    }
    lil_free_value((*env).retval);
    i= 0 as libc::c_int as size_t;
    while i < (*env).vars {
        free((**(*env).var.offset(i as isize)).n as *mut libc::c_void);
        lil_free_value((**(*env).var.offset(i as isize)).v);
        free(*(*env).var.offset(i as isize) as *mut libc::c_void);
        i= i.wrapping_add(1);
    }
    free((*env).var as *mut libc::c_void);
    free(env as *mut libc::c_void);
}
unsafe extern "C" fn lil_find_local_var(
    mut lil: *mut _lil_t,
    mut env: *mut _lil_env_t,
    mut name: *const libc::c_char,
) -> *mut _lil_var_t {
    if (*env).vars > 0 as libc::c_int as libc::c_ulong {
        let mut i = (*env).vars.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            if strcmp((**(*env).var.offset(i as isize)).n, name) == 0 {
                return *(*env).var.offset(i as isize);
            }
            if i == 0 {
                break;
            }
            i= i.wrapping_sub(1);
        }
    }
    return 0 as lil_var_t;
}
unsafe extern "C" fn lil_find_var(
    mut lil: *mut _lil_t,
    mut env: *mut _lil_env_t,
    mut name: *const libc::c_char,
) -> *mut _lil_var_t {
    let mut r = lil_find_local_var(lil, env, name);
    return if !r.is_null() {
        r
    } else if env == (*lil).rootenv {
        0 as lil_var_t
    } else {
        lil_find_var(lil, (*lil).rootenv, name)
    };
}
unsafe extern "C" fn find_cmd(
    mut lil: Option<&mut _lil_t>,
    mut name: *const libc::c_char,
) -> *mut _lil_func_t {
    if (*lil.as_deref().unwrap()).cmds > 0 as libc::c_int as libc::c_ulong {
        let mut i = (*lil.as_deref().unwrap()).cmds.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        loop {
            if strcmp((**(*lil.as_deref().unwrap()).cmd.offset(i as isize)).name, name) == 0 {
                return *(*lil.as_deref().unwrap()).cmd.offset(i as isize);
            }
            if i == 0 {
                break;
            }
            i= i.wrapping_sub(1);
        }
    }
    return 0 as lil_func_t;
}
unsafe extern "C" fn add_func(
    mut lil: Option<&mut _lil_t>,
    mut name: *const libc::c_char,
) -> *mut _lil_func_t {
    let mut cmd = 0 as *mut _lil_func_t;
    let mut ncmd = 0 as *mut lil_func_t;
    cmd= find_cmd(lil.as_deref_mut(), name);
    if !cmd.is_null() {
        return cmd;
    }else { (); }
    cmd= calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_func_t>() as libc::c_ulong,
    ) as lil_func_t;
    (*cmd).name= strclone(name);
    ncmd= realloc(
        (*lil.as_deref().unwrap()).cmd as *mut libc::c_void,
        (::std::mem::size_of::<lil_func_t>() as libc::c_ulong)
            .wrapping_mul((*lil.as_deref().unwrap()).cmds.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut lil_func_t;
    if ncmd.is_null() {();
        free(cmd as *mut libc::c_void);
        return 0 as lil_func_t;
    }
    (*lil.as_deref_mut().unwrap()).cmd= ncmd;
    let fresh20 = (*lil.as_deref().unwrap()).cmds;(*lil.as_deref_mut().unwrap()).cmds= (*lil.as_deref().unwrap()).cmds.wrapping_add(1);
    *ncmd.offset(fresh20 as isize) = cmd;
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn lil_register(
    mut lil: Option<&mut _lil_t>,
    mut name: *const libc::c_char,
    mut proc_0: Option::<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>,
) -> libc::c_int {
    let mut cmd = add_func(lil.as_deref_mut(), name);
    if cmd.is_null() {();
        return 0 as libc::c_int;
    }
    (*cmd).proc_0= proc_0;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_var(
    mut lil: *mut _lil_t,
    mut name: *const libc::c_char,
    mut val: *mut _lil_value_t,
    mut local: libc::c_int,
) -> *mut _lil_var_t {
    let mut nvar = 0 as *mut lil_var_t;
    let mut env = if local == 0 as libc::c_int { (*lil).rootenv } else { (*lil).env };
    let mut freeval = 0 as libc::c_int;
    if *name.offset(0 as libc::c_int as isize) == 0 {
        return 0 as lil_var_t;
    }
    if local != 2 as libc::c_int {
        let mut var = lil_find_var(lil, env, name);
        if local == 3 as libc::c_int && !var.is_null() && (*var).env == (*lil).rootenv
            && (*var).env != env
        {
            var= 0 as lil_var_t;
        }
        if (var.is_null() && env == (*lil).rootenv
            || !var.is_null() && (*var).env == (*lil).rootenv)
            && ((*lil).callback[6 as libc::c_int as usize]).is_some()
        {
            let mut proc_0: lil_setvar_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_setvar_callback_proc_t,
            >((*lil).callback[6 as libc::c_int as usize]);
            let mut newval = val;
            let mut r = proc_0
                .expect("non-null function pointer")(lil, name, core::ptr::addr_of_mut!(newval));
            if r < 0 as libc::c_int {
                return 0 as lil_var_t;
            }
            if r != 0 {
                val= newval;
                freeval= 1 as libc::c_int;
            }
        }
        if !var.is_null() {
            lil_free_value((*var).v);
            (*var).v= if freeval != 0 { val } else { lil_clone_value(val) };
            return var;
        }else { (); }
    }
    nvar= realloc(
        (*env).var as *mut libc::c_void,
        (::std::mem::size_of::<lil_var_t>() as libc::c_ulong)
            .wrapping_mul((*env).vars.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut lil_var_t;
    if nvar.is_null() {();
        return 0 as lil_var_t;
    }
    (*env).var= nvar;
    *nvar.offset((*env).vars as isize) = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_var_t>() as libc::c_ulong,
    ) as lil_var_t;
    (**nvar.offset((*env).vars as isize)).n = strclone(name);
    (**nvar.offset((*env).vars as isize)).env = env;
    (**nvar.offset((*env).vars as isize)).v = if freeval != 0 { val } else { lil_clone_value(val) };
    let fresh30 = (*env).vars;(*env).vars= (*env).vars.wrapping_add(1);
    return *nvar.offset(fresh30 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var(
    mut lil: Option<&mut _lil_t>,
    mut name: *const libc::c_char,
) -> *mut _lil_value_t {
    return lil_get_var_or(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), name, (*lil.as_deref().unwrap()).empty);
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_var_or(
    mut lil: *mut _lil_t,
    mut name: *const libc::c_char,
    mut defvalue: *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut var = lil_find_var(lil, (*lil).env, name);
    let mut retval = if !var.is_null() { (*var).v } else {(); defvalue };
    if ((*lil).callback[7 as libc::c_int as usize]).is_some()
        && (var.is_null() || (*var).env == (*lil).rootenv)
    {
        let mut proc_0: lil_getvar_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_getvar_callback_proc_t,
        >((*lil).callback[7 as libc::c_int as usize]);
        let mut newretval = retval;
        if proc_0.expect("non-null function pointer")(lil, name, core::ptr::addr_of_mut!(newretval)) != 0 {
            retval= newretval;
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn lil_push_env(mut lil: Option<&mut _lil_t>) -> *mut /* owning */ _lil_env_t {
    let mut env = lil_alloc_env((*lil.as_deref_mut().unwrap()).env);
    (*lil.as_deref_mut().unwrap()).env= env;
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn lil_pop_env(mut lil: Option<&mut _lil_t>) {
    if !(*(*lil.as_deref().unwrap()).env).parent.is_null() {
        let mut next = (*(*lil.as_deref_mut().unwrap()).env).parent;
        lil_free_env((*lil.as_deref_mut().unwrap()).env);
        (*lil.as_deref_mut().unwrap()).env= next;
    }else { (); }
}
#[no_mangle]
pub unsafe extern "C" fn lil_new() -> *mut /* owning */ _lil_t {
    let mut lil = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<_lil_t>() as libc::c_ulong,
    ) as lil_t;
    (*lil).env= lil_alloc_env(0 as lil_env_t); (*lil).rootenv= (*lil).env;
    (*lil).empty= alloc_value(0 as *const libc::c_char);
    (*lil).dollarprefix= strclone(b"set \0" as *const u8 as *const libc::c_char);
    register_stdcmds(lil.as_mut());
    return lil;
}
unsafe extern "C" fn islilspecial(mut ch: libc::c_char) -> libc::c_int {
    return (ch as libc::c_int == ';' as i32 || ch as libc::c_int == '$' as i32
        || ch as libc::c_int == '[' as i32 || ch as libc::c_int == ']' as i32
        || ch as libc::c_int == '{' as i32 || ch as libc::c_int == '}' as i32
        || ch as libc::c_int == '"' as i32 || ch as libc::c_int == '\'' as i32)
        as libc::c_int;
}
unsafe extern "C" fn ateol(mut lil: *mut _lil_t) -> libc::c_int {
    return ((*lil).ignoreeol == 0
        && (*(*lil).code.offset((*lil).head as isize) as libc::c_int == '\n' as i32
            || *(*lil).code.offset((*lil).head as isize) as libc::c_int == '\r' as i32
            || *(*lil).code.offset((*lil).head as isize) as libc::c_int == ';' as i32))
        as libc::c_int;
}
unsafe extern "C" fn skip_spaces(mut lil: Option<&mut _lil_t>) {
    while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen
        && (*(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '\\' as i32
            || *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '#' as i32
            || *(*__ctype_b_loc())
                .offset(
                    *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                && ((*lil.as_deref().unwrap()).ignoreeol != 0
                    || !(*(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                        == '\r' as i32
                        || *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                            == '\n' as i32)))
    {
        if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '#' as i32 {
            while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen && ateol(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) == 0 {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            }
        } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
            == '\\' as i32
            && (*(*lil.as_deref().unwrap()).code
                .offset(
                    (*lil.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize,
                ) as libc::c_int == '\r' as i32
                || *(*lil.as_deref().unwrap()).code
                    .offset(
                        (*lil.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '\n' as i32)
        {
            (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen && ateol(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            }
        } else {
            (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn get_bracketpart(mut lil: Option<&mut _lil_t>) -> *mut /* owning */ _lil_value_t {
    let mut cnt = 1 as libc::c_int as size_t;
    let mut val = 0 as *mut _lil_value_t;
    let mut cmd = alloc_value(0 as *const libc::c_char);
    (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
    while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen {
        if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '[' as i32 {
            (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            cnt= cnt.wrapping_add(1);
            lil_append_char(cmd, '[' as i32 as libc::c_char);
        } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
            == ']' as i32
        {
            (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            cnt= cnt.wrapping_sub(1);
            if cnt == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            lil_append_char(cmd, ']' as i32 as libc::c_char);
        } else {
            let fresh45 = (*lil.as_deref().unwrap()).head;(*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            lil_append_char(cmd, *(*lil.as_deref().unwrap()).code.offset(fresh45 as isize));
        }
    }
    val= lil_parse_value(lil.as_deref_mut(), cmd, 0 as libc::c_int);
    lil_free_value(cmd);
    return val;
}
unsafe extern "C" fn get_dollarpart(mut lil: Option<&mut _lil_t>) -> *mut /* owning */ _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    let mut name = 0 as *mut _lil_value_t;
    let mut tmp = 0 as *mut _lil_value_t;
    (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
    name= next_word(lil.as_deref_mut());
    tmp= alloc_value((*lil.as_deref().unwrap()).dollarprefix as *const i8);
    lil_append_val(tmp, name);
    lil_free_value(name);
    val= lil_parse_value(lil.as_deref_mut(), tmp, 0 as libc::c_int);
    lil_free_value(tmp);
    return val;
}
unsafe extern "C" fn next_word(mut lil: Option<&mut _lil_t>) -> *mut /* owning */ _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    skip_spaces(lil.as_deref_mut());
    if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '$' as i32 {
        val= get_dollarpart(lil.as_deref_mut());
    } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '{' as i32 {
        let mut cnt = 1 as libc::c_int as size_t;
        (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
        val= alloc_value(0 as *const libc::c_char);
        while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen {
            if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '{' as i32 {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
                cnt= cnt.wrapping_add(1);
                lil_append_char(val, '{' as i32 as libc::c_char);
            } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                == '}' as i32
            {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
                cnt= cnt.wrapping_sub(1);
                if cnt == 0 as libc::c_int as libc::c_ulong {
                    break;
                }
                lil_append_char(val, '}' as i32 as libc::c_char);
            } else {
                let fresh51 = (*lil.as_deref().unwrap()).head;(*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
                lil_append_char(val, *(*lil.as_deref().unwrap()).code.offset(fresh51 as isize));
            }
        }
    } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '[' as i32 {
        val= get_bracketpart(lil.as_deref_mut());
    } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '"' as i32
        || *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '\'' as i32
    {
        let fresh53 = (*lil.as_deref().unwrap()).head;(*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
        let mut sc = *(*lil.as_deref().unwrap()).code.offset(fresh53 as isize);
        val= alloc_value(0 as *const libc::c_char);
        while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen {
            if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int == '[' as i32
                || *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                    == '$' as i32
            {
                let mut tmp = if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize)
                    as libc::c_int == '$' as i32
                {
                    get_dollarpart(lil.as_deref_mut())
                } else {
                    get_bracketpart(lil.as_deref_mut())
                };
                lil_append_val(val, tmp);
                lil_free_value(tmp);
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_sub(1);
            } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                == '\\' as i32
            {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
                match  *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int {
                    98 => {
                        lil_append_char(val, '\u{8}' as i32 as libc::c_char);
                    }
                    116 => {
                        lil_append_char(val, '\t' as i32 as libc::c_char);
                    }
                    110 => {
                        lil_append_char(val, '\n' as i32 as libc::c_char);
                    }
                    118 => {
                        lil_append_char(val, '\u{b}' as i32 as libc::c_char);
                    }
                    102 => {
                        lil_append_char(val, '\u{c}' as i32 as libc::c_char);
                    }
                    114 => {
                        lil_append_char(val, '\r' as i32 as libc::c_char);
                    }
                    48 => {
                        lil_append_char(val, 0 as libc::c_int as libc::c_char);
                    }
                    97 => {
                        lil_append_char(val, '\u{7}' as i32 as libc::c_char);
                    }
                    99 => {
                        lil_append_char(val, '}' as i32 as libc::c_char);
                    }
                    111 => {
                        lil_append_char(val, '{' as i32 as libc::c_char);
                    }
                    _ => {
                        lil_append_char(
                            val,
                            *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize),
                        );
                    }
                }
            } else if *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int
                == sc as libc::c_int
            {
                (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
                break;
            } else {
                lil_append_char(val, *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize));
            }
            (*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
        }
    } else {
        val= alloc_value(0 as *const libc::c_char);
        while (*lil.as_deref().unwrap()).head < (*lil.as_deref().unwrap()).clen
            && *(*__ctype_b_loc())
                .offset(
                    *(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize) as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            && islilspecial(*(*lil.as_deref().unwrap()).code.offset((*lil.as_deref().unwrap()).head as isize)) == 0
        {
            let fresh59 = (*lil.as_deref().unwrap()).head;(*lil.as_deref_mut().unwrap()).head= (*lil.as_deref().unwrap()).head.wrapping_add(1);
            lil_append_char(val, *(*lil.as_deref().unwrap()).code.offset(fresh59 as isize));
        }
    }
    return if !val.is_null() { val } else {(); alloc_value(0 as *const libc::c_char) };
}
unsafe extern "C" fn substitute(mut lil: *mut _lil_t) -> *mut _lil_list_t {
    let mut words = lil_alloc_list();
    skip_spaces(lil.as_mut());
    while (*lil).head < (*lil).clen && ateol(lil) == 0 && (*lil).error == 0 {
        let mut w = alloc_value(0 as *const libc::c_char);
        loop {
            let mut head = (*lil).head;
            let mut wp = next_word(lil.as_mut());
            if head == (*lil).head {
                lil_free_value(w);
                lil_free_value(wp);
                lil_free_list(words);
                return 0 as lil_list_t;
            }
            lil_append_val(w, wp);
            lil_free_value(wp);
            if !((*lil).head < (*lil).clen && ateol(lil) == 0
                && *(*__ctype_b_loc())
                    .offset(
                        *(*lil).code.offset((*lil).head as isize) as libc::c_int
                            as isize,
                    ) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                && (*lil).error == 0)
            {
                break;
            }
        }
        skip_spaces(lil.as_mut());
        lil_list_append(words, w);
    }
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_list(
    mut lil: Option<&mut _lil_t>,
    mut code: *mut _lil_value_t,
) -> *mut /* owning */ _lil_list_t {
    let mut save_code = (*lil.as_deref().unwrap()).code;
    let mut save_clen = (*lil.as_deref().unwrap()).clen;
    let mut save_head = (*lil.as_deref().unwrap()).head;
    let mut save_igeol = (*lil.as_deref().unwrap()).ignoreeol;
    let mut words = 0 as *mut _lil_list_t;
    (*lil.as_deref_mut().unwrap()).code= lil_to_string(code);
    (*lil.as_deref_mut().unwrap()).clen= (*code).l;
    (*lil.as_deref_mut().unwrap()).head= 0 as libc::c_int as size_t;
    (*lil.as_deref_mut().unwrap()).ignoreeol= 1 as libc::c_int;
    words= substitute(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    (*lil.as_deref_mut().unwrap()).code= save_code;
    (*lil.as_deref_mut().unwrap()).clen= save_clen;
    (*lil.as_deref_mut().unwrap()).head= save_head;
    (*lil.as_deref_mut().unwrap()).ignoreeol= save_igeol;
    return words;
}
#[no_mangle]
pub unsafe extern "C" fn lil_subst_to_value(
    mut lil: Option<&mut _lil_t>,
    mut code: *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut words = lil_subst_to_list(lil.as_deref_mut(), code);
    let mut val = 0 as *mut _lil_value_t;
    if words.is_null() {();
        return lil_clone_value(code);
    }
    val= lil_list_to_value(words, 0 as libc::c_int);
    lil_free_list(words);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse(
    mut lil: *mut _lil_t,
    mut code: *const libc::c_char,
    mut codelen: libc::c_ulong,
    mut funclevel: libc::c_int,
) -> *mut _lil_value_t {
    let mut save_code = (*lil).code;
    let mut save_clen = (*lil).clen;
    let mut save_head = (*lil).head;
    let mut val = 0 as lil_value_t;
    let mut words = 0 as lil_list_t;
    if save_code.is_null() {();
        (*lil).rootcode= code;
    }
    (*lil).code= code;
    (*lil).clen= if codelen != 0 { codelen } else { strlen(code) };
    (*lil).head= 0 as libc::c_int as size_t;
    skip_spaces(lil.as_mut());
    (*lil).parse_depth= (*lil).parse_depth.wrapping_add(1);
    if (*lil).parse_depth == 1 as libc::c_int as libc::c_ulong {
        (*lil).error= 0 as libc::c_int;
    }
    if funclevel != 0 {
        (*(*lil).env).breakrun= 0 as libc::c_int;
    }
    while (*lil).head < (*lil).clen && (*lil).error == 0 {
        if !words.is_null() {
            lil_free_list(words);
        }else { (); }
        if !val.is_null() {
            lil_free_value(val);
        }else { (); }
        val= 0 as lil_value_t;
        words= substitute(lil);
        if words.is_null() || (*lil).error != 0 {
            break;
        }
        if (*words).c != 0 {
            let mut cmd = find_cmd(
                lil.as_mut(),
                lil_to_string(*(*words).v.offset(0 as libc::c_int as isize)),
            );
            if cmd.is_null() {();
                if (**(*words).v.offset(0 as libc::c_int as isize)).l != 0 {
                    if !(*lil).catcher.is_null() {
                        if (*lil).in_catcher < 16384 as libc::c_int {
                            let mut args = 0 as *mut _lil_value_t;
                            (*lil).in_catcher+= 1;
                            lil_push_env(lil.as_mut());
                            (*(*lil).env).catcher_for= *(*words).v.offset(0 as libc::c_int as isize);
                            args= lil_list_to_value(words, 1 as libc::c_int);
                            lil_set_var(
                                lil,
                                b"args\0" as *const u8 as *const libc::c_char,
                                args,
                                2 as libc::c_int,
                            );
                            lil_free_value(args);
                            val= lil_parse(
                                lil,
                                (*lil).catcher,
                                0 as libc::c_int as size_t,
                                1 as libc::c_int,
                            );
                            lil_pop_env(lil.as_mut());
                            (*lil).in_catcher-= 1;
                        } else {
                            let mut msg = malloc(
                                ((**(*words).v.offset(0 as libc::c_int as isize)).l)
                                    .wrapping_add(64 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            sprintf(
                                msg,
                                b"catcher limit reached while trying to call unknown function %s\0"
                                    as *const u8 as *const libc::c_char,
                                (**(*words).v.offset(0 as libc::c_int as isize)).d,
                            );
                            lil_set_error_at(lil.as_mut(), (*lil).head, msg);
                            free(msg as *mut libc::c_void);
                            break;
                        }
                    } else {();
                        let mut msg_0 = malloc(
                            ((**(*words).v.offset(0 as libc::c_int as isize)).l)
                                .wrapping_add(32 as libc::c_int as libc::c_ulong),
                        ) as *mut libc::c_char;
                        sprintf(
                            msg_0,
                            b"unknown function %s\0" as *const u8 as *const libc::c_char,
                            (**(*words).v.offset(0 as libc::c_int as isize)).d,
                        );
                        lil_set_error_at(lil.as_mut(), (*lil).head, msg_0);
                        free(msg_0 as *mut libc::c_void);
                        break;
                    }
                }
            }
            if !cmd.is_null() {
                if ((*cmd).proc_0).is_some() {
                    let mut shead = (*lil).head;
                    val= (*cmd).proc_0
                        .expect(
                            "non-null function pointer",
                        )(
                        lil,
                        (*words).c.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        (*words).v.offset(1 as libc::c_int as isize),
                    );
                    if (*lil).error == 2 as libc::c_int {
                        (*lil).error= 1 as libc::c_int;
                        (*lil).err_head= shead;
                    }
                } else {
                    lil_push_env(lil.as_mut());
                    (*(*lil).env).func= cmd;
                    if (*(*cmd).argnames).c == 1 as libc::c_int as libc::c_ulong
                        && strcmp(
                            lil_to_string(
                                *(*(*cmd).argnames).v.offset(0 as libc::c_int as isize),
                            ),
                            b"args\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        let mut args_0 = lil_list_to_value(words, 1 as libc::c_int);
                        lil_set_var(
                            lil,
                            b"args\0" as *const u8 as *const libc::c_char,
                            args_0,
                            2 as libc::c_int,
                        );
                        lil_free_value(args_0);
                    } else {
                        let mut i: size_t = 0;
                        i= 0 as libc::c_int as size_t;
                        while i < (*(*cmd).argnames).c {
                            lil_set_var(
                                lil,
                                lil_to_string(*(*(*cmd).argnames).v.offset(i as isize)),
                                if i
                                    < (*words).c
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                {
                                    *(*words).v
                                        .offset(
                                            i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                                        )
                                } else {
                                    (*lil).empty
                                },
                                2 as libc::c_int,
                            );
                            i= i.wrapping_add(1);
                        }
                    }
                    val= lil_parse_value(lil.as_mut(), (*cmd).code, 1 as libc::c_int);
                    lil_pop_env(lil.as_mut());
                }
            }else { (); }
        }
        if (*(*lil).env).breakrun != 0 {
            break;
        }
        skip_spaces(lil.as_mut());
        while ateol(lil) != 0 {
            (*lil).head= (*lil).head.wrapping_add(1);
        }
        skip_spaces(lil.as_mut());
    }
    if (*lil).error != 0 && ((*lil).callback[5 as libc::c_int as usize]).is_some()
        && (*lil).parse_depth == 1 as libc::c_int as libc::c_ulong
    {
        let mut proc_0: lil_error_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_error_callback_proc_t,
        >((*lil).callback[5 as libc::c_int as usize]);
        proc_0.expect("non-null function pointer")(lil, (*lil).err_head, (*lil).err_msg);
    }
    if !words.is_null() {
        lil_free_list(words);
    }else { (); }
    (*lil).code= save_code;
    (*lil).clen= save_clen;
    (*lil).head= save_head;
    if funclevel != 0 && (*(*lil).env).retval_set != 0 {
        if !val.is_null() {
            lil_free_value(val);
        }else { (); }
        val= (*(*lil).env).retval;
        (*(*lil).env).retval= 0 as lil_value_t;
        (*(*lil).env).retval_set= 0 as libc::c_int;
        (*(*lil).env).breakrun= 0 as libc::c_int;
    }
    (*lil).parse_depth= (*lil).parse_depth.wrapping_sub(1);
    return if !val.is_null() { val } else {(); alloc_value(0 as *const libc::c_char) };
}
#[no_mangle]
pub unsafe extern "C" fn lil_parse_value(
    mut lil: Option<&mut _lil_t>,
    mut val: *mut _lil_value_t,
    mut funclevel: libc::c_int,
) -> *mut /* owning */ _lil_value_t {
    if val.is_null() || (*val).d.is_null() || (*val).l == 0 {
        return alloc_value(0 as *const libc::c_char);
    }
    return lil_parse(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), (*val).d as *const i8, (*val).l, funclevel);
}
#[no_mangle]
pub unsafe extern "C" fn lil_callback(
    mut lil: Option<&mut _lil_t>,
    mut cb: libc::c_int,
    mut proc_0: Option::<unsafe extern "C" fn() -> ()>,
) {
    if cb < 0 as libc::c_int || cb > 8 as libc::c_int {
        return;
    }
    (*lil.as_deref_mut().unwrap()).callback[cb as usize]= proc_0;
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error(mut lil: Option<&mut _lil_t>, mut msg: *const libc::c_char) {
    if (*lil.as_deref().unwrap()).error != 0 {
        return;
    }
    free((*lil.as_deref().unwrap()).err_msg as *mut libc::c_void);
    (*lil.as_deref_mut().unwrap()).error= 2 as libc::c_int;
    (*lil.as_deref_mut().unwrap()).err_head= 0 as libc::c_int as size_t;
    (*lil.as_deref_mut().unwrap()).err_msg= strclone(
        if !msg.is_null() { msg } else {(); b"\0" as *const u8 as *const libc::c_char },
    );
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_error_at(
    mut lil: Option<&mut _lil_t>,
    mut pos: libc::c_ulong,
    mut msg: *const libc::c_char,
) {
    if (*lil.as_deref().unwrap()).error != 0 {
        return;
    }
    free((*lil.as_deref().unwrap()).err_msg as *mut libc::c_void);
    (*lil.as_deref_mut().unwrap()).error= 1 as libc::c_int;
    (*lil.as_deref_mut().unwrap()).err_head= pos;
    (*lil.as_deref_mut().unwrap()).err_msg= strclone(
        if !msg.is_null() { msg } else {(); b"\0" as *const u8 as *const libc::c_char },
    );
}
#[no_mangle]
pub unsafe extern "C" fn lil_error(
    mut lil: Option<&mut _lil_t>,
    mut msg: Option<&mut *const libc::c_char>,
    mut pos: Option<&mut libc::c_ulong>,
) -> libc::c_int {
    if (*lil.as_deref().unwrap()).error == 0 {
        return 0 as libc::c_int;
    }
    *msg.as_deref_mut().unwrap()= (*lil.as_deref().unwrap()).err_msg;
    *pos.as_deref_mut().unwrap()= (*lil.as_deref().unwrap()).err_head;
    (*lil.as_deref_mut().unwrap()).error= 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn ee_skip_spaces(mut ee: Option<&mut _expreval_t>) {
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len
        && *(*__ctype_b_loc())
            .offset(*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
    }
}
unsafe extern "C" fn ee_numeric_element(mut ee: Option<&mut _expreval_t>) {
    let mut fpart = 0 as libc::c_int as lilint_t;
    let mut fpartlen = 1 as libc::c_int as lilint_t;
    (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
    ee_skip_spaces(ee.as_deref_mut());
    (*ee.as_deref_mut().unwrap()).ival= 0 as libc::c_int as lilint_t;
    (*ee.as_deref_mut().unwrap()).dval= 0 as libc::c_int as libc::c_double;
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len {
        if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '.' as i32 {
            if (*ee.as_deref().unwrap()).type_0 == 1 as libc::c_int {
                break;
            }
            (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
            (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        } else if *(*__ctype_b_loc())
            .offset(*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            break;
        }
        if (*ee.as_deref().unwrap()).type_0 == 0 as libc::c_int {
            (*ee.as_deref_mut().unwrap()).ival= (*ee.as_deref().unwrap()).ival * 10 as libc::c_int as libc::c_long
                + (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int - '0' as i32)
                    as libc::c_long;
        } else {
            fpart= fpart * 10 as libc::c_int as libc::c_long
                + (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int - '0' as i32)
                    as libc::c_long;
            fpartlen*= 10 as libc::c_int as libc::c_long;
        }
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
    }
    if (*ee.as_deref().unwrap()).type_0 == 1 as libc::c_int {
        (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).ival as libc::c_double
            + fpart as libc::c_double / fpartlen as libc::c_double;
    }
}
unsafe extern "C" fn ee_element(mut ee: Option<&mut _expreval_t>) {
    if *(*__ctype_b_loc())
        .offset(*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        ee_numeric_element(ee.as_deref_mut());
        return;
    }
    (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
    (*ee.as_deref_mut().unwrap()).ival= 1 as libc::c_int as lilint_t;
    (*ee.as_deref_mut().unwrap()).error= 4 as libc::c_int;
}
unsafe extern "C" fn ee_paren(mut ee: Option<&mut _expreval_t>) {
    ee_skip_spaces(ee.as_deref_mut());
    if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '(' as i32 {
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        ee_expr(ee.as_deref_mut());
        ee_skip_spaces(ee.as_deref_mut());
        if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == ')' as i32 {
            (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        } else {
            (*ee.as_deref_mut().unwrap()).error= 1 as libc::c_int;
        }
    } else {
        ee_element(ee.as_deref_mut());
    };
}
unsafe extern "C" fn ee_unary(mut ee: Option<&mut _expreval_t>) {
    ee_skip_spaces(ee.as_deref_mut());
    if (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '-' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '+' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '~' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '!' as i32)
    {
        let fresh82 = (*ee.as_deref().unwrap()).head;(*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        let mut op = *(*ee.as_deref().unwrap()).code.offset(fresh82 as isize);
        ee_unary(ee.as_deref_mut());
        if (*ee.as_deref().unwrap()).error != 0 {
            return;
        }
        match  op as libc::c_int {
            45 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).dval= -(*ee.as_deref().unwrap()).dval;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= -(*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            126 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= !((*ee.as_deref().unwrap()).dval as lilint_t);
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= !(*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            33 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).dval= ((*ee.as_deref().unwrap()).dval == 0.) as libc::c_int as libc::c_double;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= ((*ee.as_deref().unwrap()).ival == 0) as libc::c_int as lilint_t;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            43 | _ => {}
        }
    } else {
        ee_paren(ee.as_deref_mut());
    };
}
unsafe extern "C" fn ee_muldiv(mut ee: Option<&mut _expreval_t>) {
    ee_unary(ee.as_deref_mut());
    if (*ee.as_deref().unwrap()).error != 0 {
        return;
    }
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && *(*__ctype_b_loc())
            .offset(
                *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int as isize,
            ) as libc::c_int & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '*' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '/' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '\\' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '%' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        match  *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int {
            42 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).dval * odval;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).ival as libc::c_double * odval;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).dval * oival as libc::c_double;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (*ee.as_deref().unwrap()).ival * oival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            37 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= fmod(odval, (*ee.as_deref().unwrap()).dval);
                                }
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= fmod(odval, (*ee.as_deref().unwrap()).ival as libc::c_double);
                                }
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= fmod(oival as libc::c_double, (*ee.as_deref().unwrap()).dval);
                                }
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).ival= oival % (*ee.as_deref().unwrap()).ival;
                                }
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {}
                }
            }
            47 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= odval / (*ee.as_deref().unwrap()).dval;
                                }
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= odval / (*ee.as_deref().unwrap()).ival as libc::c_double;
                                }
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= oival as libc::c_double / (*ee.as_deref().unwrap()).dval;
                                }
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).dval= oival as libc::c_double
                                        / (*ee.as_deref().unwrap()).ival as libc::c_double;
                                }
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {}
                }
            }
            92 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).ival= (odval / (*ee.as_deref().unwrap()).dval) as lilint_t;
                                }
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).ival= (odval / (*ee.as_deref().unwrap()).ival as libc::c_double) as lilint_t;
                                }
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_unary(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                if (*ee.as_deref().unwrap()).dval == 0.0f64 {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).ival= (oival as libc::c_double / (*ee.as_deref().unwrap()).dval) as lilint_t;
                                }
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                if (*ee.as_deref().unwrap()).ival == 0 as libc::c_int as libc::c_long {
                                    (*ee.as_deref_mut().unwrap()).error= 3 as libc::c_int;
                                } else {
                                    (*ee.as_deref_mut().unwrap()).ival= oival / (*ee.as_deref().unwrap()).ival;
                                }
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_addsub(mut ee: Option<&mut _expreval_t>) {
    ee_muldiv(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && *(*__ctype_b_loc())
            .offset(
                *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int as isize,
            ) as libc::c_int & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '+' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '-' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        match  *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int {
            43 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_muldiv(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).dval + odval;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).ival as libc::c_double + odval;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_muldiv(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= (*ee.as_deref().unwrap()).dval + oival as libc::c_double;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (*ee.as_deref().unwrap()).ival + oival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            45 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_muldiv(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= odval - (*ee.as_deref().unwrap()).dval;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).dval= odval - (*ee.as_deref().unwrap()).ival as libc::c_double;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_muldiv(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).dval= oival as libc::c_double - (*ee.as_deref().unwrap()).dval;
                                (*ee.as_deref_mut().unwrap()).type_0= 1 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= oival - (*ee.as_deref().unwrap()).ival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_shift(mut ee: Option<&mut _expreval_t>) {
    ee_addsub(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '<' as i32
            && *(*ee.as_deref().unwrap()).code
                .offset(
                    (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '<' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '>' as i32
                && *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '>' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        match  *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int {
            60 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_addsub(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (odval as lilint_t) << (*ee.as_deref().unwrap()).dval as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (odval as lilint_t) << (*ee.as_deref().unwrap()).ival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_addsub(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= oival << (*ee.as_deref().unwrap()).dval as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= oival << (*ee.as_deref().unwrap()).ival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            62 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_addsub(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t >> (*ee.as_deref().unwrap()).dval as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t >> (*ee.as_deref().unwrap()).ival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
                        ee_addsub(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= oival >> (*ee.as_deref().unwrap()).dval as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= oival >> (*ee.as_deref().unwrap()).ival;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_compare(mut ee: Option<&mut _expreval_t>) {
    ee_shift(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '<' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee.as_deref().unwrap()).code
                        .offset(
                            (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int as isize,
                ) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '>' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *(*ee.as_deref().unwrap()).code
                            .offset(
                                (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '<' as i32
                && *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '=' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '>' as i32
                && *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '=' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        let mut op = 4 as libc::c_int;
        if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '<' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee.as_deref().unwrap()).code
                        .offset(
                            (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int as isize,
                ) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            op= 1 as libc::c_int;
        } else if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '>' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee.as_deref().unwrap()).code
                        .offset(
                            (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int as isize,
                ) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0
        {
            op= 2 as libc::c_int;
        } else if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '<' as i32
            && *(*ee.as_deref().unwrap()).code
                .offset(
                    (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '=' as i32
        {
            op= 3 as libc::c_int;
        }
        (*ee.as_deref_mut().unwrap()).head= ((*ee.as_deref().unwrap()).head as libc::c_ulong)
            .wrapping_add(
                (if op > 2 as libc::c_int { 2 as libc::c_int } else { 1 as libc::c_int })
                    as libc::c_ulong,
            ) as size_t as size_t;
        match op {
            1 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval < (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval < (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if (oival as libc::c_double) < (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival < (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            2 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval > (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval > (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival as libc::c_double > (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival > (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            3 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval <= (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval <= (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival as libc::c_double <= (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival <= (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            4 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval >= (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval >= (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival as libc::c_double >= (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival >= (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_equals(mut ee: Option<&mut _expreval_t>) {
    ee_compare(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '=' as i32
            && *(*ee.as_deref().unwrap()).code
                .offset(
                    (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '=' as i32
            || *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '!' as i32
                && *(*ee.as_deref().unwrap()).code
                    .offset(
                        (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    ) as libc::c_int == '=' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        let mut op = if *(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int
            == '=' as i32
        {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        };
        (*ee.as_deref_mut().unwrap()).head= ((*ee.as_deref().unwrap()).head as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        match op {
            1 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_compare(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval == (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval == (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_compare(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival as libc::c_double == (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival == (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            2 => {
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        ee_compare(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval != (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if odval != (*ee.as_deref().unwrap()).ival as libc::c_double {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    0 => {
                        ee_compare(ee.as_deref_mut());
                        if (*ee.as_deref().unwrap()).error != 0 {
                            return;
                        }
                        match (*ee.as_deref().unwrap()).type_0 {
                            1 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival as libc::c_double != (*ee.as_deref().unwrap()).dval {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                                (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                            }
                            0 => {
                                (*ee.as_deref_mut().unwrap()).ival= (if oival != (*ee.as_deref().unwrap()).ival {
                                    1 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as lilint_t;
                            }
                            _ => {
                                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                            }
                        }
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_bitand(mut ee: Option<&mut _expreval_t>) {
    ee_equals(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '&' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee.as_deref().unwrap()).code
                        .offset(
                            (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int as isize,
                ) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        match (*ee.as_deref().unwrap()).type_0 {
            1 => {
                ee_equals(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t & (*ee.as_deref().unwrap()).dval as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t & (*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            0 => {
                ee_equals(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= oival & (*ee.as_deref().unwrap()).dval as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= oival & (*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {
                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
            }
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_bitor(mut ee: Option<&mut _expreval_t>) {
    ee_bitand(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '|' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *(*ee.as_deref().unwrap()).code
                        .offset(
                            (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize,
                        ) as libc::c_int as isize,
                ) as libc::c_int
                & _ISpunct as libc::c_int as libc::c_ushort as libc::c_int == 0)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        (*ee.as_deref_mut().unwrap()).head= (*ee.as_deref().unwrap()).head.wrapping_add(1);
        match (*ee.as_deref().unwrap()).type_0 {
            1 => {
                ee_bitand(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t | (*ee.as_deref().unwrap()).dval as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= odval as lilint_t | (*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            0 => {
                ee_bitand(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= oival | (*ee.as_deref().unwrap()).dval as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= oival | (*ee.as_deref().unwrap()).ival;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {
                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
            }
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_logand(mut ee: Option<&mut _expreval_t>) {
    ee_bitor(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '&' as i32
            && *(*ee.as_deref().unwrap()).code
                .offset(
                    (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '&' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        (*ee.as_deref_mut().unwrap()).head= ((*ee.as_deref().unwrap()).head as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        match (*ee.as_deref().unwrap()).type_0 {
            1 => {
                ee_bitor(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if odval != 0. && (*ee.as_deref().unwrap()).dval != 0. {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if odval != 0. && (*ee.as_deref().unwrap()).ival != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            0 => {
                ee_bitor(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if oival != 0 && (*ee.as_deref().unwrap()).dval != 0. {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if oival != 0 && (*ee.as_deref().unwrap()).ival != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {
                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
            }
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_logor(mut ee: Option<&mut _expreval_t>) {
    ee_logand(ee.as_deref_mut());
    ee_skip_spaces(ee.as_deref_mut());
    while (*ee.as_deref().unwrap()).head < (*ee.as_deref().unwrap()).len && (*ee.as_deref().unwrap()).error == 0
        && (*(*ee.as_deref().unwrap()).code.offset((*ee.as_deref().unwrap()).head as isize) as libc::c_int == '|' as i32
            && *(*ee.as_deref().unwrap()).code
                .offset(
                    (*ee.as_deref().unwrap()).head.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int == '|' as i32)
    {
        let mut odval = (*ee.as_deref().unwrap()).dval;
        let mut oival = (*ee.as_deref().unwrap()).ival;
        (*ee.as_deref_mut().unwrap()).head= ((*ee.as_deref().unwrap()).head as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        match (*ee.as_deref().unwrap()).type_0 {
            1 => {
                ee_logand(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if odval != 0. || (*ee.as_deref().unwrap()).dval != 0. {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if odval != 0. || (*ee.as_deref().unwrap()).ival != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            0 => {
                ee_logand(ee.as_deref_mut());
                if (*ee.as_deref().unwrap()).error != 0 {
                    return;
                }
                match (*ee.as_deref().unwrap()).type_0 {
                    1 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if oival != 0 || (*ee.as_deref().unwrap()).dval != 0. {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                        (*ee.as_deref_mut().unwrap()).type_0= 0 as libc::c_int;
                    }
                    0 => {
                        (*ee.as_deref_mut().unwrap()).ival= (if oival != 0 || (*ee.as_deref().unwrap()).ival != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as lilint_t;
                    }
                    _ => {
                        (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
                    }
                }
            }
            _ => {
                (*ee.as_deref_mut().unwrap()).error= 2 as libc::c_int;
            }
        }
        ee_skip_spaces(ee.as_deref_mut());
    }
}
unsafe extern "C" fn ee_expr(mut ee: Option<&mut _expreval_t>) {
    ee_logor(ee.as_deref_mut());
    if (*ee.as_deref().unwrap()).error == 4 as libc::c_int {
        (*ee.as_deref_mut().unwrap()).error= 0 as libc::c_int;
        (*ee.as_deref_mut().unwrap()).ival= 1 as libc::c_int as lilint_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn lil_eval_expr(
    mut lil: *mut _lil_t,
    mut code: *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut ee = expreval_t {
        code: 0 as *const libc::c_char,
        len: 0,
        head: 0,
        ival: 0,
        dval: 0.,
        type_0: 0,
        error: 0,
    };
    code= lil_subst_to_value(lil.as_mut(), code);
    if (*lil).error != 0 {
        return 0 as lil_value_t;
    }
    ee.code= lil_to_string(code);
    if *ee.code.offset(0 as libc::c_int as isize) == 0 {
        lil_free_value(code);
        return lil_alloc_integer(0 as libc::c_int as lilint_t);
    }
    ee.head= 0 as libc::c_int as size_t;
    ee.len= (*code).l;
    ee.ival= 0 as libc::c_int as lilint_t;
    ee.dval= 0 as libc::c_int as libc::c_double;
    ee.type_0= 0 as libc::c_int;
    ee.error= 0 as libc::c_int;
    ee_expr(Some(&mut ee));
    lil_free_value(code);
    if ee.error != 0 {
        match ee.error {
            3 => {
                lil_set_error(
                    lil.as_mut(),
                    b"division by zero in expression\0" as *const u8
                        as *const libc::c_char,
                );
            }
            2 => {
                lil_set_error(
                    lil.as_mut(),
                    b"mixing invalid types in expression\0" as *const u8
                        as *const libc::c_char,
                );
            }
            1 => {
                lil_set_error(
                    lil.as_mut(),
                    b"expression syntax error\0" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
        return 0 as lil_value_t;
    }
    if ee.type_0 == 0 as libc::c_int {
        return lil_alloc_integer(ee.ival)
    } else {
        return lil_alloc_double(ee.dval)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_unused_name(
    mut lil: *mut _lil_t,
    mut part: *const libc::c_char,
) -> *mut _lil_value_t {
    let mut name = malloc(
        (strlen(part)).wrapping_add(64 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut val = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < -(1 as libc::c_int) as size_t {
        sprintf(
            name,
            b"!!un!%s!%09u!nu!!\0" as *const u8 as *const libc::c_char,
            part,
            i as libc::c_uint,
        );
        if (find_cmd(lil.as_mut(), name)).is_null() {();
            if (lil_find_var(lil, (*lil).env, name)).is_null() {();
                val= lil_alloc_string(name);
                free(name as *mut libc::c_void);
                return val;
            }
        }
        i= i.wrapping_add(1);
    }
    return 0 as lil_value_t;
}
#[no_mangle]
pub unsafe extern "C" fn lil_arg(
    mut argv: *mut *mut _lil_value_t,
    mut index: libc::c_ulong,
) -> *mut _lil_value_t {
    return if !argv.is_null() { *argv.offset(index as isize) } else {(); 0 as lil_value_t };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_string(mut val: *mut _lil_value_t) -> *const libc::c_char {
    return if !val.is_null() && !(*val).d.is_null() {
        (*val).d as *const i8 as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_double(mut val: *mut _lil_value_t) -> libc::c_double {
    return atof(lil_to_string(val));
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_integer(mut val: *mut _lil_value_t) -> libc::c_long {
    return atoll(lil_to_string(val)) as lilint_t;
}
#[no_mangle]
pub unsafe extern "C" fn lil_to_boolean(mut val: Option<&mut _lil_value_t>) -> libc::c_int {
    let mut s = lil_to_string(val.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    let mut i: size_t = 0;
    let mut dots = 0 as libc::c_int as size_t;
    if *s.offset(0 as libc::c_int as isize) == 0 {
        return 0 as libc::c_int;
    }
    i= 0 as libc::c_int as size_t;
    while *s.offset(i as isize) != 0 {
        if *s.offset(i as isize) as libc::c_int != '0' as i32
            && *s.offset(i as isize) as libc::c_int != '.' as i32
        {
            return 1 as libc::c_int;
        }
        if *s.offset(i as isize) as libc::c_int == '.' as i32 {
            if dots != 0 {
                return 1 as libc::c_int;
            }
            dots= 1 as libc::c_int as size_t;
        }
        i= i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_string(mut str: *const libc::c_char) -> *mut /* owning */ _lil_value_t {
    return alloc_value(str);
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_double(mut num: libc::c_double) -> *mut /* owning */ _lil_value_t {
    let mut buff: [libc::c_char; 128] = [0; 128];
    sprintf(buff.as_mut_ptr(), b"%f\0" as *const u8 as *const libc::c_char, num);
    return alloc_value(buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn lil_alloc_integer(mut num: libc::c_long) -> *mut /* owning */ _lil_value_t {
    let mut buff: [libc::c_char; 128] = [0; 128];
    sprintf(buff.as_mut_ptr(), b"%lli\0" as *const u8 as *const libc::c_char, num);
    return alloc_value(buff.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn lil_free(mut lil: *mut /* owning */ _lil_t) {
    let mut i: size_t = 0;
    if lil.is_null() {();
        return;
    }
    free((*lil).err_msg as *mut libc::c_void);
    lil_free_value((*lil).empty);
    while !(*lil).env.is_null() {
        let mut next = (*(*lil).env).parent;
        lil_free_env((*lil).env);
        (*lil).env= next;
    }();
    i= 0 as libc::c_int as size_t;
    while i < (*lil).cmds {
        if !((**(*lil).cmd.offset(i as isize)).argnames).is_null() {
            lil_free_list((**(*lil).cmd.offset(i as isize)).argnames);
        }else { (); }
        lil_free_value((**(*lil).cmd.offset(i as isize)).code);
        free((**(*lil).cmd.offset(i as isize)).name as *mut libc::c_void);
        free(*(*lil).cmd.offset(i as isize) as *mut libc::c_void);
        i= i.wrapping_add(1);
    }
    free((*lil).cmd as *mut libc::c_void);
    free((*lil).dollarprefix as *mut libc::c_void);
    free((*lil).catcher as *mut libc::c_void);
    free(lil as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lil_set_data(mut lil: Option<&mut _lil_t>, mut data: *mut libc::c_void) {
    (*lil.as_deref_mut().unwrap()).data= data;
}
#[no_mangle]
pub unsafe extern "C" fn lil_get_data(mut lil: *mut _lil_t) -> *mut libc::c_void {
    return (*lil).data;
}
unsafe extern "C" fn fnc_reflect(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut func = 0 as *mut _lil_func_t;
    let mut type_0 = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut r = 0 as *mut _lil_value_t;
    if argc == 0 {
        return 0 as lil_value_t;
    }
    type_0= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    if strcmp(type_0, b"version\0" as *const u8 as *const libc::c_char) == 0 {
        return lil_alloc_string(b"0.1\0" as *const u8 as *const libc::c_char);
    }
    if strcmp(type_0, b"args\0" as *const u8 as *const libc::c_char) == 0 {
        if argc < 2 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        func= find_cmd(lil.as_mut(), lil_to_string(*argv.offset(1 as libc::c_int as isize)));
        if func.is_null() || (*func).argnames.is_null() {
            return 0 as lil_value_t;
        }
        return lil_list_to_value((*func).argnames, 1 as libc::c_int);
    }
    if strcmp(type_0, b"body\0" as *const u8 as *const libc::c_char) == 0 {
        if argc < 2 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        func= find_cmd(lil.as_mut(), lil_to_string(*argv.offset(1 as libc::c_int as isize)));
        if func.is_null() || ((*func).proc_0).is_some() {
            return 0 as lil_value_t;
        }
        return lil_clone_value((*func).code);
    }
    if strcmp(type_0, b"func-count\0" as *const u8 as *const libc::c_char) == 0 {
        return lil_alloc_integer((*lil).cmds as lilint_t);
    }
    if strcmp(type_0, b"funcs\0" as *const u8 as *const libc::c_char) == 0 {
        let mut funcs = lil_alloc_list();
        i= 0 as libc::c_int as size_t;
        while i < (*lil).cmds {
            lil_list_append(
                funcs,
                lil_alloc_string((**(*lil).cmd.offset(i as isize)).name),
            );
            i= i.wrapping_add(1);
        }
        r= lil_list_to_value(funcs, 1 as libc::c_int);
        lil_free_list(funcs);
        return r;
    }
    if strcmp(type_0, b"vars\0" as *const u8 as *const libc::c_char) == 0 {
        let mut vars = lil_alloc_list();
        let mut env = (*lil).env;
        while !env.is_null() {
            i= 0 as libc::c_int as size_t;
            while i < (*env).vars {
                lil_list_append(
                    vars,
                    lil_alloc_string((**(*env).var.offset(i as isize)).n),
                );
                i= i.wrapping_add(1);
            }
            env= (*env).parent;
        }();
        r= lil_list_to_value(vars, 1 as libc::c_int);
        lil_free_list(vars);
        return r;
    }
    if strcmp(type_0, b"globals\0" as *const u8 as *const libc::c_char) == 0 {
        let mut vars_0 = lil_alloc_list();
        i= 0 as libc::c_int as size_t;
        while i < (*(*lil).rootenv).vars {
            lil_list_append(
                vars_0,
                lil_alloc_string((**(*(*lil).rootenv).var.offset(i as isize)).n),
            );
            i= i.wrapping_add(1);
        }
        r= lil_list_to_value(vars_0, 1 as libc::c_int);
        lil_free_list(vars_0);
        return r;
    }
    if strcmp(type_0, b"has-func\0" as *const u8 as *const libc::c_char) == 0 {
        let mut target = 0 as *const libc::c_char;
        if argc == 1 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        target= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        i= 0 as libc::c_int as size_t;
        while i < (*lil).cmds {
            if strcmp(target, (**(*lil).cmd.offset(i as isize)).name) == 0 {
                return lil_alloc_string(b"1\0" as *const u8 as *const libc::c_char);
            }
            i= i.wrapping_add(1);
        }
        return 0 as lil_value_t;
    }
    if strcmp(type_0, b"has-var\0" as *const u8 as *const libc::c_char) == 0 {
        let mut target_0 = 0 as *const libc::c_char;
        let mut env_0 = (*lil).env;
        if argc == 1 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        target_0= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        while !env_0.is_null() {
            i= 0 as libc::c_int as size_t;
            while i < (*env_0).vars {
                if strcmp(target_0, (**(*env_0).var.offset(i as isize)).n) == 0 {
                    return lil_alloc_string(b"1\0" as *const u8 as *const libc::c_char);
                }
                i= i.wrapping_add(1);
            }
            env_0= (*env_0).parent;
        }();
        return 0 as lil_value_t;
    }
    if strcmp(type_0, b"has-global\0" as *const u8 as *const libc::c_char) == 0 {
        let mut target_1 = 0 as *const libc::c_char;
        if argc == 1 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        target_1= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        i= 0 as libc::c_int as size_t;
        while i < (*(*lil).rootenv).vars {
            if strcmp(target_1, (**(*(*lil).rootenv).var.offset(i as isize)).n) == 0 {
                return lil_alloc_string(b"1\0" as *const u8 as *const libc::c_char);
            }
            i= i.wrapping_add(1);
        }
        return 0 as lil_value_t;
    }
    if strcmp(type_0, b"error\0" as *const u8 as *const libc::c_char) == 0 {
        return if !(*lil).err_msg.is_null() {
            lil_alloc_string((*lil).err_msg)
        } else {();
            0 as lil_value_t
        };
    }
    if strcmp(type_0, b"dollar-prefix\0" as *const u8 as *const libc::c_char) == 0 {
        let mut r_0 = 0 as *mut _lil_value_t;
        if argc == 1 as libc::c_int as libc::c_ulong {
            return lil_alloc_string((*lil).dollarprefix as *const i8);
        }
        r_0= lil_alloc_string((*lil).dollarprefix as *const i8);
        free((*lil).dollarprefix as *mut libc::c_void);
        (*lil).dollarprefix= strclone(lil_to_string(*argv.offset(1 as libc::c_int as isize)));
        return r_0;
    }
    if strcmp(type_0, b"this\0" as *const u8 as *const libc::c_char) == 0 {
        let mut env_1 = (*lil).env;
        while env_1 != (*lil).rootenv && (*env_1).catcher_for.is_null()
            && (*env_1).func.is_null()
        {
            env_1= (*env_1).parent;
        }
        if !(*env_1).catcher_for.is_null() {
            return lil_alloc_string((*lil).catcher);
        }else { (); }
        if env_1 == (*lil).rootenv {
            return lil_alloc_string((*lil).rootcode);
        }
        return if !(*env_1).func.is_null() {
            (*(*env_1).func).code
        } else {();
            0 as lil_value_t
        };
    }
    if strcmp(type_0, b"name\0" as *const u8 as *const libc::c_char) == 0 {
        let mut env_2 = (*lil).env;
        while env_2 != (*lil).rootenv && (*env_2).catcher_for.is_null()
            && (*env_2).func.is_null()
        {
            env_2= (*env_2).parent;
        }
        if !(*env_2).catcher_for.is_null() {
            return (*env_2).catcher_for;
        }else { (); }
        if env_2 == (*lil).rootenv {
            return 0 as lil_value_t;
        }
        return if !(*env_2).func.is_null() {
            lil_alloc_string((*(*env_2).func).name)
        } else {();
            0 as lil_value_t
        };
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_func(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut name = 0 as *mut _lil_value_t;
    let mut cmd = 0 as *mut _lil_func_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc == 3 as libc::c_int as libc::c_ulong {
        name= lil_clone_value(*argv.offset(0 as libc::c_int as isize));
        cmd= add_func(lil.as_mut(), lil_to_string(*argv.offset(0 as libc::c_int as isize)));
        (*cmd).argnames= lil_subst_to_list(lil.as_mut(), *argv.offset(1 as libc::c_int as isize));
        (*cmd).code= lil_clone_value(*argv.offset(2 as libc::c_int as isize));
    } else {
        name= lil_unused_name(
            lil,
            b"anonymous-function\0" as *const u8 as *const libc::c_char,
        );
        cmd= add_func(lil.as_mut(), lil_to_string(name));
        if argc < 2 as libc::c_int as libc::c_ulong {
            let mut tmp = lil_alloc_string(
                b"args\0" as *const u8 as *const libc::c_char,
            );
            (*cmd).argnames= lil_subst_to_list(lil.as_mut(), tmp);
            lil_free_value(tmp);
            (*cmd).code= lil_clone_value(*argv.offset(0 as libc::c_int as isize));
        } else {
            (*cmd).argnames= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
            (*cmd).code= lil_clone_value(*argv.offset(1 as libc::c_int as isize));
        }
    }
    return name;
}
unsafe extern "C" fn fnc_rename(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut r = 0 as *mut _lil_value_t;
    let mut func = 0 as *mut _lil_func_t;
    let mut oldname = 0 as *const libc::c_char;
    let mut newname = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    oldname= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    newname= lil_to_string(*argv.offset(1 as libc::c_int as isize));
    func= find_cmd(lil.as_mut(), oldname);
    if func.is_null() {();
        let mut msg = malloc(
            (24 as libc::c_int as libc::c_ulong).wrapping_add(strlen(oldname)),
        ) as *mut libc::c_char;
        sprintf(
            msg,
            b"unknown function '%s'\0" as *const u8 as *const libc::c_char,
            oldname,
        );
        lil_set_error_at(lil.as_mut(), (*lil).head, msg as *const i8);
        free(msg as *mut libc::c_void);
        return 0 as lil_value_t;
    }
    r= lil_alloc_string((*func).name);
    free((*func).name as *mut libc::c_void);
    (*func).name= strclone(newname);
    return r;
}
unsafe extern "C" fn fnc_unusedname(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    return lil_unused_name(
        lil,
        if argc > 0 as libc::c_int as libc::c_ulong {
            lil_to_string(*argv.offset(0 as libc::c_int as isize))
        } else {
            b"unusedname\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn fnc_quote(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut r = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    r= alloc_value(0 as *const libc::c_char);
    i= 0 as libc::c_int as size_t;
    while i < argc {
        if i != 0 {
            lil_append_char(r, ' ' as i32 as libc::c_char);
        }
        lil_append_val(r, *argv.offset(i as isize));
        i= i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_set(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut i = 0 as libc::c_int as size_t;
    let mut var = 0 as lil_var_t;
    let mut access = 1 as libc::c_int;
    if argc == 0 {
        return 0 as lil_value_t;
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        b"global\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        i= 1 as libc::c_int as size_t;
        access= 0 as libc::c_int;
    }
    while i < argc {
        if argc == i.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            return lil_clone_value(
                lil_get_var(lil.as_mut(), lil_to_string(*argv.offset(i as isize))),
            );
        }
        var= lil_set_var(
            lil,
            lil_to_string(*argv.offset(i as isize)),
            *argv.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize),
            access,
        );
        i= (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return if !var.is_null() { lil_clone_value((*var).v) } else {(); 0 as lil_value_t };
}
unsafe extern "C" fn fnc_local(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < argc {
        let mut varname = lil_to_string(*argv.offset(i as isize));
        if (lil_find_local_var(lil, (*lil).env, varname)).is_null() {();
            lil_set_var(lil, varname, (*lil).empty, 2 as libc::c_int);
        }
        i= i.wrapping_add(1);
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_write(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut i: size_t = 0;
    let mut msg = lil_alloc_string(0 as *const libc::c_char);
    i= 0 as libc::c_int as size_t;
    while i < argc {
        if i != 0 {
            lil_append_char(msg, ' ' as i32 as libc::c_char);
        }
        lil_append_val(msg, *argv.offset(i as isize));
        i= i.wrapping_add(1);
    }
    if ((*lil).callback[1 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_write_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_write_callback_proc_t,
        >((*lil).callback[1 as libc::c_int as usize]);
        proc_0.expect("non-null function pointer")(lil, lil_to_string(msg));
    } else {
        printf(b"%s\0" as *const u8 as *const libc::c_char, lil_to_string(msg));
    }
    lil_free_value(msg);
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_print(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    fnc_write(lil, argc, argv);
    if ((*lil).callback[1 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_write_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_write_callback_proc_t,
        >((*lil).callback[1 as libc::c_int as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(lil, b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_eval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc == 1 as libc::c_int as libc::c_ulong {
        return lil_parse_value(
            lil.as_mut(),
            *argv.offset(0 as libc::c_int as isize),
            0 as libc::c_int,
        );
    }
    if argc > 1 as libc::c_int as libc::c_ulong {
        let mut val = alloc_value(0 as *const libc::c_char);
        let mut r = 0 as *mut _lil_value_t;
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < argc {
            if i != 0 {
                lil_append_char(val, ' ' as i32 as libc::c_char);
            }
            lil_append_val(val, *argv.offset(i as isize));
            i= i.wrapping_add(1);
        }
        r= lil_parse_value(lil.as_mut(), val, 0 as libc::c_int);
        lil_free_value(val);
        return r;
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_topeval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut thisenv = (*lil).env;
    let mut thisdownenv = (*lil).downenv;
    let mut r = 0 as *mut _lil_value_t;
    (*lil).env= (*lil).rootenv;
    (*lil).downenv= thisenv;
    r= fnc_eval(lil, argc, argv);
    (*lil).downenv= thisdownenv;
    (*lil).env= thisenv;
    return r;
}
unsafe extern "C" fn fnc_upeval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut thisenv = (*lil).env;
    let mut thisdownenv = (*lil).downenv;
    let mut r = 0 as *mut _lil_value_t;
    if (*lil).rootenv == thisenv {
        return fnc_eval(lil, argc, argv);
    }
    (*lil).env= (*thisenv).parent;
    (*lil).downenv= thisenv;
    r= fnc_eval(lil, argc, argv);
    (*lil).env= thisenv;
    (*lil).downenv= thisdownenv;
    return r;
}
unsafe extern "C" fn fnc_downeval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut r = 0 as *mut _lil_value_t;
    let mut upenv = (*lil).env;
    let mut downenv = (*lil).downenv;
    if downenv.is_null() {();
        return fnc_eval(lil, argc, argv);
    }
    (*lil).downenv= 0 as lil_env_t;
    (*lil).env= downenv;
    r= fnc_eval(lil, argc, argv);
    (*lil).downenv= downenv;
    (*lil).env= upenv;
    return r;
}
unsafe extern "C" fn fnc_enveval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut r = 0 as *mut _lil_value_t;
    let mut invars = 0 as lil_list_t;
    let mut outvars = 0 as lil_list_t;
    let mut varvalues = 0 as *mut lil_value_t;
    let mut codeindex: libc::c_int = 0;
    let mut i: size_t = 0;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc == 1 as libc::c_int as libc::c_ulong {
        codeindex= 0 as libc::c_int;
    } else if argc >= 2 as libc::c_int as libc::c_ulong {
        invars= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
        varvalues= malloc(
            (::std::mem::size_of::<lil_value_t>() as libc::c_ulong)
                .wrapping_mul(lil_list_size(invars)),
        ) as *mut lil_value_t;
        i= 0 as libc::c_int as size_t;
        while i < lil_list_size(invars) {
            *varvalues.offset(i as isize) = lil_clone_value(
                lil_get_var(lil.as_mut(), lil_to_string(lil_list_get(invars, i))),
            );
            i= i.wrapping_add(1);
        }
        if argc > 2 as libc::c_int as libc::c_ulong {
            codeindex= 2 as libc::c_int;
            outvars= lil_subst_to_list(lil.as_mut(), *argv.offset(1 as libc::c_int as isize));
        } else {
            codeindex= 1 as libc::c_int;
        }
    }
    lil_push_env(lil.as_mut());
    if !invars.is_null() {
        i= 0 as libc::c_int as size_t;
        while i < lil_list_size(invars) {
            lil_set_var(
                lil,
                lil_to_string(lil_list_get(invars, i)),
                *varvalues.offset(i as isize),
                2 as libc::c_int,
            );
            lil_free_value(*varvalues.offset(i as isize));
            i= i.wrapping_add(1);
        }
    }else { (); }
    r= lil_parse_value(lil.as_mut(), *argv.offset(codeindex as isize), 0 as libc::c_int);
    if !invars.is_null() || !outvars.is_null() {
        if !outvars.is_null() {
            varvalues= realloc(
                varvalues as *mut libc::c_void,
                (::std::mem::size_of::<lil_value_t>() as libc::c_ulong)
                    .wrapping_mul(lil_list_size(outvars)),
            ) as *mut lil_value_t;
            i= 0 as libc::c_int as size_t;
            while i < lil_list_size(outvars) {
                *varvalues.offset(i as isize) = lil_clone_value(
                    lil_get_var(lil.as_mut(), lil_to_string(lil_list_get(outvars, i))),
                );
                i= i.wrapping_add(1);
            }
        } else {();
            i= 0 as libc::c_int as size_t;
            while i < lil_list_size(invars) {
                *varvalues.offset(i as isize) = lil_clone_value(
                    lil_get_var(lil.as_mut(), lil_to_string(lil_list_get(invars, i))),
                );
                i= i.wrapping_add(1);
            }
        }
    }
    lil_pop_env(lil.as_mut());
    if !invars.is_null() {
        if !outvars.is_null() {
            i= 0 as libc::c_int as size_t;
            while i < lil_list_size(outvars) {
                lil_set_var(
                    lil,
                    lil_to_string(lil_list_get(outvars, i)),
                    *varvalues.offset(i as isize),
                    1 as libc::c_int,
                );
                lil_free_value(*varvalues.offset(i as isize));
                i= i.wrapping_add(1);
            }
        } else {();
            i= 0 as libc::c_int as size_t;
            while i < lil_list_size(invars) {
                lil_set_var(
                    lil,
                    lil_to_string(lil_list_get(invars, i)),
                    *varvalues.offset(i as isize),
                    1 as libc::c_int,
                );
                lil_free_value(*varvalues.offset(i as isize));
                i= i.wrapping_add(1);
            }
        }
        lil_free_list(invars);
        if !outvars.is_null() {
            lil_free_list(outvars);
        }else { (); }
        free(varvalues as *mut libc::c_void);
    }else { (); }
    return r;
}
unsafe extern "C" fn fnc_jaileval(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut i: size_t = 0;
    let mut sublil = 0 as *mut _lil_t;
    let mut r = 0 as *mut _lil_value_t;
    let mut base = 0 as libc::c_int as size_t;
    if argc == 0 {
        return 0 as lil_value_t;
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        b"clean\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        base= 1 as libc::c_int as size_t;
        if argc == 1 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
    }
    sublil= lil_new();
    if base != 1 as libc::c_int as libc::c_ulong {
        i= (*lil).syscmds;
        while i < (*lil).cmds {
            let mut fnc = *(*lil).cmd.offset(i as isize);
            if !((*fnc).proc_0).is_none() {
                lil_register(sublil.as_mut(), (*fnc).name, (*fnc).proc_0);
            }
            i= i.wrapping_add(1);
        }
    }
    r= lil_parse_value(sublil.as_mut(), *argv.offset(base as isize), 1 as libc::c_int);
    lil_free(sublil);
    return r;
}
unsafe extern "C" fn fnc_count(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut buff: [libc::c_char; 64] = [0; 64];
    if argc == 0 {
        return alloc_value(b"0\0" as *const u8 as *const libc::c_char);
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
    sprintf(
        buff.as_mut_ptr(),
        b"%u\0" as *const u8 as *const libc::c_char,
        (*list).c as libc::c_uint,
    );
    lil_free_list(list);
    return alloc_value(buff.as_mut_ptr());
}
unsafe extern "C" fn fnc_index(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut index: size_t = 0;
    let mut r = 0 as *mut _lil_value_t;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
    index= lil_to_integer(*argv.offset(1 as libc::c_int as isize)) as size_t;
    if index >= (*list).c {
        r= 0 as lil_value_t;
    } else {
        r= lil_clone_value(*(*list).v.offset(index as isize));
    }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_indexof(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut index: size_t = 0;
    let mut r = 0 as lil_value_t;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
    index= 0 as libc::c_int as size_t;
    while index < (*list).c {
        if strcmp(
            lil_to_string(*(*list).v.offset(index as isize)),
            lil_to_string(*argv.offset(1 as libc::c_int as isize)),
        ) == 0
        {
            r= lil_alloc_integer(index as lilint_t);
            break;
        } else {
            index= index.wrapping_add(1);
        }
    }
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_append(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut r = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    let mut base = 1 as libc::c_int as size_t;
    let mut access = 1 as libc::c_int;
    let mut varname = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    varname= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    if strcmp(varname, b"global\0" as *const u8 as *const libc::c_char) == 0 {
        if argc < 3 as libc::c_int as libc::c_ulong {
            return 0 as lil_value_t;
        }
        varname= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        base= 2 as libc::c_int as size_t;
        access= 0 as libc::c_int;
    }
    list= lil_subst_to_list(lil.as_mut(), lil_get_var(lil.as_mut(), varname));
    i= base;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i= i.wrapping_add(1);
    }
    r= lil_list_to_value(list, 1 as libc::c_int);
    lil_free_list(list);
    lil_set_var(lil, varname, r, access);
    return r;
}
unsafe extern "C" fn fnc_slice(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut slice = 0 as *mut _lil_list_t;
    let mut i: size_t = 0;
    let mut from: lilint_t = 0;
    let mut to: lilint_t = 0;
    let mut r = 0 as *mut _lil_value_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc < 2 as libc::c_int as libc::c_ulong {
        return lil_clone_value(*argv.offset(0 as libc::c_int as isize));
    }
    from= lil_to_integer(*argv.offset(1 as libc::c_int as isize));
    if from < 0 as libc::c_int as libc::c_long {
        from= 0 as libc::c_int as lilint_t;
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
    to= if argc > 2 as libc::c_int as libc::c_ulong {
        lil_to_integer(*argv.offset(2 as libc::c_int as isize))
    } else {
        (*list).c as lilint_t
    };
    if to > (*list).c as lilint_t {
        to= (*list).c as lilint_t;
    }
    if to < from {
        to= from;
    }
    slice= lil_alloc_list();
    i= from as size_t;
    while i < to as size_t {
        lil_list_append(slice, lil_clone_value(*(*list).v.offset(i as isize)));
        i= i.wrapping_add(1);
    }
    lil_free_list(list);
    r= lil_list_to_value(slice, 1 as libc::c_int);
    lil_free_list(slice);
    return r;
}
unsafe extern "C" fn fnc_filter(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut filtered = 0 as *mut _lil_list_t;
    let mut i: size_t = 0;
    let mut r = 0 as *mut _lil_value_t;
    let mut varname = b"x\0" as *const u8 as *const libc::c_char;
    let mut base = 0 as libc::c_int;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc < 2 as libc::c_int as libc::c_ulong {
        return lil_clone_value(*argv.offset(0 as libc::c_int as isize));
    }
    if argc > 2 as libc::c_int as libc::c_ulong {
        base= 1 as libc::c_int;
        varname= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(base as isize));
    filtered= lil_alloc_list();
    i= 0 as libc::c_int as size_t;
    while i < (*list).c && (*(*lil).env).breakrun == 0 {
        lil_set_var(lil, varname, *(*list).v.offset(i as isize), 3 as libc::c_int);
        r= lil_eval_expr(lil, *argv.offset((base + 1 as libc::c_int) as isize));
        if lil_to_boolean(r.as_mut()) != 0 {
            lil_list_append(filtered, lil_clone_value(*(*list).v.offset(i as isize)));
        }
        lil_free_value(r);
        i= i.wrapping_add(1);
    }
    lil_free_list(list);
    r= lil_list_to_value(filtered, 1 as libc::c_int);
    lil_free_list(filtered);
    return r;
}
unsafe extern "C" fn fnc_list(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = lil_alloc_list();
    let mut r = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    i= 0 as libc::c_int as size_t;
    while i < argc {
        lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
        i= i.wrapping_add(1);
    }
    r= lil_list_to_value(list, 1 as libc::c_int);
    lil_free_list(list);
    return r;
}
unsafe extern "C" fn fnc_subst(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    return lil_subst_to_value(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
}
unsafe extern "C" fn fnc_concat(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut r = 0 as *mut _lil_value_t;
    let mut tmp = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    r= lil_alloc_string(b"\0" as *const u8 as *const libc::c_char);
    i= 0 as libc::c_int as size_t;
    while i < argc {
        list= lil_subst_to_list(lil.as_mut(), *argv.offset(i as isize));
        tmp= lil_list_to_value(list, 1 as libc::c_int);
        lil_free_list(list);
        lil_append_val(r, tmp);
        lil_free_value(tmp);
        i= i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_foreach(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut rlist = 0 as *mut _lil_list_t;
    let mut r = 0 as *mut _lil_value_t;
    let mut i: size_t = 0;
    let mut listidx = 0 as libc::c_int as size_t;
    let mut codeidx = 1 as libc::c_int as size_t;
    let mut varname = b"i\0" as *const u8 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc >= 3 as libc::c_int as libc::c_ulong {
        varname= lil_to_string(*argv.offset(0 as libc::c_int as isize));
        listidx= 1 as libc::c_int as size_t;
        codeidx= 2 as libc::c_int as size_t;
    }
    rlist= lil_alloc_list();
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(listidx as isize));
    i= 0 as libc::c_int as size_t;
    while i < (*list).c {
        let mut rv = 0 as *mut _lil_value_t;
        lil_set_var(lil, varname, *(*list).v.offset(i as isize), 3 as libc::c_int);
        rv= lil_parse_value(lil.as_mut(), *argv.offset(codeidx as isize), 0 as libc::c_int);
        if (*rv).l != 0 {
            lil_list_append(rlist, rv);
        } else {
            lil_free_value(rv);
        }
        if (*(*lil).env).breakrun != 0 || (*lil).error != 0 {
            break;
        }
        i= i.wrapping_add(1);
    }
    r= lil_list_to_value(rlist, 1 as libc::c_int);
    lil_free_list(list);
    lil_free_list(rlist);
    return r;
}
unsafe extern "C" fn fnc_return(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    (*(*lil).env).breakrun= 1 as libc::c_int;
    lil_free_value((*(*lil).env).retval);
    (*(*lil).env).retval= if argc < 1 as libc::c_int as libc::c_ulong {
        0 as lil_value_t
    } else {
        lil_clone_value(*argv.offset(0 as libc::c_int as isize))
    };
    (*(*lil).env).retval_set= 1 as libc::c_int;
    return if argc < 1 as libc::c_int as libc::c_ulong {
        0 as lil_value_t
    } else {
        lil_clone_value(*argv.offset(0 as libc::c_int as isize))
    };
}
unsafe extern "C" fn fnc_result(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc > 0 as libc::c_int as libc::c_ulong {
        lil_free_value((*(*lil).env).retval);
        (*(*lil).env).retval= lil_clone_value(*argv.offset(0 as libc::c_int as isize));
        (*(*lil).env).retval_set= 1 as libc::c_int;
    }
    return if (*(*lil).env).retval_set != 0 {
        lil_clone_value((*(*lil).env).retval)
    } else {
        0 as lil_value_t
    };
}
unsafe extern "C" fn fnc_expr(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc == 1 as libc::c_int as libc::c_ulong {
        return lil_eval_expr(lil, *argv.offset(0 as libc::c_int as isize));
    }
    if argc > 1 as libc::c_int as libc::c_ulong {
        let mut val = alloc_value(0 as *const libc::c_char);
        let mut r = 0 as *mut _lil_value_t;
        let mut i: size_t = 0;
        i= 0 as libc::c_int as size_t;
        while i < argc {
            if i != 0 {
                lil_append_char(val, ' ' as i32 as libc::c_char);
            }
            lil_append_val(val, *argv.offset(i as isize));
            i= i.wrapping_add(1);
        }
        r= lil_eval_expr(lil, val);
        lil_free_value(val);
        return r;
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn real_inc(
    mut lil: Option<&mut _lil_t>,
    mut varname: *const libc::c_char,
    mut v: libc::c_float,
) -> *mut /* owning */ _lil_value_t {
    let mut pv = lil_get_var(lil.as_deref_mut(), varname);
    let mut dv = lil_to_double(pv) + v as libc::c_double;
    if fmod(dv, 1 as libc::c_int as libc::c_double) != 0. {
        pv= lil_alloc_double(dv);
    } else {
        pv= lil_alloc_integer((lil_to_integer(pv) as libc::c_float + v) as lilint_t);
    }
    lil_set_var(lil.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), varname, pv, 1 as libc::c_int);
    return pv;
}
unsafe extern "C" fn fnc_inc(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    return real_inc(
        lil.as_mut(),
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        (if argc > 1 as libc::c_int as libc::c_ulong {
            lil_to_double(*argv.offset(1 as libc::c_int as isize))
        } else {
            1 as libc::c_int as libc::c_double
        }) as libc::c_float,
    );
}
unsafe extern "C" fn fnc_dec(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    return real_inc(
        lil.as_mut(),
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        -if argc > 1 as libc::c_int as libc::c_ulong {
            lil_to_double(*argv.offset(1 as libc::c_int as isize))
        } else {
            1 as libc::c_int as libc::c_double
        } as libc::c_float,
    );
}
unsafe extern "C" fn fnc_read(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut f = 0 as *mut FILE;
    let mut size: size_t = 0;
    let mut buffer = 0 as *mut libc::c_char;
    let mut r = 0 as *mut _lil_value_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if ((*lil).callback[2 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_read_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_read_callback_proc_t,
        >((*lil).callback[2 as libc::c_int as usize]);
        buffer= proc_0
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as libc::c_int as isize)));
    } else {
        f= fopen(
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if f.is_null() {();
            return 0 as lil_value_t;
        }
        fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
        size= ftell(f) as size_t;
        fseek(f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
        buffer= malloc(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        fread(buffer as *mut libc::c_void, 1 as libc::c_int as libc::c_ulong, size, f);
        *buffer.offset(size as isize) = 0 as libc::c_int as libc::c_char;
        fclose(f);
    }
    r= lil_alloc_string(buffer as *const i8);
    free(buffer as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn fnc_store(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut f = 0 as *mut FILE;
    let mut buffer = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if ((*lil).callback[3 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_store_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_store_callback_proc_t,
        >((*lil).callback[3 as libc::c_int as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(
            lil,
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            lil_to_string(*argv.offset(1 as libc::c_int as isize)),
        );
    } else {
        f= fopen(
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            b"wb\0" as *const u8 as *const libc::c_char,
        );
        if f.is_null() {();
            return 0 as lil_value_t;
        }
        buffer= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        fwrite(
            buffer as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            strlen(buffer),
            f,
        );
        fclose(f);
    }
    return lil_clone_value(*argv.offset(1 as libc::c_int as isize));
}
unsafe extern "C" fn fnc_if(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    let mut r = 0 as lil_value_t;
    let mut base = 0 as libc::c_int;
    let mut not = 0 as libc::c_int;
    let mut v: libc::c_int = 0;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        b"not\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        not= 1 as libc::c_int;
        base= not;
    }
    if argc < (base as size_t).wrapping_add(2 as libc::c_int as libc::c_ulong) {
        return 0 as lil_value_t;
    }
    val= lil_eval_expr(lil, *argv.offset(base as isize));
    if val.is_null() || (*lil).error != 0 {
        return 0 as lil_value_t;
    }
    v= lil_to_boolean(val.as_mut());
    if not != 0 {
        v= (v == 0) as libc::c_int;
    }
    if v != 0 {
        r= lil_parse_value(
            lil.as_mut(),
            *argv.offset((base + 1 as libc::c_int) as isize),
            0 as libc::c_int,
        );
    } else if argc > (base as size_t).wrapping_add(2 as libc::c_int as libc::c_ulong) {
        r= lil_parse_value(
            lil.as_mut(),
            *argv.offset((base + 2 as libc::c_int) as isize),
            0 as libc::c_int,
        );
    }
    lil_free_value(val);
    return r;
}
unsafe extern "C" fn fnc_while(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    let mut r = 0 as lil_value_t;
    let mut base = 0 as libc::c_int;
    let mut not = 0 as libc::c_int;
    let mut v: libc::c_int = 0;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if strcmp(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        b"not\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        not= 1 as libc::c_int;
        base= not;
    }
    if argc < (base as size_t).wrapping_add(2 as libc::c_int as libc::c_ulong) {
        return 0 as lil_value_t;
    }
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val= lil_eval_expr(lil, *argv.offset(base as isize));
        if val.is_null() || (*lil).error != 0 {
            return 0 as lil_value_t;
        }
        v= lil_to_boolean(val.as_mut());
        if not != 0 {
            v= (v == 0) as libc::c_int;
        }
        if v == 0 {
            lil_free_value(val);
            break;
        } else {
            if !r.is_null() {
                lil_free_value(r);
            }else { (); }
            r= lil_parse_value(
                lil.as_mut(),
                *argv.offset((base + 1 as libc::c_int) as isize),
                0 as libc::c_int,
            );
            lil_free_value(val);
        }
    }
    return r;
}
unsafe extern "C" fn fnc_for(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    let mut r = 0 as lil_value_t;
    if argc < 4 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    lil_free_value(
        lil_parse_value(lil.as_mut(), *argv.offset(0 as libc::c_int as isize), 0 as libc::c_int),
    );
    while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
        val= lil_eval_expr(lil, *argv.offset(1 as libc::c_int as isize));
        if val.is_null() || (*lil).error != 0 {
            return 0 as lil_value_t;
        }
        if lil_to_boolean(val.as_mut()) == 0 {
            lil_free_value(val);
            break;
        } else {
            if !r.is_null() {
                lil_free_value(r);
            }else { (); }
            r= lil_parse_value(
                lil.as_mut(),
                *argv.offset(3 as libc::c_int as isize),
                0 as libc::c_int,
            );
            lil_free_value(val);
            lil_free_value(
                lil_parse_value(
                    lil.as_mut(),
                    *argv.offset(2 as libc::c_int as isize),
                    0 as libc::c_int,
                ),
            );
        }
    }
    return r;
}
unsafe extern "C" fn fnc_char(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut s: [libc::c_char; 2] = [0; 2];
    if argc == 0 {
        return 0 as lil_value_t;
    }
    s[0 as libc::c_int
        as usize]= lil_to_integer(*argv.offset(0 as libc::c_int as isize))
        as libc::c_char;
    s[1 as libc::c_int as usize]= 0 as libc::c_int as libc::c_char;
    return lil_alloc_string(s.as_mut_ptr());
}
unsafe extern "C" fn fnc_charat(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut index: size_t = 0;
    let mut chstr: [libc::c_char; 2] = [0; 2];
    let mut str = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    str= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    index= lil_to_integer(*argv.offset(1 as libc::c_int as isize)) as size_t;
    if index >= strlen(str) {
        return 0 as lil_value_t;
    }
    chstr[0 as libc::c_int as usize]= *str.offset(index as isize);
    chstr[1 as libc::c_int as usize]= 0 as libc::c_int as libc::c_char;
    return lil_alloc_string(chstr.as_mut_ptr());
}
unsafe extern "C" fn fnc_codeat(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut index: size_t = 0;
    let mut str = 0 as *const libc::c_char;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    str= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    index= lil_to_integer(*argv.offset(1 as libc::c_int as isize)) as size_t;
    if index >= strlen(str) {
        return 0 as lil_value_t;
    }
    return lil_alloc_integer(*str.offset(index as isize) as lilint_t);
}
unsafe extern "C" fn fnc_substr(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut str = 0 as *const libc::c_char;
    let mut r = 0 as *mut _lil_value_t;
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut i: size_t = 0;
    let mut slen: size_t = 0;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    str= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    if *str.offset(0 as libc::c_int as isize) == 0 {
        return 0 as lil_value_t;
    }
    slen= strlen(str);
    start= atoll(lil_to_string(*argv.offset(1 as libc::c_int as isize))) as size_t;
    end= if argc > 2 as libc::c_int as libc::c_ulong {
        atoll(lil_to_string(*argv.offset(2 as libc::c_int as isize))) as size_t
    } else {
        slen
    };
    if end > slen {
        end= slen;
    }
    if start >= end {
        return 0 as lil_value_t;
    }
    r= lil_alloc_string(b"\0" as *const u8 as *const libc::c_char);
    i= start;
    while i < end {
        lil_append_char(r, *str.offset(i as isize));
        i= i.wrapping_add(1);
    }
    return r;
}
unsafe extern "C" fn fnc_strpos(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut hay = 0 as *const libc::c_char;
    let mut str = 0 as *const libc::c_char;
    let mut min = 0 as libc::c_int as size_t;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return lil_alloc_integer(-(1 as libc::c_int) as lilint_t);
    }
    hay= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    if argc > 2 as libc::c_int as libc::c_ulong {
        min= atoll(lil_to_string(*argv.offset(2 as libc::c_int as isize))) as size_t;
        if min >= strlen(hay) {
            return lil_alloc_integer(-(1 as libc::c_int) as lilint_t);
        }
    }
    str= strstr(
        hay.offset(min as isize),
        lil_to_string(*argv.offset(1 as libc::c_int as isize)),
    );
    if str.is_null() {();
        return lil_alloc_integer(-(1 as libc::c_int) as lilint_t);
    }
    return lil_alloc_integer(str.offset_from(hay) as libc::c_long);
}
unsafe extern "C" fn fnc_length(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut i: size_t = 0;
    let mut total = 0 as libc::c_int as size_t;
    i= 0 as libc::c_int as size_t;
    while i < argc {
        if i != 0 {
            total= total.wrapping_add(1);
        }
        total= (total as libc::c_ulong)
            .wrapping_add(strlen(lil_to_string(*argv.offset(i as isize)))) as size_t
            as size_t;
        i= i.wrapping_add(1);
    }
    return lil_alloc_integer(total as lilint_t);
}
unsafe extern "C" fn real_trim(
    mut str: *const libc::c_char,
    mut chars: *const libc::c_char,
    mut left: libc::c_int,
    mut right: libc::c_int,
) -> *mut _lil_value_t {
    let mut base = 0 as libc::c_int;
    let mut r = 0 as lil_value_t;
    if left != 0 {
        while *str.offset(base as isize) as libc::c_int != 0
            && !(strchr(chars, *str.offset(base as isize) as libc::c_int)).is_null()
        {
            base+= 1;
        }
        if right == 0 {
            r= lil_alloc_string(
                if *str.offset(base as isize) as libc::c_int != 0 {
                    str.offset(base as isize)
                } else {
                    0 as *const libc::c_char
                },
            );
        }
    }
    if right != 0 {
        let mut len: size_t = 0;
        let mut s = 0 as *mut libc::c_char;
        s= strclone(str.offset(base as isize));
        len= strlen(s);
        while len != 0
            && !(strchr(
                chars,
                *s.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int,
            ))
                .is_null()
        {
            len= len.wrapping_sub(1);
        }
        *s.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        r= lil_alloc_string(s);
        free(s as *mut libc::c_void);
    }
    return r;
}
unsafe extern "C" fn fnc_trim(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc == 0 {
        return 0 as lil_value_t;
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        if argc < 2 as libc::c_int as libc::c_ulong {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char
        } else {
            lil_to_string(*argv.offset(1 as libc::c_int as isize))
        },
        1 as libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn fnc_ltrim(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc == 0 {
        return 0 as lil_value_t;
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        if argc < 2 as libc::c_int as libc::c_ulong {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char
        } else {
            lil_to_string(*argv.offset(1 as libc::c_int as isize))
        },
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn fnc_rtrim(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if argc == 0 {
        return 0 as lil_value_t;
    }
    return real_trim(
        lil_to_string(*argv.offset(0 as libc::c_int as isize)),
        if argc < 2 as libc::c_int as libc::c_ulong {
            b" \x0C\n\r\t\x0B\0" as *const u8 as *const libc::c_char
        } else {
            lil_to_string(*argv.offset(1 as libc::c_int as isize))
        },
        0 as libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn fnc_strcmp(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    return lil_alloc_integer(
        strcmp(
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            lil_to_string(*argv.offset(1 as libc::c_int as isize)),
        ) as lilint_t,
    );
}
unsafe extern "C" fn fnc_streq(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    return lil_alloc_integer(
        (if strcmp(
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            lil_to_string(*argv.offset(1 as libc::c_int as isize)),
        ) != 0
        {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        }) as lilint_t,
    );
}
unsafe extern "C" fn fnc_repstr(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut from = 0 as *const libc::c_char;
    let mut to = 0 as *const libc::c_char;
    let mut src = 0 as *mut libc::c_char;
    let mut sub = 0 as *const libc::c_char;
    let mut idx: size_t = 0;
    let mut fromlen: size_t = 0;
    let mut tolen: size_t = 0;
    let mut srclen: size_t = 0;
    let mut r = 0 as *mut _lil_value_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc < 3 as libc::c_int as libc::c_ulong {
        return lil_clone_value(*argv.offset(0 as libc::c_int as isize));
    }
    from= lil_to_string(*argv.offset(1 as libc::c_int as isize));
    to= lil_to_string(*argv.offset(2 as libc::c_int as isize));
    if *from.offset(0 as libc::c_int as isize) == 0 {
        return 0 as lil_value_t;
    }
    src= strclone(lil_to_string(*argv.offset(0 as libc::c_int as isize)));
    srclen= strlen(src as *const i8);
    fromlen= strlen(from);
    tolen= strlen(to);
    loop {
        sub= strstr(src as *const i8, from);
        if sub.is_null() {();
            break;
        }
        let mut newsrc = malloc(
            srclen
                .wrapping_sub(fromlen)
                .wrapping_add(tolen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        idx= sub.offset_from(src as *const i8) as libc::c_long as size_t;
        if idx != 0 {
            memcpy(newsrc as *mut libc::c_void, src as *const i8 as *const libc::c_void, idx);
        }
        memcpy(
            newsrc.offset(idx as isize) as *mut libc::c_void,
            to as *const libc::c_void,
            tolen,
        );
        memcpy(
            newsrc.offset(idx as isize).offset(tolen as isize) as *mut libc::c_void,
            src.offset(idx as isize).offset(fromlen as isize) as *const libc::c_void,
            srclen.wrapping_sub(idx).wrapping_sub(fromlen),
        );
        srclen= srclen.wrapping_sub(fromlen).wrapping_add(tolen);
        free(src as *mut libc::c_void);
        src= newsrc;
        *src.offset(srclen as isize) = 0 as libc::c_int as libc::c_char;
    }
    r= lil_alloc_string(src as *const i8);
    free(src as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn fnc_split(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut sep = b" \0" as *const u8 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut val = 0 as *mut _lil_value_t;
    let mut str = 0 as *const libc::c_char;
    if argc == 0 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if argc > 1 as libc::c_int as libc::c_ulong {
        sep= lil_to_string(*argv.offset(1 as libc::c_int as isize));
        if sep.is_null() || *sep.offset(0 as libc::c_int as isize) == 0 {
            return lil_clone_value(*argv.offset(0 as libc::c_int as isize));
        }
    }
    val= lil_alloc_string(b"\0" as *const u8 as *const libc::c_char);
    str= lil_to_string(*argv.offset(0 as libc::c_int as isize));
    list= lil_alloc_list();
    i= 0 as libc::c_int as size_t;
    while *str.offset(i as isize) != 0 {
        if !(strchr(sep, *str.offset(i as isize) as libc::c_int)).is_null() {
            lil_list_append(list, val);
            val= lil_alloc_string(b"\0" as *const u8 as *const libc::c_char);
        } else {();
            lil_append_char(val, *str.offset(i as isize));
        }
        i= i.wrapping_add(1);
    }
    lil_list_append(list, val);
    val= lil_list_to_value(list, 1 as libc::c_int);
    lil_free_list(list);
    return val;
}
unsafe extern "C" fn fnc_try(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    let mut r = 0 as *mut _lil_value_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if (*lil).error != 0 {
        return 0 as lil_value_t;
    }
    r= lil_parse_value(lil.as_mut(), *argv.offset(0 as libc::c_int as isize), 0 as libc::c_int);
    if (*lil).error != 0 {
        (*lil).error= 0 as libc::c_int;
        lil_free_value(r);
        if argc > 1 as libc::c_int as libc::c_ulong {
            r= lil_parse_value(
                lil.as_mut(),
                *argv.offset(1 as libc::c_int as isize),
                0 as libc::c_int,
            );
        } else {
            r= 0 as lil_value_t;
        }
    }
    return r;
}
unsafe extern "C" fn fnc_error(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    lil_set_error(
        lil.as_mut(),
        if argc > 0 as libc::c_int as libc::c_ulong {
            lil_to_string(*argv.offset(0 as libc::c_int as isize))
        } else {
            0 as *const libc::c_char
        },
    );
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_exit(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    if ((*lil).callback[0 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_exit_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_exit_callback_proc_t,
        >((*lil).callback[0 as libc::c_int as usize]);
        proc_0
            .expect(
                "non-null function pointer",
            )(
            lil,
            if argc > 0 as libc::c_int as libc::c_ulong {
                *argv.offset(0 as libc::c_int as isize)
            } else {
                0 as lil_value_t
            },
        );
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_source(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut f = 0 as *mut FILE;
    let mut size: size_t = 0;
    let mut buffer = 0 as *mut libc::c_char;
    let mut r = 0 as *mut _lil_value_t;
    if argc < 1 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    if ((*lil).callback[4 as libc::c_int as usize]).is_some() {
        let mut proc_0: lil_source_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_source_callback_proc_t,
        >((*lil).callback[4 as libc::c_int as usize]);
        buffer= proc_0
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as libc::c_int as isize)));
    } else if ((*lil).callback[2 as libc::c_int as usize]).is_some() {
        let mut proc_1: lil_read_callback_proc_t = ::std::mem::transmute::<
            lil_callback_proc_t,
            lil_read_callback_proc_t,
        >((*lil).callback[2 as libc::c_int as usize]);
        buffer= proc_1
            .expect(
                "non-null function pointer",
            )(lil, lil_to_string(*argv.offset(0 as libc::c_int as isize)));
    } else {
        f= fopen(
            lil_to_string(*argv.offset(0 as libc::c_int as isize)),
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if f.is_null() {();
            return 0 as lil_value_t;
        }
        fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
        size= ftell(f) as size_t;
        fseek(f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
        buffer= malloc(size.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        fread(buffer as *mut libc::c_void, 1 as libc::c_int as libc::c_ulong, size, f);
        *buffer.offset(size as isize) = 0 as libc::c_int as libc::c_char;
        fclose(f);
    }
    r= lil_parse(lil, buffer as *const i8, 0 as libc::c_int as size_t, 0 as libc::c_int);
    free(buffer as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn fnc_lmap(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut _lil_value_t {
    let mut list = 0 as *mut _lil_list_t;
    let mut i: size_t = 0;
    if argc < 2 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    list= lil_subst_to_list(lil.as_mut(), *argv.offset(0 as libc::c_int as isize));
    i= 1 as libc::c_int as size_t;
    while i < argc {
        lil_set_var(
            lil,
            lil_to_string(*argv.offset(i as isize)),
            lil_list_get(list, i.wrapping_sub(1 as libc::c_int as libc::c_ulong)),
            1 as libc::c_int,
        );
        i= i.wrapping_add(1);
    }
    lil_free_list(list);
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_rand(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    return lil_alloc_double(
        rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double,
    );
}
unsafe extern "C" fn fnc_catcher(
    mut lil: *mut _lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut _lil_value_t,
) -> *mut /* owning */ _lil_value_t {
    if argc == 0 as libc::c_int as libc::c_ulong {
        return lil_alloc_string((*lil).catcher)
    } else {
        let mut catcher = lil_to_string(*argv.offset(0 as libc::c_int as isize));
        free((*lil).catcher as *mut libc::c_void);
        (*lil).catcher= if *catcher.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            strclone(catcher)
        } else {
            0 as *mut libc::c_char
        };
    }
    return 0 as lil_value_t;
}
unsafe extern "C" fn register_stdcmds(mut lil: Option<&mut _lil_t>) {
    lil_register(
        lil.as_deref_mut(),
        b"reflect\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_reflect
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"func\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_func
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"rename\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_rename
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"unusedname\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_unusedname
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"quote\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_quote
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"set\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_set
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"local\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_local
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"write\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_write
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"print\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_print
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"eval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_eval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"topeval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_topeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"upeval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_upeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"downeval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_downeval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"enveval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_enveval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"jaileval\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_jaileval
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"count\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_count
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"index\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_index
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"indexof\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_indexof
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"filter\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_filter
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"list\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_list
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"append\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_append
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"slice\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_slice
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"subst\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_subst
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"concat\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_concat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"foreach\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_foreach
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"return\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_return
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"result\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_result
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"expr\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_expr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"inc\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_inc
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"dec\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_dec
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"read\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_read
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"store\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_store
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"if\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_if
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"while\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_while
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"for\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_for
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"char\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_char
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"charat\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_charat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"codeat\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_codeat
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"substr\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_substr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"strpos\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_strpos
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"length\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_length
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"trim\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_trim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"ltrim\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_ltrim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"rtrim\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_rtrim
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"strcmp\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_strcmp
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"streq\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_streq
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"repstr\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_repstr
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"split\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_split
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"try\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_try
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"error\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_error
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"exit\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_exit
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"source\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_source
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"lmap\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_lmap
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"rand\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_rand
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil.as_deref_mut(),
        b"catcher\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_catcher
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    (*lil.as_deref_mut().unwrap()).syscmds= (*lil.as_deref().unwrap()).cmds;
}
