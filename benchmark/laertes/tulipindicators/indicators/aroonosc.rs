
extern "C" {
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    #[no_mangle]
    fn ti_aroon_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
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
pub unsafe extern "C" fn ti_aroonosc_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroonosc(mut size: std::os::raw::c_int,
                                     mut inputs: *const *const std::os::raw::c_double,
                                     mut options: *const std::os::raw::c_double,
                                     mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_aroon_start(options) { return 0 as std::os::raw::c_int }
    let scale: std::os::raw::c_double = 100.0f64 / period as std::os::raw::c_double;
    let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut maxi: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut max: std::os::raw::c_double = *high.offset(0 as std::os::raw::c_int as isize);
    let mut min: std::os::raw::c_double = *low.offset(0 as std::os::raw::c_int as isize);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = period;
    while i < size {
        /* Maintain highest. */
        let mut bar: std::os::raw::c_double = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar = *high.offset(j as isize);
                if bar >= max { max = bar; maxi = j }
            }
        } else if bar >= max { maxi = i; max = bar }
        /* Maintain lowest. */
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar = *low.offset(j as isize);
                if bar <= min { min = bar; mini = j }
            }
        } else if bar <= min { mini = i; min = bar }
        /* Calculate the indicator. */
        /*
            const TI_REAL adown = ((TI_REAL)period - (i-mini)) * scale;
            const TI_REAL aup = ((TI_REAL)period - (i-maxi)) * scale;
            *output = aup - adown
        That simplifies to:
            (maxi-mini) * scale
        */
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = (maxi - mini) as std::os::raw::c_double * scale;
        i += 1;
        trail += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_aroonosc_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 12],
                                               &[std::os::raw::c_char; 12]>(b"ti_aroonosc\x00")).as_ptr(),
                     b"indicators/aroonosc.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 102 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_aroonosc_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
