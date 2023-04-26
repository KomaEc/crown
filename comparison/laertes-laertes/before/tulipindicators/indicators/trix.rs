
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
pub unsafe extern "C" fn ti_trix_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    return (period - 1 as std::os::raw::c_int) * 3 as std::os::raw::c_int + 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_trix(mut size: std::os::raw::c_int,
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
    if size <= ti_trix_start(options) { return 0 as std::os::raw::c_int }
    let start: std::os::raw::c_int = period * 3 as std::os::raw::c_int - 2 as std::os::raw::c_int;
    if !(start == ti_trix_start(options)) as std::os::raw::c_int as std::os::raw::c_long != 0
       {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_trix\x00")).as_ptr(),
                     b"indicators/trix.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 43 as std::os::raw::c_int,
                     b"start == ti_trix_start(options)\x00" as *const u8 as
                         *const std::os::raw::c_char);
    } else { };
    let per: std::os::raw::c_double =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (period as std::os::raw::c_double + 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut ema1: std::os::raw::c_double = *input.offset(0 as std::os::raw::c_int as isize);
    let mut ema2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut ema3: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < start {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        if i == period - 1 as std::os::raw::c_int {
            ema2 = ema1
        } else if i > period - 1 as std::os::raw::c_int {
            ema2 = (ema1 - ema2) * per + ema2;
            if i == period * 2 as std::os::raw::c_int - 2 as std::os::raw::c_int {
                ema3 = ema2
            } else if i > period * 2 as std::os::raw::c_int - 2 as std::os::raw::c_int {
                ema3 = (ema2 - ema3) * per + ema3
            }
        }
        i += 1
    }
    i = start;
    while i < size {
        ema1 = (*input.offset(i as isize) - ema1) * per + ema1;
        ema2 = (ema1 - ema2) * per + ema2;
        let last: std::os::raw::c_double = ema3;
        ema3 = (ema2 - ema3) * per + ema3;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (ema3 - last) / ema3 * 100.0f64;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_trix_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_trix\x00")).as_ptr(),
                     b"indicators/trix.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 75 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_trix_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
