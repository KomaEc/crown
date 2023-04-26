
extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    static mut ti_indicators: [ti_indicator_info; 0];
    
    
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators_index::ti_find_indicator;
pub use crate::example2::ti_indicator_start_function;
pub use crate::example2::ti_indicator_function;
// #[derive(Copy, Clone)]

pub use crate::example2::ti_indicator_info;
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2017 Tulip Charts LLC
 * Lewis Van Winkle (LV@tulipcharts.org)
 *
 * This file is part of Tulip Indicators.
 *
 * Tulip Indicators is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * Tulip Indicators is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Tulip Indicators.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
#[no_mangle]
pub static mut optionsd: [std::os::raw::c_double; 29] =
    [-(20 as std::os::raw::c_int) as std::os::raw::c_double,
     -(1 as std::os::raw::c_int) as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0.1f64, 0.5f64, 0.7f64,
     1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     2.5f64, 3 as std::os::raw::c_int as std::os::raw::c_double,
     4 as std::os::raw::c_int as std::os::raw::c_double, 5 as std::os::raw::c_int as std::os::raw::c_double,
     6 as std::os::raw::c_int as std::os::raw::c_double, 7 as std::os::raw::c_int as std::os::raw::c_double,
     8 as std::os::raw::c_int as std::os::raw::c_double, 9 as std::os::raw::c_int as std::os::raw::c_double,
     10 as std::os::raw::c_int as std::os::raw::c_double, 11 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double, 20 as std::os::raw::c_int as std::os::raw::c_double,
     21 as std::os::raw::c_int as std::os::raw::c_double, 22 as std::os::raw::c_int as std::os::raw::c_double,
     23 as std::os::raw::c_int as std::os::raw::c_double, 24 as std::os::raw::c_int as std::os::raw::c_double,
     25 as std::os::raw::c_int as std::os::raw::c_double, 26 as std::os::raw::c_int as std::os::raw::c_double,
     100 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_in: [std::os::raw::c_double; 20] =
    [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_in0: [std::os::raw::c_double; 20] =
    [0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_ot: [std::os::raw::c_double; 20] =
    [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub unsafe extern "C" fn banner() {
    printf(b"  ______ _    _ __________________ _____  \n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" |  ____| |  | |___  /___  /  ____|  __ \\ \n\x00" as *const u8
               as *const std::os::raw::c_char);
    printf(b" | |__  | |  | |  / /   / /| |__  | |__) |\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" |  __| | |  | | / /   / / |  __| |  _  / \n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" | |    | |__| |/ /__ / /__| |____| | \\ \\ \n\x00" as *const u8
               as *const std::os::raw::c_char);
    printf(b" |_|     \\____//_____/_____|______|_|  \\_\\\n\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    printf(b"This program tries each indicator with a lot of options.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    printf(b"It\'s good for checking for crashes or assertion failures.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    printf(b"Use with a memory tool to check for memory errors in ti.\n\n\x00"
               as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn check_output(mut info: *const ti_indicator_info,
                                      mut size: std::os::raw::c_int,
                                      mut inputs:
                                          *const *const std::os::raw::c_double,
                                      mut options: *const std::os::raw::c_double,
                                      mut outputs:
                                          *const *mut std::os::raw::c_double) {
    let mut s: std::os::raw::c_int = 0;
    s = (*info).start.expect("non-null function pointer")(options);
    let mut o: std::os::raw::c_int = 0;
    o = 0 as std::os::raw::c_int;
    while o < (*info).outputs {
        let mut max: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut min: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size - s {
            let op: std::os::raw::c_double =
                *(*outputs.offset(o as isize)).offset(i as isize);
            let in_0: std::os::raw::c_double =
                *(*inputs.offset(0 as std::os::raw::c_int as
                                     isize)).offset((i + s) as isize);
            max = if in_0 > max { in_0 } else { max };
            min = if in_0 < min { in_0 } else { min };
            match (*info).type_0 {
                1 => {
                    if op > max * 1.5f64 + 2 as std::os::raw::c_int as std::os::raw::c_double
                           ||
                           op <
                               min * 0.5f64 -
                                   2 as std::os::raw::c_int as std::os::raw::c_double {
                        let mut k: std::os::raw::c_int = 0;
                        let mut j: std::os::raw::c_int = 0;
                        printf(b"\nInputs:\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                        k = 0 as std::os::raw::c_int;
                        while k < size {
                            printf(b" %f\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   *(*inputs.offset(0 as std::os::raw::c_int as
                                                        isize)).offset(k as
                                                                           isize));
                            k += 1
                        }
                        printf(b"\nOptions:\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                        k = 0 as std::os::raw::c_int;
                        while k < (*info).options {
                            printf(b" %f\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   *options.offset(k as isize));
                            k += 1
                        }
                        j = 0 as std::os::raw::c_int;
                        while j < (*info).outputs {
                            printf(b"\nOutput[%d]:\x00" as *const u8 as
                                       *const std::os::raw::c_char, j);
                            k = 0 as std::os::raw::c_int;
                            while k < size {
                                printf(b" %f\x00" as *const u8 as
                                           *const std::os::raw::c_char,
                                       *(*outputs.offset(j as
                                                             isize)).offset(k
                                                                                as
                                                                                isize));
                                k += 1
                            }
                            j += 1
                        }
                        printf(b"\nERROR Output is out of range for input: input: %f output: %f\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, in_0,
                               op);
                        if (0 as std::os::raw::c_int == 0) as std::os::raw::c_int as
                               std::os::raw::c_long != 0 {
                            __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                                   &[std::os::raw::c_char; 13]>(b"check_output\x00")).as_ptr(),
                                         b"fuzzer.c\x00" as *const u8 as
                                             *const std::os::raw::c_char,
                                         94 as std::os::raw::c_int,
                                         b"0\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        } else { };
                    }
                }
                _ => { }
            }
            i += 1
        }
        o += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn stress(mut info: *const ti_indicator_info) {
    let opt_count: std::os::raw::c_int = (*info).options;
    printf(b"%s (%s) (%d options)\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*info).name, (*info).full_name, opt_count);
    let mut inputs: [*const std::os::raw::c_double; 10] =
        [0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double];
    let mut inputs0: [*const std::os::raw::c_double; 10] =
        [0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double];
    let mut outputs: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        inputs[i as usize] =
            if i < (*info).inputs {
                dummy_in.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        inputs0[i as usize] =
            if i < (*info).inputs {
                dummy_in0.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        outputs[i as usize] =
            if i < (*info).outputs {
                dummy_ot.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        i += 1
    }
    let mut options_index: [std::os::raw::c_int; 11] =
        [0 as std::os::raw::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut options: [std::os::raw::c_double; 10] = [0.; 10];
    let mut choices: std::os::raw::c_int =
        (::std::mem::size_of::<[std::os::raw::c_double; 29]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut size: std::os::raw::c_int =
        (::std::mem::size_of::<[std::os::raw::c_double; 20]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut j: std::os::raw::c_int = 0;
    let mut count: std::os::raw::c_int = 0 as std::os::raw::c_int;
    loop  {
        count += 1;
        j = 0 as std::os::raw::c_int;
        while j < opt_count {
            options[j as usize] =
                optionsd[options_index[j as usize] as usize];
            printf(b" %f\x00" as *const u8 as *const std::os::raw::c_char,
                   options[j as usize]);
            j += 1
        }
        let mut r: std::os::raw::c_int = 0;
        printf(b" 0\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(0 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 0 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 1\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(1 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 1 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 2\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(2 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 2 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 3\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(3 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 3 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" %d\x00" as *const u8 as *const std::os::raw::c_char, size);
        r =
            (*info).indicator.expect("non-null function pointer")(size,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, size, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 0s\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(size,
                                                                  inputs0.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, size, inputs0.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b"\r                                                  \r\x00"
                   as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        loop  {
            options_index[j as usize] =
                (options_index[j as usize] + 1 as std::os::raw::c_int) % choices;
            let fresh0 = j;
            j = j + 1;
            if !(options_index[fresh0 as usize] == 0 as std::os::raw::c_int) {
                break ;
            }
        }
        if !(j <= opt_count) { break ; }
    }
    if !(0.1f64 >
             fabs(count as std::os::raw::c_double -
                      pow(choices as std::os::raw::c_double,
                          opt_count as std::os::raw::c_double))) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"stress\x00")).as_ptr(),
                     b"fuzzer.c\x00" as *const u8 as *const std::os::raw::c_char,
                     154 as std::os::raw::c_int,
                     b".1 > fabs(count - (pow(choices, opt_count)))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    banner();
    if argc > 1 as std::os::raw::c_int {
        let mut info: *const ti_indicator_info =
            ti_find_indicator(*argv.offset(1 as std::os::raw::c_int as isize));
        if !info.is_null() { stress(info); }
    } else {
        let mut info_0: *const ti_indicator_info = ti_indicators.as_mut_ptr();
        loop  {
            stress(info_0);
            info_0 = info_0.offset(1);
            if (*info_0).name.is_null() { break ; }
        }
    }
    printf(b"\r                                                  \rDone\n\x00"
               as *const u8 as *const std::os::raw::c_char);
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

