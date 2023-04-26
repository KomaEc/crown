
extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
}
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
pub unsafe extern "C" fn ti_atr_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_atr(mut size: std::os::raw::c_int,
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
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_atr_start(options) { return 0 as std::os::raw::c_int }
    let per: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut truerange: std::os::raw::c_double = 0.;
    sum +=
        *high.offset(0 as std::os::raw::c_int as isize) -
            *low.offset(0 as std::os::raw::c_int as isize);
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < period {
        let l: std::os::raw::c_double = *low.offset(i as isize);
        let h: std::os::raw::c_double = *high.offset(i as isize);
        let c: std::os::raw::c_double =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych: std::os::raw::c_double = fabs(h - c);
        let ycl: std::os::raw::c_double = fabs(l - c);
        let mut v: std::os::raw::c_double = h - l;
        if ych > v { v = ych }
        if ycl > v { v = ycl }
        truerange = v;
        sum += truerange;
        i += 1
    }
    let mut val: std::os::raw::c_double = sum / period as std::os::raw::c_double;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = val;
    i = period;
    while i < size {
        let l_0: std::os::raw::c_double = *low.offset(i as isize);
        let h_0: std::os::raw::c_double = *high.offset(i as isize);
        let c_0: std::os::raw::c_double =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych_0: std::os::raw::c_double = fabs(h_0 - c_0);
        let ycl_0: std::os::raw::c_double = fabs(l_0 - c_0);
        let mut v_0: std::os::raw::c_double = h_0 - l_0;
        if ych_0 > v_0 { v_0 = ych_0 }
        if ycl_0 > v_0 { v_0 = ycl_0 }
        truerange = v_0;
        val = (truerange - val) * per + val;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = val;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_atr_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_atr\x00")).as_ptr(),
                     b"indicators/atr.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 69 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_atr_start(options)\x00"
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

