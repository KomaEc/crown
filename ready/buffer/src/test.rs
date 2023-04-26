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
    
    
    
    
    
    
    
    
    
    // fn buffer_appendf(
    //     self_0: *mut buffer_t,
    //     format: *const libc::c_char,
    //     _: ...
    // ) -> libc::c_int;
    
    
    
    
    
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
impl Default for ErasedByPreprocessor0 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

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
    let mut buf = crate::src::buffer::buffer_new();
    if 64 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_size(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"BUFFER_DEFAULT_SIZE == buffer_size(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            b"void test_buffer_new()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            b"void test_buffer_new()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new_with_size() {
    let mut buf = crate::src::buffer::buffer_new_with_size(1024 as libc::c_int as size_t);
    if 1024 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_size(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            b"void test_buffer_new_with_size()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            b"void test_buffer_new_with_size()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Hello\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            b"void test_buffer_append()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" World\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            b"void test_buffer_append()\0" as *const u8 as *const i8,
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()))
    {} else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            b"void test_buffer_append()\0" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append_n(
            buf.as_deref_mut(),
            b"subway\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"subway\", 3)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int as libc::c_uint,
            b"void test_buffer_append_n()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append_n(
            buf.as_deref_mut(),
            b"marines\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"marines\", 6)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            b"void test_buffer_append_n()\0" as *const u8 as *const i8,
        );
    };
    if strlen(b"submarine\0" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()))
    {} else {
        __assert_fail(
            b"strlen(\"submarine\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            b"void test_buffer_append_n()\0" as *const u8 as *const i8,
        );
    };
    equal(
        b"submarine\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append__grow() {
    let mut buf = crate::src::buffer::buffer_new_with_size(10 as libc::c_int as size_t);
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Hello\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" tobi\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" tobi\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" was\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" was\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" here\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" here\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    let mut str = b"Hello tobi was here\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    equal(str, (*buf.as_deref().unwrap()).data);
    if 1024 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_size(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"1024 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    if strlen(str) == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"strlen(str) == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            b"void test_buffer_append__grow()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_prepend() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as libc::c_int
        == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" World\0" as *const u8 as *const libc::c_char)
    {} else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            b"void test_buffer_prepend()\0" as *const u8 as *const i8,
        );
    };
    if 0 as libc::c_int
        == crate::src::buffer::buffer_prepend(
            buf.as_deref_mut(),
            b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    {} else {
        __assert_fail(
            b"0 == buffer_prepend(buf, \"Hello\")\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            b"void test_buffer_prepend()\0" as *const u8 as *const i8,
        );
    };
    if strlen(b"Hello World\0" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()))
    {} else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\0" as *const u8
                as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            b"void test_buffer_prepend()\0" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello World\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice() {
    let mut buf = crate::src::buffer::buffer_new();
    crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Tobi Ferret\0" as *const u8 as *const libc::c_char);
    let mut a = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        2 as libc::c_int as size_t,
        8 as libc::c_int as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"bi Fer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    crate::src::buffer::buffer_free(a);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__range_error() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        10 as libc::c_int as size_t,
        2 as libc::c_int as ssize_t,
    );
    if a.as_deref().is_none() {();} else {
        __assert_fail(
            b"NULL == a\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            b"void test_buffer_slice__range_error()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        5 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    let mut b = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        5 as libc::c_int as size_t,
        -(3 as libc::c_int) as ssize_t,
    );
    equal(b"Ferr\0" as *const u8 as *const libc::c_char as *mut libc::c_char, (*b.as_deref().unwrap()).data);
    let mut c = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        8 as libc::c_int as size_t,
        -(1 as libc::c_int) as ssize_t,
    );
    equal(b"ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char, (*c.as_deref().unwrap()).data);
    crate::src::buffer::buffer_free(buf);
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(b);
    crate::src::buffer::buffer_free(c);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end_overflow() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        5 as libc::c_int as size_t,
        1000 as libc::c_int as ssize_t,
    );
    equal(
        b"Tobi Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"Ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() {
    let mut a = crate::src::buffer::buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut b = crate::src::buffer::buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 1 as libc::c_int == crate::src::buffer::buffer_equals(a.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), b.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"1 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            b"void test_buffer_equals()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_append(b.as_deref_mut(), b" World\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == crate::src::buffer::buffer_equals(a.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), b.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"0 == buffer_equals(a, b)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            b"void test_buffer_equals()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(b);
}
// #[no_mangle]
// pub unsafe extern "C" fn test_buffer_formatting() {
//     let mut buf = buffer_new();
//     let mut result = buffer_appendf(
//         buf,
//         b"%d %s\0" as *const u8 as *const libc::c_char,
//         3 as libc::c_int,
//         b"cow\0" as *const u8 as *const libc::c_char,
//     );
//     if 0 as libc::c_int == result {} else {
//         __assert_fail(
//             b"0 == result\0" as *const u8 as *const libc::c_char,
//             b"test.c\0" as *const u8 as *const libc::c_char,
//             154 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<
//                 &[u8; 30],
//                 &[libc::c_char; 30],
//             >(b"void test_buffer_formatting()\0"))
//                 .as_ptr(),
//         );
//     };
//     equal(
//         b"3 cow\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         (*buf).data,
//     );
//     result = buffer_appendf(
//         buf,
//         b" - 0x%08X\0" as *const u8 as *const libc::c_char,
//         0xdeadbeef as libc::c_uint,
//     );
//     if 0 as libc::c_int == result {} else {
//         __assert_fail(
//             b"0 == result\0" as *const u8 as *const libc::c_char,
//             b"test.c\0" as *const u8 as *const libc::c_char,
//             157 as libc::c_int as libc::c_uint,
//             (*::std::mem::transmute::<
//                 &[u8; 30],
//                 &[libc::c_char; 30],
//             >(b"void test_buffer_formatting()\0"))
//                 .as_ptr(),
//         );
//     };
//     equal(
//         b"3 cow - 0xDEADBEEF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
//         (*buf).data,
//     );
//     buffer_free(buf);
// }
#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi is a ferret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut i = crate::src::buffer::buffer_indexof(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        b"is\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_long == i {} else {
        __assert_fail(
            b"5 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            b"void test_buffer_indexof()\0" as *const u8 as *const i8,
        );
    };
    i= crate::src::buffer::buffer_indexof(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 8 as libc::c_int as libc::c_long == i {} else {
        __assert_fail(
            b"8 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            b"void test_buffer_indexof()\0" as *const u8 as *const i8,
        );
    };
    i= crate::src::buffer::buffer_indexof(
        buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
        b"something\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if -(1 as libc::c_int) as libc::c_long == i {} else {
        __assert_fail(
            b"-1 == i\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            b"void test_buffer_indexof()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            b"void test_buffer_fill()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_fill(buf.as_deref_mut(), 0 as libc::c_int);
    if 0 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            b"void test_buffer_fill()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_clear() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            b"void test_buffer_clear()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_clear(buf.as_deref_mut());
    if 0 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"0 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            b"void test_buffer_clear()\0" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim(buf.as_deref_mut());
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    buf= crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim_left(buf.as_deref_mut());
    equal(
        b"Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    buf= crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim_right(buf.as_deref_mut());
    equal(
        b"  Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim(buf.as_deref_mut());
    if 5 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            b"void test_buffer_compact()\0" as *const u8 as *const i8,
        );
    };
    if 10 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_size(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"10 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            b"void test_buffer_compact()\0" as *const u8 as *const i8,
        );
    };
    let mut removed = crate::src::buffer::buffer_compact(buf.as_deref_mut());
    if 5 as libc::c_int as libc::c_long == removed {} else {
        __assert_fail(
            b"5 == removed\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            b"void test_buffer_compact()\0" as *const u8 as *const i8,
        );
    };
    if 5 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_length(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"5 == buffer_length(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            b"void test_buffer_compact()\0" as *const u8 as *const i8,
        );
    };
    if 5 as libc::c_int as libc::c_ulong == crate::src::buffer::buffer_size(buf.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) {} else {
        __assert_fail(
            b"5 == buffer_size(buf)\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            b"void test_buffer_compact()\0" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
pub unsafe fn main_0() -> libc::c_int {
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
    // test_buffer_formatting();
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
// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
