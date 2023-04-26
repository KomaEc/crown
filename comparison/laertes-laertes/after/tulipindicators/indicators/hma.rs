
extern "C" {
    #[no_mangle]
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
}
pub use crate::utils::buffer::ti_buffer_free;
pub use crate::utils::buffer::ti_buffer_new;
// #[derive(Copy, Clone)]

pub use crate::indicators::adxr::ti_buffer;
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
pub unsafe extern "C" fn ti_hma_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let periodsqrt: std::os::raw::c_int =
        sqrt(period as std::os::raw::c_double) as std::os::raw::c_int;
    return period + periodsqrt - 2 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_hma(mut size: std::os::raw::c_int,
                                mut inputs: *const *const std::os::raw::c_double,
                                mut options: *const std::os::raw::c_double,
                                mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_hma_start(options) { return 0 as std::os::raw::c_int }
    /* HMA(input, N) = WMA((2 * WMA(input, N/2) - WMA(input, N)), sqrt(N)) */
    /* Need to do three WMAs, with periods N, N/2, and sqrt N.*/
    let period2: std::os::raw::c_int =
        period / 2 as std::os::raw::c_int; /* Flat sum of previous numbers. */
    let periodsqrt: std::os::raw::c_int =
        sqrt(period as std::os::raw::c_double) as
            std::os::raw::c_int; /* Weighted sum of previous numbers. */
    let weights: std::os::raw::c_double =
        (period * (period + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int) as
            std::os::raw::c_double;
    let weights2: std::os::raw::c_double =
        (period2 * (period2 + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int) as
            std::os::raw::c_double;
    let weightssqrt: std::os::raw::c_double =
        (periodsqrt * (periodsqrt + 1 as std::os::raw::c_int) / 2 as std::os::raw::c_int) as
            std::os::raw::c_double;
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut weight_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut weight_sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sumsqrt: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut weight_sumsqrt: std::os::raw::c_double =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    /* Setup up the WMA(period) and WMA(period/2) on the input. */
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < period - 1 as std::os::raw::c_int {
        weight_sum +=
            *input.offset(i as isize) *
                (i + 1 as std::os::raw::c_int) as std::os::raw::c_double;
        sum += *input.offset(i as isize);
        if i >= period - period2 {
            weight_sum2 +=
                *input.offset(i as isize) *
                    (i + 1 as std::os::raw::c_int - (period - period2)) as
                        std::os::raw::c_double;
            sum2 += *input.offset(i as isize)
        }
        i += 1
    }
    let mut buff: *mut ti_buffer = ti_buffer_new(periodsqrt);
    i = period - 1 as std::os::raw::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize) * period as std::os::raw::c_double;
        sum += *input.offset(i as isize);
        weight_sum2 += *input.offset(i as isize) * period2 as std::os::raw::c_double;
        sum2 += *input.offset(i as isize);
        let wma: std::os::raw::c_double = weight_sum / weights;
        let wma2: std::os::raw::c_double = weight_sum2 / weights2;
        let diff: std::os::raw::c_double =
            2 as std::os::raw::c_int as std::os::raw::c_double * wma2 - wma;
        weight_sumsqrt += diff * periodsqrt as std::os::raw::c_double;
        sumsqrt += diff;
        *(*buff).vals.as_mut_ptr().offset((*buff).index as isize) = diff;
        (*buff).index = (*buff).index + 1 as std::os::raw::c_int;
        if (*buff).index >= (*buff).size { (*buff).index = 0 as std::os::raw::c_int }
        if i >= period - 1 as std::os::raw::c_int + (periodsqrt - 1 as std::os::raw::c_int) {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = weight_sumsqrt / weightssqrt;
            weight_sumsqrt -= sumsqrt;
            sumsqrt -=
                *(*buff).vals.as_mut_ptr().offset((((*buff).index +
                                                        (*buff).size -
                                                        1 as std::os::raw::c_int +
                                                        1 as std::os::raw::c_int) %
                                                       (*buff).size) as isize)
        } else { weight_sumsqrt -= sumsqrt }
        weight_sum -= sum;
        sum -= *input.offset((i - period + 1 as std::os::raw::c_int) as isize);
        weight_sum2 -= sum2;
        sum2 -= *input.offset((i - period2 + 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    ti_buffer_free(buff);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_hma_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_hma\x00")).as_ptr(),
                     b"indicators/hma.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 111 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_hma_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
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

