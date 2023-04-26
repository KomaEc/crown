
extern "C" {
    fn __assert_fail(
        __assertion: * const std::os::raw::c_char,
        __file: * const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: * const std::os::raw::c_char,
    ) -> !;
    fn free(_: * mut core::ffi::c_void);
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strdup(_: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht {
    pub entries: * mut crate::src::ht::ht_entry,
    pub capacity: std::os::raw::c_ulong,
    pub length: std::os::raw::c_ulong,
}
impl ht {
    pub const fn new() -> Self {
        ht {
        entries: (0 as * mut crate::src::ht::ht_entry),
        capacity: 0,
        length: 0
        }
    }
}

impl std::default::Default for ht {
    fn default() -> Self { ht::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: * const std::os::raw::c_char,
    pub value: * mut core::ffi::c_void,
}
impl ht_entry {
    pub const fn new() -> Self {
        ht_entry {
        key: (0 as * const std::os::raw::c_char),
        value: (0 as * mut core::ffi::c_void)
        }
    }
}

impl std::default::Default for ht_entry {
    fn default() -> Self { ht_entry::new() }
}

pub type uint64_t = std::os::raw::c_ulong;
pub type __uint64_t = std::os::raw::c_ulong;
// #[derive(Copy, Clone)]
#[repr(C)]
pub struct hti<'a1> {
    pub key: * const std::os::raw::c_char,
    pub value: * mut core::ffi::c_void,
    pub _table: Option<&'a1 mut crate::src::ht::ht>,
    pub _index: std::os::raw::c_ulong,
}
impl<'a1> hti<'a1> {
    pub const fn new() -> Self {
        hti {
        key: (0 as * const std::os::raw::c_char),
        value: (0 as * mut core::ffi::c_void),
        _table: None,
        _index: 0
        }
    }
}

impl<'a1> std::default::Default for hti<'a1> {
    fn default() -> Self { hti::new() }
}

#[no_mangle]
pub unsafe extern "C" fn ht_create() -> * mut crate::src::ht::ht {
    let mut table = malloc(::std::mem::size_of::<ht>() as std::os::raw::c_ulong) as *mut ht;
    if table.is_null() {
        return 0 as *mut ht;
    }
    (*table).length = 0 as std::os::raw::c_int as size_t;
    (*table).capacity = 16 as std::os::raw::c_int as size_t;
    let ref mut fresh0 = (*table).entries;
    *fresh0 = calloc(
        (*table).capacity,
        ::std::mem::size_of::<ht_entry>() as std::os::raw::c_ulong,
    ) as *mut ht_entry;
    if ((*table).entries).is_null() {
        free(table as *mut std::os::raw::c_void);
        return 0 as *mut ht;
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn ht_destroy(mut table: * mut crate::src::ht::ht) {
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < (*table).capacity {
        free((*((*table).entries).offset(i as isize)).key as *mut std::os::raw::c_void);
        i = i.wrapping_add(1);
    }
    free((*table).entries as *mut std::os::raw::c_void);
    free(table as *mut std::os::raw::c_void);
}
unsafe extern "C" fn hash_key(mut key: * const std::os::raw::c_char) -> std::os::raw::c_ulong {
    let mut hash = 14695981039346656037 as std::os::raw::c_ulong;
    let mut p = key;
    while *p != 0 {
        hash ^= *p as std::os::raw::c_uchar as uint64_t;
        hash = (hash as std::os::raw::c_ulong).wrapping_mul(1099511628211 as std::os::raw::c_ulong)
            as uint64_t as uint64_t;
        p = p.offset(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn ht_get<'a1>(
    mut table: Option<&'a1 mut crate::src::ht::ht>,
    mut key: * const std::os::raw::c_char,
) -> * mut core::ffi::c_void {
    let mut hash = hash_key(key);
    let mut index = hash
        & ((*(borrow(& table)).unwrap()).capacity).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while !((*((*(borrow(& table)).unwrap()).entries).offset(index as isize)).key).is_null() {
        if strcmp(key, (*((*(borrow(& table)).unwrap()).entries).offset(index as isize)).key)
            == 0 as std::os::raw::c_int
        {
            return (*((*(borrow(& table)).unwrap()).entries).offset(index as isize)).value;
        }
        index = index.wrapping_add(1);
        if index >= (*(borrow(& table)).unwrap()).capacity {
            index = 0 as std::os::raw::c_int as size_t;
        }
    }
    return (0 as * mut core::ffi::c_void);
}
unsafe extern "C" fn ht_set_entry(
    mut entries: * mut crate::src::ht::ht_entry,
    mut capacity: std::os::raw::c_ulong,
    mut key: * const std::os::raw::c_char,
    mut value: * mut core::ffi::c_void,
    mut plength: * mut std::os::raw::c_ulong,
) -> * const std::os::raw::c_char {
    let mut hash = hash_key(key);
    let mut index = hash & capacity.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    while !((*entries.offset(index as isize)).key).is_null() {
        if strcmp(key, (*entries.offset(index as isize)).key) == 0 as std::os::raw::c_int {
            let ref mut fresh1 = (*entries.offset(index as isize)).value;
            *fresh1 = value;
            return (*entries.offset(index as isize)).key;
        }
        index = index.wrapping_add(1);
        if index >= capacity {
            index = 0 as std::os::raw::c_int as size_t;
        }
    }
    if !plength.is_null() {
        key = strdup(key);
        if key.is_null() {
            return 0 as *const std::os::raw::c_char;
        }
        *plength = (*plength).wrapping_add(1);
    }
    let ref mut fresh2 = (*entries.offset(index as isize)).key;
    *fresh2 = key as *mut std::os::raw::c_char;
    let ref mut fresh3 = (*entries.offset(index as isize)).value;
    *fresh3 = value;
    return key;
}
unsafe extern "C" fn ht_expand<'a1>(mut table: Option<&'a1 mut crate::src::ht::ht>) -> bool {
    let mut new_capacity = ((*(borrow(& table)).unwrap()).capacity)
        .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong);
    if new_capacity < (*(borrow(& table)).unwrap()).capacity {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut new_entries = calloc(
        new_capacity,
        ::std::mem::size_of::<ht_entry>() as std::os::raw::c_ulong,
    ) as *mut ht_entry;
    if new_entries.is_null() {
        return 0 as std::os::raw::c_int != 0;
    }
    let mut i = 0 as std::os::raw::c_int as size_t;
    while i < (*(borrow(& table)).unwrap()).capacity {
        let mut entry = *((*(borrow(& table)).unwrap()).entries).offset(i as isize);
        if !(entry.key).is_null() {
            ht_set_entry(
                new_entries,
                new_capacity,
                entry.key,
                entry.value,
                (0 as * mut u64),
            );
        }
        i = i.wrapping_add(1);
    }
    free((*(borrow_mut(&mut table)).unwrap()).entries as *mut std::os::raw::c_void);
    let ref mut fresh4 = (*(borrow_mut(&mut table)).unwrap()).entries;
    *fresh4 = new_entries;
    (*(borrow_mut(&mut table)).unwrap()).capacity = new_capacity;
    return 1 as std::os::raw::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ht_set<'a1>(
    mut table: Option<&'a1 mut crate::src::ht::ht>,
    mut key: * const std::os::raw::c_char,
    mut value: * mut core::ffi::c_void,
) -> * const std::os::raw::c_char {
    if !value.is_null() {} else {
        __assert_fail(
            b"value != NULL\0" as *const u8 as *const std::os::raw::c_char,
            b"ht.c\0" as *const u8 as *const std::os::raw::c_char,
            154 as std::os::raw::c_int as std::os::raw::c_uint,
            (*core::intrinsics::transmute::<&'_ [u8; 47], &'_ [i8; 47]>(b"const char *ht_set(ht *, const char *, void *)\0"))
                .as_ptr(),
        );
    }
    if value.is_null() {
        return 0 as *const std::os::raw::c_char;
    }
    if (*(borrow(& table)).unwrap()).length
        >= ((*(borrow(& table)).unwrap()).capacity).wrapping_div(2 as std::os::raw::c_int as std::os::raw::c_ulong)
    {
        if !ht_expand(borrow_mut(&mut table)) {
            return 0 as *const std::os::raw::c_char;
        }
    }
    return ht_set_entry(
        (*(borrow_mut(&mut table)).unwrap()).entries,
        (*(borrow_mut(&mut table)).unwrap()).capacity,
        key,
        value,
        &mut (*(borrow_mut(&mut table)).unwrap()).length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ht_length<'a1>(mut table: Option<&'a1 mut crate::src::ht::ht>) -> std::os::raw::c_ulong {
    return (*(borrow(& table)).unwrap()).length;
}
#[no_mangle]
pub extern "C" fn ht_iterator<'a1, 'a2>(mut table: Option<&'a1 mut crate::src::ht::ht>) -> crate::src::ht::hti<'a2> where 'a1: 'a2 {
    let mut it = hti {
        key: 0 as *const std::os::raw::c_char,
        value: (0 as * mut core::ffi::c_void),
        _table: Option::<&'_ mut crate::src::ht::ht>::None,
        _index: 0,
    };
    it._table = table;
    it._index = 0 as std::os::raw::c_int as size_t;
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn ht_next<'a1>(mut it: * mut crate::src::ht::hti<'a1>) -> bool {
    let mut table = borrow_mut(&mut (*it)._table);
    while (*it)._index < (*(borrow(& table)).unwrap()).capacity {
        let mut i = (*it)._index;
        let ref mut fresh5 = (*it)._index;
        *fresh5 = (*fresh5).wrapping_add(1);
        if !((*((*(borrow(& table)).unwrap()).entries).offset(i as isize)).key).is_null() {
            let mut entry = *((*(borrow(& table)).unwrap()).entries).offset(i as isize);
            let ref mut fresh6 = (*it).key;
            *fresh6 = entry.key;
            let ref mut fresh7 = (*it).value;
            *fresh7 = entry.value;
            return 1 as std::os::raw::c_int != 0;
        }
    }
    return 0 as std::os::raw::c_int != 0;
}
use crate::laertes_rt::*;
