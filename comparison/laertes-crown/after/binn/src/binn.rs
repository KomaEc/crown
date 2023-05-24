
extern "C" {
    fn snprintf(
        _: * mut std::os::raw::c_char,
        _: std::os::raw::c_ulong,
        _: * const std::os::raw::c_char,
        _: ...
    ) -> std::os::raw::c_int;
    fn atof(__nptr: * const std::os::raw::c_char) -> std::os::raw::c_double;
    fn atoi(__nptr: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn realloc(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
    fn strcpy(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
    fn strdup(_: * const std::os::raw::c_char) -> * mut std::os::raw::c_char;
    fn memset(
        _: * mut core::ffi::c_void,
        _: std::os::raw::c_int,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn memmove(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn strcasecmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strncasecmp(
        _: * const std::os::raw::c_char,
        _: * const std::os::raw::c_char,
        _: std::os::raw::c_ulong,
    ) -> std::os::raw::c_int;
    fn strlen(_: * const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
pub type BOOL = std::os::raw::c_int;
pub type int64 = std::os::raw::c_longlong;
pub type uint64 = std::os::raw::c_ulonglong;
pub type binn_mem_free<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_struct {
    pub header: std::os::raw::c_int,
    pub allocated: std::os::raw::c_int,
    pub writable: std::os::raw::c_int,
    pub dirty: std::os::raw::c_int,
    pub pbuf: * mut core::ffi::c_void,
    pub pre_allocated: std::os::raw::c_int,
    pub alloc_size: std::os::raw::c_int,
    pub used_size: std::os::raw::c_int,
    pub type_0: std::os::raw::c_int,
    pub ptr: * mut core::ffi::c_void,
    pub size: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub freefn: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
    pub c2rust_unnamed: crate::src::binn::C2RustUnnamed,
    pub disable_int_compression: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vint8: std::os::raw::c_schar,
    pub vint16: std::os::raw::c_short,
    pub vint32: std::os::raw::c_int,
    pub vint64: int64,
    pub vuint8: std::os::raw::c_uchar,
    pub vuint16: std::os::raw::c_ushort,
    pub vuint32: std::os::raw::c_uint,
    pub vuint64: uint64,
    pub vchar: std::os::raw::c_schar,
    pub vuchar: std::os::raw::c_uchar,
    pub vshort: std::os::raw::c_short,
    pub vushort: std::os::raw::c_ushort,
    pub vint: std::os::raw::c_int,
    pub vuint: std::os::raw::c_uint,
    pub vfloat: std::os::raw::c_float,
    pub vdouble: std::os::raw::c_double,
    pub vbool: BOOL,
}
pub type binn = crate::src::binn::binn_struct;
pub type u32_0 = std::os::raw::c_uint;
pub type u64_0 = std::os::raw::c_ulonglong;
pub type u16_0 = std::os::raw::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_iter_struct {
    pub pnext: * mut std::os::raw::c_uchar,
    pub plimit: * mut std::os::raw::c_uchar,
    pub type_0: std::os::raw::c_int,
    pub count: std::os::raw::c_int,
    pub current: std::os::raw::c_int,
}
impl binn_iter_struct {
    pub const fn new() -> Self {
        binn_iter_struct {
        pnext: (0 as * mut std::os::raw::c_uchar),
        plimit: (0 as * mut std::os::raw::c_uchar),
        type_0: 0,
        count: 0,
        current: 0
        }
    }
}

impl std::default::Default for binn_iter_struct {
    fn default() -> Self { binn_iter_struct::new() }
}

pub type binn_iter = crate::src::binn::binn_iter_struct;
#[inline(always)]
unsafe extern "C" fn binn_list_add_value(
    mut list: * mut crate::src::binn::binn_struct,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_list_add(
        list,
        (*value).type_0,
        binn_ptr(value as *mut std::os::raw::c_void),
        binn_size(value as *mut std::os::raw::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_map_set_value(
    mut map: * mut crate::src::binn::binn_struct,
    mut id: std::os::raw::c_int,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_map_set(
        map,
        id,
        (*value).type_0,
        binn_ptr(value as *mut std::os::raw::c_void),
        binn_size(value as *mut std::os::raw::c_void),
    );
}
#[inline(always)]
unsafe extern "C" fn binn_object_set_value(
    mut obj: * mut crate::src::binn::binn_struct,
    mut key: * const std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_object_set(
        obj,
        key,
        (*value).type_0,
        binn_ptr(value as *mut std::os::raw::c_void),
        binn_size(value as *mut std::os::raw::c_void),
    );
}
#[no_mangle]
pub static mut malloc_fn: Option<unsafe extern "C"  fn(_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void> = None; unsafe fn laertes_init_malloc_fn() {
malloc_fn = None;}//;
#[no_mangle]
pub static mut realloc_fn: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void> = None; unsafe fn laertes_init_realloc_fn() {
realloc_fn = None;}//;
#[no_mangle]
pub static mut free_fn: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()> = None; unsafe fn laertes_init_free_fn() {
free_fn = None;}//;
unsafe extern "C" fn copy_be16(mut pdest: * mut std::os::raw::c_ushort, mut psource: * mut std::os::raw::c_ushort) {
    let mut source = psource as *mut std::os::raw::c_uchar;
    let mut dest = pdest as *mut std::os::raw::c_uchar;
    *dest.offset(0 as std::os::raw::c_int as isize) = *source.offset(1 as std::os::raw::c_int as isize);
    *dest.offset(1 as std::os::raw::c_int as isize) = *source.offset(0 as std::os::raw::c_int as isize);
}
unsafe extern "C" fn copy_be32(mut pdest: * mut std::os::raw::c_uint, mut psource: * mut std::os::raw::c_uint) {
    let mut source = psource as *mut std::os::raw::c_uchar;
    let mut dest = pdest as *mut std::os::raw::c_uchar;
    *dest.offset(0 as std::os::raw::c_int as isize) = *source.offset(3 as std::os::raw::c_int as isize);
    *dest.offset(1 as std::os::raw::c_int as isize) = *source.offset(2 as std::os::raw::c_int as isize);
    *dest.offset(2 as std::os::raw::c_int as isize) = *source.offset(1 as std::os::raw::c_int as isize);
    *dest.offset(3 as std::os::raw::c_int as isize) = *source.offset(0 as std::os::raw::c_int as isize);
}
unsafe extern "C" fn copy_be64(mut pdest: * mut std::os::raw::c_ulonglong, mut psource: * mut std::os::raw::c_ulonglong) {
    let mut source = psource as *mut std::os::raw::c_uchar;
    let mut dest = pdest as *mut std::os::raw::c_uchar;
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 8 as std::os::raw::c_int {
        *dest.offset(i as isize) = *source.offset((7 as std::os::raw::c_int - i) as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn binn_version() -> * mut std::os::raw::c_char {
    return b"3.0.0\0" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_alloc_functions(
    mut new_malloc: Option<unsafe extern "C"  fn(_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void>,
    mut new_realloc: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void>,
    mut new_free: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) {
    malloc_fn = new_malloc;
    realloc_fn = new_realloc;
    free_fn = new_free;
}
unsafe extern "C" fn check_alloc_functions() {
    if malloc_fn.is_none() {
        malloc_fn = Some(
            malloc,
        );
    }
    if realloc_fn.is_none() {
        realloc_fn = Some(
            realloc,
        );
    }
    if free_fn.is_none() {
        free_fn = Some(free);
    }
}
unsafe extern "C" fn binn_malloc(mut size: std::os::raw::c_int) -> * mut core::ffi::c_void {
    check_alloc_functions();
    return malloc_fn.expect("non-null function pointer")(size as size_t);
}
unsafe extern "C" fn binn_memdup(
    mut src: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut dest = 0 as *mut std::os::raw::c_void;
    if src.is_null() || size <= 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    dest = binn_malloc(size);
    if dest.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    memcpy(dest, src, size as std::os::raw::c_ulong);
    return dest;
}
unsafe extern "C" fn strlen2(mut str: * mut std::os::raw::c_char) -> std::os::raw::c_ulong {
    if str.is_null() {
        return 0 as std::os::raw::c_int as size_t;
    }
    return strlen(str);
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_type(
    mut storage_type: std::os::raw::c_int,
    mut data_type_index: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if data_type_index < 0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int);
    }
    if storage_type < 0 as std::os::raw::c_int || storage_type > 0xe0 as std::os::raw::c_int {
        return -(1 as std::os::raw::c_int);
    }
    if data_type_index < 16 as std::os::raw::c_int {
        return storage_type | data_type_index
    } else if data_type_index < 4096 as std::os::raw::c_int {
        storage_type |= 0x10 as std::os::raw::c_int;
        storage_type <<= 8 as std::os::raw::c_int;
        data_type_index >>= 4 as std::os::raw::c_int;
        return storage_type | data_type_index;
    } else {
        return -(1 as std::os::raw::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_type_info<'a1, 'a2>(
    mut long_type: std::os::raw::c_int,
    mut pstorage_type: Option<&'a1 mut std::os::raw::c_int>,
    mut pextra_type: Option<&'a2 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut storage_type: i32 = 0;
    let mut extra_type: i32 = 0;
    let mut retval = 1 as std::os::raw::c_int;
    let mut current_block_11: u64;
    loop {
        if long_type < 0 as std::os::raw::c_int {
            current_block_11 = 9393356739754453642;
            break;
        }
        if long_type <= 0xff as std::os::raw::c_int {
            storage_type = long_type & 0xe0 as std::os::raw::c_int;
            extra_type = long_type & 0xf as std::os::raw::c_int;
            current_block_11 = 17833034027772472439;
            break;
        } else if long_type <= 0xffff as std::os::raw::c_int {
            storage_type = long_type & 0xe000 as std::os::raw::c_int;
            storage_type >>= 8 as std::os::raw::c_int;
            extra_type = long_type & 0xfff as std::os::raw::c_int;
            extra_type >>= 4 as std::os::raw::c_int;
            current_block_11 = 17833034027772472439;
            break;
        } else {
            if !(long_type & 0x80000 as std::os::raw::c_int != 0) {
                current_block_11 = 9393356739754453642;
                break;
            }
            long_type &= 0xffff as std::os::raw::c_int;
        }
    }
    match current_block_11 {
        9393356739754453642 => {
            storage_type = -(1 as std::os::raw::c_int);
            extra_type = -(1 as std::os::raw::c_int);
            retval = 0 as std::os::raw::c_int;
        }
        _ => {}
    }
    if !borrow(& pstorage_type).is_none() {
        *(borrow_mut(&mut pstorage_type)).unwrap() = storage_type;
    }
    if !borrow(& pextra_type).is_none() {
        *(borrow_mut(&mut pextra_type)).unwrap() = extra_type;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create(
    mut item: * mut crate::src::binn::binn_struct,
    mut type_0: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut pointer: * mut core::ffi::c_void,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut retval = 0 as std::os::raw::c_int;
    match type_0 {
        224 | 225 | 226 => {
            if !(item.is_null() || size < 0 as std::os::raw::c_int) {
                if size < 3 as std::os::raw::c_int {
                    if !pointer.is_null() {
                        current_block = 13652966409768237295;
                    } else {
                        size = 0 as std::os::raw::c_int;
                        current_block = 14523784380283086299;
                    }
                } else {
                    current_block = 14523784380283086299;
                }
                match current_block {
                    13652966409768237295 => {}
                    _ => {
                        memset(
                            item as *mut std::os::raw::c_void,
                            0 as std::os::raw::c_int,
                            ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
                        );
                        if !pointer.is_null() {
                            (*item).pre_allocated = 1 as std::os::raw::c_int;
                            let ref mut fresh0 = (*item).pbuf;
                            *fresh0 = pointer;
                            (*item).alloc_size = size;
                        } else {
                            (*item).pre_allocated = 0 as std::os::raw::c_int;
                            if size == 0 as std::os::raw::c_int {
                                size = 256 as std::os::raw::c_int;
                            }
                            pointer = binn_malloc(size);
                            if pointer.is_null() {
                                return 0 as std::os::raw::c_int;
                            }
                            let ref mut fresh1 = (*item).pbuf;
                            *fresh1 = pointer;
                            (*item).alloc_size = size;
                        }
                        (*item).header = 0x1f22b11f as std::os::raw::c_int;
                        (*item).writable = 1 as std::os::raw::c_int;
                        (*item).used_size = 9 as std::os::raw::c_int;
                        (*item).type_0 = type_0;
                        (*item).dirty = 1 as std::os::raw::c_int;
                        retval = 1 as std::os::raw::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_new(
    mut type_0: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut pointer: * mut core::ffi::c_void,
) -> * mut crate::src::binn::binn_struct {
    let mut item = 0 as *mut binn;
    item = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_create(item, type_0, size, pointer) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(item as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*item).allocated = 1 as std::os::raw::c_int;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_list(mut list: * mut crate::src::binn::binn_struct) -> std::os::raw::c_int {
    return binn_create(
        list,
        0xe0 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_map(mut map: * mut crate::src::binn::binn_struct) -> std::os::raw::c_int {
    return binn_create(
        map,
        0xe1 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_create_object(mut object: * mut crate::src::binn::binn_struct) -> std::os::raw::c_int {
    return binn_create(
        object,
        0xe2 as std::os::raw::c_int,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list() -> * mut crate::src::binn::binn_struct {
    return binn_new(0xe0 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map() -> * mut crate::src::binn::binn_struct {
    return binn_new(0xe1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object() -> * mut crate::src::binn::binn_struct {
    return binn_new(0xe2 as std::os::raw::c_int, 0 as std::os::raw::c_int, 0 as *mut std::os::raw::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn binn_copy(mut old: * mut core::ffi::c_void) -> * mut crate::src::binn::binn_struct {
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size: i32 = 0;
    let mut header_size: i32 = 0;
    let mut old_ptr = binn_ptr(old) as *mut std::os::raw::c_uchar;
    let mut item = 0 as *mut binn;
    size = 0 as std::os::raw::c_int;
    if IsValidBinnHeader(
        old_ptr as *mut std::os::raw::c_void,
        Some(&mut type_0),
        Some(&mut count),
        Some(&mut size),
        Some(&mut header_size),
    ) == 0
    {
        return 0 as *mut binn;
    }
    item = binn_new(
        type_0,
        size - header_size + 9 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
    );
    if !item.is_null() {
        let mut dest = 0 as *mut std::os::raw::c_uchar;
        dest = ((*item).pbuf as *mut std::os::raw::c_uchar).offset(9 as std::os::raw::c_int as isize);
        memcpy(
            dest as *mut std::os::raw::c_void,
            old_ptr.offset(header_size as isize) as *const std::os::raw::c_void,
            (size - header_size) as std::os::raw::c_ulong,
        );
        (*item).used_size = 9 as std::os::raw::c_int + size - header_size;
        (*item).count = count;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_load(
    mut data: * mut core::ffi::c_void,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    if data.is_null() || value.is_null() {
        return 0 as std::os::raw::c_int;
    }
    memset(
        value as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
    );
    (*value).header = 0x1f22b11f as std::os::raw::c_int;
    if binn_is_valid(data, &mut (*value).type_0, &mut (*value).count, &mut (*value).size)
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh2 = (*value).ptr;
    *fresh2 = data;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_open(mut data: * mut core::ffi::c_void) -> * mut crate::src::binn::binn_struct {
    let mut item = 0 as *mut binn;
    item = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_load(data, item) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(item as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*item).allocated = 1 as std::os::raw::c_int;
    return item;
}
unsafe extern "C" fn binn_get_ptr_type(mut ptr: * mut core::ffi::c_void) -> std::os::raw::c_int {
    if ptr.is_null() {
        return 0 as std::os::raw::c_int;
    }
    match *(ptr as *mut std::os::raw::c_uint) {
        522367263 => return 1 as std::os::raw::c_int,
        _ => return 2 as std::os::raw::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_struct(mut ptr: * mut core::ffi::c_void) -> std::os::raw::c_int {
    if ptr.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if *(ptr as *mut std::os::raw::c_uint) == 0x1f22b11f as std::os::raw::c_int as std::os::raw::c_uint {
        return 1 as std::os::raw::c_int
    } else {
        return 0 as std::os::raw::c_int
    };
}
unsafe extern "C" fn CalcAllocation(
    mut needed_size: std::os::raw::c_int,
    mut alloc_size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut calc_size: i32 = 0;
    calc_size = alloc_size;
    while calc_size < needed_size {
        calc_size <<= 1 as std::os::raw::c_int;
    }
    return calc_size;
}
unsafe extern "C" fn CheckAllocation(
    mut item: * mut crate::src::binn::binn_struct,
    mut add_size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut alloc_size: i32 = 0;
    let mut ptr = 0 as *mut std::os::raw::c_void;
    if (*item).used_size + add_size > (*item).alloc_size {
        if (*item).pre_allocated != 0 {
            return 0 as std::os::raw::c_int;
        }
        alloc_size = CalcAllocation((*item).used_size + add_size, (*item).alloc_size);
        ptr = realloc_fn
            .expect("non-null function pointer")((*item).pbuf, alloc_size as size_t);
        if ptr.is_null() {
            return 0 as std::os::raw::c_int;
        }
        let ref mut fresh3 = (*item).pbuf;
        *fresh3 = ptr;
        (*item).alloc_size = alloc_size;
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn AdvanceDataPos(
    mut p: * mut std::os::raw::c_uchar,
    mut plimit: * mut std::os::raw::c_uchar,
) -> * mut std::os::raw::c_uchar {
    let mut byte: u8 = 0;
    let mut storage_type: i32 = 0;
    let mut DataSize: i32 = 0;
    if p > plimit {
        return 0 as *mut std::os::raw::c_uchar;
    }
    byte = *p;
    p = p.offset(1);
    storage_type = byte as std::os::raw::c_int & 0xe0 as std::os::raw::c_int;
    if byte as std::os::raw::c_int & 0x10 as std::os::raw::c_int != 0 {
        p = p.offset(1);
    }
    match storage_type {
        0 => {}
        32 => {
            p = p.offset(1);
        }
        64 => {
            p = p.offset(2 as std::os::raw::c_int as isize);
        }
        96 => {
            p = p.offset(4 as std::os::raw::c_int as isize);
        }
        128 => {
            p = p.offset(8 as std::os::raw::c_int as isize);
        }
        192 | 160 => {
            if p > plimit {
                return 0 as *mut std::os::raw::c_uchar;
            }
            DataSize = *p as std::os::raw::c_int;
            if DataSize & 0x80 as std::os::raw::c_int != 0 {
                if p
                    .offset(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as isize,
                    )
                    .offset(-(1 as std::os::raw::c_int as isize)) > plimit
                {
                    return 0 as *mut std::os::raw::c_uchar;
                }
                copy_be32(
                    &mut DataSize as *mut std::os::raw::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as std::os::raw::c_int;
                p = p.offset(4 as std::os::raw::c_int as isize);
            } else {
                p = p.offset(1);
            }
            p = p.offset(DataSize as isize);
            if storage_type == 0xa0 as std::os::raw::c_int {
                p = p.offset(1);
            }
        }
        224 => {
            if p > plimit {
                return 0 as *mut std::os::raw::c_uchar;
            }
            DataSize = *p as std::os::raw::c_int;
            if DataSize & 0x80 as std::os::raw::c_int != 0 {
                if p
                    .offset(
                        ::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as isize,
                    )
                    .offset(-(1 as std::os::raw::c_int as isize)) > plimit
                {
                    return 0 as *mut std::os::raw::c_uchar;
                }
                copy_be32(
                    &mut DataSize as *mut std::os::raw::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as std::os::raw::c_int;
            }
            DataSize -= 1;
            p = p.offset(DataSize as isize);
        }
        _ => return 0 as *mut std::os::raw::c_uchar,
    }
    if p > plimit {
        return 0 as *mut std::os::raw::c_uchar;
    }
    return p;
}
unsafe extern "C" fn read_map_id<'a1>(
    mut pp: Option<&'a1 mut * mut std::os::raw::c_uchar>,
    mut plimit: * mut std::os::raw::c_uchar,
) -> std::os::raw::c_int {
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut c: u8 = 0;
    let mut sign: u8 = 0;
    let mut type_0: u8 = 0;
    let mut id: i32 = 0;
    let mut extra_bytes: i32 = 0;
    p = *(borrow_mut(&mut pp)).unwrap();
    let mut fresh4 = p;
    p = p.offset(1);
    c = *fresh4;
    if c as std::os::raw::c_int & 0x80 as std::os::raw::c_int != 0 {
        extra_bytes = ((c as std::os::raw::c_int & 0x60 as std::os::raw::c_int) >> 5 as std::os::raw::c_int)
            + 1 as std::os::raw::c_int;
        if p.offset(extra_bytes as isize) > plimit {
            *(borrow_mut(&mut pp)).unwrap() = p.offset(extra_bytes as isize);
            return 0 as std::os::raw::c_int;
        }
    }
    type_0 = (c as std::os::raw::c_int & 0xe0 as std::os::raw::c_int) as std::os::raw::c_uchar;
    sign = (c as std::os::raw::c_int & 0x10 as std::os::raw::c_int) as std::os::raw::c_uchar;
    if c as std::os::raw::c_int & 0x80 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        sign = (c as std::os::raw::c_int & 0x40 as std::os::raw::c_int) as std::os::raw::c_uchar;
        id = c as std::os::raw::c_int & 0x3f as std::os::raw::c_int;
    } else if type_0 as std::os::raw::c_int == 0x80 as std::os::raw::c_int {
        id = c as std::os::raw::c_int & 0xf as std::os::raw::c_int;
        let mut fresh5 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh5 as std::os::raw::c_int;
    } else if type_0 as std::os::raw::c_int == 0xa0 as std::os::raw::c_int {
        id = c as std::os::raw::c_int & 0xf as std::os::raw::c_int;
        let mut fresh6 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh6 as std::os::raw::c_int;
        let mut fresh7 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh7 as std::os::raw::c_int;
    } else if type_0 as std::os::raw::c_int == 0xc0 as std::os::raw::c_int {
        id = c as std::os::raw::c_int & 0xf as std::os::raw::c_int;
        let mut fresh8 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh8 as std::os::raw::c_int;
        let mut fresh9 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh9 as std::os::raw::c_int;
        let mut fresh10 = p;
        p = p.offset(1);
        id = id << 8 as std::os::raw::c_int | *fresh10 as std::os::raw::c_int;
    } else if type_0 as std::os::raw::c_int == 0xe0 as std::os::raw::c_int {
        copy_be32(&mut id as *mut std::os::raw::c_int as *mut u32_0, p as *mut u32_0);
        p = p.offset(4 as std::os::raw::c_int as isize);
    } else {
        *(borrow_mut(&mut pp)).unwrap() = plimit.offset(2 as std::os::raw::c_int as isize);
        return 0 as std::os::raw::c_int;
    }
    if sign != 0 {
        id = -id;
    }
    *(borrow_mut(&mut pp)).unwrap() = p;
    return id;
}
unsafe extern "C" fn SearchForID(
    mut p: * mut std::os::raw::c_uchar,
    mut header_size: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut numitems: std::os::raw::c_int,
    mut id: std::os::raw::c_int,
) -> * mut std::os::raw::c_uchar {
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut base = 0 as *mut std::os::raw::c_uchar;
    let mut i: i32 = 0;
    let mut int32: i32 = 0;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as std::os::raw::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as std::os::raw::c_int;
    while i < numitems {
        int32 = read_map_id(Some(&mut p), plimit);
        if p > plimit {
            break;
        }
        if int32 == id {
            return p;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
    }
    return 0 as *mut std::os::raw::c_uchar;
}
unsafe extern "C" fn SearchForKey(
    mut p: * mut std::os::raw::c_uchar,
    mut header_size: std::os::raw::c_int,
    mut size: std::os::raw::c_int,
    mut numitems: std::os::raw::c_int,
    mut key: * const std::os::raw::c_char,
) -> * mut std::os::raw::c_uchar {
    let mut len: u8 = 0;
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut base = 0 as *mut std::os::raw::c_uchar;
    let mut i: i32 = 0;
    let mut keylen: i32 = 0;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as std::os::raw::c_int as isize));
    p = p.offset(header_size as isize);
    keylen = strlen(key) as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < numitems {
        len = *p;
        p = p.offset(1);
        if p > plimit {
            break;
        }
        if len as std::os::raw::c_int > 0 as std::os::raw::c_int {
            if strncasecmp(p as *mut std::os::raw::c_char, key, len as std::os::raw::c_ulong)
                == 0 as std::os::raw::c_int
            {
                if keylen == len as std::os::raw::c_int {
                    p = p.offset(len as std::os::raw::c_int as isize);
                    return p;
                }
            }
            p = p.offset(len as std::os::raw::c_int as isize);
            if p > plimit {
                break;
            }
        } else if len as std::os::raw::c_int == keylen {
            return p
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            break;
        }
        i += 1;
    }
    return 0 as *mut std::os::raw::c_uchar;
}
unsafe extern "C" fn binn_list_add_raw(
    mut item: * mut crate::src::binn::binn_struct,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if item.is_null() || (*item).type_0 != 0xe0 as std::os::raw::c_int
        || (*item).writable == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if AddValue(item, type_0, pvalue, size) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh11 = (*item).count;
    *fresh11 += 1;
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn binn_object_set_raw(
    mut item: * mut crate::src::binn::binn_struct,
    mut key: * const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut len: u8 = 0;
    let mut int32: i32 = 0;
    if item.is_null() || (*item).type_0 != 0xe2 as std::os::raw::c_int
        || (*item).writable == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if key.is_null() {
        return 0 as std::os::raw::c_int;
    }
    int32 = strlen(key) as std::os::raw::c_int;
    if int32 > 255 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    p = SearchForKey(
        (*item).pbuf as *mut std::os::raw::c_uchar,
        9 as std::os::raw::c_int,
        (*item).used_size,
        (*item).count,
        key,
    );
    if !p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if CheckAllocation(item, 1 as std::os::raw::c_int + int32) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    p = ((*item).pbuf as *mut std::os::raw::c_uchar).offset((*item).used_size as isize);
    len = int32 as std::os::raw::c_uchar;
    *p = len;
    p = p.offset(1);
    memcpy(p as *mut std::os::raw::c_void, key as *const std::os::raw::c_void, int32 as std::os::raw::c_ulong);
    int32 += 1;
    (*item).used_size += int32;
    if AddValue(item, type_0, pvalue, size) == 0 as std::os::raw::c_int {
        (*item).used_size -= int32;
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh12 = (*item).count;
    *fresh12 += 1;
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn binn_map_set_raw(
    mut item: * mut crate::src::binn::binn_struct,
    mut id: std::os::raw::c_int,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut base = 0 as *mut std::os::raw::c_uchar;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut sign: u8 = 0;
    let mut id_size: i32 = 0;
    if item.is_null() || (*item).type_0 != 0xe1 as std::os::raw::c_int
        || (*item).writable == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    p = SearchForID(
        (*item).pbuf as *mut std::os::raw::c_uchar,
        9 as std::os::raw::c_int,
        (*item).used_size,
        (*item).count,
        id,
    );
    if !p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if CheckAllocation(item, 5 as std::os::raw::c_int) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    base = ((*item).pbuf as *mut std::os::raw::c_uchar).offset((*item).used_size as isize);
    p = base;
    sign = (id < 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_uchar;
    if sign != 0 {
        id = -id;
    }
    if id <= 0x3f as std::os::raw::c_int {
        let mut fresh13 = p;
        p = p.offset(1);
        *fresh13 = ((sign as std::os::raw::c_int) << 6 as std::os::raw::c_int | id) as std::os::raw::c_uchar;
    } else if id <= 0xfff as std::os::raw::c_int {
        let mut fresh14 = p;
        p = p.offset(1);
        *fresh14 = (0x80 as std::os::raw::c_int | (sign as std::os::raw::c_int) << 4 as std::os::raw::c_int
            | (id & 0xf00 as std::os::raw::c_int) >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        let mut fresh15 = p;
        p = p.offset(1);
        *fresh15 = (id & 0xff as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else if id <= 0xfffff as std::os::raw::c_int {
        let mut fresh16 = p;
        p = p.offset(1);
        *fresh16 = (0xa0 as std::os::raw::c_int | (sign as std::os::raw::c_int) << 4 as std::os::raw::c_int
            | (id & 0xf0000 as std::os::raw::c_int) >> 16 as std::os::raw::c_int) as std::os::raw::c_uchar;
        let mut fresh17 = p;
        p = p.offset(1);
        *fresh17 = ((id & 0xff00 as std::os::raw::c_int) >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        let mut fresh18 = p;
        p = p.offset(1);
        *fresh18 = (id & 0xff as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else if id <= 0xfffffff as std::os::raw::c_int {
        let mut fresh19 = p;
        p = p.offset(1);
        *fresh19 = (0xc0 as std::os::raw::c_int | (sign as std::os::raw::c_int) << 4 as std::os::raw::c_int
            | (id & 0xf000000 as std::os::raw::c_int) >> 24 as std::os::raw::c_int) as std::os::raw::c_uchar;
        let mut fresh20 = p;
        p = p.offset(1);
        *fresh20 = ((id & 0xff0000 as std::os::raw::c_int) >> 16 as std::os::raw::c_int)
            as std::os::raw::c_uchar;
        let mut fresh21 = p;
        p = p.offset(1);
        *fresh21 = ((id & 0xff00 as std::os::raw::c_int) >> 8 as std::os::raw::c_int) as std::os::raw::c_uchar;
        let mut fresh22 = p;
        p = p.offset(1);
        *fresh22 = (id & 0xff as std::os::raw::c_int) as std::os::raw::c_uchar;
    } else {
        let mut fresh23 = p;
        p = p.offset(1);
        *fresh23 = 0xe0 as std::os::raw::c_int as std::os::raw::c_uchar;
        if sign != 0 {
            id = -id;
        }
        copy_be32(p as *mut u32_0, &mut id as *mut std::os::raw::c_int as *mut u32_0);
        p = p.offset(4 as std::os::raw::c_int as isize);
    }
    id_size = p.offset_from(base) as std::os::raw::c_long as std::os::raw::c_int;
    (*item).used_size += id_size;
    if AddValue(item, type_0, pvalue, size) == 0 as std::os::raw::c_int {
        (*item).used_size -= id_size;
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh24 = (*item).count;
    *fresh24 += 1;
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn compress_int<'a1, 'a2>(
    mut pstorage_type: Option<&'a1 mut std::os::raw::c_int>,
    mut ptype: Option<&'a2 mut std::os::raw::c_int>,
    mut psource: * mut core::ffi::c_void,
) -> * mut core::ffi::c_void {
    let mut current_block: u64;
    let mut storage_type: i32 = 0;
    let mut storage_type2: i32 = 0;
    let mut type_0: i32 = 0;
    let mut type2 = 0 as std::os::raw::c_int;
    let mut vint = 0 as std::os::raw::c_int as int64;
    let mut vuint: u64 = 0;
    let mut pvalue = 0 as *mut std::os::raw::c_char;
    storage_type = *(borrow_mut(&mut pstorage_type)).unwrap();
    if storage_type == 0x20 as std::os::raw::c_int {
        return psource;
    }
    type_0 = *(borrow_mut(&mut ptype)).unwrap();
    match type_0 {
        129 => {
            vint = *(psource as *mut int64);
            current_block = 11048997260478501647;
        }
        97 => {
            vint = *(psource as *mut std::os::raw::c_int) as int64;
            current_block = 11048997260478501647;
        }
        65 => {
            vint = *(psource as *mut std::os::raw::c_short) as int64;
            current_block = 11048997260478501647;
        }
        128 => {
            vuint = *(psource as *mut uint64);
            current_block = 1818554471676300950;
        }
        96 => {
            vuint = *(psource as *mut std::os::raw::c_uint) as uint64;
            current_block = 1818554471676300950;
        }
        64 => {
            vuint = *(psource as *mut std::os::raw::c_ushort) as uint64;
            current_block = 1818554471676300950;
        }
        _ => {
            current_block = 11048997260478501647;
        }
    }
    match current_block {
        11048997260478501647 => {
            if vint >= 0 as std::os::raw::c_int as std::os::raw::c_longlong {
                vuint = vint as uint64;
                current_block = 1818554471676300950;
            } else {
                if vint >= -(128 as std::os::raw::c_int) as std::os::raw::c_longlong {
                    type2 = 0x21 as std::os::raw::c_int;
                } else if vint
                    >= (-(32767 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as std::os::raw::c_longlong
                {
                    type2 = 0x41 as std::os::raw::c_int;
                } else if vint
                    >= (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int)
                        as std::os::raw::c_longlong
                {
                    type2 = 0x61 as std::os::raw::c_int;
                }
                current_block = 1757844130948290377;
            }
        }
        _ => {}
    }
    match current_block {
        1818554471676300950 => {
            if vuint <= 255 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                type2 = 0x20 as std::os::raw::c_int;
            } else if vuint <= 65535 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                type2 = 0x40 as std::os::raw::c_int;
            } else if vuint <= 4294967295 as std::os::raw::c_uint as std::os::raw::c_ulonglong {
                type2 = 0x60 as std::os::raw::c_int;
            }
        }
        _ => {}
    }
    pvalue = psource as *mut std::os::raw::c_char;
    if type2 != 0 && type2 != type_0 {
        *(borrow_mut(&mut ptype)).unwrap() = type2;
        storage_type2 = binn_get_write_storage(type2);
        *(borrow_mut(&mut pstorage_type)).unwrap() = storage_type2;
    }
    return pvalue as *mut std::os::raw::c_void;
}
unsafe extern "C" fn AddValue(
    mut item: * mut crate::src::binn::binn_struct,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut int32: i32 = 0;
    let mut ArgSize: i32 = 0;
    let mut storage_type: i32 = 0;
    let mut extra_type: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    binn_get_type_info(type_0, Some(&mut storage_type), Some(&mut extra_type));
    if pvalue.is_null() {
        let mut current_block_1: u64;
        match storage_type {
            0 => {
                current_block_1 = 11006700562992250127;
            }
            192 | 160 => {
                if size == 0 as std::os::raw::c_int {
                    current_block_1 = 11006700562992250127;
                } else {
                    current_block_1 = 14336010400674820937;
                }
            }
            _ => {
                current_block_1 = 14336010400674820937;
            }
        }
        match current_block_1 {
            14336010400674820937 => return 0 as std::os::raw::c_int,
            _ => {}
        }
    }
    if type_family(type_0) == 0xf2 as std::os::raw::c_int
        && (*item).disable_int_compression == 0 as std::os::raw::c_int
    {
        pvalue = compress_int(Some(&mut storage_type), Some(&mut type_0), pvalue);
    }
    match storage_type {
        0 => {
            size = 0 as std::os::raw::c_int;
            ArgSize = size;
        }
        32 => {
            size = 1 as std::os::raw::c_int;
            ArgSize = size;
        }
        64 => {
            size = 2 as std::os::raw::c_int;
            ArgSize = size;
        }
        96 => {
            size = 4 as std::os::raw::c_int;
            ArgSize = size;
        }
        128 => {
            size = 8 as std::os::raw::c_int;
            ArgSize = size;
        }
        192 => {
            if size < 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int;
            }
            ArgSize = size + 4 as std::os::raw::c_int;
        }
        160 => {
            if size < 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int;
            }
            if size == 0 as std::os::raw::c_int {
                size = strlen2(pvalue as *mut std::os::raw::c_char) as std::os::raw::c_int;
            }
            ArgSize = size + 5 as std::os::raw::c_int;
        }
        224 => {
            if size <= 0 as std::os::raw::c_int {
                return 0 as std::os::raw::c_int;
            }
            ArgSize = size;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    ArgSize += 2 as std::os::raw::c_int;
    if CheckAllocation(item, ArgSize) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    p = ((*item).pbuf as *mut std::os::raw::c_uchar).offset((*item).used_size as isize);
    if storage_type != 0xe0 as std::os::raw::c_int {
        if type_0 > 255 as std::os::raw::c_int {
            let mut type16 = type_0 as u16_0;
            copy_be16(p as *mut u16_0, &mut type16 as *mut u16_0);
            p = p.offset(2 as std::os::raw::c_int as isize);
            (*item).used_size += 2 as std::os::raw::c_int;
        } else {
            *p = type_0 as std::os::raw::c_uchar;
            p = p.offset(1);
            let ref mut fresh25 = (*item).used_size;
            *fresh25 += 1;
        }
    }
    match storage_type {
        32 => {
            *(p as *mut std::os::raw::c_char) = *(pvalue as *mut std::os::raw::c_char);
            (*item).used_size += 1 as std::os::raw::c_int;
        }
        64 => {
            copy_be16(p as *mut u16_0, pvalue as *mut u16_0);
            (*item).used_size += 2 as std::os::raw::c_int;
        }
        96 => {
            copy_be32(p as *mut u32_0, pvalue as *mut u32_0);
            (*item).used_size += 4 as std::os::raw::c_int;
        }
        128 => {
            copy_be64(p as *mut u64_0, pvalue as *mut u64_0);
            (*item).used_size += 8 as std::os::raw::c_int;
        }
        192 | 160 => {
            if size > 127 as std::os::raw::c_int {
                int32 = (size as std::os::raw::c_uint | 0x80000000 as std::os::raw::c_uint)
                    as std::os::raw::c_int;
                copy_be32(p as *mut u32_0, &mut int32 as *mut std::os::raw::c_int as *mut u32_0);
                p = p.offset(4 as std::os::raw::c_int as isize);
                (*item).used_size += 4 as std::os::raw::c_int;
            } else {
                *p = size as std::os::raw::c_uchar;
                p = p.offset(1);
                let ref mut fresh26 = (*item).used_size;
                *fresh26 += 1;
            }
            memcpy(p as *mut std::os::raw::c_void, pvalue, size as std::os::raw::c_ulong);
            if storage_type == 0xa0 as std::os::raw::c_int {
                p = p.offset(size as isize);
                *(p as *mut std::os::raw::c_char) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                size += 1;
            }
            (*item).used_size += size;
        }
        224 => {
            memcpy(p as *mut std::os::raw::c_void, pvalue, size as std::os::raw::c_ulong);
            (*item).used_size += size;
        }
        0 | _ => {}
    }
    (*item).dirty = 1 as std::os::raw::c_int;
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn binn_save_header(mut item: * mut crate::src::binn::binn_struct) -> std::os::raw::c_int {
    let mut byte: u8 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut int32: i32 = 0;
    let mut size: i32 = 0;
    if item.is_null() {
        return 0 as std::os::raw::c_int;
    }
    p = ((*item).pbuf as *mut std::os::raw::c_uchar).offset(9 as std::os::raw::c_int as isize);
    size = (*item).used_size - 9 as std::os::raw::c_int + 3 as std::os::raw::c_int;
    if (*item).count > 127 as std::os::raw::c_int {
        p = p.offset(-(4 as std::os::raw::c_int as isize));
        size += 3 as std::os::raw::c_int;
        int32 = ((*item).count as std::os::raw::c_uint | 0x80000000 as std::os::raw::c_uint)
            as std::os::raw::c_int;
        copy_be32(p as *mut u32_0, &mut int32 as *mut std::os::raw::c_int as *mut u32_0);
    } else {
        p = p.offset(-1);
        *p = (*item).count as std::os::raw::c_uchar;
    }
    if size > 127 as std::os::raw::c_int {
        p = p.offset(-(4 as std::os::raw::c_int as isize));
        size += 3 as std::os::raw::c_int;
        int32 = (size as std::os::raw::c_uint | 0x80000000 as std::os::raw::c_uint) as std::os::raw::c_int;
        copy_be32(p as *mut u32_0, &mut int32 as *mut std::os::raw::c_int as *mut u32_0);
    } else {
        p = p.offset(-1);
        *p = size as std::os::raw::c_uchar;
    }
    p = p.offset(-1);
    *p = (*item).type_0 as std::os::raw::c_uchar;
    let ref mut fresh27 = (*item).ptr;
    *fresh27 = p as *mut std::os::raw::c_void;
    (*item).size = size;
    (*item).dirty = 0 as std::os::raw::c_int;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_free(mut item: * mut crate::src::binn::binn_struct) {
    if item.is_null() {
        return;
    }
    if (*item).writable != 0 && (*item).pre_allocated == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")((*item).pbuf);
    }
    if ((*item).freefn).is_some() {
        ((*item).freefn).expect("non-null function pointer")((*item).ptr);
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut std::os::raw::c_void);
    } else {
        memset(
            item as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
        );
        (*item).header = 0x1f22b11f as std::os::raw::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_release(mut item: * mut crate::src::binn::binn_struct) -> * mut core::ffi::c_void {
    let mut data = 0 as *mut std::os::raw::c_void;
    if item.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    data = binn_ptr(item as *mut std::os::raw::c_void);
    if data > (*item).pbuf {
        memmove((*item).pbuf, data, (*item).size as std::os::raw::c_ulong);
        data = (*item).pbuf;
    }
    if (*item).allocated != 0 {
        free_fn.expect("non-null function pointer")(item as *mut std::os::raw::c_void);
    } else {
        memset(
            item as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
        );
        (*item).header = 0x1f22b11f as std::os::raw::c_int;
    }
    return data;
}
unsafe extern "C" fn IsValidBinnHeader<'a1, 'a2, 'a3, 'a4>(
    mut pbuf: * mut core::ffi::c_void,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut pcount: Option<&'a2 mut std::os::raw::c_int>,
    mut psize: Option<&'a3 mut std::os::raw::c_int>,
    mut pheadersize: Option<&'a4 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut byte: u8 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut int32: i32 = 0;
    let mut type_0: i32 = 0;
    let mut size: i32 = 0;
    let mut count: i32 = 0;
    if pbuf.is_null() {
        return 0 as std::os::raw::c_int;
    }
    p = pbuf as *mut std::os::raw::c_uchar;
    if !borrow(& psize).is_none() && *(borrow(& psize)).unwrap() > 0 as std::os::raw::c_int {
        plimit = p.offset(*(borrow(& psize)).unwrap() as isize).offset(-(1 as std::os::raw::c_int as isize));
    }
    byte = *p;
    p = p.offset(1);
    if byte as std::os::raw::c_int & 0xe0 as std::os::raw::c_int != 0xe0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if byte as std::os::raw::c_int & 0x10 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int;
    }
    type_0 = byte as std::os::raw::c_int;
    match type_0 {
        224 | 225 | 226 => {}
        _ => return 0 as std::os::raw::c_int,
    }
    if !plimit.is_null() && p > plimit {
        return 0 as std::os::raw::c_int;
    }
    int32 = *p as std::os::raw::c_int;
    if int32 & 0x80 as std::os::raw::c_int != 0 {
        if !plimit.is_null()
            && p
                .offset(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as isize)
                .offset(-(1 as std::os::raw::c_int as isize)) > plimit
        {
            return 0 as std::os::raw::c_int;
        }
        copy_be32(&mut int32 as *mut std::os::raw::c_int as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as std::os::raw::c_int;
        p = p.offset(4 as std::os::raw::c_int as isize);
    } else {
        p = p.offset(1);
    }
    size = int32;
    if !plimit.is_null() && p > plimit {
        return 0 as std::os::raw::c_int;
    }
    int32 = *p as std::os::raw::c_int;
    if int32 & 0x80 as std::os::raw::c_int != 0 {
        if !plimit.is_null()
            && p
                .offset(::std::mem::size_of::<std::os::raw::c_int>() as std::os::raw::c_ulong as isize)
                .offset(-(1 as std::os::raw::c_int as isize)) > plimit
        {
            return 0 as std::os::raw::c_int;
        }
        copy_be32(&mut int32 as *mut std::os::raw::c_int as *mut u32_0, p as *mut u32_0);
        int32 &= 0x7fffffff as std::os::raw::c_int;
        p = p.offset(4 as std::os::raw::c_int as isize);
    } else {
        p = p.offset(1);
    }
    count = int32;
    if size < 3 as std::os::raw::c_int || count < 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = type_0;
    }
    if !borrow(& pcount).is_none() {
        *(borrow_mut(&mut pcount)).unwrap() = count;
    }
    if !borrow(& psize).is_none() && *(borrow(& psize)).unwrap() == 0 as std::os::raw::c_int {
        *(borrow_mut(&mut psize)).unwrap() = size;
    }
    if !borrow(& pheadersize).is_none() {
        *(borrow_mut(&mut pheadersize)).unwrap() = p.offset_from(pbuf as *mut std::os::raw::c_uchar) as std::os::raw::c_long
            as std::os::raw::c_int;
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn binn_buf_type(mut pbuf: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut type_0: i32 = 0;
    if IsValidBinnHeader(
        pbuf,
        Some(&mut type_0),
        Option::<&'_ mut i32>::None,
        Option::<&'_ mut i32>::None,
        Option::<&'_ mut i32>::None,
    ) == 0
    {
        return 0 as std::os::raw::c_int;
    }
    return type_0;
}
unsafe extern "C" fn binn_buf_count(mut pbuf: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut nitems: i32 = 0;
    if IsValidBinnHeader(
        pbuf,
        Option::<&'_ mut i32>::None,
        Some(&mut nitems),
        Option::<&'_ mut i32>::None,
        Option::<&'_ mut i32>::None,
    ) == 0
    {
        return 0 as std::os::raw::c_int;
    }
    return nitems;
}
unsafe extern "C" fn binn_buf_size(mut pbuf: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut size = 0 as std::os::raw::c_int;
    if IsValidBinnHeader(
        pbuf,
        Option::<&'_ mut i32>::None,
        Option::<&'_ mut i32>::None,
        Some(&mut size),
        Option::<&'_ mut i32>::None,
    ) == 0
    {
        return 0 as std::os::raw::c_int;
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn binn_ptr(mut ptr: * mut core::ffi::c_void) -> * mut core::ffi::c_void {
    let mut item = (0 as * mut crate::src::binn::binn_struct);
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).ptr;
        }
        2 => return ptr,
        _ => return 0 as *mut std::os::raw::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_size(mut ptr: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut item = (0 as * mut crate::src::binn::binn_struct);
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            if (*item).writable != 0 && (*item).dirty != 0 {
                binn_save_header(item);
            }
            return (*item).size;
        }
        2 => return binn_buf_size(ptr),
        _ => return 0 as std::os::raw::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_type(mut ptr: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut item = (0 as * mut crate::src::binn::binn_struct);
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            return (*item).type_0;
        }
        2 => return binn_buf_type(ptr),
        _ => return -(1 as std::os::raw::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_count(mut ptr: * mut core::ffi::c_void) -> std::os::raw::c_int {
    let mut item = (0 as * mut crate::src::binn::binn_struct);
    match binn_get_ptr_type(ptr) {
        1 => {
            item = ptr as *mut binn;
            return (*item).count;
        }
        2 => return binn_buf_count(ptr),
        _ => return -(1 as std::os::raw::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid_ex(
    mut ptr: * mut core::ffi::c_void,
    mut ptype: * mut std::os::raw::c_int,
    mut pcount: * mut std::os::raw::c_int,
    mut psize: * mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size: i32 = 0;
    let mut header_size: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut base = 0 as *mut std::os::raw::c_uchar;
    let mut len: u8 = 0;
    let mut pbuf = 0 as *mut std::os::raw::c_void;
    pbuf = binn_ptr(ptr);
    if pbuf.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if !psize.is_null() && *psize > 0 as std::os::raw::c_int {
        size = *psize;
    } else {
        size = 0 as std::os::raw::c_int;
    }
    if IsValidBinnHeader(pbuf, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size)) == 0
    {
        return 0 as std::os::raw::c_int;
    }
    if !psize.is_null() && *psize > 0 as std::os::raw::c_int {
        if size != *psize {
            return 0 as std::os::raw::c_int;
        }
    }
    if !pcount.is_null() && *pcount > 0 as std::os::raw::c_int {
        if count != *pcount {
            return 0 as std::os::raw::c_int;
        }
    }
    if !ptype.is_null() && *ptype != 0 as std::os::raw::c_int {
        if type_0 != *ptype {
            return 0 as std::os::raw::c_int;
        }
    }
    p = pbuf as *mut std::os::raw::c_uchar;
    base = p;
    plimit = p.offset(size as isize);
    p = p.offset(header_size as isize);
    i = 0 as std::os::raw::c_int;
    loop {
        if !(i < count) {
            current_block = 7245201122033322888;
            break;
        }
        match type_0 {
            226 => {
                len = *p;
                p = p.offset(1);
                p = p.offset(len as std::os::raw::c_int as isize);
            }
            225 => {
                read_map_id(Some(&mut p), plimit);
            }
            _ => {}
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            current_block = 11017899362201962608;
            break;
        }
        i += 1;
    }
    match current_block {
        11017899362201962608 => return 0 as std::os::raw::c_int,
        _ => {
            if !ptype.is_null() && *ptype == 0 as std::os::raw::c_int {
                *ptype = type_0;
            }
            if !pcount.is_null() && *pcount == 0 as std::os::raw::c_int {
                *pcount = count;
            }
            if !psize.is_null() && *psize == 0 as std::os::raw::c_int {
                *psize = size;
            }
            return 1 as std::os::raw::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_valid(
    mut ptr: * mut core::ffi::c_void,
    mut ptype: * mut std::os::raw::c_int,
    mut pcount: * mut std::os::raw::c_int,
    mut psize: * mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    if !ptype.is_null() {
        *ptype = 0 as std::os::raw::c_int;
    }
    if !pcount.is_null() {
        *pcount = 0 as std::os::raw::c_int;
    }
    if !psize.is_null() {
        *psize = 0 as std::os::raw::c_int;
    }
    return binn_is_valid_ex(ptr, ptype, pcount, psize);
}
unsafe extern "C" fn GetValue(mut p: * mut std::os::raw::c_uchar, mut value: * mut crate::src::binn::binn_struct) -> std::os::raw::c_int {
    let mut byte: u8 = 0;
    let mut data_type: i32 = 0;
    let mut storage_type: i32 = 0;
    let mut DataSize: i32 = 0;
    let mut p2 = 0 as *mut std::os::raw::c_void;
    if value.is_null() {
        return 0 as std::os::raw::c_int;
    }
    memset(
        value as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
    );
    (*value).header = 0x1f22b11f as std::os::raw::c_int;
    p2 = p as *mut std::os::raw::c_void;
    byte = *p;
    p = p.offset(1);
    storage_type = byte as std::os::raw::c_int & 0xe0 as std::os::raw::c_int;
    if byte as std::os::raw::c_int & 0x10 as std::os::raw::c_int != 0 {
        data_type = (byte as std::os::raw::c_int) << 8 as std::os::raw::c_int;
        byte = *p;
        p = p.offset(1);
        data_type |= byte as std::os::raw::c_int;
    } else {
        data_type = byte as std::os::raw::c_int;
    }
    (*value).type_0 = data_type;
    match storage_type {
        0 => {}
        32 => {
            (*value).c2rust_unnamed.vuint8 = *p;
            let ref mut fresh28 = (*value).ptr;
            *fresh28 = p as *mut std::os::raw::c_void;
        }
        64 => {
            copy_be16(
                &mut (*value).c2rust_unnamed.vint16 as *mut std::os::raw::c_short as *mut u16_0,
                p as *mut u16_0,
            );
            let ref mut fresh29 = (*value).ptr;
            *fresh29 = &mut (*value).c2rust_unnamed.vint16 as *mut std::os::raw::c_short
                as *mut std::os::raw::c_void;
        }
        96 => {
            copy_be32(
                &mut (*value).c2rust_unnamed.vint32 as *mut std::os::raw::c_int as *mut u32_0,
                p as *mut u32_0,
            );
            let ref mut fresh30 = (*value).ptr;
            *fresh30 = &mut (*value).c2rust_unnamed.vint32 as *mut std::os::raw::c_int
                as *mut std::os::raw::c_void;
        }
        128 => {
            copy_be64(
                &mut (*value).c2rust_unnamed.vint64 as *mut int64 as *mut u64_0,
                p as *mut u64_0,
            );
            let ref mut fresh31 = (*value).ptr;
            *fresh31 = &mut (*value).c2rust_unnamed.vint64 as *mut int64
                as *mut std::os::raw::c_void;
        }
        192 | 160 => {
            DataSize = *p as std::os::raw::c_int;
            if DataSize & 0x80 as std::os::raw::c_int != 0 {
                copy_be32(
                    &mut DataSize as *mut std::os::raw::c_int as *mut u32_0,
                    p as *mut u32_0,
                );
                DataSize &= 0x7fffffff as std::os::raw::c_int;
                p = p.offset(4 as std::os::raw::c_int as isize);
            } else {
                p = p.offset(1);
            }
            (*value).size = DataSize;
            let ref mut fresh32 = (*value).ptr;
            *fresh32 = p as *mut std::os::raw::c_void;
        }
        224 => {
            let ref mut fresh33 = (*value).ptr;
            *fresh33 = p2;
            if IsValidBinnHeader(
                p2,
                Option::<&'_ mut i32>::None,
                Some(&mut (*value).count),
                Some(&mut (*value).size),
                Option::<&'_ mut i32>::None,
            ) == 0 as std::os::raw::c_int
            {
                return 0 as std::os::raw::c_int;
            }
        }
        _ => return 0 as std::os::raw::c_int,
    }
    match (*value).type_0 {
        1 => {
            (*value).type_0 = 0x80061 as std::os::raw::c_int;
            (*value).c2rust_unnamed.vbool = 1 as std::os::raw::c_int;
            let ref mut fresh34 = (*value).ptr;
            *fresh34 = &mut (*value).c2rust_unnamed.vbool as *mut BOOL
                as *mut std::os::raw::c_void;
        }
        2 => {
            (*value).type_0 = 0x80061 as std::os::raw::c_int;
            (*value).c2rust_unnamed.vbool = 0 as std::os::raw::c_int;
            let ref mut fresh35 = (*value).ptr;
            *fresh35 = &mut (*value).c2rust_unnamed.vbool as *mut BOOL
                as *mut std::os::raw::c_void;
        }
        _ => {}
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub static mut local_value: crate::src::binn::binn_struct = binn {
    header: 0,
    allocated: 0,
    writable: 0,
    dirty: 0,
    pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    pre_allocated: 0,
    alloc_size: 0,
    used_size: 0,
    type_0: 0,
    ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
    size: 0,
    count: 0,
    freefn: None,
    c2rust_unnamed: C2RustUnnamed { vint8: 0 },
    disable_int_compression: 0,
};
unsafe extern "C" fn store_value(mut value: * mut crate::src::binn::binn_struct) -> * mut core::ffi::c_void {
    memcpy(
        &mut local_value as *mut binn as *mut std::os::raw::c_void,
        value as *const std::os::raw::c_void,
        ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
    );
    's_18: {
        match binn_get_read_storage((*value).type_0) {
            0 => {}
            64 | 96 | 128 => {}
            _ => {
                break 's_18;
            }
        }
        return &mut local_value.c2rust_unnamed.vint32 as *mut std::os::raw::c_int
            as *mut std::os::raw::c_void;
    }
    return (*value).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_value(
    mut ptr: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size = 0 as std::os::raw::c_int;
    let mut header_size: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr.is_null() || key.is_null() || value.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if IsValidBinnHeader(ptr, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size))
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if type_0 != 0xe2 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if count == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    p = ptr as *mut std::os::raw::c_uchar;
    p = SearchForKey(p, header_size, size, count, key);
    if p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    return GetValue(p, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_value(
    mut ptr: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size = 0 as std::os::raw::c_int;
    let mut header_size: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr.is_null() || value.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if IsValidBinnHeader(ptr, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size))
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if type_0 != 0xe1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if count == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    p = ptr as *mut std::os::raw::c_uchar;
    p = SearchForID(p, header_size, size, count, id);
    if p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    return GetValue(p, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get_value(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut i: i32 = 0;
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size = 0 as std::os::raw::c_int;
    let mut header_size: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut base = 0 as *mut std::os::raw::c_uchar;
    ptr = binn_ptr(ptr);
    if ptr.is_null() || value.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if IsValidBinnHeader(ptr, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size))
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if type_0 != 0xe0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if count == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if (pos <= 0 as std::os::raw::c_int) as std::os::raw::c_int | (pos > count) as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int;
    }
    pos -= 1;
    p = ptr as *mut std::os::raw::c_uchar;
    base = p;
    plimit = p.offset(size as isize);
    p = p.offset(header_size as isize);
    i = 0 as std::os::raw::c_int;
    while i < pos {
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
    }
    return GetValue(p, value);
}
unsafe extern "C" fn binn_read_pair<'a1>(
    mut expected_type: std::os::raw::c_int,
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut pkey: * mut std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut current_block: u64;
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size = 0 as std::os::raw::c_int;
    let mut header_size: i32 = 0;
    let mut i: i32 = 0;
    let mut int32: i32 = 0;
    let mut id = 0 as std::os::raw::c_int;
    let mut counter = 0 as std::os::raw::c_int;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut plimit = 0 as *mut std::os::raw::c_uchar;
    let mut base = 0 as *mut std::os::raw::c_uchar;
    let mut key = 0 as *mut std::os::raw::c_uchar;
    let mut len = 0 as std::os::raw::c_int as std::os::raw::c_uchar;
    ptr = binn_ptr(ptr);
    if IsValidBinnHeader(ptr, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size))
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if type_0 != expected_type || count == 0 as std::os::raw::c_int || pos < 1 as std::os::raw::c_int
        || pos > count
    {
        return 0 as std::os::raw::c_int;
    }
    p = ptr as *mut std::os::raw::c_uchar;
    base = p;
    plimit = p.offset(size as isize).offset(-(1 as std::os::raw::c_int as isize));
    p = p.offset(header_size as isize);
    i = 0 as std::os::raw::c_int;
    loop {
        if !(i < count) {
            current_block = 14359455889292382949;
            break;
        }
        match type_0 {
            225 => {
                int32 = read_map_id(Some(&mut p), plimit);
                if p > plimit {
                    return 0 as std::os::raw::c_int;
                }
                id = int32;
            }
            226 => {
                len = *p;
                p = p.offset(1);
                if p > plimit {
                    return 0 as std::os::raw::c_int;
                }
                key = p;
                p = p.offset(len as std::os::raw::c_int as isize);
                if p > plimit {
                    return 0 as std::os::raw::c_int;
                }
            }
            _ => {}
        }
        counter += 1;
        if counter == pos {
            current_block = 3690914394173635162;
            break;
        }
        p = AdvanceDataPos(p, plimit);
        if p.is_null() || p < base {
            return 0 as std::os::raw::c_int;
        }
        i += 1;
    }
    match current_block {
        14359455889292382949 => return 0 as std::os::raw::c_int,
        _ => {
            match type_0 {
                225 => {
                    if !borrow(& pid).is_none() {
                        *(borrow_mut(&mut pid)).unwrap() = id;
                    }
                }
                226 => {
                    if !pkey.is_null() {
                        memcpy(
                            pkey as *mut std::os::raw::c_void,
                            key as *const std::os::raw::c_void,
                            len as std::os::raw::c_ulong,
                        );
                        *pkey.offset(len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
                    }
                }
                _ => {}
            }
            return GetValue(p, value);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get_pair<'a1>(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_read_pair(
        0xe1 as std::os::raw::c_int,
        ptr,
        pos,
        borrow_mut(&mut pid),
        0 as *mut std::os::raw::c_char,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get_pair(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pkey: * mut std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_read_pair(
        0xe2 as std::os::raw::c_int,
        ptr,
        pos,
        Option::<&'_ mut i32>::None,
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_pair<'a1>(
    mut map: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_read_pair(0xe1 as std::os::raw::c_int, map, pos, borrow_mut(&mut pid), 0 as *mut std::os::raw::c_char, value)
        == 0 as std::os::raw::c_int
    {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_pair(
    mut obj: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pkey: * mut std::os::raw::c_char,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_read_pair(0xe2 as std::os::raw::c_int, obj, pos, Option::<&'_ mut i32>::None, pkey, value)
        == 0 as std::os::raw::c_int
    {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_pair<'a1, 'a2, 'a3>(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut ptype: Option<&'a2 mut std::os::raw::c_int>,
    mut psize: Option<&'a3 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_pair(ptr, pos, borrow_mut(&mut pid), &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_pair<'a1, 'a2>(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut pkey: * mut std::os::raw::c_char,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_pair(ptr, pos, pkey, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_iter_init(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut ptr: * mut core::ffi::c_void,
    mut expected_type: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut type_0: i32 = 0;
    let mut count: i32 = 0;
    let mut size = 0 as std::os::raw::c_int;
    let mut header_size: i32 = 0;
    ptr = binn_ptr(ptr);
    if ptr.is_null() || iter.is_null() {
        return 0 as std::os::raw::c_int;
    }
    memset(
        iter as *mut std::os::raw::c_void,
        0 as std::os::raw::c_int,
        ::std::mem::size_of::<binn_iter>() as std::os::raw::c_ulong,
    );
    if IsValidBinnHeader(ptr, Some(&mut type_0), Some(&mut count), Some(&mut size), Some(&mut header_size))
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if type_0 != expected_type {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh36 = (*iter).plimit;
    *fresh36 = (ptr as *mut std::os::raw::c_uchar)
        .offset(size as isize)
        .offset(-(1 as std::os::raw::c_int as isize));
    let ref mut fresh37 = (*iter).pnext;
    *fresh37 = (ptr as *mut std::os::raw::c_uchar).offset(header_size as isize);
    (*iter).count = count;
    (*iter).current = 0 as std::os::raw::c_int;
    (*iter).type_0 = type_0;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut pnow = 0 as *mut std::os::raw::c_uchar;
    if iter.is_null() || ((*iter).pnext).is_null() || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count || (*iter).type_0 != 0xe0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh38 = (*iter).current;
    *fresh38 += 1;
    if (*iter).current > (*iter).count {
        return 0 as std::os::raw::c_int;
    }
    pnow = (*iter).pnext;
    let ref mut fresh39 = (*iter).pnext;
    *fresh39 = AdvanceDataPos(pnow, (*iter).plimit);
    if !((*iter).pnext).is_null() && (*iter).pnext < pnow {
        return 0 as std::os::raw::c_int;
    }
    return GetValue(pnow, value);
}
unsafe extern "C" fn binn_read_next_pair<'a1>(
    mut expected_type: std::os::raw::c_int,
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut pkey: * mut std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut int32: i32 = 0;
    let mut id: i32 = 0;
    let mut p = 0 as *mut std::os::raw::c_uchar;
    let mut key = 0 as *mut std::os::raw::c_uchar;
    let mut len: u16 = 0;
    if iter.is_null() || ((*iter).pnext).is_null() || (*iter).pnext > (*iter).plimit
        || (*iter).current > (*iter).count || (*iter).type_0 != expected_type
    {
        return 0 as std::os::raw::c_int;
    }
    let ref mut fresh40 = (*iter).current;
    *fresh40 += 1;
    if (*iter).current > (*iter).count {
        return 0 as std::os::raw::c_int;
    }
    p = (*iter).pnext;
    match expected_type {
        225 => {
            int32 = read_map_id(Some(&mut p), (*iter).plimit);
            if p > (*iter).plimit {
                return 0 as std::os::raw::c_int;
            }
            id = int32;
            if !borrow(& pid).is_none() {
                *(borrow_mut(&mut pid)).unwrap() = id;
            }
        }
        226 => {
            len = *p as std::os::raw::c_ushort;
            p = p.offset(1);
            key = p;
            p = p.offset(len as std::os::raw::c_int as isize);
            if p > (*iter).plimit {
                return 0 as std::os::raw::c_int;
            }
            if !pkey.is_null() {
                memcpy(
                    pkey as *mut std::os::raw::c_void,
                    key as *const std::os::raw::c_void,
                    len as std::os::raw::c_ulong,
                );
                *pkey.offset(len as isize) = 0 as std::os::raw::c_int as std::os::raw::c_char;
            }
        }
        _ => {}
    }
    let ref mut fresh41 = (*iter).pnext;
    *fresh41 = AdvanceDataPos(p, (*iter).plimit);
    if !((*iter).pnext).is_null() && (*iter).pnext < p {
        return 0 as std::os::raw::c_int;
    }
    return GetValue(p, value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next<'a1>(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_read_next_pair(
        0xe1 as std::os::raw::c_int,
        iter,
        borrow_mut(&mut pid),
        0 as *mut std::os::raw::c_char,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pkey: * mut std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    return binn_read_next_pair(
        0xe2 as std::os::raw::c_int,
        iter,
        Option::<&'_ mut i32>::None,
        pkey,
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_next_value(mut iter: * mut crate::src::binn::binn_iter_struct) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_list_next(iter, value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_next_value<'a1>(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_map_next(iter, borrow_mut(&mut pid), value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_next_value(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pkey: * mut std::os::raw::c_char,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_object_next(iter, pkey, value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read_next<'a1, 'a2>(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_next(iter, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read_next<'a1, 'a2, 'a3>(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pid: Option<&'a1 mut std::os::raw::c_int>,
    mut ptype: Option<&'a2 mut std::os::raw::c_int>,
    mut psize: Option<&'a3 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_next(iter, borrow_mut(&mut pid), &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read_next<'a1, 'a2>(
    mut iter: * mut crate::src::binn::binn_iter_struct,
    mut pkey: * mut std::os::raw::c_char,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_next(iter, pkey, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_write_storage(mut type_0: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut storage_type: i32 = 0;
    match type_0 {
        166 | 167 => return 0xa0 as std::os::raw::c_int,
        524385 => return 0 as std::os::raw::c_int,
        _ => {
            binn_get_type_info(type_0, Some(&mut storage_type), Option::<&'_ mut i32>::None);
            return storage_type;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_read_storage(mut type_0: std::os::raw::c_int) -> std::os::raw::c_int {
    let mut storage_type: i32 = 0;
    match type_0 {
        524385 | 1 | 2 => return 0x60 as std::os::raw::c_int,
        _ => {
            binn_get_type_info(type_0, Some(&mut storage_type), Option::<&'_ mut i32>::None);
            return storage_type;
        }
    };
}
unsafe extern "C" fn GetWriteConvertedData<'a1, 'a2>(
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut ppvalue: * mut * mut core::ffi::c_void,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut type_0: i32 = 0;
    let mut f1: f32 = 0.;
    let mut d1: f64 = 0.;
    let mut pstr: [i8; 128] = [0; 128];
    type_0 = *(borrow_mut(&mut ptype)).unwrap();
    if (*ppvalue).is_null() {
        let mut current_block_4: u64;
        match type_0 {
            0 | 1 | 2 => {
                current_block_4 = 17965632435239708295;
            }
            160 | 192 => {
                if *(borrow(& psize)).unwrap() == 0 as std::os::raw::c_int {
                    current_block_4 = 17965632435239708295;
                } else {
                    current_block_4 = 4326909834884863099;
                }
            }
            _ => {
                current_block_4 = 4326909834884863099;
            }
        }
        match current_block_4 {
            4326909834884863099 => return 0 as std::os::raw::c_int,
            _ => {}
        }
    }
    match type_0 {
        164 | 165 => return 1 as std::os::raw::c_int,
        162 | 161 | 163 => return 1 as std::os::raw::c_int,
        524385 => {
            if **(ppvalue as *mut *mut BOOL) == 0 as std::os::raw::c_int {
                type_0 = 0x2 as std::os::raw::c_int;
            } else {
                type_0 = 0x1 as std::os::raw::c_int;
            }
            *(borrow_mut(&mut ptype)).unwrap() = type_0;
        }
        _ => {}
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn type_family(mut type_0: std::os::raw::c_int) -> std::os::raw::c_int {
    match type_0 {
        224 | 225 | 226 => return 0xf7 as std::os::raw::c_int,
        33 | 65 | 97 | 129 | 32 | 64 | 96 | 128 => return 0xf2 as std::os::raw::c_int,
        98 | 130 => {}
        166 | 167 => {}
        160 | 45057 | 45061 | 45058 | 45059 | 45060 => return 0xf4 as std::os::raw::c_int,
        192 | 53249 | 53250 | 53251 | 53252 => return 0xf5 as std::os::raw::c_int,
        164 | 131 | 162 | 163 | 161 => return 0xf4 as std::os::raw::c_int,
        524385 => return 0xf6 as std::os::raw::c_int,
        0 => return 0xf1 as std::os::raw::c_int,
        _ => return 0 as std::os::raw::c_int,
    }
    return 0xf3 as std::os::raw::c_int;
}
unsafe extern "C" fn int_type(mut type_0: std::os::raw::c_int) -> std::os::raw::c_int {
    match type_0 {
        33 | 65 | 97 | 129 => return 11 as std::os::raw::c_int,
        32 | 64 | 96 | 128 => return 22 as std::os::raw::c_int,
        _ => return 0 as std::os::raw::c_int,
    };
}
unsafe extern "C" fn copy_raw_value(
    mut psource: * mut core::ffi::c_void,
    mut pdest: * mut core::ffi::c_void,
    mut data_store: std::os::raw::c_int,
) -> std::os::raw::c_int {
    match data_store {
        0 => {}
        32 => {
            *(pdest as *mut std::os::raw::c_char) = *(psource as *mut std::os::raw::c_char);
        }
        64 => {
            *(pdest as *mut std::os::raw::c_short) = *(psource as *mut std::os::raw::c_short);
        }
        96 => {
            *(pdest as *mut std::os::raw::c_int) = *(psource as *mut std::os::raw::c_int);
        }
        128 => {
            *(pdest as *mut uint64) = *(psource as *mut uint64);
        }
        192 | 160 | 224 => {
            let ref mut fresh42 = *(pdest as *mut *mut std::os::raw::c_char);
            *fresh42 = psource as *mut std::os::raw::c_char;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn copy_int_value(
    mut psource: * mut core::ffi::c_void,
    mut pdest: * mut core::ffi::c_void,
    mut source_type: std::os::raw::c_int,
    mut dest_type: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut vuint64 = 0 as std::os::raw::c_int as uint64;
    let mut vint64 = 0 as std::os::raw::c_int as int64;
    match source_type {
        33 => {
            vint64 = *(psource as *mut std::os::raw::c_schar) as int64;
        }
        65 => {
            vint64 = *(psource as *mut std::os::raw::c_short) as int64;
        }
        97 => {
            vint64 = *(psource as *mut std::os::raw::c_int) as int64;
        }
        129 => {
            vint64 = *(psource as *mut int64);
        }
        32 => {
            vuint64 = *(psource as *mut std::os::raw::c_uchar) as uint64;
        }
        64 => {
            vuint64 = *(psource as *mut std::os::raw::c_ushort) as uint64;
        }
        96 => {
            vuint64 = *(psource as *mut std::os::raw::c_uint) as uint64;
        }
        128 => {
            vuint64 = *(psource as *mut uint64);
        }
        _ => return 0 as std::os::raw::c_int,
    }
    if int_type(source_type) == 22 as std::os::raw::c_int
        && int_type(dest_type) == 11 as std::os::raw::c_int
    {
        if vuint64 > 9223372036854775807 as std::os::raw::c_long as std::os::raw::c_ulonglong {
            return 0 as std::os::raw::c_int;
        }
        vint64 = vuint64 as int64;
    } else if int_type(source_type) == 11 as std::os::raw::c_int
        && int_type(dest_type) == 22 as std::os::raw::c_int
    {
        if vint64 < 0 as std::os::raw::c_int as std::os::raw::c_longlong {
            return 0 as std::os::raw::c_int;
        }
        vuint64 = vint64 as uint64;
    }
    match dest_type {
        33 => {
            if vint64 < -(128 as std::os::raw::c_int) as std::os::raw::c_longlong
                || vint64 > 127 as std::os::raw::c_int as std::os::raw::c_longlong
            {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_schar) = vint64 as std::os::raw::c_schar;
        }
        65 => {
            if vint64 < (-(32767 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as std::os::raw::c_longlong
                || vint64 > 32767 as std::os::raw::c_int as std::os::raw::c_longlong
            {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_short) = vint64 as std::os::raw::c_short;
        }
        97 => {
            if vint64
                < (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as std::os::raw::c_longlong
                || vint64 > 2147483647 as std::os::raw::c_int as std::os::raw::c_longlong
            {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_int) = vint64 as std::os::raw::c_int;
        }
        129 => {
            *(pdest as *mut int64) = vint64;
        }
        32 => {
            if vuint64 > 255 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_uchar) = vuint64 as std::os::raw::c_uchar;
        }
        64 => {
            if vuint64 > 65535 as std::os::raw::c_int as std::os::raw::c_ulonglong {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_ushort) = vuint64 as std::os::raw::c_ushort;
        }
        96 => {
            if vuint64 > 4294967295 as std::os::raw::c_uint as std::os::raw::c_ulonglong {
                return 0 as std::os::raw::c_int;
            }
            *(pdest as *mut std::os::raw::c_uint) = vuint64 as std::os::raw::c_uint;
        }
        128 => {
            *(pdest as *mut uint64) = vuint64;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn copy_float_value(
    mut psource: * mut core::ffi::c_void,
    mut pdest: * mut core::ffi::c_void,
    mut source_type: std::os::raw::c_int,
    mut dest_type: std::os::raw::c_int,
) -> std::os::raw::c_int {
    match source_type {
        98 => {
            *(pdest
                as *mut std::os::raw::c_double) = *(psource as *mut std::os::raw::c_float)
                as std::os::raw::c_double;
        }
        130 => {
            *(pdest
                as *mut std::os::raw::c_float) = *(psource as *mut std::os::raw::c_double)
                as std::os::raw::c_float;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
unsafe extern "C" fn zero_value(mut pvalue: * mut core::ffi::c_void, mut type_0: std::os::raw::c_int) {
    match binn_get_read_storage(type_0) {
        32 => {
            *(pvalue as *mut std::os::raw::c_char) = 0 as std::os::raw::c_int as std::os::raw::c_char;
        }
        64 => {
            *(pvalue as *mut std::os::raw::c_short) = 0 as std::os::raw::c_int as std::os::raw::c_short;
        }
        96 => {
            *(pvalue as *mut std::os::raw::c_int) = 0 as std::os::raw::c_int;
        }
        128 => {
            *(pvalue as *mut uint64) = 0 as std::os::raw::c_int as uint64;
        }
        192 | 160 | 224 => {
            let ref mut fresh43 = *(pvalue as *mut *mut std::os::raw::c_char);
            *fresh43 = (0 as * mut i8);
        }
        0 | _ => {}
    };
}
unsafe extern "C" fn copy_value(
    mut psource: * mut core::ffi::c_void,
    mut pdest: * mut core::ffi::c_void,
    mut source_type: std::os::raw::c_int,
    mut dest_type: std::os::raw::c_int,
    mut data_store: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if type_family(source_type) != type_family(dest_type) {
        return 0 as std::os::raw::c_int;
    }
    if type_family(source_type) == 0xf2 as std::os::raw::c_int && source_type != dest_type {
        return copy_int_value(psource, pdest, source_type, dest_type)
    } else if type_family(source_type) == 0xf3 as std::os::raw::c_int && source_type != dest_type
    {
        return copy_float_value(psource, pdest, source_type, dest_type)
    } else {
        return copy_raw_value(psource, pdest, data_store)
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add(
    mut list: * mut crate::src::binn::binn_struct,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if GetWriteConvertedData(Some(&mut type_0), &mut pvalue, Some(&mut size)) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    return binn_list_add_raw(list, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set(
    mut map: * mut crate::src::binn::binn_struct,
    mut id: std::os::raw::c_int,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if GetWriteConvertedData(Some(&mut type_0), &mut pvalue, Some(&mut size)) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    return binn_map_set_raw(map, id, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set(
    mut obj: * mut crate::src::binn::binn_struct,
    mut key: * const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    if GetWriteConvertedData(Some(&mut type_0), &mut pvalue, Some(&mut size)) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    return binn_object_set_raw(obj, key, type_0, pvalue, size);
}
#[no_mangle]
pub unsafe extern "C" fn binn_add_value(
    mut item: * mut crate::src::binn::binn_struct,
    mut binn_type_0: std::os::raw::c_int,
    mut id: std::os::raw::c_int,
    mut name: * mut std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
) -> std::os::raw::c_int {
    match binn_type_0 {
        224 => return binn_list_add(item, type_0, pvalue, size),
        225 => return binn_map_set(item, id, type_0, pvalue, size),
        226 => return binn_object_set(item, name, type_0, pvalue, size),
        _ => return 0 as std::os::raw::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_add_new(
    mut list: * mut crate::src::binn::binn_struct,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut retval: i32 = 0;
    retval = binn_list_add_value(list, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_set_new(
    mut map: * mut crate::src::binn::binn_struct,
    mut id: std::os::raw::c_int,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut retval: i32 = 0;
    retval = binn_map_set_value(map, id, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_set_new(
    mut obj: * mut crate::src::binn::binn_struct,
    mut key: * const std::os::raw::c_char,
    mut value: * mut crate::src::binn::binn_struct,
) -> std::os::raw::c_int {
    let mut retval: i32 = 0;
    retval = binn_object_set_value(obj, key, value);
    if !value.is_null() {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_value(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_list_get_value(ptr, pos, value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_value(
    mut ptr: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_map_get_value(ptr, id, value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_value(
    mut ptr: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> * mut crate::src::binn::binn_struct {
    let mut value = 0 as *mut binn;
    value = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if binn_object_get_value(ptr, key, value) == 0 as std::os::raw::c_int {
        free_fn.expect("non-null function pointer")(value as *mut std::os::raw::c_void);
        return 0 as *mut binn;
    }
    (*value).allocated = 1 as std::os::raw::c_int;
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_read<'a1, 'a2>(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_list_get_value(list, pos, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_read<'a1, 'a2>(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_map_get_value(map, id, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_read<'a1, 'a2>(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
    mut ptype: Option<&'a1 mut std::os::raw::c_int>,
    mut psize: Option<&'a2 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    if binn_object_get_value(obj, key, &mut value) == 0 as std::os::raw::c_int {
        return 0 as *mut std::os::raw::c_void;
    }
    if !borrow(& ptype).is_none() {
        *(borrow_mut(&mut ptype)).unwrap() = value.type_0;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return store_value(&mut value);
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_get<'a1>(
    mut ptr: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: i32 = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as std::os::raw::c_int && pvalue.is_null() {
        return 0 as std::os::raw::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_list_get_value(ptr, pos, &mut value) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_get<'a1>(
    mut ptr: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: i32 = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as std::os::raw::c_int && pvalue.is_null() {
        return 0 as std::os::raw::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_map_get_value(ptr, id, &mut value) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_get<'a1>(
    mut ptr: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut value = binn {
        header: 0,
        allocated: 0,
        writable: 0,
        dirty: 0,
        pbuf: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        pre_allocated: 0,
        alloc_size: 0,
        used_size: 0,
        type_0: 0,
        ptr: 0 as *const std::os::raw::c_void as *mut std::os::raw::c_void,
        size: 0,
        count: 0,
        freefn: None,
        c2rust_unnamed: C2RustUnnamed { vint8: 0 },
        disable_int_compression: 0,
    };
    let mut storage_type: i32 = 0;
    storage_type = binn_get_read_storage(type_0);
    if storage_type != 0 as std::os::raw::c_int && pvalue.is_null() {
        return 0 as std::os::raw::c_int;
    }
    zero_value(pvalue, type_0);
    if binn_object_get_value(ptr, key, &mut value) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type)
        == 0 as std::os::raw::c_int
    {
        return 0 as std::os::raw::c_int;
    }
    if !borrow(& psize).is_none() {
        *(borrow_mut(&mut psize)).unwrap() = value.size;
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int8(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_schar {
    let mut value: i8 = 0;
    binn_list_get(
        list,
        pos,
        0x21 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_schar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int16(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_short {
    let mut value: i16 = 0;
    binn_list_get(
        list,
        pos,
        0x41 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_short as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int32(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_list_get(
        list,
        pos,
        0x61 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_int64(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_longlong {
    let mut value: i64 = 0;
    binn_list_get(
        list,
        pos,
        0x81 as std::os::raw::c_int,
        &mut value as *mut int64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint8(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_uchar {
    let mut value: u8 = 0;
    binn_list_get(
        list,
        pos,
        0x20 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uchar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint16(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_ushort {
    let mut value: u16 = 0;
    binn_list_get(
        list,
        pos,
        0x40 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_ushort as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint32(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_uint {
    let mut value: u32 = 0;
    binn_list_get(
        list,
        pos,
        0x60 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uint as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_uint64(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_ulonglong {
    let mut value: u64 = 0;
    binn_list_get(
        list,
        pos,
        0x80 as std::os::raw::c_int,
        &mut value as *mut uint64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_float(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_float {
    let mut value: f32 = 0.;
    binn_list_get(
        list,
        pos,
        0x62 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_float as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_double(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut value: f64 = 0.;
    binn_list_get(
        list,
        pos,
        0x82 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_double as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_bool(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_list_get(
        list,
        pos,
        0x80061 as std::os::raw::c_int,
        &mut value as *mut BOOL as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_null(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> std::os::raw::c_int {
    return binn_list_get(
        list,
        pos,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_str(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> * mut std::os::raw::c_char {
    let mut value = 0 as *mut std::os::raw::c_char;
    binn_list_get(
        list,
        pos,
        0xa0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_blob<'a1>(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_list_get(
        list,
        pos,
        0xc0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        borrow_mut(&mut psize),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_list(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_list_get(
        list,
        pos,
        0xe0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_map(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_list_get(
        list,
        pos,
        0xe1 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_list_object(
    mut list: * mut core::ffi::c_void,
    mut pos: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_list_get(
        list,
        pos,
        0xe2 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int8(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_schar {
    let mut value: i8 = 0;
    binn_map_get(
        map,
        id,
        0x21 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_schar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int16(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_short {
    let mut value: i16 = 0;
    binn_map_get(
        map,
        id,
        0x41 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_short as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int32(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_map_get(
        map,
        id,
        0x61 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_int64(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_longlong {
    let mut value: i64 = 0;
    binn_map_get(
        map,
        id,
        0x81 as std::os::raw::c_int,
        &mut value as *mut int64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint8(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_uchar {
    let mut value: u8 = 0;
    binn_map_get(
        map,
        id,
        0x20 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uchar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint16(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_ushort {
    let mut value: u16 = 0;
    binn_map_get(
        map,
        id,
        0x40 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_ushort as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint32(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_uint {
    let mut value: u32 = 0;
    binn_map_get(
        map,
        id,
        0x60 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uint as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_uint64(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_ulonglong {
    let mut value: u64 = 0;
    binn_map_get(
        map,
        id,
        0x80 as std::os::raw::c_int,
        &mut value as *mut uint64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_float(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_float {
    let mut value: f32 = 0.;
    binn_map_get(
        map,
        id,
        0x62 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_float as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_double(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_double {
    let mut value: f64 = 0.;
    binn_map_get(
        map,
        id,
        0x82 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_double as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_bool(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_map_get(
        map,
        id,
        0x80061 as std::os::raw::c_int,
        &mut value as *mut BOOL as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_null(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> std::os::raw::c_int {
    return binn_map_get(
        map,
        id,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_str(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> * mut std::os::raw::c_char {
    let mut value = 0 as *mut std::os::raw::c_char;
    binn_map_get(
        map,
        id,
        0xa0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_blob<'a1>(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_map_get(
        map,
        id,
        0xc0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        borrow_mut(&mut psize),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_list(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_map_get(
        map,
        id,
        0xe0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_map(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_map_get(
        map,
        id,
        0xe1 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_map_object(
    mut map: * mut core::ffi::c_void,
    mut id: std::os::raw::c_int,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_map_get(
        map,
        id,
        0xe2 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int8(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_schar {
    let mut value: i8 = 0;
    binn_object_get(
        obj,
        key,
        0x21 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_schar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int16(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_short {
    let mut value: i16 = 0;
    binn_object_get(
        obj,
        key,
        0x41 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_short as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int32(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_object_get(
        obj,
        key,
        0x61 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_int as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_int64(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_longlong {
    let mut value: i64 = 0;
    binn_object_get(
        obj,
        key,
        0x81 as std::os::raw::c_int,
        &mut value as *mut int64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint8(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_uchar {
    let mut value: u8 = 0;
    binn_object_get(
        obj,
        key,
        0x20 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uchar as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint16(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_ushort {
    let mut value: u16 = 0;
    binn_object_get(
        obj,
        key,
        0x40 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_ushort as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint32(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_uint {
    let mut value: u32 = 0;
    binn_object_get(
        obj,
        key,
        0x60 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_uint as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_uint64(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_ulonglong {
    let mut value: u64 = 0;
    binn_object_get(
        obj,
        key,
        0x80 as std::os::raw::c_int,
        &mut value as *mut uint64 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_float(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_float {
    let mut value: f32 = 0.;
    binn_object_get(
        obj,
        key,
        0x62 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_float as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_double(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_double {
    let mut value: f64 = 0.;
    binn_object_get(
        obj,
        key,
        0x82 as std::os::raw::c_int,
        &mut value as *mut std::os::raw::c_double as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_bool(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    let mut value: i32 = 0;
    binn_object_get(
        obj,
        key,
        0x80061 as std::os::raw::c_int,
        &mut value as *mut BOOL as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_null(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> std::os::raw::c_int {
    return binn_object_get(
        obj,
        key,
        0 as std::os::raw::c_int,
        0 as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_str(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut value = 0 as *mut std::os::raw::c_char;
    binn_object_get(
        obj,
        key,
        0xa0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_char as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_blob<'a1>(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
    mut psize: Option<&'a1 mut std::os::raw::c_int>,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_object_get(
        obj,
        key,
        0xc0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        borrow_mut(&mut psize),
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_list(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_object_get(
        obj,
        key,
        0xe0 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_map(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_object_get(
        obj,
        key,
        0xe1 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn binn_object_object(
    mut obj: * mut core::ffi::c_void,
    mut key: * const std::os::raw::c_char,
) -> * mut core::ffi::c_void {
    let mut value = 0 as *mut std::os::raw::c_void;
    binn_object_get(
        obj,
        key,
        0xe2 as std::os::raw::c_int,
        &mut value as *mut *mut std::os::raw::c_void as *mut std::os::raw::c_void,
        Option::<&'_ mut i32>::None,
    );
    return value;
}
unsafe extern "C" fn binn_alloc_item() -> * mut crate::src::binn::binn_struct {
    let mut item = 0 as *mut binn;
    item = binn_malloc(::std::mem::size_of::<binn>() as std::os::raw::c_ulong as std::os::raw::c_int)
        as *mut binn;
    if !item.is_null() {
        memset(
            item as *mut std::os::raw::c_void,
            0 as std::os::raw::c_int,
            ::std::mem::size_of::<binn>() as std::os::raw::c_ulong,
        );
        (*item).header = 0x1f22b11f as std::os::raw::c_int;
        (*item).allocated = 1 as std::os::raw::c_int;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_value(
    mut type_0: std::os::raw::c_int,
    mut pvalue: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
    mut freefn: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) -> * mut crate::src::binn::binn_struct {
    let mut storage_type: i32 = 0;
    let mut item = binn_alloc_item();
    if !item.is_null() {
        (*item).type_0 = type_0;
        binn_get_type_info(type_0, Some(&mut storage_type), Option::<&'_ mut i32>::None);
        let mut current_block_19: u64;
        match storage_type {
            0 => {
                current_block_19 = 1109700713171191020;
            }
            160 => {
                if size == 0 as std::os::raw::c_int {
                    size = (strlen(pvalue as *mut std::os::raw::c_char))
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int;
                }
                current_block_19 = 3116744561211014279;
            }
            192 | 224 => {
                current_block_19 = 3116744561211014279;
            }
            _ => {
                let ref mut fresh48 = (*item).ptr;
                *fresh48 = &mut (*item).c2rust_unnamed.vint32 as *mut std::os::raw::c_int
                    as *mut std::os::raw::c_void;
                copy_raw_value(pvalue, (*item).ptr, storage_type);
                current_block_19 = 1109700713171191020;
            }
        }
        match current_block_19 {
            3116744561211014279 => {
                if (freefn
                    ).map(|f| f as usize) == ( core::intrinsics::transmute::<isize, Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>>(-(1 as std::os::raw::c_int) as isize)).map(|f| f as usize)
                {
                    let ref mut fresh44 = (*item).ptr;
                    *fresh44 = binn_memdup(pvalue, size);
                    if ((*item).ptr).is_null() {
                        free_fn
                            .expect(
                                "non-null function pointer",
                            )(item as *mut std::os::raw::c_void);
                        return 0 as *mut binn;
                    }
                    let ref mut fresh45 = (*item).freefn;
                    *fresh45 = free_fn;
                    if storage_type == 0xa0 as std::os::raw::c_int {
                        size -= 1;
                    }
                } else {
                    let ref mut fresh46 = (*item).ptr;
                    *fresh46 = pvalue;
                    let ref mut fresh47 = (*item).freefn;
                    *fresh47 = freefn;
                }
                (*item).size = size;
            }
            _ => {}
        }
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_string<'a1>(
    mut item: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut str: * mut std::os::raw::c_char,
    mut pfree: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) -> std::os::raw::c_int {
    if borrow(& item).is_none() || str.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if (pfree
        ).map(|f| f as usize) == ( core::intrinsics::transmute::<isize, Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>>(-(1 as std::os::raw::c_int) as isize)).map(|f| f as usize)
    {
        let ref mut fresh49 = (*(borrow_mut(&mut item)).unwrap()).ptr;
        *fresh49 = binn_memdup(
            str as *mut std::os::raw::c_void,
            (strlen(str)).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as std::os::raw::c_int,
        );
        if ((*(borrow_mut(&mut item)).unwrap()).ptr).is_null() {
            return 0 as std::os::raw::c_int;
        }
        let ref mut fresh50 = (*(borrow_mut(&mut item)).unwrap()).freefn;
        *fresh50 = free_fn;
    } else {
        let ref mut fresh51 = (*(borrow_mut(&mut item)).unwrap()).ptr;
        *fresh51 = str as *mut std::os::raw::c_void;
        let ref mut fresh52 = (*(borrow_mut(&mut item)).unwrap()).freefn;
        *fresh52 = pfree;
    }
    (*(borrow_mut(&mut item)).unwrap()).type_0 = 0xa0 as std::os::raw::c_int;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_set_blob<'a1>(
    mut item: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut ptr: * mut core::ffi::c_void,
    mut size: std::os::raw::c_int,
    mut pfree: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>,
) -> std::os::raw::c_int {
    if borrow(& item).is_none() || ptr.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if (pfree
        ).map(|f| f as usize) == ( core::intrinsics::transmute::<isize, Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>>(-(1 as std::os::raw::c_int) as isize)).map(|f| f as usize)
    {
        let ref mut fresh53 = (*(borrow_mut(&mut item)).unwrap()).ptr;
        *fresh53 = binn_memdup(ptr, size);
        if ((*(borrow_mut(&mut item)).unwrap()).ptr).is_null() {
            return 0 as std::os::raw::c_int;
        }
        let ref mut fresh54 = (*(borrow_mut(&mut item)).unwrap()).freefn;
        *fresh54 = free_fn;
    } else {
        let ref mut fresh55 = (*(borrow_mut(&mut item)).unwrap()).ptr;
        *fresh55 = ptr;
        let ref mut fresh56 = (*(borrow_mut(&mut item)).unwrap()).freefn;
        *fresh56 = pfree;
    }
    (*(borrow_mut(&mut item)).unwrap()).type_0 = 0xc0 as std::os::raw::c_int;
    (*(borrow_mut(&mut item)).unwrap()).size = size;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn atoi64(mut str: * mut std::os::raw::c_char) -> std::os::raw::c_longlong {
    let mut retval: i64 = 0;
    let mut is_negative = 0 as std::os::raw::c_int;
    if *str as std::os::raw::c_int == '-' as i32 {
        is_negative = 1 as std::os::raw::c_int;
        str = str.offset(1);
    }
    retval = 0 as std::os::raw::c_int as int64;
    while *str != 0 {
        retval = 10 as std::os::raw::c_int as std::os::raw::c_longlong * retval
            + (*str as std::os::raw::c_int - '0' as i32) as std::os::raw::c_longlong;
        str = str.offset(1);
    }
    if is_negative != 0 {
        retval *= -(1 as std::os::raw::c_int) as std::os::raw::c_longlong;
    }
    return retval;
}
unsafe extern "C" fn is_integer(mut p: * mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut retval: i32 = 0;
    if p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if *p as std::os::raw::c_int == '-' as i32 {
        p = p.offset(1);
    }
    if *p as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    retval = 1 as std::os::raw::c_int;
    while *p != 0 {
        if (*p as std::os::raw::c_int) < '0' as i32 || *p as std::os::raw::c_int > '9' as i32 {
            retval = 0 as std::os::raw::c_int;
        }
        p = p.offset(1);
    }
    return retval;
}
unsafe extern "C" fn is_float(mut p: * mut std::os::raw::c_char) -> std::os::raw::c_int {
    let mut retval: i32 = 0;
    let mut number_found = 0 as std::os::raw::c_int;
    if p.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if *p as std::os::raw::c_int == '-' as i32 {
        p = p.offset(1);
    }
    if *p as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int;
    }
    retval = 1 as std::os::raw::c_int;
    while *p != 0 {
        if *p as std::os::raw::c_int == '.' as i32 || *p as std::os::raw::c_int == ',' as i32 {
            if number_found == 0 {
                retval = 0 as std::os::raw::c_int;
            }
        } else if *p as std::os::raw::c_int >= '0' as i32 && *p as std::os::raw::c_int <= '9' as i32 {
            number_found = 1 as std::os::raw::c_int;
        } else {
            return 0 as std::os::raw::c_int
        }
        p = p.offset(1);
    }
    return retval;
}
unsafe extern "C" fn is_bool_str<'a1>(
    mut str: * mut std::os::raw::c_char,
    mut pbool: Option<&'a1 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut vint: i64 = 0;
    let mut vdouble: f64 = 0.;
    if str.is_null() || borrow(& pbool).is_none() {
        return 0 as std::os::raw::c_int;
    }
    if !(strcasecmp(str, b"true\0" as *const u8 as *const std::os::raw::c_char)
        == 0 as std::os::raw::c_int)
    {
        if !(strcasecmp(str, b"yes\0" as *const u8 as *const std::os::raw::c_char)
            == 0 as std::os::raw::c_int)
        {
            if !(strcasecmp(str, b"on\0" as *const u8 as *const std::os::raw::c_char)
                == 0 as std::os::raw::c_int)
            {
                if !(strcasecmp(str, b"false\0" as *const u8 as *const std::os::raw::c_char)
                    == 0 as std::os::raw::c_int)
                {
                    if !(strcasecmp(str, b"no\0" as *const u8 as *const std::os::raw::c_char)
                        == 0 as std::os::raw::c_int)
                    {
                        if !(strcasecmp(
                            str,
                            b"off\0" as *const u8 as *const std::os::raw::c_char,
                        ) == 0 as std::os::raw::c_int)
                        {
                            if is_integer(str) != 0 {
                                vint = atoi64(str);
                                *(borrow_mut(&mut pbool)).unwrap() = if vint != 0 as std::os::raw::c_int as std::os::raw::c_longlong {
                                    1 as std::os::raw::c_int
                                } else {
                                    0 as std::os::raw::c_int
                                };
                                return 1 as std::os::raw::c_int;
                            } else {
                                if is_float(str) != 0 {
                                    vdouble = atof(str);
                                    *(borrow_mut(&mut pbool)).unwrap() = if vdouble != 0 as std::os::raw::c_int as std::os::raw::c_double {
                                        1 as std::os::raw::c_int
                                    } else {
                                        0 as std::os::raw::c_int
                                    };
                                    return 1 as std::os::raw::c_int;
                                }
                            }
                            return 0 as std::os::raw::c_int;
                        }
                    }
                }
                *(borrow_mut(&mut pbool)).unwrap() = 0 as std::os::raw::c_int;
                return 1 as std::os::raw::c_int;
            }
        }
    }
    *(borrow_mut(&mut pbool)).unwrap() = 1 as std::os::raw::c_int;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int32<'a1>(
    mut value: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut pint: * mut std::os::raw::c_int,
) -> std::os::raw::c_int {
    if borrow(& value).is_none() || pint.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if type_family((*(borrow_mut(&mut value)).unwrap()).type_0) == 0xf2 as std::os::raw::c_int {
        return copy_int_value(
            (*(borrow_mut(&mut value)).unwrap()).ptr,
            pint as *mut std::os::raw::c_void,
            (*(borrow_mut(&mut value)).unwrap()).type_0,
            0x61 as std::os::raw::c_int,
        );
    }
    match (*(borrow(& value)).unwrap()).type_0 {
        98 => {
            if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat
                < (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as std::os::raw::c_float
                || (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat
                    > 2147483647 as std::os::raw::c_int as std::os::raw::c_float
            {
                return 0 as std::os::raw::c_int;
            }
            *pint = if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double >= 0.0f64 {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double + 0.5f64)
                    as std::os::raw::c_int
            } else if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double
                - (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_int as std::os::raw::c_double
                <= -0.5f64
            {
                (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_int
            } else {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double - 0.5f64)
                    as std::os::raw::c_int
            };
        }
        130 => {
            if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                < (-(2147483647 as std::os::raw::c_int) - 1 as std::os::raw::c_int) as std::os::raw::c_double
                || (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                    > 2147483647 as std::os::raw::c_int as std::os::raw::c_double
            {
                return 0 as std::os::raw::c_int;
            }
            *pint = if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble + 0.5f64) as std::os::raw::c_int
            } else if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                - (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble as std::os::raw::c_int as std::os::raw::c_double
                <= -0.5f64
            {
                (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vdouble as std::os::raw::c_int
            } else {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble - 0.5f64) as std::os::raw::c_int
            };
        }
        160 => {
            if is_integer((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *pint = atoi((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char);
            } else if is_float((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *pint = if atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) >= 0.0f64 {
                    (atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) + 0.5f64) as std::os::raw::c_int
                } else if atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char)
                    - atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) as std::os::raw::c_int
                        as std::os::raw::c_double <= -0.5f64
                {
                    atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) as std::os::raw::c_int
                } else {
                    (atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) - 0.5f64) as std::os::raw::c_int
                };
            } else {
                return 0 as std::os::raw::c_int
            }
        }
        524385 => {
            *pint = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vbool;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_int64<'a1>(
    mut value: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut pint: * mut std::os::raw::c_longlong,
) -> std::os::raw::c_int {
    if borrow(& value).is_none() || pint.is_null() {
        return 0 as std::os::raw::c_int;
    }
    if type_family((*(borrow_mut(&mut value)).unwrap()).type_0) == 0xf2 as std::os::raw::c_int {
        return copy_int_value(
            (*(borrow_mut(&mut value)).unwrap()).ptr,
            pint as *mut std::os::raw::c_void,
            (*(borrow_mut(&mut value)).unwrap()).type_0,
            0x81 as std::os::raw::c_int,
        );
    }
    match (*(borrow(& value)).unwrap()).type_0 {
        98 => {
            if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat
                < (-(9223372036854775807 as std::os::raw::c_long)
                    - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_float
                || (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat
                    > 9223372036854775807 as std::os::raw::c_long as std::os::raw::c_float
            {
                return 0 as std::os::raw::c_int;
            }
            *pint = (if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double >= 0.0f64 {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double + 0.5f64)
                    as std::os::raw::c_int
            } else if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double
                - (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_int as std::os::raw::c_double
                <= -0.5f64
            {
                (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_int
            } else {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double - 0.5f64)
                    as std::os::raw::c_int
            }) as int64;
        }
        130 => {
            if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                < (-(9223372036854775807 as std::os::raw::c_long)
                    - 1 as std::os::raw::c_int as std::os::raw::c_long) as std::os::raw::c_double
                || (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                    > 9223372036854775807 as std::os::raw::c_long as std::os::raw::c_double
            {
                return 0 as std::os::raw::c_int;
            }
            *pint = (if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble >= 0.0f64 {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble + 0.5f64) as std::os::raw::c_int
            } else if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                - (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble as std::os::raw::c_int as std::os::raw::c_double
                <= -0.5f64
            {
                (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vdouble as std::os::raw::c_int
            } else {
                ((*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble - 0.5f64) as std::os::raw::c_int
            }) as int64;
        }
        160 => {
            if is_integer((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *pint = atoi64((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char);
            } else if is_float((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *pint = (if atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) >= 0.0f64 {
                    (atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) + 0.5f64) as std::os::raw::c_int
                } else if atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char)
                    - atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) as std::os::raw::c_int
                        as std::os::raw::c_double <= -0.5f64
                {
                    atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) as std::os::raw::c_int
                } else {
                    (atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char) - 0.5f64) as std::os::raw::c_int
                }) as int64;
            } else {
                return 0 as std::os::raw::c_int
            }
        }
        524385 => {
            *pint = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vbool as int64;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_double<'a1, 'a2>(
    mut value: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut pfloat: Option<&'a2 mut std::os::raw::c_double>,
) -> std::os::raw::c_int {
    let mut vint: i64 = 0;
    if borrow(& value).is_none() || borrow(& pfloat).is_none() {
        return 0 as std::os::raw::c_int;
    }
    if type_family((*(borrow_mut(&mut value)).unwrap()).type_0) == 0xf2 as std::os::raw::c_int {
        if copy_int_value(
            (*(borrow_mut(&mut value)).unwrap()).ptr,
            &mut vint as *mut int64 as *mut std::os::raw::c_void,
            (*(borrow_mut(&mut value)).unwrap()).type_0,
            0x81 as std::os::raw::c_int,
        ) == 0 as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        *(borrow_mut(&mut pfloat)).unwrap() = vint as std::os::raw::c_double;
        return 1 as std::os::raw::c_int;
    }
    match (*(borrow(& value)).unwrap()).type_0 {
        98 => {
            *(borrow_mut(&mut pfloat)).unwrap() = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double;
        }
        130 => {
            *(borrow_mut(&mut pfloat)).unwrap() = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vdouble;
        }
        160 => {
            if is_integer((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *(borrow_mut(&mut pfloat)).unwrap() = atoi64((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) as std::os::raw::c_double;
            } else if is_float((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char) != 0 {
                *(borrow_mut(&mut pfloat)).unwrap() = atof((*(borrow(& value)).unwrap()).ptr as *mut std::os::raw::c_char);
            } else {
                return 0 as std::os::raw::c_int
            }
        }
        524385 => {
            *(borrow_mut(&mut pfloat)).unwrap() = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vbool as std::os::raw::c_double;
        }
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_bool<'a1, 'a2>(
    mut value: Option<&'a1 mut crate::src::binn::binn_struct>,
    mut pbool: Option<&'a2 mut std::os::raw::c_int>,
) -> std::os::raw::c_int {
    let mut vint: i64 = 0;
    if borrow(& value).is_none() || borrow(& pbool).is_none() {
        return 0 as std::os::raw::c_int;
    }
    if type_family((*(borrow_mut(&mut value)).unwrap()).type_0) == 0xf2 as std::os::raw::c_int {
        if copy_int_value(
            (*(borrow_mut(&mut value)).unwrap()).ptr,
            &mut vint as *mut int64 as *mut std::os::raw::c_void,
            (*(borrow_mut(&mut value)).unwrap()).type_0,
            0x81 as std::os::raw::c_int,
        ) == 0 as std::os::raw::c_int
        {
            return 0 as std::os::raw::c_int;
        }
        *(borrow_mut(&mut pbool)).unwrap() = if vint != 0 as std::os::raw::c_int as std::os::raw::c_longlong {
            1 as std::os::raw::c_int
        } else {
            0 as std::os::raw::c_int
        };
        return 1 as std::os::raw::c_int;
    }
    match (*(borrow(& value)).unwrap()).type_0 {
        524385 => {
            *(borrow_mut(&mut pbool)).unwrap() = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vbool;
        }
        98 => {
            *(borrow_mut(&mut pbool)).unwrap() = if (*(borrow(& value)).unwrap()).c2rust_unnamed.vfloat
                != 0 as std::os::raw::c_int as std::os::raw::c_float
            {
                1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            };
        }
        130 => {
            *(borrow_mut(&mut pbool)).unwrap() = if (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble
                != 0 as std::os::raw::c_int as std::os::raw::c_double
            {
                1 as std::os::raw::c_int
            } else {
                0 as std::os::raw::c_int
            };
        }
        160 => return is_bool_str((*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char, borrow_mut(&mut pbool)),
        _ => return 0 as std::os::raw::c_int,
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binn_get_str<'a1>(mut value: Option<&'a1 mut crate::src::binn::binn_struct>) -> * mut std::os::raw::c_char {
    let mut current_block: u64;
    let mut vint: i64 = 0;
    let mut buf: [i8; 128] = [0; 128];
    if borrow(& value).is_none() {
        return (0 as * mut i8);
    }
    if type_family((*(borrow_mut(&mut value)).unwrap()).type_0) == 0xf2 as std::os::raw::c_int {
        if copy_int_value(
            (*(borrow_mut(&mut value)).unwrap()).ptr,
            &mut vint as *mut int64 as *mut std::os::raw::c_void,
            (*(borrow_mut(&mut value)).unwrap()).type_0,
            0x81 as std::os::raw::c_int,
        ) == 0 as std::os::raw::c_int
        {
            return (0 as * mut i8);
        }
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as std::os::raw::c_ulong,
            b"%lli\0" as *const u8 as *const std::os::raw::c_char,
            vint,
        );
    } else {
        match (*(borrow(& value)).unwrap()).type_0 {
            98 => {
                (*(borrow_mut(&mut value)).unwrap())
                    .c2rust_unnamed
                    .vdouble = (*(borrow_mut(&mut value)).unwrap()).c2rust_unnamed.vfloat as std::os::raw::c_double;
                current_block = 18111289606297366774;
            }
            130 => {
                current_block = 18111289606297366774;
            }
            160 => return (*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char,
            524385 => {
                if (*(borrow(& value)).unwrap()).c2rust_unnamed.vbool != 0 {
                    strcpy(
                        buf.as_mut_ptr(),
                        b"true\0" as *const u8 as *const std::os::raw::c_char,
                    );
                } else {
                    strcpy(
                        buf.as_mut_ptr(),
                        b"false\0" as *const u8 as *const std::os::raw::c_char,
                    );
                }
                current_block = 8192695711153237833;
            }
            _ => return (0 as * mut i8),
        }
        match current_block {
            8192695711153237833 => {}
            _ => {
                snprintf(
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as std::os::raw::c_ulong,
                    b"%g\0" as *const u8 as *const std::os::raw::c_char,
                    (*(borrow(& value)).unwrap()).c2rust_unnamed.vdouble,
                );
            }
        }
    }
    let ref mut fresh57 = (*(borrow_mut(&mut value)).unwrap()).ptr;
    *fresh57 = strdup(buf.as_mut_ptr()) as *mut std::os::raw::c_void;
    if ((*(borrow_mut(&mut value)).unwrap()).ptr).is_null() {
        return (0 as * mut i8);
    }
    let ref mut fresh58 = (*(borrow_mut(&mut value)).unwrap()).freefn;
    *fresh58 = Some(free);
    (*(borrow_mut(&mut value)).unwrap()).type_0 = 0xa0 as std::os::raw::c_int;
    return (*(borrow_mut(&mut value)).unwrap()).ptr as *mut std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn binn_is_container<'a1>(mut item: Option<&'a1 mut crate::src::binn::binn_struct>) -> std::os::raw::c_int {
    if borrow(& item).is_none() {
        return 0 as std::os::raw::c_int;
    }
    match (*(borrow(& item)).unwrap()).type_0 {
        224 | 225 | 226 => return 1 as std::os::raw::c_int,
        _ => return 0 as std::os::raw::c_int,
    };
}
use crate::laertes_rt::*;
