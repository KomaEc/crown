
extern "C" {
    #[no_mangle]
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility_start(mut options:
                                                 *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as
               std::os::raw::c_int; /* Multiplier, number of trading days in year. */
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility(mut size: std::os::raw::c_int,
                                       mut inputs:
                                           *const *const std::os::raw::c_double,
                                       mut options: *const std::os::raw::c_double,
                                       mut outputs:
                                           *const *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: *const std::os::raw::c_double =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    let annual: std::os::raw::c_double = sqrt(252 as std::os::raw::c_int as std::os::raw::c_double);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_volatility_start(options) { return 0 as std::os::raw::c_int }
    let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: std::os::raw::c_int = 0;
    i = 1 as std::os::raw::c_int;
    while i <= period {
        let c: std::os::raw::c_double =
            *input.offset(i as isize) /
                *input.offset((i - 1 as std::os::raw::c_int) as isize) - 1.0f64;
        sum += c;
        sum2 += c * c;
        i += 1
    }
    let fresh0 = output;
    output = output.offset(1);
    *fresh0 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
    i = period + 1 as std::os::raw::c_int;
    while i < size {
        let c_0: std::os::raw::c_double =
            *input.offset(i as isize) /
                *input.offset((i - 1 as std::os::raw::c_int) as isize) - 1.0f64;
        sum += c_0;
        sum2 += c_0 * c_0;
        let cp: std::os::raw::c_double =
            *input.offset((i - period) as isize) /
                *input.offset((i - period - 1 as std::os::raw::c_int) as isize) -
                1.0f64;
        sum -= cp;
        sum2 -= cp * cp;
        let fresh1 = output;
        output = output.offset(1);
        *fresh1 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_volatility_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 14],
                                               &[std::os::raw::c_char; 14]>(b"ti_volatility\x00")).as_ptr(),
                     b"indicators/volatility.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 69 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_volatility_start(options)\x00"
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

