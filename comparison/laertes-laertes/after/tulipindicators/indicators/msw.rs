
extern "C" {
    #[no_mangle]
    fn atan(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub unsafe extern "C" fn ti_msw_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_msw(mut size: std::os::raw::c_int,
                                mut inputs: *const *const std::os::raw::c_double,
                                mut options: *const std::os::raw::c_double,
                                mut outputs: *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut sine: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut lead: *mut std::os::raw::c_double =
        *outputs.offset(1 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_msw_start(options) { return 0 as std::os::raw::c_int }
    let pi: std::os::raw::c_double = 3.1415926f64;
    let tpi: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double * pi;
    let mut weight: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut phase: std::os::raw::c_double = 0.;
    let mut rp: std::os::raw::c_double = 0.;
    let mut ip: std::os::raw::c_double = 0.;
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i = period;
    while i < size {
        rp = 0 as std::os::raw::c_int as std::os::raw::c_double;
        ip = 0 as std::os::raw::c_int as std::os::raw::c_double;
        j = 0 as std::os::raw::c_int;
        while j < period {
            weight = *input.offset((i - j) as isize);
            rp =
                rp +
                    cos(tpi * j as std::os::raw::c_double / period as std::os::raw::c_double)
                        * weight;
            ip =
                ip +
                    sin(tpi * j as std::os::raw::c_double / period as std::os::raw::c_double)
                        * weight;
            j += 1
        }
        if fabs(rp) > 0.001f64 {
            phase = atan(ip / rp)
        } else {
            phase =
                tpi / 2.0f64 *
                    (if ip < 0 as std::os::raw::c_int as std::os::raw::c_double {
                         -1.0f64
                     } else { 1.0f64 })
        }
        if rp < 0.0f64 { phase += pi }
        phase += pi / 2.0f64;
        if phase < 0.0f64 { phase += tpi }
        if phase > tpi { phase -= tpi }
        //phase = 180 * phase / pi;
        let fresh0 = sine;
        sine = sine.offset(1);
        *fresh0 = sin(phase);
        let fresh1 = lead;
        lead = lead.offset(1);
        *fresh1 = sin(phase + pi / 4.0f64);
        i += 1
    }
    if !(sine.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_msw_start(options)) as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_msw\x00")).as_ptr(),
                     b"indicators/msw.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 73 as std::os::raw::c_int,
                     b"sine - outputs[0] == size - ti_msw_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(lead.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_msw_start(options)) as std::os::raw::c_long) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_msw\x00")).as_ptr(),
                     b"indicators/msw.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 74 as std::os::raw::c_int,
                     b"lead - outputs[1] == size - ti_msw_start(options)\x00"
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

