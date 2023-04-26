
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
pub unsafe extern "C" fn ti_cvi_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    let n: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    return n * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cvi(mut size: std::os::raw::c_int,
                                mut inputs: *const *const std::os::raw::c_double,
                                mut options: *const std::os::raw::c_double,
                                mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_cvi_start(options) { return 0 as std::os::raw::c_int }
    let per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (period as std::os::raw::c_double + 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut lag: *mut ti_buffer = ti_buffer_new(period);
    let mut val: std::os::raw::c_double =
        *high.offset(0 as std::os::raw::c_int as isize) -
            *low.offset(0 as std::os::raw::c_int as isize);
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < period * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int {
        val =
            (*high.offset(i as isize) - *low.offset(i as isize) - val) * per +
                val;
        *(*lag).vals.as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as std::os::raw::c_int;
        if (*lag).index >= (*lag).size { (*lag).index = 0 as std::os::raw::c_int }
        i += 1
    }
    i = period * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    while i < size {
        val =
            (*high.offset(i as isize) - *low.offset(i as isize) - val) * per +
                val;
        let old: std::os::raw::c_double =
            *(*lag).vals.as_mut_ptr().offset((*lag).index as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = 100.0f64 * (val - old) / old;
        *(*lag).vals.as_mut_ptr().offset((*lag).index as isize) = val;
        (*lag).index = (*lag).index + 1 as std::os::raw::c_int;
        if (*lag).index >= (*lag).size { (*lag).index = 0 as std::os::raw::c_int }
        i += 1
    }
    ti_buffer_free(lag);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_cvi_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_cvi\x00")).as_ptr(),
                     b"indicators/cvi.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 66 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_cvi_start(options)\x00"
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

