
extern "C" {
    #[no_mangle]
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub unsafe extern "C" fn ti_bbands_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_bbands(mut size: std::os::raw::c_int,
                                   mut inputs: *const *const std::os::raw::c_double,
                                   mut options: *const std::os::raw::c_double,
                                   mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut lower: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut middle: *mut std::os::raw::c_double =
        *outputs.offset(1 as std::os::raw::c_int as isize);
    let mut upper: *mut std::os::raw::c_double =
        *outputs.offset(2 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let stddev: std::os::raw::c_double = *options.offset(1 as std::os::raw::c_int as isize);
    let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_bbands_start(options) { return 0 as std::os::raw::c_int }
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1
    }
    let mut sd: std::os::raw::c_double =
        sqrt(sum2 * scale - sum * scale * (sum * scale));
    *middle = sum * scale;
    let fresh0 = lower;
    lower = lower.offset(1);
    *fresh0 = *middle - stddev * sd;
    let fresh1 = upper;
    upper = upper.offset(1);
    *fresh1 = *middle + stddev * sd;
    middle = middle.offset(1);
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2 -=
            *input.offset((i - period) as isize) *
                *input.offset((i - period) as isize);
        sd = sqrt(sum2 * scale - sum * scale * (sum * scale));
        *middle = sum * scale;
        let fresh2 = upper;
        upper = upper.offset(1);
        *fresh2 = *middle + stddev * sd;
        let fresh3 = lower;
        lower = lower.offset(1);
        *fresh3 = *middle - stddev * sd;
        middle = middle.offset(1);
        i += 1
    }
    if !(lower.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                        isize)) as
             std::os::raw::c_long ==
             (size - ti_bbands_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_bbands\x00")).as_ptr(),
                     b"indicators/bbands.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 76 as std::os::raw::c_int,
                     b"lower - outputs[0] == size - ti_bbands_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(middle.offset_from(*outputs.offset(1 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_bbands_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_bbands\x00")).as_ptr(),
                     b"indicators/bbands.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 77 as std::os::raw::c_int,
                     b"middle - outputs[1] == size - ti_bbands_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(upper.offset_from(*outputs.offset(2 as std::os::raw::c_int as
                                                        isize)) as
             std::os::raw::c_long ==
             (size - ti_bbands_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_bbands\x00")).as_ptr(),
                     b"indicators/bbands.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 78 as std::os::raw::c_int,
                     b"upper - outputs[2] == size - ti_bbands_start(options)\x00"
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
