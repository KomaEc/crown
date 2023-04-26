
extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
}
pub use crate::utils::buffer::ti_buffer_free;
pub use crate::utils::buffer::ti_buffer_new;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: std::os::raw::c_int,
    pub pushes: std::os::raw::c_int,
    pub index: std::os::raw::c_int,
    pub sum: std::os::raw::c_double,
    pub vals: [std::os::raw::c_double; 1],
}
impl std::default::Default for ti_buffer {
    fn default() -> Self {
        ti_buffer {
        size: std::os::raw::c_int::default(),
        pushes: std::os::raw::c_int::default(),
        index: std::os::raw::c_int::default(),
        sum: std::os::raw::c_double::default(),
        vals: [std::os::raw::c_double::default(); 1]
        }
    }
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
pub unsafe extern "C" fn ti_adxr_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return (*options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
                1 as std::os::raw::c_int) * 3 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_adxr(mut size: std::os::raw::c_int,
                                 mut inputs: *const *const std::os::raw::c_double,
                                 mut options: *const std::os::raw::c_double,
                                 mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: *const std::os::raw::c_double =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut close: *const std::os::raw::c_double =
        *inputs.offset(2 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_adxr_start(options) { return 0 as std::os::raw::c_int }
    let per: std::os::raw::c_double =
        (period as std::os::raw::c_double - 1 as std::os::raw::c_int as std::os::raw::c_double) /
            period as std::os::raw::c_double;
    let invper: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    let mut atr: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dmup: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dmdown: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i < period {
        let mut truerange: std::os::raw::c_double = 0.;
        let l: std::os::raw::c_double = *low.offset(i as isize);
        let h: std::os::raw::c_double = *high.offset(i as isize);
        let c: std::os::raw::c_double =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych: std::os::raw::c_double = fabs(h - c);
        let ycl: std::os::raw::c_double = fabs(l - c);
        let mut v: std::os::raw::c_double = h - l;
        if ych > v { v = ych }
        if ycl > v { v = ycl }
        truerange = v;
        atr += truerange;
        let mut dp: std::os::raw::c_double = 0.;
        let mut dm: std::os::raw::c_double = 0.;
        dp =
            *high.offset(i as isize) -
                *high.offset((i - 1 as std::os::raw::c_int) as isize);
        dm =
            *low.offset((i - 1 as std::os::raw::c_int) as isize) -
                *low.offset(i as isize);
        if dp < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dp = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dp > dm { dm = 0 as std::os::raw::c_int as std::os::raw::c_double }
        if dm < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dm = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dm > dp { dp = 0 as std::os::raw::c_int as std::os::raw::c_double }
        dmup += dp;
        dmdown += dm;
        i += 1
    }
    let mut adx: std::os::raw::c_double = 0.0f64;
    let mut di_up: std::os::raw::c_double = dmup / atr;
    let mut di_down: std::os::raw::c_double = dmdown / atr;
    let mut dm_diff: std::os::raw::c_double = fabs(di_up - di_down);
    let mut dm_sum: std::os::raw::c_double = di_up + di_down;
    let mut dx: std::os::raw::c_double =
        dm_diff / dm_sum * 100 as std::os::raw::c_int as std::os::raw::c_double;
    adx += dx;
    let mut adxr: *mut ti_buffer = ti_buffer_new(period - 1 as std::os::raw::c_int);
    let first_adxr: std::os::raw::c_int = ti_adxr_start(options);
    i = period;
    while i < size {
        let mut truerange_0: std::os::raw::c_double = 0.;
        let l_0: std::os::raw::c_double = *low.offset(i as isize);
        let h_0: std::os::raw::c_double = *high.offset(i as isize);
        let c_0: std::os::raw::c_double =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych_0: std::os::raw::c_double = fabs(h_0 - c_0);
        let ycl_0: std::os::raw::c_double = fabs(l_0 - c_0);
        let mut v_0: std::os::raw::c_double = h_0 - l_0;
        if ych_0 > v_0 { v_0 = ych_0 }
        if ycl_0 > v_0 { v_0 = ycl_0 }
        truerange_0 = v_0;
        atr = atr * per + truerange_0;
        let mut dp_0: std::os::raw::c_double = 0.;
        let mut dm_0: std::os::raw::c_double = 0.;
        dp_0 =
            *high.offset(i as isize) -
                *high.offset((i - 1 as std::os::raw::c_int) as isize);
        dm_0 =
            *low.offset((i - 1 as std::os::raw::c_int) as isize) -
                *low.offset(i as isize);
        if dp_0 < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dp_0 = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dp_0 > dm_0 { dm_0 = 0 as std::os::raw::c_int as std::os::raw::c_double }
        if dm_0 < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dm_0 = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dm_0 > dp_0 { dp_0 = 0 as std::os::raw::c_int as std::os::raw::c_double }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let mut di_up_0: std::os::raw::c_double = dmup / atr;
        let mut di_down_0: std::os::raw::c_double = dmdown / atr;
        let mut dm_diff_0: std::os::raw::c_double = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: std::os::raw::c_double = di_up_0 + di_down_0;
        let mut dx_0: std::os::raw::c_double =
            dm_diff_0 / dm_sum_0 * 100 as std::os::raw::c_int as std::os::raw::c_double;
        if i - period < period - 2 as std::os::raw::c_int {
            adx += dx_0
        } else if i - period == period - 2 as std::os::raw::c_int {
            adx += dx_0;
            *(*adxr).vals.as_mut_ptr().offset((*adxr).index as isize) =
                adx * invper;
            (*adxr).index = (*adxr).index + 1 as std::os::raw::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as std::os::raw::c_int
            }
        } else {
            adx = adx * per + dx_0;
            if i >= first_adxr {
                let fresh0 = output;
                output = output.offset(1);
                *fresh0 =
                    0.5f64 *
                        (adx * invper +
                             *(*adxr).vals.as_mut_ptr().offset((((*adxr).index
                                                                     +
                                                                     (*adxr).size
                                                                     -
                                                                     1 as
                                                                         std::os::raw::c_int
                                                                     +
                                                                     1 as
                                                                         std::os::raw::c_int)
                                                                    %
                                                                    (*adxr).size)
                                                                   as isize))
            }
            *(*adxr).vals.as_mut_ptr().offset((*adxr).index as isize) =
                adx * invper;
            (*adxr).index = (*adxr).index + 1 as std::os::raw::c_int;
            if (*adxr).index >= (*adxr).size {
                (*adxr).index = 0 as std::os::raw::c_int
            }
        }
        i += 1
    }
    ti_buffer_free(adxr);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_adxr_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 8],
                                               &[std::os::raw::c_char; 8]>(b"ti_adxr\x00")).as_ptr(),
                     b"indicators/adxr.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 122 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_adxr_start(options)\x00"
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

