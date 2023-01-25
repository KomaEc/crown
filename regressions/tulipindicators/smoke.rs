
extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fgets(_: *mut std::os::raw::c_char, _: std::os::raw::c_int, _: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn strtok(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
    #[no_mangle]
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    
    
    
    
    #[no_mangle]
    static mut ti_indicators: [crate::example2::ti_indicator_info; 0];
    
    
    
    
    
    
}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_clock_t = std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type clock_t = __darwin_clock_t;
pub type ti_indicator_start_function
    =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_double) -> std::os::raw::c_int>;
pub type ti_indicator_function
    =
    Option<unsafe extern "C" fn(_: std::os::raw::c_int,
                                _: *const *const std::os::raw::c_double,
                                _: *const std::os::raw::c_double,
                                _: *const *mut std::os::raw::c_double)
               -> std::os::raw::c_int>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor11 { dummy: () }
#[derive(Copy, Clone)]

struct ErasedByPreprocessor12 { dummy: () }
static mut lfails: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut ltests: std::os::raw::c_int = 0 as std::os::raw::c_int;
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
pub static mut tested: [std::os::raw::c_int; 104] =
    [0 as std::os::raw::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/* Compare two arrays, assert each element is equal. */
#[no_mangle]
pub unsafe extern "C" fn compare_arrays(mut a: *mut std::os::raw::c_double,
                                        mut b: *mut std::os::raw::c_double,
                                        mut size_a: std::os::raw::c_int,
                                        mut size_b: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    crate::smoke::ltests+= 1;
    if size_a != size_b {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               36 as std::os::raw::c_int, size_a, size_b);
    }
    if size_a != size_b {
        printf(b"Size mismatch.\n\x00" as *const u8 as *const std::os::raw::c_char);
    }
    i= 0 as std::os::raw::c_int;
    while i < size_a {
        crate::smoke::ltests+= 1;
        if fabs(*a.offset(i as isize) - *b.offset(i as isize)) > 0.001f64 {
            crate::smoke::lfails+= 1;
            printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   42 as std::os::raw::c_int, *a.offset(i as isize),
                   *b.offset(i as isize));
        }
        i+= 1
    };
}
/*Return next non-comment, non-blank line.*/
#[no_mangle]
pub unsafe extern "C" fn next_line(mut fp: *mut FILE) -> *mut std::os::raw::c_char {
    static mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    while !fgets(buf.as_mut_ptr(), 1024 as std::os::raw::c_int, fp).is_null() {
        /*Skip Comments*/
        if buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '#' as i32 {
            continue ;
        }
        /*Skip blank lines*/
        if buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 10 as std::os::raw::c_int
               ||
               buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   13 as std::os::raw::c_int {
            continue ;
        }
        return buf.as_mut_ptr()
    }();
    return 0 as *mut std::os::raw::c_char;
}
/*Read in float array.*/
#[no_mangle]
pub unsafe extern "C" fn get_array(mut fp: Option<&mut FILE>,
                                   mut s: *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut line: *mut std::os::raw::c_char = next_line(fp.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    if *line.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '{' as i32 {
        printf(b"Bad input\n\x00" as *const u8 as *const std::os::raw::c_char);
        crate::smoke::ltests+= 1;
        if 0 as std::os::raw::c_int == 0 {
            crate::smoke::lfails+= 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   70 as std::os::raw::c_int);
        }
        return 0 as std::os::raw::c_int
    }
    //#pragma warning(disable:4996) //MSVC
    let mut num: *mut std::os::raw::c_char =
        strtok(line.offset(1 as std::os::raw::c_int as isize),
               b",}\r\n\x00" as *const u8 as *const std::os::raw::c_char);
    if num.is_null() {(); return 0 as std::os::raw::c_int }
    let mut inp: *mut std::os::raw::c_double = s;
    loop  {
        *inp= atof(num);
        inp= inp.offset(1);
        num=
            strtok(0 as *mut std::os::raw::c_char,
                   b",}\r\n\x00" as *const u8 as *const std::os::raw::c_char);
        if num.is_null() {(); break ; }
    }
    return inp.offset_from(s) as std::os::raw::c_long as std::os::raw::c_int;
}
/*Read in options, inputs, answers, and test.*/
#[no_mangle]
pub unsafe extern "C" fn test_ind_name(mut fp: *mut FILE,
                                       mut name: *const std::os::raw::c_char,
                                       mut count: std::os::raw::c_int) {
    /*Find indicator from name.*/
    let mut info: *const crate::example2::ti_indicator_info = crate::indicators_index::ti_find_indicator(name);
    if info.is_null() {();
        printf(b"Couldn\'t find indicator %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, name);
        crate::smoke::ltests+= 1;
        if 0 as std::os::raw::c_int == 0 {
            crate::smoke::lfails+= 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   99 as std::os::raw::c_int);
        }
        return
    }
    let ind_num: std::os::raw::c_int =
        info.offset_from(crate::smoke::ti_indicators.as_mut_ptr()) as std::os::raw::c_long
            as std::os::raw::c_int;
    if count != 0 { crate::smoke::tested[ind_num as usize]= 1 as std::os::raw::c_int }
    let mut options: [std::os::raw::c_double; 10] = [0.; 10];
    let mut o: *mut std::os::raw::c_double = options.as_mut_ptr();
    let mut s: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    loop  {
        s=
            strtok(0 as *mut std::os::raw::c_char,
                   b" \n\r\x00" as *const u8 as *const std::os::raw::c_char);
        if s.is_null() {(); break ; }
        *o= atof(s);
        o= o.offset(1)
    }
    crate::smoke::ltests+= 1;
    if o.offset_from(options.as_mut_ptr()) as std::os::raw::c_long as
           std::os::raw::c_int != (*info).options {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               117 as std::os::raw::c_int,
               o.offset_from(options.as_mut_ptr()) as std::os::raw::c_long as
                   std::os::raw::c_int, (*info).options);
    }
    let mut i: std::os::raw::c_int = 0;
    let mut inputs: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut answers: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut outputs: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut input_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i= 0 as std::os::raw::c_int;
    while i < (*info).inputs {
        inputs[i as usize]=
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        input_size= get_array(fp.as_mut(), inputs[i as usize]);
        i+= 1
    }
    let mut answer_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i= 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        answers[i as usize]=
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        outputs[i as usize]=
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        answer_size= get_array(fp.as_mut(), answers[i as usize]);
        i+= 1
    }
    let ret: std::os::raw::c_int =
        (*info).indicator.expect("non-null function pointer")(input_size,
                                                              inputs.as_mut_ptr()
                                                                  as
                                                                  *const *const std::os::raw::c_double,
                                                              options.as_mut_ptr(),
                                                              outputs.as_mut_ptr());
    crate::smoke::ltests+= 1;
    if !(ret == 0 as std::os::raw::c_int) {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               139 as std::os::raw::c_int);
    }
    let mut output_size: std::os::raw::c_int =
        input_size -
            (*info).start.expect("non-null function pointer")(options.as_mut_ptr());
    if output_size < 0 as std::os::raw::c_int { output_size= 0 as std::os::raw::c_int }
    i= 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        compare_arrays(answers[i as usize], outputs[i as usize], answer_size,
                       output_size);
        i+= 1
    }
    i= 0 as std::os::raw::c_int;
    while i < (*info).inputs {
        free(inputs[i as usize] as *mut std::os::raw::c_void);
        i+= 1
    }
    i= 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        free(answers[i as usize] as *mut std::os::raw::c_void);
        i+= 1
    }
    i= 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        free(outputs[i as usize] as *mut std::os::raw::c_void);
        i+= 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn test(mut fname: *const std::os::raw::c_char,
                              mut count: std::os::raw::c_int) {
    /*Read in test values from external file.*/
    printf(b"%s:\n\x00" as *const u8 as *const std::os::raw::c_char, fname);
    let mut fp: *mut FILE =
        fopen(fname, b"r\x00" as *const u8 as *const std::os::raw::c_char);
    if fp.is_null() {();
        crate::smoke::ltests+= 1;
        if 0 as std::os::raw::c_int == 0 {
            crate::smoke::lfails+= 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   159 as std::os::raw::c_int);
        }
        return
    }
    let mut line: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    loop  {
        line= next_line(fp);
        if line.is_null() {(); break ; }
        /*Looking for function name.*/
        if (*line.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int) <
               'a' as i32 ||
               *line.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >
                   'z' as i32 {
            crate::smoke::ltests+= 1;
            if 0 as std::os::raw::c_int == 0 {
                crate::smoke::lfails+= 1;
                printf(b"%s:%d error \n\x00" as *const u8 as
                           *const std::os::raw::c_char,
                       b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                       169 as std::os::raw::c_int);
            }
        } else {
            /*Found function*/
            let mut name: *mut std::os::raw::c_char =
                strtok(line,
                       b" \n\r\x00" as *const u8 as *const std::os::raw::c_char);
            let ts: std::os::raw::c_int = crate::smoke::ltests;
            let fs: std::os::raw::c_int = crate::smoke::lfails;
            let start: clock_t = clock();
            printf(b"\t%-16s\x00" as *const u8 as *const std::os::raw::c_char, name);
            test_ind_name(fp, name, count);
            printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
                       *const std::os::raw::c_char, crate::smoke::ltests - ts - (crate::smoke::lfails - fs),
                   crate::smoke::lfails - fs,
                   clock().wrapping_sub(start).wrapping_mul(1000 as
                                                                std::os::raw::c_int as
                                                                std::os::raw::c_ulong).wrapping_div(1000000
                                                                                                as
                                                                                                std::os::raw::c_int
                                                                                                as
                                                                                                std::os::raw::c_ulong)
                       as std::os::raw::c_int);
        }
    }
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn test_version() {
    crate::smoke::ltests+= 1;
    if !(strcmp(b"0.8.4\x00" as *const u8 as *const std::os::raw::c_char,
                crate::indicators_index::ti_version()) == 0 as std::os::raw::c_int) {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               183 as std::os::raw::c_int);
    }
    crate::smoke::ltests+= 1;
    if !(1537377628 as std::os::raw::c_int as std::os::raw::c_long == crate::indicators_index::ti_build()) {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               184 as std::os::raw::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_buffer() {
    let mut b: *mut crate::indicators::adxr::ti_buffer = crate::utils::buffer::ti_buffer_new(3 as std::os::raw::c_int);
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 5.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 5.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               190 as std::os::raw::c_int, (*b).sum, 5.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 5.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 5.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 10.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               191 as std::os::raw::c_int, (*b).sum, 10.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 1.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 11.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               192 as std::os::raw::c_int, (*b).sum, 11.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 1.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 7.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               193 as std::os::raw::c_int, (*b).sum, 7.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 3.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 3.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               194 as std::os::raw::c_int, (*b).sum, 5.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 1.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 1.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 5.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               195 as std::os::raw::c_int, (*b).sum, 5.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 2.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 2.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 6.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               196 as std::os::raw::c_int, (*b).sum, 6.0f64);
    }
    if (*b).pushes >= (*b).size {
        (*b).sum-= *(*b).vals.as_mut_ptr().offset((*b).index as isize)
    }
    (*b).sum+= 3.0f64;
    *(*b).vals.as_mut_ptr().offset((*b).index as isize) = 3.0f64;
    (*b).pushes+= 1 as std::os::raw::c_int;
    (*b).index= (*b).index + 1 as std::os::raw::c_int;
    if (*b).index >= (*b).size { (*b).index= 0 as std::os::raw::c_int }
    crate::smoke::ltests+= 1;
    if fabs((*b).sum - 6.0f64) > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               197 as std::os::raw::c_int, (*b).sum, 6.0f64);
    }
    crate::smoke::ltests+= 1;
    if fabs(*(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                 1 as std::os::raw::c_int +
                                                 0 as std::os::raw::c_int) %
                                                (*b).size) as isize) - 3.0f64)
           > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               199 as std::os::raw::c_int,
               *(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                    1 as std::os::raw::c_int +
                                                    0 as std::os::raw::c_int) %
                                                   (*b).size) as isize),
               3.0f64);
    }
    crate::smoke::ltests+= 1;
    if fabs(*(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                 1 as std::os::raw::c_int +
                                                 -(1 as std::os::raw::c_int)) %
                                                (*b).size) as isize) - 2.0f64)
           > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               200 as std::os::raw::c_int,
               *(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                    1 as std::os::raw::c_int +
                                                    -(1 as std::os::raw::c_int)) %
                                                   (*b).size) as isize),
               2.0f64);
    }
    crate::smoke::ltests+= 1;
    if fabs(*(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                 1 as std::os::raw::c_int +
                                                 -(2 as std::os::raw::c_int)) %
                                                (*b).size) as isize) - 1.0f64)
           > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               201 as std::os::raw::c_int,
               *(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                    1 as std::os::raw::c_int +
                                                    -(2 as std::os::raw::c_int)) %
                                                   (*b).size) as isize),
               1.0f64);
    }
    crate::smoke::ltests+= 1;
    if fabs(*(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                 1 as std::os::raw::c_int +
                                                 -(3 as std::os::raw::c_int)) %
                                                (*b).size) as isize) - 3.0f64)
           > 0.001f64 {
        crate::smoke::lfails+= 1;
        printf(b"%s:%d (%f != %f)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               202 as std::os::raw::c_int,
               *(*b).vals.as_mut_ptr().offset((((*b).index + (*b).size -
                                                    1 as std::os::raw::c_int +
                                                    -(3 as std::os::raw::c_int)) %
                                                   (*b).size) as isize),
               3.0f64);
    }
    crate::utils::buffer::ti_buffer_free(b);
}
unsafe fn main_0() -> std::os::raw::c_int {
    printf(b"TI TEST SUITE\n\x00" as *const u8 as *const std::os::raw::c_char);
    let ts: std::os::raw::c_int = crate::smoke::ltests;
    let fs: std::os::raw::c_int = crate::smoke::lfails;
    let start: clock_t = clock();
    printf(b"\t%-16s\x00" as *const u8 as *const std::os::raw::c_char,
           b"buffer\x00" as *const u8 as *const std::os::raw::c_char);
    test_buffer();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, crate::smoke::ltests - ts - (crate::smoke::lfails - fs), crate::smoke::lfails - fs,
           clock().wrapping_sub(start).wrapping_mul(1000 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong).wrapping_div(1000000
                                                                                        as
                                                                                        std::os::raw::c_int
                                                                                        as
                                                                                        std::os::raw::c_ulong)
               as std::os::raw::c_int);
    let ts_0: std::os::raw::c_int = crate::smoke::ltests;
    let fs_0: std::os::raw::c_int = crate::smoke::lfails;
    let start_0: clock_t = clock();
    printf(b"\t%-16s\x00" as *const u8 as *const std::os::raw::c_char,
           b"version\x00" as *const u8 as *const std::os::raw::c_char);
    test_version();
    printf(b"pass:%2d   fail:%2d   %4dms\n\x00" as *const u8 as
               *const std::os::raw::c_char, crate::smoke::ltests - ts_0 - (crate::smoke::lfails - fs_0),
           crate::smoke::lfails - fs_0,
           clock().wrapping_sub(start_0).wrapping_mul(1000 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong).wrapping_div(1000000
                                                                                          as
                                                                                          std::os::raw::c_int
                                                                                          as
                                                                                          std::os::raw::c_ulong)
               as std::os::raw::c_int);
    test(b"tests/untest.txt\x00" as *const u8 as *const std::os::raw::c_char,
         0 as std::os::raw::c_int);
    test(b"tests/atoz.txt\x00" as *const u8 as *const std::os::raw::c_char,
         1 as std::os::raw::c_int);
    test(b"tests/extra.txt\x00" as *const u8 as *const std::os::raw::c_char,
         1 as std::os::raw::c_int);
    let mut i: std::os::raw::c_int = 0;
    i= 0 as std::os::raw::c_int;
    while i < 104 as std::os::raw::c_int {
        if crate::smoke::tested[i as usize] == 0 {
            printf(b"WARNING: no test for %s\n\x00" as *const u8 as
                       *const std::os::raw::c_char,
                   (*crate::smoke::ti_indicators.as_mut_ptr().offset(i as isize)).name);
        }
        i+= 1
    }
    if crate::smoke::lfails == 0 as std::os::raw::c_int {
        printf(b"ALL TESTS PASSED (%d/%d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, crate::smoke::ltests, crate::smoke::ltests);
    } else {
        printf(b"%d TESTS FAILED (of %d)\n\x00" as *const u8 as
                   *const std::os::raw::c_char, crate::smoke::lfails, crate::smoke::ltests);
    }
    return (crate::smoke::lfails != 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
// #[main]
// pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
