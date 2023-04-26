
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
pub unsafe extern "C" fn ti_macd_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    /* NB we return data before signal is strictly valid. */
    let long_period: std::os::raw::c_int =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    return long_period - 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_macd(mut size: std::os::raw::c_int,
                                 mut inputs: *const *const std::os::raw::c_double,
                                 mut options: *const std::os::raw::c_double,
                                 mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut macd: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut signal: *mut std::os::raw::c_double =
        *outputs.offset(1 as std::os::raw::c_int as isize);
    let mut hist: *mut std::os::raw::c_double =
        *outputs.offset(2 as std::os::raw::c_int as isize);
    let short_period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: std::os::raw::c_int =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let signal_period: std::os::raw::c_int =
        *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < short_period { return 1 as std::os::raw::c_int }
    if signal_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_macd_start(options) { return 0 as std::os::raw::c_int }
    let mut short_per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (short_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut long_per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (long_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut signal_per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (signal_period as std::os::raw::c_double +
                 1 as std::os::raw::c_int as std::os::raw::c_double);
    if short_period == 12 as std::os::raw::c_int && long_period == 26 as std::os::raw::c_int {
        /* I don't like this, but it's what people expect. */
        short_per = 0.15f64;
        long_per = 0.075f64
    }
    let mut short_ema: std::os::raw::c_double =
        *input.offset(0 as std::os::raw::c_int as isize);
    let mut long_ema: std::os::raw::c_double =
        *input.offset(0 as std::os::raw::c_int as isize);
    let mut signal_ema: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        short_ema =
            (*input.offset(i as isize) - short_ema) * short_per + short_ema;
        long_ema =
            (*input.offset(i as isize) - long_ema) * long_per + long_ema;
        let out: std::os::raw::c_double = short_ema - long_ema;
        if i == long_period - 1 as std::os::raw::c_int { signal_ema = out }
        if i >= long_period - 1 as std::os::raw::c_int {
            signal_ema = (out - signal_ema) * signal_per + signal_ema;
            let fresh0 = macd;
            macd = macd.offset(1);
            *fresh0 = out;
            let fresh1 = signal;
            signal = signal.offset(1);
            *fresh1 = signal_ema;
            let fresh2 = hist;
            hist = hist.offset(1);
            *fresh2 = out - signal_ema
        }
        i += 1
    }
    if !(macd.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_macd_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_macd\x00")).as_ptr(),
                     b"indicators/macd.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 86 as std::os::raw::c_int,
                     b"macd - outputs[0] == size - ti_macd_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(signal.offset_from(*outputs.offset(1 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_macd_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_macd\x00")).as_ptr(),
                     b"indicators/macd.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 87 as std::os::raw::c_int,
                     b"signal - outputs[1] == size - ti_macd_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(hist.offset_from(*outputs.offset(2 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_macd_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_macd\x00")).as_ptr(),
                     b"indicators/macd.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 88 as std::os::raw::c_int,
                     b"hist - outputs[2] == size - ti_macd_start(options)\x00"
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

