use ::libc;
extern "C" {
    
    pub type _IO_codecvt;
    
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    
    
    
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor2 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::example3::_IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor3 { dummy: () }
#[no_mangle]
pub static mut iris_data: *const libc::c_char = b"example/iris.data\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut input: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
#[no_mangle]
pub static mut class: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
#[no_mangle]
pub static mut samples: libc::c_int = 0;
#[no_mangle]
pub static mut class_names: [*const libc::c_char; 3] = [
    b"Iris-setosa\0" as *const u8 as *const libc::c_char,
    b"Iris-versicolor\0" as *const u8 as *const libc::c_char,
    b"Iris-virginica\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn load_data() {
    let mut in_0 = fopen(
        b"example/iris.data\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if in_0.is_null() {();
        printf(
            b"Could not open file: %s\n\0" as *const u8 as *const libc::c_char,
            crate::src::example4::iris_data,
        );
        exit(1 as libc::c_int);
    }
    let mut line: [libc::c_char; 1024] = [0; 1024];
    while feof(in_0) == 0
        && !(fgets(line.as_mut_ptr(), 1024 as libc::c_int, in_0)).is_null()
    {
        crate::src::example4::samples+= 1;
    }
    fseek(in_0, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    printf(
        b"Loading %d data points from %s\n\0" as *const u8 as *const libc::c_char,
        crate::src::example4::samples,
        crate::src::example4::iris_data,
    );
    crate::src::example4::input= malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(crate::src::example4::samples as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_double;
    crate::src::example4::class= malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(crate::src::example4::samples as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < crate::src::example4::samples {
        let mut p = crate::src::example4::input.offset((i * 4 as libc::c_int) as isize);
        let mut c = crate::src::example4::class.offset((i * 3 as libc::c_int) as isize);
        *c.offset(2 as libc::c_int as isize) = 0.0f64; *c.offset(1 as libc::c_int as isize) = *c.offset(2 as libc::c_int as isize); *c.offset(0 as libc::c_int as isize) = *c.offset(1 as libc::c_int as isize);
        if (fgets(line.as_mut_ptr(), 1024 as libc::c_int, in_0)).is_null() {();
            perror(b"fgets\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        let mut split = strtok(
            line.as_mut_ptr(),
            b",\0" as *const u8 as *const libc::c_char,
        );
        j= 0 as libc::c_int;
        while j < 4 as libc::c_int {
            *p.offset(j as isize) = atof(split);
            split= strtok(
                0 as *mut libc::c_char,
                b",\0" as *const u8 as *const libc::c_char,
            );
            j+= 1;
        }
        *split
            .offset(
                (strlen(split)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        if strcmp(split, crate::src::example4::class_names[0 as libc::c_int as usize]) == 0 as libc::c_int {
            *c.offset(0 as libc::c_int as isize) = 1.0f64;
        } else if strcmp(split, crate::src::example4::class_names[1 as libc::c_int as usize])
            == 0 as libc::c_int
        {
            *c.offset(1 as libc::c_int as isize) = 1.0f64;
        } else if strcmp(split, crate::src::example4::class_names[2 as libc::c_int as usize])
            == 0 as libc::c_int
        {
            *c.offset(2 as libc::c_int as isize) = 1.0f64;
        } else {
            printf(b"Unknown class %s.\n\0" as *const u8 as *const libc::c_char, split);
            exit(1 as libc::c_int);
        }
        i+= 1;
    }
    fclose(in_0);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 4.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train an ANN on the IRIS dataset using backpropagation.\n\0" as *const u8
            as *const libc::c_char,
    );
    load_data();
    let mut ann = crate::src::genann::genann_init(
        4 as libc::c_int,
        1 as libc::c_int,
        4 as libc::c_int,
        3 as libc::c_int,
    );
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut loops = 5000 as libc::c_int;
    printf(
        b"Training for %d loops over data.\n\0" as *const u8 as *const libc::c_char,
        loops,
    );
    i= 0 as libc::c_int;
    while i < loops {
        j= 0 as libc::c_int;
        while j < crate::src::example4::samples {
            crate::src::genann::genann_train(
                ann,
                crate::src::example4::input.offset((j * 4 as libc::c_int) as isize),
                crate::src::example4::class.offset((j * 3 as libc::c_int) as isize),
                0.01f64,
            );
            j+= 1;
        }
        i+= 1;
    }
    let mut correct = 0 as libc::c_int;
    j= 0 as libc::c_int;
    while j < crate::src::example4::samples {
        let mut guess = crate::src::genann::genann_run(ann.as_mut(), crate::src::example4::input.offset((j * 4 as libc::c_int) as isize));
        if *crate::src::example4::class.offset((j * 3 as libc::c_int + 0 as libc::c_int) as isize) == 1.0f64 {
            if *guess.offset(0 as libc::c_int as isize)
                > *guess.offset(1 as libc::c_int as isize)
                && *guess.offset(0 as libc::c_int as isize)
                    > *guess.offset(2 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else if *crate::src::example4::class.offset((j * 3 as libc::c_int + 1 as libc::c_int) as isize)
            == 1.0f64
        {
            if *guess.offset(1 as libc::c_int as isize)
                > *guess.offset(0 as libc::c_int as isize)
                && *guess.offset(1 as libc::c_int as isize)
                    > *guess.offset(2 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else if *crate::src::example4::class.offset((j * 3 as libc::c_int + 2 as libc::c_int) as isize)
            == 1.0f64
        {
            if *guess.offset(2 as libc::c_int as isize)
                > *guess.offset(0 as libc::c_int as isize)
                && *guess.offset(2 as libc::c_int as isize)
                    > *guess.offset(1 as libc::c_int as isize)
            {
                correct+= 1;
            }
        } else {
            printf(b"Logic error.\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        j+= 1;
    }
    printf(
        b"%d/%d correct (%0.1f%%).\n\0" as *const u8 as *const libc::c_char,
        correct,
        crate::src::example4::samples,
        correct as libc::c_double / crate::src::example4::samples as libc::c_double * 100.0f64,
    );
    crate::src::genann::genann_free(ann);
    return 0 as libc::c_int;
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
