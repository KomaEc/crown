use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    
    
    
    
}
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: libc::c_int,
    pub hidden_layers: libc::c_int,
    pub hidden: libc::c_int,
    pub outputs: libc::c_int,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: libc::c_int,
    pub total_neurons: libc::c_int,
    pub weight: *mut libc::c_double,
    pub output: *mut libc::c_double,
    pub delta: *mut libc::c_double,
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 1.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Train a small ANN to the XOR function using backpropagation.\n\0" as *const u8
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
    i= 0 as libc::c_int;
    while i < 300 as libc::c_int {
        crate::src::genann::genann_train(
            ann,
            (input[0 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(0 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[1 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(1 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[2 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(2 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        crate::src::genann::genann_train(
            ann,
            (input[3 as libc::c_int as usize]).as_ptr(),
            output.as_ptr().offset(3 as libc::c_int as isize),
            3 as libc::c_int as libc::c_double,
        );
        i+= 1;
    }
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
