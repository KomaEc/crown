
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
pub extern "C" fn ti_psar_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_psar(mut size: std::os::raw::c_int,
                                 mut inputs: *const *const std::os::raw::c_double,
                                 mut options: *const std::os::raw::c_double,
                                 mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let accel_step: std::os::raw::c_double =
        *options.offset(0 as std::os::raw::c_int as isize);
    let accel_max: std::os::raw::c_double =
        *options.offset(1 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if accel_step <= 0 as std::os::raw::c_int as std::os::raw::c_double {
        return 1 as std::os::raw::c_int
    }
    if accel_max <= accel_step { return 1 as std::os::raw::c_int }
    if size < 2 as std::os::raw::c_int { return 0 as std::os::raw::c_int }
    /* Try to choose if we start as short or long.
     * There is really no right answer here. */
    let mut lng: std::os::raw::c_int = 0;
    if *high.offset(0 as std::os::raw::c_int as isize) +
           *low.offset(0 as std::os::raw::c_int as isize) <=
           *high.offset(1 as std::os::raw::c_int as isize) +
               *low.offset(1 as std::os::raw::c_int as isize) {
        lng = 1 as std::os::raw::c_int
    } else { lng = 0 as std::os::raw::c_int }
    let mut sar: std::os::raw::c_double = 0.;
    let mut extreme: std::os::raw::c_double = 0.;
    if lng != 0 {
        extreme = *high.offset(0 as std::os::raw::c_int as isize);
        sar = *low.offset(0 as std::os::raw::c_int as isize)
    } else {
        extreme = *low.offset(0 as std::os::raw::c_int as isize);
        sar = *high.offset(0 as std::os::raw::c_int as isize)
    }
    let mut accel: std::os::raw::c_double = accel_step;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        sar = (extreme - sar) * accel + sar;
        if lng != 0 {
            if i >= 2 as std::os::raw::c_int &&
                   sar > *low.offset((i - 2 as std::os::raw::c_int) as isize) {
                sar = *low.offset((i - 2 as std::os::raw::c_int) as isize)
            }
            if sar > *low.offset((i - 1 as std::os::raw::c_int) as isize) {
                sar = *low.offset((i - 1 as std::os::raw::c_int) as isize)
            }
            if accel < accel_max && *high.offset(i as isize) > extreme {
                accel += accel_step;
                if accel > accel_max { accel = accel_max }
            }
            if *high.offset(i as isize) > extreme {
                extreme = *high.offset(i as isize)
            }
        } else {
            if i >= 2 as std::os::raw::c_int &&
                   sar < *high.offset((i - 2 as std::os::raw::c_int) as isize) {
                sar = *high.offset((i - 2 as std::os::raw::c_int) as isize)
            }
            if sar < *high.offset((i - 1 as std::os::raw::c_int) as isize) {
                sar = *high.offset((i - 1 as std::os::raw::c_int) as isize)
            }
            if accel < accel_max && *low.offset(i as isize) < extreme {
                accel += accel_step;
                if accel > accel_max { accel = accel_max }
            }
            if *low.offset(i as isize) < extreme {
                extreme = *low.offset(i as isize)
            }
        }
        if lng != 0 && *low.offset(i as isize) < sar ||
               lng == 0 && *high.offset(i as isize) > sar {
            accel = accel_step;
            sar = extreme;
            lng = (lng == 0) as std::os::raw::c_int;
            if lng == 0 {
                extreme = *low.offset(i as isize)
            } else { extreme = *high.offset(i as isize) }
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = sar;
        i += 1
        /*
        printf("%s%2d %.4f %.4f %.4f %.4f %.4f %s\n", i == 1 ? "\n" : "", i, high[i], low[i], accel, extreme, sar, reverse ? (!lng ? "short" : "long") : "");
        */
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_psar_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_psar\x00")).as_ptr(),
                     b"indicators/psar.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 122 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_psar_start(options)\x00"
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

