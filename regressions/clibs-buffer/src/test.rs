use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn exit(_: i32) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> u64;
    // prototypes
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub type size_t = u64;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;


#[derive(Copy, Clone)]
struct ErasedByPreprocessor0;
impl Default for ErasedByPreprocessor0 {fn default() -> Self {Self {
}}}

//
// test.c
//
// Copyright (c) 2012 TJ Holowaychuk <tj@vision-media.ca>
//
#[no_mangle]
pub unsafe extern "C" fn equal(mut a: *const libc::c_char, mut b: *const libc::c_char) {
    if strcmp(a, b) != 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(
            b"  expected: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            a,
        );
        printf(
            b"    actual: \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            b,
        );
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new() {
    let mut buf = crate::src::buffer::buffer_new();
    if 64 as i32 as u64 == crate::src::buffer::buffer_size(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"BUFFER_DEFAULT_SIZE == buffer_size(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            28 as i32 as libc::c_uint,
            b"void test_buffer_new()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            29 as i32 as libc::c_uint,
            b"void test_buffer_new()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_new_with_size() {
    let mut buf = crate::src::buffer::buffer_new_with_size(1024 as i32 as size_t);
    if 1024 as i32 as u64 == crate::src::buffer::buffer_size(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"1024 == buffer_size(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            36 as i32 as libc::c_uint,
            b"void test_buffer_new_with_size()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            37 as i32 as libc::c_uint,
            b"void test_buffer_new_with_size()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Hello\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            44 as i32 as libc::c_uint,
            b"void test_buffer_append()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" World\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            45 as i32 as libc::c_uint,
            b"void test_buffer_append()\x00" as *const u8 as *const i8,
        );
    };
    if strlen(b"Hello World\x00" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\x00" as *const u8
                as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            46 as i32 as libc::c_uint,
            b"void test_buffer_append()\x00" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello World\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append_n() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as i32
        == crate::src::buffer::buffer_append_n(
            buf.as_deref_mut(),
            b"subway\x00" as *const u8 as *const libc::c_char,
            3 as i32 as size_t,
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"subway\", 3)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            54 as i32 as libc::c_uint,
            b"void test_buffer_append_n()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32
        == crate::src::buffer::buffer_append_n(
            buf.as_deref_mut(),
            b"marines\x00" as *const u8 as *const libc::c_char,
            6 as i32 as size_t,
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_append_n(buf, \"marines\", 6)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            55 as i32 as libc::c_uint,
            b"void test_buffer_append_n()\x00" as *const u8 as *const i8,
        );
    };
    if strlen(b"submarine\x00" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"strlen(\"submarine\") == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            56 as i32 as libc::c_uint,
            b"void test_buffer_append_n()\x00" as *const u8 as *const i8,
        );
    };
    equal(
        b"submarine\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_append__grow() {
    let mut buf = crate::src::buffer::buffer_new_with_size(10 as i32 as size_t);
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Hello\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \"Hello\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            64 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" tobi\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" tobi\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            65 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" was\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" was\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            66 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" here\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" here\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            67 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    let mut str =
        b"Hello tobi was here\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    equal(str, (*buf.as_deref().unwrap()).data);
    if 1024 as i32 as u64 == crate::src::buffer::buffer_size(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"1024 == buffer_size(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            71 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    if strlen(str) == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"strlen(str) == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            72 as i32 as libc::c_uint,
            b"void test_buffer_append__grow()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_prepend() {
    let mut buf = crate::src::buffer::buffer_new();
    if 0 as i32 == crate::src::buffer::buffer_append(buf.as_deref_mut(), b" World\x00" as *const u8 as *const libc::c_char) {
    } else {
        __assert_fail(
            b"0 == buffer_append(buf, \" World\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            79 as i32 as libc::c_uint,
            b"void test_buffer_prepend()\x00" as *const u8 as *const i8,
        );
    };
    if 0 as i32
        == crate::src::buffer::buffer_prepend(
            buf.as_deref_mut(),
            b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    {
    } else {
        __assert_fail(
            b"0 == buffer_prepend(buf, \"Hello\")\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            80 as i32 as libc::c_uint,
            b"void test_buffer_prepend()\x00" as *const u8 as *const i8,
        );
    };
    if strlen(b"Hello World\x00" as *const u8 as *const libc::c_char) == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"strlen(\"Hello World\") == buffer_length(buf)\x00" as *const u8
                as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            81 as i32 as libc::c_uint,
            b"void test_buffer_prepend()\x00" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello World\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice() {
    let mut buf = crate::src::buffer::buffer_new();
    crate::src::buffer::buffer_append(buf.as_deref_mut(), b"Tobi Ferret\x00" as *const u8 as *const libc::c_char);
    let mut a = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 2 as i32 as size_t, 8 as i32 as ssize_t);
    equal(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"bi Fer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    crate::src::buffer::buffer_free(a);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__range_error() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 10 as i32 as size_t, 2 as i32 as ssize_t);
    if a.as_deref().is_none() {();
    } else {
        __assert_fail(
            b"NULL == a\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            103 as i32 as libc::c_uint,
            b"void test_buffer_slice__range_error()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 5 as i32 as size_t, -(1 as i32) as ssize_t);
    equal(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    let mut b = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 5 as i32 as size_t, -(3 as i32) as ssize_t);
    equal(
        b"Ferr\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*b.as_deref().unwrap()).data,
    );
    let mut c = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 8 as i32 as size_t, -(1 as i32) as ssize_t);
    equal(
        b"ret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*c.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(b);
    crate::src::buffer::buffer_free(c);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_slice__end_overflow() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut a = crate::src::buffer::buffer_slice(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()), 5 as i32 as size_t, 1000 as i32 as ssize_t);
    equal(
        b"Tobi Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    equal(
        b"Ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*a.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_equals() {
    let mut a =
        crate::src::buffer::buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    let mut b =
        crate::src::buffer::buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if 1 as i32 == crate::src::buffer::buffer_equals(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(a.as_deref()), core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(b.as_deref())) {
    } else {
        __assert_fail(
            b"1 == buffer_equals(a, b)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            142 as i32 as libc::c_uint,
            b"void test_buffer_equals()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_append(b.as_deref_mut(), b" World\x00" as *const u8 as *const libc::c_char);
    if 0 as i32 == crate::src::buffer::buffer_equals(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(a.as_deref()), core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(b.as_deref())) {
    } else {
        __assert_fail(
            b"0 == buffer_equals(a, b)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            145 as i32 as libc::c_uint,
            b"void test_buffer_equals()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(a);
    crate::src::buffer::buffer_free(b);
}
/*
void test_buffer_formatting() {
  buffer_t *buf = buffer_new();
  int result = buffer_appendf(buf, "%d %s", 3, "cow");
  assert(0 == result);
  equal("3 cow", buffer_string(buf));
  result = buffer_appendf(buf, " - 0x%08X", 0xdeadbeef);
  assert(0 == result);
  equal("3 cow - 0xDEADBEEF", buffer_string(buf));
  buffer_free(buf);
}
*/
#[no_mangle]
pub unsafe extern "C" fn test_buffer_indexof() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"Tobi is a ferret\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut i = crate::src::buffer::buffer_indexof(
        core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()),
        b"is\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 5 as i32 as libc::c_long == i {
    } else {
        __assert_fail(
            b"5 == i\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            169 as i32 as libc::c_uint,
            b"void test_buffer_indexof()\x00" as *const u8 as *const i8,
        );
    };
    i= crate::src::buffer::buffer_indexof(
        core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()),
        b"a\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if 8 as i32 as libc::c_long == i {
    } else {
        __assert_fail(
            b"8 == i\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            172 as i32 as libc::c_uint,
            b"void test_buffer_indexof()\x00" as *const u8 as *const i8,
        );
    };
    i= crate::src::buffer::buffer_indexof(
        core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref()),
        b"something\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if -(1 as i32) as libc::c_long == i {
    } else {
        __assert_fail(
            b"-1 == i\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            175 as i32 as libc::c_uint,
            b"void test_buffer_indexof()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_fill() {
    let mut buf =
        crate::src::buffer::buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if 5 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            183 as i32 as libc::c_uint,
            b"void test_buffer_fill()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_fill(buf.as_deref_mut(), 0 as i32);
    if 0 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            186 as i32 as libc::c_uint,
            b"void test_buffer_fill()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_clear() {
    let mut buf =
        crate::src::buffer::buffer_new_with_copy(b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if 5 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            193 as i32 as libc::c_uint,
            b"void test_buffer_clear()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_clear(buf.as_deref_mut());
    if 0 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"0 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            196 as i32 as libc::c_uint,
            b"void test_buffer_clear()\x00" as *const u8 as *const i8,
        );
    };
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_trim() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim(buf.as_deref_mut());
    equal(
        b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    buf= crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim_left(buf.as_deref_mut());
    equal(
        b"Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
    buf= crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim_right(buf.as_deref_mut());
    equal(
        b"  Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer_compact() {
    let mut buf = crate::src::buffer::buffer_new_with_copy(
        b"  Hello\n\n \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    crate::src::buffer::buffer_trim(buf.as_deref_mut());
    if 5 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            222 as i32 as libc::c_uint,
            b"void test_buffer_compact()\x00" as *const u8 as *const i8,
        );
    };
    if 10 as i32 as u64 == crate::src::buffer::buffer_size(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"10 == buffer_size(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            223 as i32 as libc::c_uint,
            b"void test_buffer_compact()\x00" as *const u8 as *const i8,
        );
    };
    let mut removed = crate::src::buffer::buffer_compact(buf.as_deref_mut());
    if 5 as i32 as libc::c_long == removed {
    } else {
        __assert_fail(
            b"5 == removed\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            226 as i32 as libc::c_uint,
            b"void test_buffer_compact()\x00" as *const u8 as *const i8,
        );
    };
    if 5 as i32 as u64 == crate::src::buffer::buffer_length(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"5 == buffer_length(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            227 as i32 as libc::c_uint,
            b"void test_buffer_compact()\x00" as *const u8 as *const i8,
        );
    };
    if 5 as i32 as u64 == crate::src::buffer::buffer_size(core::mem::transmute::<_, *const crate::src::buffer::buffer_t>(buf.as_deref())) {
    } else {
        __assert_fail(
            b"5 == buffer_size(buf)\x00" as *const u8 as *const libc::c_char,
            b"test.c\x00" as *const u8 as *const libc::c_char,
            228 as i32 as libc::c_uint,
            b"void test_buffer_compact()\x00" as *const u8 as *const i8,
        );
    };
    equal(
        b"Hello\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*buf.as_deref().unwrap()).data,
    );
    crate::src::buffer::buffer_free(buf);
}
unsafe fn main_0() -> i32 {
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
    /*test_buffer_formatting();*/
    test_buffer_indexof();
    test_buffer_fill();
    test_buffer_clear();
    test_buffer_trim();
    test_buffer_compact();
    printf(
        b"\n  \x1b[32m\xe2\x9c\x93 \x1b[90mok\x1b[0m\n\n\x00" as *const u8 as *const libc::c_char,
    );
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
