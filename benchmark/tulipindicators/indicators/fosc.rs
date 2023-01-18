
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
pub unsafe extern "C" fn ti_fosc_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fosc(mut size: std::os::raw::c_int,
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
    if size <= ti_fosc_start(options) { return 0 as std::os::raw::c_int }
    let mut x: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut x2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut y: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut xy: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let p: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    let mut tsf: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < period - 1 as std::os::raw::c_int {
        x += (i + 1 as std::os::raw::c_int) as std::os::raw::c_double;
        x2 +=
            ((i + 1 as std::os::raw::c_int) * (i + 1 as std::os::raw::c_int)) as
                std::os::raw::c_double;
        xy +=
            *input.offset(i as isize) *
                (i + 1 as std::os::raw::c_int) as std::os::raw::c_double;
        y += *input.offset(i as isize);
        i += 1
    }
    x += period as std::os::raw::c_double;
    x2 += (period * period) as std::os::raw::c_double;
    let bd: std::os::raw::c_double = 1.0f64 / (period as std::os::raw::c_double * x2 - x * x);
    i = period - 1 as std::os::raw::c_int;
    while i < size {
        xy += *input.offset(i as isize) * period as std::os::raw::c_double;
        y += *input.offset(i as isize);
        let b: std::os::raw::c_double = (period as std::os::raw::c_double * xy - x * y) * bd;
        let a: std::os::raw::c_double = (y - b * x) * p;
        if i >= period {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 =
                100 as std::os::raw::c_int as std::os::raw::c_double *
                    (*input.offset(i as isize) - tsf) /
                    *input.offset(i as isize)
        }
        tsf = a + b * (period + 1 as std::os::raw::c_int) as std::os::raw::c_double;
        xy -= y;
        y -= *input.offset((i - period + 1 as std::os::raw::c_int) as isize);
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_fosc_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_fosc\x00")).as_ptr(),
                     b"indicators/fosc.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 54 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_fosc_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
