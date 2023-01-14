
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
pub unsafe extern "C" fn ti_avgprice_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_avgprice(mut size: std::os::raw::c_int,
                                     mut inputs: *const *const std::os::raw::c_double,
                                     mut options: *const std::os::raw::c_double,
                                     mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut open: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(2 as std::os::raw::c_int as isize);
    let mut close: *const std::os::raw::c_double =
        *inputs.offset(3 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        *output.offset(i as isize) =
            (*open.offset(i as isize) + *high.offset(i as isize) +
                 *low.offset(i as isize) + *close.offset(i as isize)) *
                0.25f64;
        i += 1
    }
    return 0 as std::os::raw::c_int;
}
