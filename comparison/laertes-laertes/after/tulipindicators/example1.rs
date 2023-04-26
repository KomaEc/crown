
extern "C" {
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    
    
    
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators::sma::ti_sma;
pub use crate::indicators::sma::ti_sma_start;
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
pub unsafe extern "C" fn print_array(mut p: *const std::os::raw::c_double,
                                     size: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        if i != 0 { printf(b", \x00" as *const u8 as *const std::os::raw::c_char); }
        printf(b"%.1f\x00" as *const u8 as *const std::os::raw::c_char,
               *p.offset(i as isize));
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
unsafe fn main_0() -> std::os::raw::c_int {
    let data_in: [std::os::raw::c_double; 10] =
        [5 as std::os::raw::c_int as std::os::raw::c_double,
         8 as std::os::raw::c_int as std::os::raw::c_double,
         12 as std::os::raw::c_int as std::os::raw::c_double,
         11 as std::os::raw::c_int as std::os::raw::c_double,
         9 as std::os::raw::c_int as std::os::raw::c_double,
         8 as std::os::raw::c_int as std::os::raw::c_double,
         7 as std::os::raw::c_int as std::os::raw::c_double,
         10 as std::os::raw::c_int as std::os::raw::c_double,
         11 as std::os::raw::c_int as std::os::raw::c_double,
         13 as std::os::raw::c_int as std::os::raw::c_double];
    let input_length: std::os::raw::c_int =
        (::std::mem::size_of::<[std::os::raw::c_double; 10]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    printf(b"We have %d bars of input data.\n\x00" as *const u8 as
               *const std::os::raw::c_char, input_length);
    print_array(data_in.as_ptr(), input_length);
    let options: [std::os::raw::c_double; 1] = [3 as std::os::raw::c_int as std::os::raw::c_double];
    printf(b"Our option array is: \x00" as *const u8 as *const std::os::raw::c_char);
    print_array(options.as_ptr(),
                (::std::mem::size_of::<[std::os::raw::c_double; 1]>() as
                     std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                                     as std::os::raw::c_ulong) as
                    std::os::raw::c_int);
    let start: std::os::raw::c_int = ti_sma_start(options.as_ptr());
    printf(b"The start amount is: %d\n\x00" as *const u8 as
               *const std::os::raw::c_char, start);
    let output_length: std::os::raw::c_int = input_length - start;
    let mut data_out: *mut std::os::raw::c_double =
        malloc((output_length as std::os::raw::c_uint as
                    std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<std::os::raw::c_double>()
                                                    as std::os::raw::c_ulong)) as
            *mut std::os::raw::c_double;
    if data_out.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"example1.c\x00" as *const u8 as *const std::os::raw::c_char,
                     56 as std::os::raw::c_int,
                     b"data_out != 0\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    printf(b"The output length is: %d\n\x00" as *const u8 as
               *const std::os::raw::c_char, output_length);
    let mut all_inputs: [*const std::os::raw::c_double; 1] = [data_in.as_ptr()];
    let mut all_outputs: [*mut std::os::raw::c_double; 1] = [data_out];
    let mut error: std::os::raw::c_int =
        ti_sma(input_length, all_inputs.as_mut_ptr(), options.as_ptr(),
               all_outputs.as_mut_ptr());
    if !(error == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 5],
                                               &[std::os::raw::c_char; 5]>(b"main\x00")).as_ptr(),
                     b"example1.c\x00" as *const u8 as *const std::os::raw::c_char,
                     62 as std::os::raw::c_int,
                     b"error == TI_OKAY\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    printf(b"The output data is: \x00" as *const u8 as *const std::os::raw::c_char);
    print_array(data_out, output_length);
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
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

