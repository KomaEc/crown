
extern "C" {
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
}
pub use crate::indicators::sma::ti_sma;
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
pub unsafe extern "C" fn ti_trima_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trima(mut size: std::os::raw::c_int,
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
    if size <= ti_trima_start(options) { return 0 as std::os::raw::c_int }
    if period <= 2 as std::os::raw::c_int {
        return ti_sma(size, inputs, options, outputs)
    }
    /* Weights for 6 period TRIMA:
     * 1 2 3 3 2 1 = 12
     *
     * Weights for 7 period TRIMA:
     * 1 2 3 4 3 2 1 = 16
     */
    let mut weights: std::os::raw::c_double =
        1 as std::os::raw::c_int as std::os::raw::c_double /
            (if period % 2 as std::os::raw::c_int != 0 {
                 (period / 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) *
                     (period / 2 as std::os::raw::c_int + 1 as std::os::raw::c_int)
             } else {
                 (period / 2 as std::os::raw::c_int + 1 as std::os::raw::c_int) *
                     (period / 2 as std::os::raw::c_int)
             }) as
                std::os::raw::c_double; /* Weighted sum of previous numbers, spans one period back. */
    let mut weight_sum: std::os::raw::c_double =
        0 as std::os::raw::c_int as
            std::os::raw::c_double; /* Flat sum of most recent numbers. */
    let mut lead_sum: std::os::raw::c_double =
        0 as std::os::raw::c_int as std::os::raw::c_double; /* Flat sum of oldest numbers. */
    let mut trail_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    /* example for period of 9 */
    /* weight_sum       1 2 3 4 5 4 3 2 1 */
    /* lead_sum                   1 1 1 1 */
    /* trail_sum        1 1 1 1 1        */
    let lead_period: std::os::raw::c_int =
        if period % 2 as std::os::raw::c_int != 0 {
            (period) / 2 as std::os::raw::c_int
        } else { (period / 2 as std::os::raw::c_int) - 1 as std::os::raw::c_int };
    let trail_period: std::os::raw::c_int = lead_period + 1 as std::os::raw::c_int;
    let mut i: std::os::raw::c_int = 0;
    let mut w: std::os::raw::c_int = 1 as std::os::raw::c_int;
    /* Initialize until before the first value. */
    i = 0 as std::os::raw::c_int;
    while i < period - 1 as std::os::raw::c_int {
        weight_sum += *input.offset(i as isize) * w as std::os::raw::c_double;
        if i + 1 as std::os::raw::c_int > period - lead_period {
            lead_sum += *input.offset(i as isize)
        }
        if i + 1 as std::os::raw::c_int <= trail_period {
            trail_sum += *input.offset(i as isize)
        }
        if (i + 1 as std::os::raw::c_int) < trail_period { w += 1 }
        if i + 1 as std::os::raw::c_int >= period - lead_period { w -= 1 }
        i += 1
    }
    let mut lsi: std::os::raw::c_int =
        period - 1 as std::os::raw::c_int - lead_period + 1 as std::os::raw::c_int;
    let mut tsi1: std::os::raw::c_int =
        period - 1 as std::os::raw::c_int - period + 1 as std::os::raw::c_int + trail_period;
    let mut tsi2: std::os::raw::c_int =
        period - 1 as std::os::raw::c_int - period + 1 as std::os::raw::c_int;
    i = period - 1 as std::os::raw::c_int;
    while i < size {
        weight_sum += *input.offset(i as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = weight_sum * weights;
        lead_sum += *input.offset(i as isize);
        /* 1 2 3 4 5 4 3 2 1 */
        weight_sum += lead_sum;
        /* 1 2 3 4 5 5 4 3 2 */
        weight_sum -= trail_sum;
        /*   1 2 3 4 5 4 3 2 */
        /* weight_sum       1 2 3 4 5 4 3 2 1 */
        /* lead_sum                   1 1 1 1 */
        /* trail_sum        1 1 1 1 1        */
        let fresh1 = lsi;
        lsi = lsi + 1;
        lead_sum -= *input.offset(fresh1 as isize);
        let fresh2 = tsi1;
        tsi1 = tsi1 + 1;
        trail_sum += *input.offset(fresh2 as isize);
        let fresh3 = tsi2;
        tsi2 = tsi2 + 1;
        trail_sum -= *input.offset(fresh3 as isize);
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_trima_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 9],
                                               &[std::os::raw::c_char; 9]>(b"ti_trima\x00")).as_ptr(),
                     b"indicators/trima.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 103 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_trima_start(options)\x00"
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

