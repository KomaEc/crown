
extern "C" {
    #[no_mangle]
    fn log(_: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub unsafe extern "C" fn ti_fisher_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_fisher(mut size: std::os::raw::c_int,
                                   mut inputs: *const *const std::os::raw::c_double,
                                   mut options: *const std::os::raw::c_double,
                                   mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut fisher: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut signal: *mut std::os::raw::c_double =
        *outputs.offset(1 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_fisher_start(options) { return 0 as std::os::raw::c_int }
    let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut maxi: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
    let mut max: std::os::raw::c_double =
        0.5f64 *
            (*high.offset(0 as std::os::raw::c_int as isize) +
                 *low.offset(0 as std::os::raw::c_int as isize));
    let mut min: std::os::raw::c_double =
        0.5f64 *
            (*high.offset(0 as std::os::raw::c_int as isize) +
                 *low.offset(0 as std::os::raw::c_int as isize));
    let mut val1: std::os::raw::c_double = 0.0f64;
    let mut bar: std::os::raw::c_double = 0.;
    let mut fish: std::os::raw::c_double = 0.0f64;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = period - 1 as std::os::raw::c_int;
    while i < size {
        /* Maintain highest. */
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if maxi < trail {
            maxi = trail;
            max =
                0.5f64 *
                    (*high.offset(maxi as isize) +
                         *low.offset(maxi as isize));
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar =
                    0.5f64 *
                        (*high.offset(j as isize) + *low.offset(j as isize));
                if bar >= max { max = bar; maxi = j }
            }
        } else if bar >= max { maxi = i; max = bar }
        /* Maintain lowest. */
        bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
        if mini < trail {
            mini = trail;
            min =
                0.5f64 *
                    (*high.offset(mini as isize) +
                         *low.offset(mini as isize));
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar =
                    0.5f64 *
                        (*high.offset(j as isize) + *low.offset(j as isize));
                if bar <= min { min = bar; mini = j }
            }
        } else if bar <= min { mini = i; min = bar }
        let mut mm: std::os::raw::c_double = max - min;
        if mm == 0.0f64 { mm = 0.001f64 }
        val1 =
            0.33f64 * 2.0f64 *
                ((0.5f64 *
                      (*high.offset(i as isize) + *low.offset(i as isize)) -
                      min) / mm - 0.5f64) + 0.67f64 * val1;
        if val1 > 0.99f64 { val1 = 0.999f64 }
        if val1 < -0.99f64 { val1 = -0.999f64 }
        let fresh0 = signal;
        signal = signal.offset(1);
        *fresh0 = fish;
        fish =
            0.5f64 * log((1.0f64 + val1) / (1.0f64 - val1)) + 0.5f64 * fish;
        let fresh1 = fisher;
        fisher = fisher.offset(1);
        *fresh1 = fish;
        i += 1;
        trail += 1
    }
    if !(fisher.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_fisher_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_fisher\x00")).as_ptr(),
                     b"indicators/fisher.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 103 as std::os::raw::c_int,
                     b"fisher - outputs[0] == size - ti_fisher_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(signal.offset_from(*outputs.offset(1 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_fisher_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 10],
                                               &[std::os::raw::c_char; 10]>(b"ti_fisher\x00")).as_ptr(),
                     b"indicators/fisher.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 104 as std::os::raw::c_int,
                     b"signal - outputs[1] == size - ti_fisher_start(options)\x00"
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

