use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    
    
    
    
    
}
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 2.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train a small ANN to the XOR function using random search.\n\0" as *const u8
            as *const libc::c_char,
    );
    let input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
    ];
    let output: [libc::c_double; 4] = [
        0 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ];
    let mut i: libc::c_int = 0;
    let mut ann = crate::src::genann::genann_init(
        2 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
    );
    let mut err: libc::c_double = 0.;
    let mut last_err = 1000 as libc::c_int as libc::c_double;
    let mut count = 0 as libc::c_int;
    loop {
        count+= 1;
        if count % 1000 as libc::c_int == 0 as libc::c_int {
            crate::src::genann::genann_randomize(ann.as_mut());
        }
        let mut save = crate::src::genann::genann_copy(ann);
        i= 0 as libc::c_int;
        while i < (*ann).total_weights {
            *(*ann).weight.offset(i as isize)
                += rand() as libc::c_double / 2147483647 as libc::c_int as libc::c_double
                    - 0.5f64;
            i+= 1;
        }
        err= 0 as libc::c_int as libc::c_double;
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr())
                    - output[0 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr())
                    - output[1 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr())
                    - output[2 as libc::c_int as usize],
                2.0f64,
            );
        err+= pow(
                *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr())
                    - output[3 as libc::c_int as usize],
                2.0f64,
            );
        if err < last_err {
            crate::src::genann::genann_free(save);
            last_err= err;
        } else {
            crate::src::genann::genann_free(ann);
            ann= save;
        }
        if !(err > 0.01f64) {
            break;
        }
    }
    printf(b"Finished in %d loops.\n\0" as *const u8 as *const libc::c_char, count);
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[0 as libc::c_int as usize][0 as libc::c_int as usize],
        input[0 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[1 as libc::c_int as usize][0 as libc::c_int as usize],
        input[1 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[2 as libc::c_int as usize][0 as libc::c_int as usize],
        input[2 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[3 as libc::c_int as usize][0 as libc::c_int as usize],
        input[3 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr()),
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
