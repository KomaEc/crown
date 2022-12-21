
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
pub unsafe extern "C" fn ti_ao_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return 33 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_ao(mut size: std::os::raw::c_int,
                               mut inputs: *const *const std::os::raw::c_double,
                               mut options: *const std::os::raw::c_double,
                               mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int = 34 as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if size <= ti_ao_start(options) { return 0 as std::os::raw::c_int }
    let mut sum34: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum5: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let per34: std::os::raw::c_double = 1.0f64 / 34.0f64;
    let per5: std::os::raw::c_double = 1.0f64 / 5.0f64;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < 34 as std::os::raw::c_int {
        let mut hl: std::os::raw::c_double =
            0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl;
        if i >= 29 as std::os::raw::c_int { sum5 += hl }
        i += 1
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = per5 * sum5 - per34 * sum34;
    i = period;
    while i < size {
        let mut hl_0: std::os::raw::c_double =
            0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        sum34 += hl_0;
        sum5 += hl_0;
        sum34 -=
            0.5f64 *
                (*high.offset((i - 34 as std::os::raw::c_int) as isize) +
                     *low.offset((i - 34 as std::os::raw::c_int) as isize));
        sum5 -=
            0.5f64 *
                (*high.offset((i - 5 as std::os::raw::c_int) as isize) +
                     *low.offset((i - 5 as std::os::raw::c_int) as isize));
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = per5 * sum5 - per34 * sum34;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_ao_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 6],
                                               &[std::os::raw::c_char; 6]>(b"ti_ao\x00")).as_ptr(),
                     b"indicators/ao.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 68 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_ao_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
