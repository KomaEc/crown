extern "C" {
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn rand() -> std::os::raw::c_int;
    #[no_mangle]
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;

}
pub type genann_actfun =
    Option<unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0;
impl Default for ErasedByPreprocessor0 {
    fn default() -> Self {
        Self {}
    }
}

unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *const *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    printf(b"GENANN example 2.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(
        b"Train a small ANN to the XOR function using random search.\n\x00" as *const u8
            as *const std::os::raw::c_char,
    );
    /* Input and expected out data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] = [
        [
            0 as std::os::raw::c_int as std::os::raw::c_double,
            0 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            0 as std::os::raw::c_int as std::os::raw::c_double,
            1 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            1 as std::os::raw::c_int as std::os::raw::c_double,
            0 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            1 as std::os::raw::c_int as std::os::raw::c_double,
            1 as std::os::raw::c_int as std::os::raw::c_double,
        ],
    ];
    let output: [std::os::raw::c_double; 4] = [
        0 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        1 as std::os::raw::c_int as std::os::raw::c_double,
        0 as std::os::raw::c_int as std::os::raw::c_double,
    ];
    let mut i: std::os::raw::c_int = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: *mut crate::example1::genann = crate::genann::genann_init(
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
        2 as std::os::raw::c_int,
        1 as std::os::raw::c_int,
    );
    let mut err: std::os::raw::c_double = 0.;
    let mut last_err: std::os::raw::c_double =
        1000 as std::os::raw::c_int as std::os::raw::c_double;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop {
        count += 1;
        if count % 1000 as std::os::raw::c_int == 0 as std::os::raw::c_int {
            /* We're stuck, start over. */
            crate::genann::genann_randomize(ann.as_mut());
        }
        let mut save: *mut crate::example1::genann = crate::genann::genann_copy(ann);
        /* Take a random guess at the ANN weights. */
        i = 0 as std::os::raw::c_int;
        while i < (*ann).total_weights {
            *(*ann).weight.offset(i as isize) += rand() as std::os::raw::c_double
                / 0x7fffffff as std::os::raw::c_int as std::os::raw::c_double
                - 0.5f64;
            i += 1
        }
        /* See how we did. */
        err = 0 as std::os::raw::c_int as std::os::raw::c_double;
        err += pow(
            *crate::genann::genann_run(
                ann.as_mut(),
                input[0 as std::os::raw::c_int as usize].as_ptr(),
            ) - output[0 as std::os::raw::c_int as usize],
            2.0f64,
        );
        err += pow(
            *crate::genann::genann_run(
                ann.as_mut(),
                input[1 as std::os::raw::c_int as usize].as_ptr(),
            ) - output[1 as std::os::raw::c_int as usize],
            2.0f64,
        );
        err += pow(
            *crate::genann::genann_run(
                ann.as_mut(),
                input[2 as std::os::raw::c_int as usize].as_ptr(),
            ) - output[2 as std::os::raw::c_int as usize],
            2.0f64,
        );
        err += pow(
            *crate::genann::genann_run(
                ann.as_mut(),
                input[3 as std::os::raw::c_int as usize].as_ptr(),
            ) - output[3 as std::os::raw::c_int as usize],
            2.0f64,
        );
        /* Keep these weights if they're an improvement. */
        if err < last_err {
            crate::genann::genann_free(Some(Box::from_raw(save)));
            last_err = err
        } else {
            crate::genann::genann_free(Some(Box::from_raw(ann)));
            ann = save
        }
        if !(err > 0.01f64) {
            break;
        }
    }
    printf(
        b"Finished in %d loops.\n\x00" as *const u8 as *const std::os::raw::c_char,
        count,
    );
    /* Run the network and see what it predicts. */
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[0 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[1 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[2 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[3 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    crate::genann::genann_free(Some(Box::from_raw(ann)));
    return 0 as std::os::raw::c_int;
}
// pub fn main() {
//     let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
//     };
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
//                                     args.as_mut_ptr() as
//                                         *mut *mut std::os::raw::c_char) as i32)
//     }
// }
