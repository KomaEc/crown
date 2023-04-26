
extern "C" {
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    static mut ti_indicators: [ti_indicator_info; 0];
    
    
}
pub use crate::indicators_index::ti_find_indicator;
pub use crate::example2::ti_indicator_start_function;
pub use crate::example2::ti_indicator_function;
// #[derive(Copy, Clone)]

pub use crate::example2::ti_indicator_info;
#[no_mangle]
pub static mut out: [[std::os::raw::c_double; 15]; 5] = [[0.; 15]; 5];
/* Example data on IBM. */
#[no_mangle]
pub static mut datet: [*const std::os::raw::c_char; 15] =
    [b"2005-11-01\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-02\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-03\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-04\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-07\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-08\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-09\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-10\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-11\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-14\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-15\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-16\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-17\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-18\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-21\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub static mut date: [std::os::raw::c_double; 15] =
    [51101 as std::os::raw::c_int as std::os::raw::c_double,
     51102 as std::os::raw::c_int as std::os::raw::c_double,
     51103 as std::os::raw::c_int as std::os::raw::c_double,
     51104 as std::os::raw::c_int as std::os::raw::c_double,
     51107 as std::os::raw::c_int as std::os::raw::c_double,
     51108 as std::os::raw::c_int as std::os::raw::c_double,
     51109 as std::os::raw::c_int as std::os::raw::c_double,
     51110 as std::os::raw::c_int as std::os::raw::c_double,
     51111 as std::os::raw::c_int as std::os::raw::c_double,
     51114 as std::os::raw::c_int as std::os::raw::c_double,
     51115 as std::os::raw::c_int as std::os::raw::c_double,
     51116 as std::os::raw::c_int as std::os::raw::c_double,
     51117 as std::os::raw::c_int as std::os::raw::c_double,
     51118 as std::os::raw::c_int as std::os::raw::c_double,
     51121 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut open: [std::os::raw::c_double; 15] =
    [81.85f64, 81.2f64, 81.55f64, 82.91f64, 83.1f64, 83.41f64, 82.71f64,
     82.7f64, 84.2f64, 84.25f64, 84.03f64, 85.45f64, 86.18f64,
     88 as std::os::raw::c_int as std::os::raw::c_double, 87.6f64];
#[no_mangle]
pub static mut high: [std::os::raw::c_double; 15] =
    [82.15f64, 81.89f64, 83.03f64, 83.3f64, 83.85f64, 83.9f64, 83.33f64,
     84.3f64, 84.84f64, 85 as std::os::raw::c_int as std::os::raw::c_double, 85.9f64,
     86.58f64, 86.98f64, 88 as std::os::raw::c_int as std::os::raw::c_double, 87.87f64];
#[no_mangle]
pub static mut low: [std::os::raw::c_double; 15] =
    [81.29f64, 80.64f64, 81.31f64, 82.65f64, 83.07f64, 83.11f64, 82.49f64,
     82.3f64, 84.15f64, 84.11f64, 84.03f64, 85.39f64, 85.76f64, 87.17f64,
     87.01f64];
#[no_mangle]
pub static mut close: [std::os::raw::c_double; 15] =
    [81.59f64, 81.06f64, 82.87f64, 83 as std::os::raw::c_int as std::os::raw::c_double,
     83.61f64, 83.15f64, 82.84f64, 83.99f64, 84.55f64, 84.36f64, 85.53f64,
     86.54f64, 86.89f64, 87.77f64, 87.29f64];
#[no_mangle]
pub static mut volume: [std::os::raw::c_double; 15] =
    [5653100 as std::os::raw::c_int as std::os::raw::c_double,
     6447400 as std::os::raw::c_int as std::os::raw::c_double,
     7690900 as std::os::raw::c_int as std::os::raw::c_double,
     3831400 as std::os::raw::c_int as std::os::raw::c_double,
     4455100 as std::os::raw::c_int as std::os::raw::c_double,
     3798000 as std::os::raw::c_int as std::os::raw::c_double,
     3936200 as std::os::raw::c_int as std::os::raw::c_double,
     4732000 as std::os::raw::c_int as std::os::raw::c_double,
     4841300 as std::os::raw::c_int as std::os::raw::c_double,
     3915300 as std::os::raw::c_int as std::os::raw::c_double,
     6830800 as std::os::raw::c_int as std::os::raw::c_double,
     6694100 as std::os::raw::c_int as std::os::raw::c_double,
     5293600 as std::os::raw::c_int as std::os::raw::c_double,
     7985800 as std::os::raw::c_int as std::os::raw::c_double,
     4807900 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut alternative: [std::os::raw::c_double; 15] =
    [0.2f64, 0.3f64, 0.4f64, 0.3f64, 0.5f64, 0.7f64, 0.75f64, 0.9f64, 0.9f64,
     1 as std::os::raw::c_int as std::os::raw::c_double, 1 as std::os::raw::c_int as std::os::raw::c_double,
     0.2f64, 0.1f64, -0.1f64, -0.5f64];
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
    if argc < 2 as std::os::raw::c_int {
        printf(b"No indicator given.\n\x00" as *const u8 as
                   *const std::os::raw::c_char);
        printf(b"Example:\n\x00" as *const u8 as *const std::os::raw::c_char);
        printf(b"\tsample ma 5\x00" as *const u8 as *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int
    }
    if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
              b"--version\x00" as *const u8 as *const std::os::raw::c_char) ==
           0 as std::os::raw::c_int {
        printf(b"TI VERSION: %s, TI BUILD: %d\n\x00" as *const u8 as
                   *const std::os::raw::c_char,
               b"0.8.4\x00" as *const u8 as *const std::os::raw::c_char,
               1537377628 as std::os::raw::c_int);
        return 0 as std::os::raw::c_int
    }
    if strcmp(*argv.offset(1 as std::os::raw::c_int as isize),
              b"--list\x00" as *const u8 as *const std::os::raw::c_char) ==
           0 as std::os::raw::c_int {
        loop  {
            if (*info).type_0 == 1 as std::os::raw::c_int {
                printf(b"type overlay \x00" as *const u8 as
                           *const std::os::raw::c_char);
            } else if (*info).type_0 == 2 as std::os::raw::c_int {
                printf(b"type indicator \x00" as *const u8 as
                           *const std::os::raw::c_char);
            } else if (*info).type_0 == 3 as std::os::raw::c_int {
                printf(b"type math \x00" as *const u8 as *const std::os::raw::c_char);
            } else if (*info).type_0 == 4 as std::os::raw::c_int {
                printf(b"type simple \x00" as *const u8 as
                           *const std::os::raw::c_char);
            } else {
                printf(b"type unknown \x00" as *const u8 as
                           *const std::os::raw::c_char);
            }
            printf(b"name %s \x00" as *const u8 as *const std::os::raw::c_char,
                   (*info).name);
            printf(b"full_name {%s} \x00" as *const u8 as *const std::os::raw::c_char,
                   (*info).full_name);
            let mut i: std::os::raw::c_int = 0;
            printf(b"inputs {\x00" as *const u8 as *const std::os::raw::c_char);
            i = 0 as std::os::raw::c_int;
            while i < (*info).inputs {
                printf(b"%s%s\x00" as *const u8 as *const std::os::raw::c_char,
                       if i != 0 {
                           b" \x00" as *const u8 as *const std::os::raw::c_char
                       } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                       (*info).input_names[i as usize]);
                i += 1
            }
            printf(b"} \x00" as *const u8 as *const std::os::raw::c_char);
            printf(b"options {\x00" as *const u8 as *const std::os::raw::c_char);
            i = 0 as std::os::raw::c_int;
            while i < (*info).options {
                printf(b"%s{%s}\x00" as *const u8 as *const std::os::raw::c_char,
                       if i != 0 {
                           b" \x00" as *const u8 as *const std::os::raw::c_char
                       } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                       (*info).option_names[i as usize]);
                i += 1
            }
            printf(b"} \x00" as *const u8 as *const std::os::raw::c_char);
            printf(b"outputs {\x00" as *const u8 as *const std::os::raw::c_char);
            i = 0 as std::os::raw::c_int;
            while i < (*info).outputs {
                printf(b"%s{%s}\x00" as *const u8 as *const std::os::raw::c_char,
                       if i != 0 {
                           b" \x00" as *const u8 as *const std::os::raw::c_char
                       } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                       (*info).output_names[i as usize]);
                i += 1
            }
            printf(b"}\x00" as *const u8 as *const std::os::raw::c_char);
            printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            info = info.offset(1);
            if (*info).name.is_null() { break ; }
        }
        return 0 as std::os::raw::c_int
    }
    info = ti_find_indicator(*argv.offset(1 as std::os::raw::c_int as isize));
    if info.is_null() {
        printf(b"Couldn\'t find indicator: %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char,
               *argv.offset(1 as std::os::raw::c_int as isize));
        return 1 as std::os::raw::c_int
    }
    let mut inputs: [*const std::os::raw::c_double; 5] =
        [0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double];
    let mut outputs: [*mut std::os::raw::c_double; 5] =
        [out[0 as std::os::raw::c_int as usize].as_mut_ptr(),
         out[1 as std::os::raw::c_int as usize].as_mut_ptr(),
         out[2 as std::os::raw::c_int as usize].as_mut_ptr(),
         out[3 as std::os::raw::c_int as usize].as_mut_ptr(),
         out[4 as std::os::raw::c_int as usize].as_mut_ptr()];
    let mut o: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut h: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut l: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut r: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut r2: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut v: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut a: std::os::raw::c_int = 0 as std::os::raw::c_int;
    /* Setup the inputs. */
    let mut j: std::os::raw::c_int = 0;
    j = 0 as std::os::raw::c_int;
    while j < (*info).inputs {
        if strcmp((*info).input_names[j as usize],
                  b"open\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            inputs[j as usize] = open.as_mut_ptr();
            o = 1 as std::os::raw::c_int
        } else if strcmp((*info).input_names[j as usize],
                         b"high\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            inputs[j as usize] = high.as_mut_ptr();
            h = 1 as std::os::raw::c_int
        } else if strcmp((*info).input_names[j as usize],
                         b"low\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            inputs[j as usize] = low.as_mut_ptr();
            l = 1 as std::os::raw::c_int
        } else if strcmp((*info).input_names[j as usize],
                         b"close\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            inputs[j as usize] = close.as_mut_ptr();
            c = 1 as std::os::raw::c_int
        } else if strcmp((*info).input_names[j as usize],
                         b"volume\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            inputs[j as usize] = volume.as_mut_ptr();
            v = 1 as std::os::raw::c_int
        } else if strcmp((*info).input_names[j as usize],
                         b"real\x00" as *const u8 as *const std::os::raw::c_char) ==
                      0 as std::os::raw::c_int {
            if r == 0 {
                inputs[j as usize] = close.as_mut_ptr();
                r = 1 as std::os::raw::c_int
            } else {
                inputs[j as usize] = open.as_mut_ptr();
                r2 = 1 as std::os::raw::c_int
            }
        } else {
            if (0 as std::os::raw::c_int == 0) as std::os::raw::c_int as std::os::raw::c_long != 0 {
                __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                                       &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                             b"sample.c\x00" as *const u8 as
                                 *const std::os::raw::c_char, 144 as std::os::raw::c_int,
                             b"0\x00" as *const u8 as *const std::os::raw::c_char);
            } else { };
        }
        j += 1
    }
    /* See if we need alternative inputs (for the indicators that can't take large numbers. */
    let mut alts: [*const std::os::raw::c_char; 8] =
        [b"acos\x00" as *const u8 as *const std::os::raw::c_char,
         b"asin\x00" as *const u8 as *const std::os::raw::c_char,
         b"atan\x00" as *const u8 as *const std::os::raw::c_char,
         b"cosh\x00" as *const u8 as *const std::os::raw::c_char,
         b"sinh\x00" as *const u8 as *const std::os::raw::c_char,
         b"tanh\x00" as *const u8 as *const std::os::raw::c_char,
         b"todeg\x00" as *const u8 as *const std::os::raw::c_char,
         0 as *const std::os::raw::c_char];
    let mut alt: *mut *const std::os::raw::c_char = alts.as_mut_ptr();
    while !(*alt).is_null() {
        if strcmp(*alt, (*info).name) == 0 as std::os::raw::c_int {
            r = 0 as std::os::raw::c_int;
            a = 1 as std::os::raw::c_int;
            j = 0 as std::os::raw::c_int;
            while j < (*info).inputs {
                inputs[j as usize] = alternative.as_mut_ptr();
                j += 1
            }
            break ;
        } else { alt = alt.offset(1) }
    }
    /* Set options, save offset. */
    let mut options: [std::os::raw::c_double; 10] = [0.; 10];
    let mut i_0: std::os::raw::c_int = 0;
    i_0 = 0 as std::os::raw::c_int;
    while i_0 < (*info).options {
        if argc < 3 as std::os::raw::c_int + i_0 {
            printf(b"*ERROR NOT ENOUGH OPTIONS*\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
            return 1 as std::os::raw::c_int
        }
        options[i_0 as usize] =
            atof(*argv.offset((2 as std::os::raw::c_int + i_0) as isize));
        i_0 += 1
    }
    let mut start: std::os::raw::c_int =
        (*info).start.expect("non-null function pointer")(options.as_mut_ptr());
    /* Run it. */
    let ret: std::os::raw::c_int =
        (*info).indicator.expect("non-null function pointer")(15 as
                                                                  std::os::raw::c_int,
                                                              inputs.as_mut_ptr(),
                                                              options.as_mut_ptr(),
                                                              outputs.as_mut_ptr());
    if ret == 0 as std::os::raw::c_int {
        let mut i_1: std::os::raw::c_int = 0;
        let mut k: std::os::raw::c_int = 0;
        let mut bad: std::os::raw::c_int = 0 as std::os::raw::c_int;
        printf(b"date        \x00" as *const u8 as *const std::os::raw::c_char);
        if o != 0 {
            printf(b" open   \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if h != 0 {
            printf(b" high   \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if l != 0 {
            printf(b" low    \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if c != 0 {
            printf(b" close  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if v != 0 {
            printf(b" volume \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if r != 0 {
            printf(b" input  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if r2 != 0 {
            printf(b" input2  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if a != 0 {
            printf(b" input  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        i_1 = 0 as std::os::raw::c_int;
        while i_1 < (*info).outputs {
            printf(b" %s\x00" as *const u8 as *const std::os::raw::c_char,
                   (*info).output_names[i_1 as usize]);
            i_1 += 1
        }
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        i_1 = 0 as std::os::raw::c_int;
        while i_1 < 15 as std::os::raw::c_int {
            printf(b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                   datet[i_1 as usize]);
            if o != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       open[i_1 as usize]);
            }
            if h != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       high[i_1 as usize]);
            }
            if l != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       low[i_1 as usize]);
            }
            if c != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       close[i_1 as usize]);
            }
            if v != 0 {
                printf(b" %8.0f\x00" as *const u8 as *const std::os::raw::c_char,
                       volume[i_1 as usize]);
            }
            if r != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       close[i_1 as usize]);
            }
            if r2 != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       open[i_1 as usize]);
            }
            if a != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       alternative[i_1 as usize]);
            }
            if i_1 >= start {
                k = 0 as std::os::raw::c_int;
                while k < (*info).outputs {
                    if out[k as usize][(i_1 - start) as usize] !=
                           out[k as usize][(i_1 - start) as usize] {
                        bad = 1 as std::os::raw::c_int
                    }
                    printf(b" %8.3f\x00" as *const u8 as *const std::os::raw::c_char,
                           out[k as usize][(i_1 - start) as usize]);
                    k += 1
                }
            }
            printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            i_1 += 1
        }
        if bad != 0 {
            printf(b"\nERROR NaN in outputs (%s).\n\x00" as *const u8 as
                       *const std::os::raw::c_char, (*info).name);
            return 1 as std::os::raw::c_int
        }
        return 0 as std::os::raw::c_int
    } else {
        if ret == 1 as std::os::raw::c_int {
            printf(b"*ERROR INVALID OPTION*\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
        } else {
            printf(b"*ERROR*\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return 1 as std::os::raw::c_int
    };
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

