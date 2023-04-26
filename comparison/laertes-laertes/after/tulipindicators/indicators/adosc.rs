
extern "C" {
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
pub unsafe extern "C" fn ti_adosc_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adosc(mut size: std::os::raw::c_int,
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
    let mut volume: *const std::os::raw::c_double =
        *inputs.offset(3 as std::os::raw::c_int as isize);
    let short_period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: std::os::raw::c_int =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let start: std::os::raw::c_int = long_period - 1 as std::os::raw::c_int;
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < short_period { return 1 as std::os::raw::c_int }
    if size <= ti_adosc_start(options) { return 0 as std::os::raw::c_int }
    let short_per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (short_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let long_per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (long_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut short_ema: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut long_ema: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        let hl: std::os::raw::c_double =
            *high.offset(i as isize) - *low.offset(i as isize);
        if hl != 0.0f64 {
            sum +=
                (*close.offset(i as isize) - *low.offset(i as isize) -
                     *high.offset(i as isize) + *close.offset(i as isize)) /
                    hl * *volume.offset(i as isize)
        }
        if i == 0 as std::os::raw::c_int {
            short_ema = sum;
            long_ema = sum
        } else {
            short_ema = (sum - short_ema) * short_per + short_ema;
            long_ema = (sum - long_ema) * long_per + long_ema
        }
        if i >= start {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = short_ema - long_ema
        }
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_adosc_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 9],
                                               &[std::os::raw::c_char; 9]>(b"ti_adosc\x00")).as_ptr(),
                     b"indicators/adosc.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 73 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_adosc_start(options)\x00"
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

