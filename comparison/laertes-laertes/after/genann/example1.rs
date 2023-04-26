#![feature(main)]

extern "C" {
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_init;
pub use crate::genann::genann_run;
pub use crate::genann::genann_train;
pub type genann_actfun
    =
    Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: std::os::raw::c_int,
    pub hidden_layers: std::os::raw::c_int,
    pub hidden: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub activation_hidden: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub activation_output: Option<unsafe extern "C"  fn(_: std::os::raw::c_double,) -> std::os::raw::c_double>,
    pub total_weights: std::os::raw::c_int,
    pub total_neurons: std::os::raw::c_int,
    pub weight: * mut std::os::raw::c_double,
    pub output: * mut std::os::raw::c_double,
    pub delta: * mut std::os::raw::c_double,
}
impl std::default::Default for genann {
    fn default() -> Self {
        genann {
        inputs: std::os::raw::c_int::default(),
        hidden_layers: std::os::raw::c_int::default(),
        hidden: std::os::raw::c_int::default(),
        outputs: std::os::raw::c_int::default(),
        activation_hidden: None,
        activation_output: None,
        total_weights: std::os::raw::c_int::default(),
        total_neurons: std::os::raw::c_int::default(),
        weight: 0 as * mut std::os::raw::c_double,
        output: 0 as * mut std::os::raw::c_double,
        delta: 0 as * mut std::os::raw::c_double
        }
    }
}

unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    printf(b"GENANN example 1.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(b"Train a small ANN to the XOR function using backpropagation.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    /* Input and expected out data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] =
        [[0 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [0 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          0 as std::os::raw::c_int as std::os::raw::c_double],
         [1 as std::os::raw::c_int as std::os::raw::c_double,
          1 as std::os::raw::c_int as std::os::raw::c_double]];
    let output: [std::os::raw::c_double; 4] =
        [0 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         1 as std::os::raw::c_int as std::os::raw::c_double,
         0 as std::os::raw::c_int as std::os::raw::c_double];
    let mut i: std::os::raw::c_int = 0;
    /* New network with 2 inputs,
     * 1 hidden layer of 2 neurons,
     * and 1 output. */
    let mut ann: *mut genann =
        genann_init(2 as std::os::raw::c_int, 1 as std::os::raw::c_int, 2 as std::os::raw::c_int,
                    1 as std::os::raw::c_int);
    /* Train on the four labeled data points many times. */
    i = 0 as std::os::raw::c_int;
    while i < 300 as std::os::raw::c_int {
        genann_train(ann, input[0 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(0 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[1 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(1 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[2 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(2 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        genann_train(ann, input[3 as std::os::raw::c_int as usize].as_ptr(),
                     output.as_ptr().offset(3 as std::os::raw::c_int as isize),
                     3 as std::os::raw::c_int as std::os::raw::c_double);
        i += 1
    }
    /* Run the network and see what it predicts. */
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[0 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[1 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[2 as std::os::raw::c_int as usize].as_ptr()));
    printf(b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as
               *const std::os::raw::c_char,
           input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
           input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
           *genann_run(ann, input[3 as std::os::raw::c_int as usize].as_ptr()));
    genann_free(ann);
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
pub fn borrow<'a, 'b: 'a, T>(p: &'a Option<&'b mut T>) -> Option<&'a T> {
    p.as_ref().map(|x| &**x)
}

pub fn borrow_mut<'a, 'b : 'a, T>(p: &'a mut Option<&'b mut T>) -> Option<&'a mut T> {
    p.as_mut().map(|x| &mut **x)
}

pub fn owned_as_ref<'a, T>(p: &'a Option<Box<T>>) -> Option<&'a T> {
    p.as_ref().map(|x| x.as_ref())
}

pub fn owned_as_mut<'a, T>(p: &'a mut Option<Box<T>>) -> Option<&'a mut T> {
    p.as_mut().map(|x| x.as_mut())
}

pub fn option_to_raw<T>(p: Option<&T>) -> * const T {
    p.map_or(core::ptr::null(), |p| p as * const T)
}

pub fn _ref_eq<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) == option_to_raw(q)
}

pub fn _ref_ne<T>(p: Option<&T>, q: Option<&T>) -> bool {
    option_to_raw(p) != option_to_raw(q)
}

