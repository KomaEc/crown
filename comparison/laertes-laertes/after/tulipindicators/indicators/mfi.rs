
extern "C" {
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
}
pub use crate::utils::buffer::ti_buffer_free;
pub use crate::utils::buffer::ti_buffer_new;
// #[derive(Copy, Clone)]

pub use crate::indicators::adxr::ti_buffer;
#[no_mangle]
pub unsafe extern "C" fn ti_mfi_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mfi(mut size: std::os::raw::c_int,
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
    let mut volume: *const std::os::raw::c_double =
        *inputs.offset(3 as std::os::raw::c_int as isize);
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_mfi_start(options) { return 0 as std::os::raw::c_int }
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut ytyp: std::os::raw::c_double =
        (*high.offset(0 as std::os::raw::c_int as isize) +
             *low.offset(0 as std::os::raw::c_int as isize) +
             *close.offset(0 as std::os::raw::c_int as isize)) * (1.0f64 / 3.0f64);
    let mut i: std::os::raw::c_int = 0;
    let mut up: *mut ti_buffer = ti_buffer_new(period);
    let mut down: *mut ti_buffer = ti_buffer_new(period);
    i = 1 as std::os::raw::c_int;
    while i < size {
        let typ: std::os::raw::c_double =
            (*high.offset(i as isize) + *low.offset(i as isize) +
                 *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        let bar: std::os::raw::c_double = typ * *volume.offset(i as isize);
        if typ > ytyp {
            if (*up).pushes >= (*up).size {
                (*up).sum -=
                    *(*up).vals.as_mut_ptr().offset((*up).index as isize)
            }
            (*up).sum += bar;
            *(*up).vals.as_mut_ptr().offset((*up).index as isize) = bar;
            (*up).pushes += 1 as std::os::raw::c_int;
            (*up).index = (*up).index + 1 as std::os::raw::c_int;
            if (*up).index >= (*up).size { (*up).index = 0 as std::os::raw::c_int }
            if (*down).pushes >= (*down).size {
                (*down).sum -=
                    *(*down).vals.as_mut_ptr().offset((*down).index as isize)
            }
            (*down).sum += 0.0f64;
            *(*down).vals.as_mut_ptr().offset((*down).index as isize) =
                0.0f64;
            (*down).pushes += 1 as std::os::raw::c_int;
            (*down).index = (*down).index + 1 as std::os::raw::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as std::os::raw::c_int
            }
        } else if typ < ytyp {
            if (*down).pushes >= (*down).size {
                (*down).sum -=
                    *(*down).vals.as_mut_ptr().offset((*down).index as isize)
            }
            (*down).sum += bar;
            *(*down).vals.as_mut_ptr().offset((*down).index as isize) = bar;
            (*down).pushes += 1 as std::os::raw::c_int;
            (*down).index = (*down).index + 1 as std::os::raw::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as std::os::raw::c_int
            }
            if (*up).pushes >= (*up).size {
                (*up).sum -=
                    *(*up).vals.as_mut_ptr().offset((*up).index as isize)
            }
            (*up).sum += 0.0f64;
            *(*up).vals.as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as std::os::raw::c_int;
            (*up).index = (*up).index + 1 as std::os::raw::c_int;
            if (*up).index >= (*up).size { (*up).index = 0 as std::os::raw::c_int }
        } else {
            if (*up).pushes >= (*up).size {
                (*up).sum -=
                    *(*up).vals.as_mut_ptr().offset((*up).index as isize)
            }
            (*up).sum += 0.0f64;
            *(*up).vals.as_mut_ptr().offset((*up).index as isize) = 0.0f64;
            (*up).pushes += 1 as std::os::raw::c_int;
            (*up).index = (*up).index + 1 as std::os::raw::c_int;
            if (*up).index >= (*up).size { (*up).index = 0 as std::os::raw::c_int }
            if (*down).pushes >= (*down).size {
                (*down).sum -=
                    *(*down).vals.as_mut_ptr().offset((*down).index as isize)
            }
            (*down).sum += 0.0f64;
            *(*down).vals.as_mut_ptr().offset((*down).index as isize) =
                0.0f64;
            (*down).pushes += 1 as std::os::raw::c_int;
            (*down).index = (*down).index + 1 as std::os::raw::c_int;
            if (*down).index >= (*down).size {
                (*down).index = 0 as std::os::raw::c_int
            }
        }
        ytyp = typ;
        if i >= period {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = (*up).sum / ((*up).sum + (*down).sum) * 100.0f64
        }
        i += 1
    }
    ti_buffer_free(up);
    ti_buffer_free(down);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_mfi_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*::std::mem::transmute::<&[u8; 7],
                                               &[std::os::raw::c_char; 7]>(b"ti_mfi\x00")).as_ptr(),
                     b"indicators/mfi.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 78 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_mfi_start(options)\x00"
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

