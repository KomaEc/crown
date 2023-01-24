use ::libc;
extern "C" {
    pub type _IO_wide_data;
    
    
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    
    
    
    
    
    
    
    
    
    
    
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn clock() -> clock_t;
    fn srand(__seed: libc::c_uint);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor6 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::example3::_IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor7 { dummy: () }
pub type clock_t = __clock_t;
static mut ltests: libc::c_int = 0 as libc::c_int;
static mut lfails: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn basic() {
    let mut ann = crate::src::genann::genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    crate::src::test::ltests+= 1;
    if (*ann).total_weights != 2 as libc::c_int {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            (*ann).total_weights,
            2 as libc::c_int,
        );
    }
    let mut a: libc::c_double = 0.;
    a= 0 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 11 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.5f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            0.5f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= 10 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(1.0f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            1.0f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    a= -(10 as libc::c_int) as libc::c_double;
    crate::src::test::ltests+= 1;
    if fabs(0.0f64 - *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a))) > 0.001f64 {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            0.0f64,
            *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(a)),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn xor() {
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    (*ann).activation_hidden= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if (*ann).total_weights != 9 as libc::c_int {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_int,
            (*ann).total_weights,
            9 as libc::c_int,
        );
    }
    *(*ann).weight.offset(0 as libc::c_int as isize) = 0.5f64;
    *(*ann).weight
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(2 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(4 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(5 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight.offset(6 as libc::c_int as isize) = 0.5f64;
    *(*ann).weight
        .offset(7 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
    *(*ann).weight
        .offset(8 as libc::c_int as isize) = -(1 as libc::c_int) as libc::c_double;
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn backprop() {
    let mut ann = crate::src::genann::genann_init(
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut input: libc::c_double = 0.;
    let mut output: libc::c_double = 0.;
    input= 0.5f64;
    output= 1 as libc::c_int as libc::c_double;
    let mut first_try = *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(input));
    crate::src::genann::genann_train(ann, core::ptr::addr_of!(input), core::ptr::addr_of!(output), 0.5f64);
    let mut second_try = *crate::src::genann::genann_run(ann.as_mut(), core::ptr::addr_of!(input));
    crate::src::test::ltests+= 1;
    if !(fabs(first_try - output) > fabs(second_try - output)) {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_and() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_or() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    crate::src::genann::genann_randomize(ann.as_mut());
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 50 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                0.8f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn train_xor() {
    let mut input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let mut output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 500 as libc::c_int {
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            crate::src::genann::genann_train(
                ann,
                input[j as usize].as_mut_ptr(),
                output.as_mut_ptr().offset(j as isize),
                3 as libc::c_int as libc::c_double,
            );
            j+= 1;
        }
        i+= 1;
    }
    (*ann).activation_output= Some(
        crate::src::genann::genann_act_threshold as unsafe extern "C" fn(libc::c_double) -> libc::c_double,
    );
    crate::src::test::ltests+= 1;
    if fabs(
        output[0 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            output[0 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[0 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[1 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            output[1 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[1 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[2 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            output[2 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[2 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::test::ltests+= 1;
    if fabs(
        output[3 as libc::c_int as usize]
            - *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
    ) > 0.001f64
    {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            output[3 as libc::c_int as usize],
            *crate::src::genann::genann_run(ann.as_mut(), input[3 as libc::c_int as usize].as_mut_ptr()),
        );
    }
    crate::src::genann::genann_free(ann);
}
#[no_mangle]
pub unsafe extern "C" fn persist() {
    let mut first = crate::src::genann::genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut out = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    crate::src::genann::genann_write(first as *const crate::src::example1::genann, out);
    fclose(out);
    let mut in_0 = fopen(
        b"persist.txt\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut second = crate::src::genann::genann_read(in_0);
    fclose(out);
    crate::src::test::ltests+= 1;
    if (*first).inputs != (*second).inputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden != (*second).hidden {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).outputs != (*second).outputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).total_weights != (*second).total_weights {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*first).total_weights {
        crate::src::test::ltests+= 1;
        if !(*(*first).weight.offset(i as isize)
            == *(*second).weight.offset(i as isize))
        {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d error \n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                216 as libc::c_int,
            );
        }
        i+= 1;
    }
    crate::src::genann::genann_free(first);
    crate::src::genann::genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn copy() {
    let mut first = crate::src::genann::genann_init(
        1000 as libc::c_int,
        5 as libc::c_int,
        50 as libc::c_int,
        10 as libc::c_int,
    );
    let mut second = crate::src::genann::genann_copy(first as *const crate::src::example1::genann);
    crate::src::test::ltests+= 1;
    if (*first).inputs != (*second).inputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
            (*first).inputs,
            (*second).inputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden_layers != (*second).hidden_layers {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            (*first).hidden_layers,
            (*second).hidden_layers,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).hidden != (*second).hidden {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            (*first).hidden,
            (*second).hidden,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).outputs != (*second).outputs {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            (*first).outputs,
            (*second).outputs,
        );
    }
    crate::src::test::ltests+= 1;
    if (*first).total_weights != (*second).total_weights {
        crate::src::test::lfails+= 1;
        printf(
            b"%s:%d (%d != %d)\n\0" as *const u8 as *const libc::c_char,
            b"test.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int,
            (*first).total_weights,
            (*second).total_weights,
        );
    }
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < (*first).total_weights {
        crate::src::test::ltests+= 1;
        if fabs(
            *(*first).weight.offset(i as isize)
                - *(*second).weight.offset(i as isize),
        ) > 0.001f64
        {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int,
                *(*first).weight.offset(i as isize),
                *(*second).weight.offset(i as isize),
            );
        }
        i+= 1;
    }
    crate::src::genann::genann_free(first);
    crate::src::genann::genann_free(second);
}
#[no_mangle]
pub unsafe extern "C" fn sigmoid() {
    let mut i = -(20 as libc::c_int) as libc::c_double;
    let max = 20 as libc::c_int as libc::c_double;
    let d = 0.0001f64;
    while i < max {
        crate::src::test::ltests+= 1;
        if fabs(crate::src::genann::genann_act_sigmoid(i) - crate::src::genann::genann_act_sigmoid_cached(i)) > 0.001f64 {
            crate::src::test::lfails+= 1;
            printf(
                b"%s:%d (%f != %f)\n\0" as *const u8 as *const libc::c_char,
                b"test.c\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int,
                crate::src::genann::genann_act_sigmoid(i),
                crate::src::genann::genann_act_sigmoid_cached(i),
            );
        }
        i+= d;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN TEST SUITE\n\0" as *const u8 as *const libc::c_char);
    srand(100 as libc::c_int as libc::c_uint);
    let ts = crate::src::test::ltests;
    let fs = crate::src::test::lfails;
    let start = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"basic\0" as *const u8 as *const libc::c_char,
    );
    basic();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts - (crate::src::test::lfails - fs),
        crate::src::test::lfails - fs,
        ((clock() - start) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_0 = crate::src::test::ltests;
    let fs_0 = crate::src::test::lfails;
    let start_0 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"xor\0" as *const u8 as *const libc::c_char,
    );
    xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_0 - (crate::src::test::lfails - fs_0),
        crate::src::test::lfails - fs_0,
        ((clock() - start_0) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_1 = crate::src::test::ltests;
    let fs_1 = crate::src::test::lfails;
    let start_1 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"backprop\0" as *const u8 as *const libc::c_char,
    );
    backprop();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_1 - (crate::src::test::lfails - fs_1),
        crate::src::test::lfails - fs_1,
        ((clock() - start_1) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_2 = crate::src::test::ltests;
    let fs_2 = crate::src::test::lfails;
    let start_2 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train and\0" as *const u8 as *const libc::c_char,
    );
    train_and();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_2 - (crate::src::test::lfails - fs_2),
        crate::src::test::lfails - fs_2,
        ((clock() - start_2) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_3 = crate::src::test::ltests;
    let fs_3 = crate::src::test::lfails;
    let start_3 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train or\0" as *const u8 as *const libc::c_char,
    );
    train_or();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_3 - (crate::src::test::lfails - fs_3),
        crate::src::test::lfails - fs_3,
        ((clock() - start_3) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_4 = crate::src::test::ltests;
    let fs_4 = crate::src::test::lfails;
    let start_4 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"train xor\0" as *const u8 as *const libc::c_char,
    );
    train_xor();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_4 - (crate::src::test::lfails - fs_4),
        crate::src::test::lfails - fs_4,
        ((clock() - start_4) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_5 = crate::src::test::ltests;
    let fs_5 = crate::src::test::lfails;
    let start_5 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"persist\0" as *const u8 as *const libc::c_char,
    );
    persist();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_5 - (crate::src::test::lfails - fs_5),
        crate::src::test::lfails - fs_5,
        ((clock() - start_5) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_6 = crate::src::test::ltests;
    let fs_6 = crate::src::test::lfails;
    let start_6 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"copy\0" as *const u8 as *const libc::c_char,
    );
    copy();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_6 - (crate::src::test::lfails - fs_6),
        crate::src::test::lfails - fs_6,
        ((clock() - start_6) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    let ts_7 = crate::src::test::ltests;
    let fs_7 = crate::src::test::lfails;
    let start_7 = clock();
    printf(
        b"\t%-14s\0" as *const u8 as *const libc::c_char,
        b"sigmoid\0" as *const u8 as *const libc::c_char,
    );
    sigmoid();
    printf(
        b"pass:%2d   fail:%2d   %4dms\n\0" as *const u8 as *const libc::c_char,
        crate::src::test::ltests - ts_7 - (crate::src::test::lfails - fs_7),
        crate::src::test::lfails - fs_7,
        ((clock() - start_7) * 1000 as libc::c_int as libc::c_long
            / 1000000 as libc::c_int as __clock_t) as libc::c_int,
    );
    if crate::src::test::lfails == 0 as libc::c_int {
        printf(
            b"ALL TESTS PASSED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            crate::src::test::ltests,
            crate::src::test::ltests,
        );
    } else {
        printf(
            b"SOME TESTS FAILED (%d/%d)\n\0" as *const u8 as *const libc::c_char,
            crate::src::test::ltests - crate::src::test::lfails,
            crate::src::test::ltests,
        );
    }
    return (crate::src::test::lfails != 0 as libc::c_int) as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
