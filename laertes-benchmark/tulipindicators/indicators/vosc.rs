
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
pub unsafe extern "C" fn ti_vosc_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_vosc(mut size: std::os::raw::c_int,
                                 mut inputs: *const *const std::os::raw::c_double,
                                 mut options: *const std::os::raw::c_double,
                                 mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let short_period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let long_period: std::os::raw::c_int =
        *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let short_div: std::os::raw::c_double = 1.0f64 / short_period as std::os::raw::c_double;
    let long_div: std::os::raw::c_double = 1.0f64 / long_period as std::os::raw::c_double;
    if short_period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if long_period < short_period { return 1 as std::os::raw::c_int }
    if size <= ti_vosc_start(options) { return 0 as std::os::raw::c_int }
    let mut short_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut long_sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < long_period {
        if i >= long_period - short_period {
            short_sum += *input.offset(i as isize)
        }
        long_sum += *input.offset(i as isize);
        i += 1
    }
    let savg: std::os::raw::c_double = short_sum * short_div;
    let lavg: std::os::raw::c_double = long_sum * long_div;
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = 100.0f64 * (savg - lavg) / lavg;
    i = long_period;
    while i < size {
        short_sum += *input.offset(i as isize);
        short_sum -= *input.offset((i - short_period) as isize);
        long_sum += *input.offset(i as isize);
        long_sum -= *input.offset((i - long_period) as isize);
        let savg_0: std::os::raw::c_double = short_sum * short_div;
        let lavg_0: std::os::raw::c_double = long_sum * long_div;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = 100.0f64 * (savg_0 - lavg_0) / lavg_0;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_vosc_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_vosc\x00")).as_ptr(),
                     b"indicators/vosc.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 76 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_vosc_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
