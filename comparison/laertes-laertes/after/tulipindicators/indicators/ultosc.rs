
extern "C" {
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
pub unsafe extern "C" fn ti_ultosc_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ultosc(mut size: std::os::raw::c_int,
                                   mut inputs: *const *const std::os::raw::c_double,
                                   mut options: *const std::os::raw::c_double,
                                   mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut close: *const std::os::raw::c_double =
        *inputs.offset(2 as std::os::raw::c_int as isize);
    let short_period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let medium_period: std::os::raw::c_int =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: std::os::raw::c_int =
        *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if medium_period < short_period { return 1 as std::os::raw::c_int }
    if long_period < medium_period { return 1 as std::os::raw::c_int }
    if size <= ti_ultosc_start(options) { return 0 as std::os::raw::c_int }
    let mut bp_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut r_buf: *mut ti_buffer = ti_buffer_new(long_period);
    let mut bp_short_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut bp_medium_sum: std::os::raw::c_double =
        0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut r_short_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut r_medium_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        let true_low: std::os::raw::c_double =
            if *low.offset(i as isize) <
                   *close.offset((i - 1 as std::os::raw::c_int) as isize) {
                *low.offset(i as isize)
            } else { *close.offset((i - 1 as std::os::raw::c_int) as isize) };
        let true_high: std::os::raw::c_double =
            if *high.offset(i as isize) >
                   *close.offset((i - 1 as std::os::raw::c_int) as isize) {
                *high.offset(i as isize)
            } else { *close.offset((i - 1 as std::os::raw::c_int) as isize) };
        let bp: std::os::raw::c_double = *close.offset(i as isize) - true_low;
        let r: std::os::raw::c_double = true_high - true_low;
        bp_short_sum += bp;
        bp_medium_sum += bp;
        r_short_sum += r;
        r_medium_sum += r;
        if (*bp_buf).pushes >= (*bp_buf).size {
            (*bp_buf).sum -=
                *(*bp_buf).vals.as_mut_ptr().offset((*bp_buf).index as isize)
        }
        (*bp_buf).sum += bp;
        *(*bp_buf).vals.as_mut_ptr().offset((*bp_buf).index as isize) = bp;
        (*bp_buf).pushes += 1 as std::os::raw::c_int;
        (*bp_buf).index = (*bp_buf).index + 1 as std::os::raw::c_int;
        if (*bp_buf).index >= (*bp_buf).size {
            (*bp_buf).index = 0 as std::os::raw::c_int
        }
        if (*r_buf).pushes >= (*r_buf).size {
            (*r_buf).sum -=
                *(*r_buf).vals.as_mut_ptr().offset((*r_buf).index as isize)
        }
        (*r_buf).sum += r;
        *(*r_buf).vals.as_mut_ptr().offset((*r_buf).index as isize) = r;
        (*r_buf).pushes += 1 as std::os::raw::c_int;
        (*r_buf).index = (*r_buf).index + 1 as std::os::raw::c_int;
        if (*r_buf).index >= (*r_buf).size {
            (*r_buf).index = 0 as std::os::raw::c_int
        }
        /* The long sum takes care of itself, but we're piggy-backing
         * the medium and short sums off the same buffers. */
        if i > short_period {
            let mut short_index: std::os::raw::c_int =
                (*bp_buf).index - short_period - 1 as std::os::raw::c_int;
            if short_index < 0 as std::os::raw::c_int { short_index += long_period }
            bp_short_sum -=
                *(*bp_buf).vals.as_mut_ptr().offset(short_index as isize);
            r_short_sum -=
                *(*r_buf).vals.as_mut_ptr().offset(short_index as isize);
            if i > medium_period {
                let mut medium_index: std::os::raw::c_int =
                    (*bp_buf).index - medium_period - 1 as std::os::raw::c_int;
                if medium_index < 0 as std::os::raw::c_int {
                    medium_index += long_period
                }
                bp_medium_sum -=
                    *(*bp_buf).vals.as_mut_ptr().offset(medium_index as
                                                            isize);
                r_medium_sum -=
                    *(*r_buf).vals.as_mut_ptr().offset(medium_index as isize)
            }
        }
        if i >= long_period {
            let first: std::os::raw::c_double =
                4 as std::os::raw::c_int as std::os::raw::c_double * bp_short_sum /
                    r_short_sum;
            let second: std::os::raw::c_double =
                2 as std::os::raw::c_int as std::os::raw::c_double * bp_medium_sum /
                    r_medium_sum;
            let third: std::os::raw::c_double =
                1 as std::os::raw::c_int as std::os::raw::c_double * (*bp_buf).sum /
                    (*r_buf).sum;
            let ult: std::os::raw::c_double =
                (first + second + third) * 100.0f64 / 7.0f64;
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = ult
        }
        i += 1
    }
    ti_buffer_free(bp_buf);
    ti_buffer_free(r_buf);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_ultosc_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_ultosc\x00")).as_ptr(),
                     b"indicators/ultosc.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 103 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_ultosc_start(options)\x00"
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

