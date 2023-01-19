use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn buffer_slice(self_0: *mut buffer_t, from: size_t, to: ssize_t) -> *mut buffer_t;
    fn buffer_new() -> *mut buffer_t;
    fn buffer_new_with_size(n: size_t) -> *mut buffer_t;
    fn buffer_new_with_copy(str: *mut libc::c_char) -> *mut buffer_t;
    fn buffer_size(self_0: *mut buffer_t) -> size_t;
    fn buffer_length(self_0: *mut buffer_t) -> size_t;
    fn buffer_free(self_0: *mut buffer_t);
    fn buffer_prepend(self_0: *mut buffer_t, str: *mut libc::c_char) -> libc::c_int;
    fn buffer_append(self_0: *mut buffer_t, str: *const libc::c_char) -> libc::c_int;
    fn buffer_appendf(
        self_0: *mut buffer_t,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn buffer_append_n(
        self_0: *mut buffer_t,
        str: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_equals(self_0: *mut buffer_t, other: *mut buffer_t) -> libc::c_int;
    fn buffer_indexof(self_0: *mut buffer_t, str: *mut libc::c_char) -> ssize_t;
    fn buffer_compact(self_0: *mut buffer_t) -> ssize_t;
    fn buffer_fill(self_0: *mut buffer_t, c: libc::c_int);
    fn buffer_clear(self_0: *mut buffer_t);
    fn buffer_trim_left(self_0: *mut buffer_t);
    fn buffer_trim_right(self_0: *mut buffer_t);
    fn buffer_trim(self_0: *mut buffer_t);
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_t {
    pub len: size_t,
    pub alloc: *mut libc::c_char,
    pub data: *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn equal(mut a: *mut libc::c_char, mut b: *mut libc::c_char) {
    if strcmp(a, b) != 0 {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"  expected: '%s'\n\0" as *const u8 as *const libc::c_char, a);
        printf(b"    actual: '%s'\n\0" as *const u8 as *const libc::c_char, b);
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new() {
    let mut buf = buffer_new();
    if 64 as libc::c_int as libc::c_ulong == buffer_size(buf) {} else {
        __assert_fail(
            b"BUFFER_DEFAULT_SIZE == buffer_size(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void test_buffer_new()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void test_buffer_new()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new_with_size() {
    let mut buf = buffer_new_with_size(1024 as libc::c_int as size_t);
    if 1024 as libc::c_int as libc::c_ulong == buffer_size(buf) {} else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_buffer_new_with_size()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_buffer_new_with_size()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append() {
    let mut buf = buffer_new();
    if 0 as libc::c_int
        == buffer_append(buf, b"Hello\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_buffer_append()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" World\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_buffer_append()\0"))
                .as_ptr(),
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char) == buffer_length(buf)
    {} else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_buffer_append()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf = buffer_new();
    if 0 as libc::c_int
        == buffer_append_n(
            buf,
            b"subway\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"subway\", 3)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_buffer_append_n()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_append_n(
            buf,
            b"marines\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"marines\", 6)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_buffer_append_n()\0"))
                .as_ptr(),
        );
    };
    if strlen(b"submarine\0" as *const u8 as *const libc::c_char) == buffer_length(buf)
    {} else {
        __assert_fail(
            b"strlen(\"submarine\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_buffer_append_n()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"submarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append__grow() {
    let mut buf = buffer_new_with_size(10 as libc::c_int as size_t);
    if 0 as libc::c_int
        == buffer_append(buf, b"Hello\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" tobi\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" tobi\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" was\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" was\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_append(buf, b" here\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" here\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    let mut str = b"Hello tobi was here\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    equal(str, (*buf).data);
    if 1024 as libc::c_int as libc::c_ulong == buffer_size(buf) {} else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    if strlen(str) == buffer_length(buf) {} else {
        __assert_fail(
            b"strlen(str) == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_buffer_append__grow()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_prepend() {
    let mut buf = buffer_new();
    if 0 as libc::c_int
        == buffer_append(buf, b" World\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_prepend()\0"))
                .as_ptr(),
        );
    };
    if 0 as libc::c_int
        == buffer_prepend(
            buf,
            b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_prepend(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_prepend()\0"))
                .as_ptr(),
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char) == buffer_length(buf)
    {} else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_prepend()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice() {
    let mut buf = buffer_new();
    buffer_append(buf, b"Tobi Ferret\0" as *const u8 as *const libc::c_char);
    let mut a = buffer_slice(
        buf,
        2 as libc::c_int as size_t,
        8 as libc::c_int as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    equal(
        b"bi Fer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data,
    );
    buffer_free(buf);
    buffer_free(a);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__range_error() {
    let mut buf = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = buffer_slice(
        buf,
        10 as libc::c_int as size_t,
        2 as libc::c_int as ssize_t,
    );
    if a.is_null() {} else {
        __assert_fail(
            b"NULL == a\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_buffer_slice__range_error()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end() {
    let mut buf = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data,
    );
    let mut b = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        -(3 as libc::c_int) as ssize_t,
    );
    equal(b"Ferr\0" as *const u8 as *const libc::c_char as *mut libc::c_char, (*b).data);
    let mut c = buffer_slice(
        buf,
        8 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t,
    );
    equal(b"ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char, (*c).data);
    buffer_free(buf);
    buffer_free(a);
    buffer_free(b);
    buffer_free(c);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end_overflow() {
    let mut buf = buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = buffer_slice(
        buf,
        5 as libc::c_int as size_t,
        1000 as libc::c_int as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a).data,
    );
    buffer_free(a);
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() {
    let mut a = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut b = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 1 as libc::c_int == buffer_equals(a, b) {} else {
        __assert_fail(
            b"1 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_buffer_equals()\0"))
                .as_ptr(),
        );
    };
    buffer_append(b, b" World\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == buffer_equals(a, b) {} else {
        __assert_fail(
            b"0 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_buffer_equals()\0"))
                .as_ptr(),
        );
    };
    buffer_free(a);
    buffer_free(b);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_formatting() {
    let mut buf = buffer_new();
    let mut result = buffer_appendf(
        buf,
        b"%d %s\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int,
        b"cow\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == result {} else {
        __assert_fail(
            b"0 == result\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_buffer_formatting()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"3 cow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    result = buffer_appendf(
        buf,
        b" - 0x%08X\0" as *const u8 as *const libc::c_char,
        0xdeadbeef as libc::c_uint,
    );
    if 0 as libc::c_int == result {} else {
        __assert_fail(
            b"0 == result\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_buffer_formatting()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"3 cow - 0xDEADBEEF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() {
    let mut buf = buffer_new_with_copy(
        b"Tobi is a ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut i = buffer_indexof(
        buf,
        b"is\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_long == i {} else {
        __assert_fail(
            b"5 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_indexof()\0"))
                .as_ptr(),
        );
    };
    i = buffer_indexof(
        buf,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 8 as libc::c_int as libc::c_long == i {} else {
        __assert_fail(
            b"8 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_indexof()\0"))
                .as_ptr(),
        );
    };
    i = buffer_indexof(
        buf,
        b"something\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if -(1 as libc::c_int) as libc::c_long == i {} else {
        __assert_fail(
            b"-1 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_indexof()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() {
    let mut buf = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void test_buffer_fill()\0"))
                .as_ptr(),
        );
    };
    buffer_fill(buf, 0 as libc::c_int);
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"void test_buffer_fill()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_clear() {
    let mut buf = buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_buffer_clear()\0"))
                .as_ptr(),
        );
    };
    buffer_clear(buf);
    if 0 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_buffer_clear()\0"))
                .as_ptr(),
        );
    };
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() {
    let mut buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    buffer_trim(buf);
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    buffer_trim_left(buf);
    equal(
        b"Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
    buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    buffer_trim_right(buf);
    equal(
        b"  Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() {
    let mut buf = buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    buffer_trim(buf);
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_compact()\0"))
                .as_ptr(),
        );
    };
    if 10 as libc::c_int as libc::c_ulong == buffer_size(buf) {} else {
        __assert_fail(
            b"10 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_compact()\0"))
                .as_ptr(),
        );
    };
    let mut removed = buffer_compact(buf);
    if 5 as libc::c_int as libc::c_long == removed {} else {
        __assert_fail(
            b"5 == removed\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_compact()\0"))
                .as_ptr(),
        );
    };
    if 5 as libc::c_int as libc::c_ulong == buffer_length(buf) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_compact()\0"))
                .as_ptr(),
        );
    };
    if 5 as libc::c_int as libc::c_ulong == buffer_size(buf) {} else {
        __assert_fail(
            b"5 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_buffer_compact()\0"))
                .as_ptr(),
        );
    };
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf).data,
    );
    buffer_free(buf);
}
unsafe fn main_0() -> libc::c_int {
    test_buffer_new();
    test_buffer_new_with_size();
    test_buffer_append();
    test_buffer_append__grow();
    test_buffer_append_n();
    test_buffer_prepend();
    test_buffer_slice();
    test_buffer_slice__range_error();
    test_buffer_slice__end();
    test_buffer_slice__end_overflow();
    test_buffer_equals();
    test_buffer_formatting();
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf(
        b"\n  \x1B[32m\xE2\x9C\x93 \x1B[90mok\x1B[0m\n\n\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
