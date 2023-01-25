
extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
#[no_mangle]
pub unsafe extern "C" fn ti_cci_start(mut options: *const std::os::raw::c_double)
 -> std::os::raw::c_int {
    let period: std::os::raw::c_int =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    return (period - 1 as std::os::raw::c_int) * 2 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_cci(mut size: std::os::raw::c_int,
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
    let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_cci_start(options) { return 0 as std::os::raw::c_int }
    let mut output: *mut std::os::raw::c_double =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut sum: *mut crate::indicators::adxr::ti_buffer = crate::utils::buffer::ti_buffer_new(period);
    let mut i: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    i= 0 as std::os::raw::c_int;
    while i < size {
        let today: std::os::raw::c_double =
            (*high.offset(i as isize) + *low.offset(i as isize) +
                 *close.offset(i as isize)) * (1.0f64 / 3.0f64);
        if (*sum).pushes >= (*sum).size {
            (*sum).sum-=
                *(*sum).vals.as_mut_ptr().offset((*sum).index as isize)
        }
        (*sum).sum+= today;
        *(*sum).vals.as_mut_ptr().offset((*sum).index as isize) = today;
        (*sum).pushes+= 1 as std::os::raw::c_int;
        (*sum).index= (*sum).index + 1 as std::os::raw::c_int;
        if (*sum).index >= (*sum).size { (*sum).index= 0 as std::os::raw::c_int }
        let avg: std::os::raw::c_double = (*sum).sum * scale;
        if i >= period * 2 as std::os::raw::c_int - 2 as std::os::raw::c_int {
            let mut acc: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
            j= 0 as std::os::raw::c_int;
            while j < period {
                acc+=
                    fabs(avg - *(*sum).vals.as_mut_ptr().offset(j as isize));
                j+= 1
            }
            let mut cci: std::os::raw::c_double = acc * scale;
            cci*= 0.015f64;
            cci= (today - avg) / cci;
            let fresh0 = output;
            output= output.offset(1);
            *fresh0= cci
        }
        i+= 1
    }
    crate::utils::buffer::ti_buffer_free(sum);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_cci_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn(b"ti_cci\x00" as *const u8 as *const i8,
                     b"indicators/cci.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 73 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_cci_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
