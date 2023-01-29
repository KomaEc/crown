use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht {
    pub entries: *mut ht_entry,
    pub capacity: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *const libc::c_char,
    pub value: *mut libc::c_void,
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hti {
    pub key: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub _table: *mut ht,
    pub _index: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn ht_create() -> *mut ht {
    let mut table = malloc(::std::mem::size_of::<ht>() as libc::c_ulong) as *mut ht;
    if table.is_null() {
        return 0 as *mut ht;
    }
    (*table).length = 0 as libc::c_int as size_t;
    (*table).capacity = 16 as libc::c_int as size_t;
    let ref mut fresh0 = (*table).entries;
    *fresh0 = calloc(
        (*table).capacity,
        ::std::mem::size_of::<ht_entry>() as libc::c_ulong,
    ) as *mut ht_entry;
    if ((*table).entries).is_null() {
        free(table as *mut libc::c_void);
        return 0 as *mut ht;
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn ht_destroy(mut table: *mut ht) {
    let mut i = 0 as libc::c_int as size_t;
    while i < (*table).capacity {
        free((*((*table).entries).offset(i as isize)).key as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free((*table).entries as *mut libc::c_void);
    free(table as *mut libc::c_void);
}
unsafe extern "C" fn hash_key(mut key: *const libc::c_char) -> uint64_t {
    let mut hash = 14695981039346656037 as libc::c_ulong;
    let mut p = key;
    while *p != 0 {
        hash ^= *p as libc::c_uchar as uint64_t;
        hash = (hash as libc::c_ulong).wrapping_mul(1099511628211 as libc::c_ulong)
            as uint64_t as uint64_t;
        p = p.offset(1);
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn ht_get(
    mut table: *mut ht,
    mut key: *const libc::c_char,
) -> *mut libc::c_void {
    let mut hash = hash_key(key);
    let mut index = hash
        & ((*table).capacity).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while !((*((*table).entries).offset(index as isize)).key).is_null() {
        if strcmp(key, (*((*table).entries).offset(index as isize)).key)
            == 0 as libc::c_int
        {
            return (*((*table).entries).offset(index as isize)).value;
        }
        index = index.wrapping_add(1);
        if index >= (*table).capacity {
            index = 0 as libc::c_int as size_t;
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn ht_set_entry(
    mut entries: *mut ht_entry,
    mut capacity: size_t,
    mut key: *const libc::c_char,
    mut value: *mut libc::c_void,
    mut plength: *mut size_t,
) -> *const libc::c_char {
    let mut hash = hash_key(key);
    let mut index = hash & capacity.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while !((*entries.offset(index as isize)).key).is_null() {
        if strcmp(key, (*entries.offset(index as isize)).key) == 0 as libc::c_int {
            let ref mut fresh1 = (*entries.offset(index as isize)).value;
            *fresh1 = value;
            return (*entries.offset(index as isize)).key;
        }
        index = index.wrapping_add(1);
        if index >= capacity {
            index = 0 as libc::c_int as size_t;
        }
    }
    if !plength.is_null() {
        key = strdup(key);
        if key.is_null() {
            return 0 as *const libc::c_char;
        }
        *plength = (*plength).wrapping_add(1);
    }
    let ref mut fresh2 = (*entries.offset(index as isize)).key;
    *fresh2 = key as *mut libc::c_char;
    let ref mut fresh3 = (*entries.offset(index as isize)).value;
    *fresh3 = value;
    return key;
}
unsafe extern "C" fn ht_expand(mut table: *mut ht) -> bool {
    let mut new_capacity = ((*table).capacity)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if new_capacity < (*table).capacity {
        return 0 as libc::c_int != 0;
    }
    let mut new_entries = calloc(
        new_capacity,
        ::std::mem::size_of::<ht_entry>() as libc::c_ulong,
    ) as *mut ht_entry;
    if new_entries.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut i = 0 as libc::c_int as size_t;
    while i < (*table).capacity {
        let mut entry = *((*table).entries).offset(i as isize);
        if !(entry.key).is_null() {
            ht_set_entry(
                new_entries,
                new_capacity,
                entry.key,
                entry.value,
                0 as *mut size_t,
            );
        }
        i = i.wrapping_add(1);
    }
    free((*table).entries as *mut libc::c_void);
    let ref mut fresh4 = (*table).entries;
    *fresh4 = new_entries;
    (*table).capacity = new_capacity;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ht_set(
    mut table: *mut ht,
    mut key: *const libc::c_char,
    mut value: *mut libc::c_void,
) -> *const libc::c_char {
    if !value.is_null() {} else {
        __assert_fail(
            b"value != NULL\0" as *const u8 as *const libc::c_char,
            b"ht.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 47],
                &[libc::c_char; 47],
            >(b"const char *ht_set(ht *, const char *, void *)\0"))
                .as_ptr(),
        );
    }
    if value.is_null() {
        return 0 as *const libc::c_char;
    }
    if (*table).length
        >= ((*table).capacity).wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        if !ht_expand(table) {
            return 0 as *const libc::c_char;
        }
    }
    return ht_set_entry(
        (*table).entries,
        (*table).capacity,
        key,
        value,
        &mut (*table).length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ht_length(mut table: *mut ht) -> size_t {
    return (*table).length;
}
#[no_mangle]
pub unsafe extern "C" fn ht_iterator(mut table: *mut ht) -> hti {
    let mut it = hti {
        key: 0 as *const libc::c_char,
        value: 0 as *mut libc::c_void,
        _table: 0 as *mut ht,
        _index: 0,
    };
    it._table = table;
    it._index = 0 as libc::c_int as size_t;
    return it;
}
#[no_mangle]
pub unsafe extern "C" fn ht_next(mut it: *mut hti) -> bool {
    let mut table = (*it)._table;
    while (*it)._index < (*table).capacity {
        let mut i = (*it)._index;
        let ref mut fresh5 = (*it)._index;
        *fresh5 = (*fresh5).wrapping_add(1);
        if !((*((*table).entries).offset(i as isize)).key).is_null() {
            let mut entry = *((*table).entries).offset(i as isize);
            let ref mut fresh6 = (*it).key;
            *fresh6 = entry.key;
            let ref mut fresh7 = (*it).value;
            *fresh7 = entry.value;
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
