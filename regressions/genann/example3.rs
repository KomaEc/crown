extern "C" {
    pub type __sFILEX;
    #[no_mangle]
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn exit(_: std::os::raw::c_int) -> !;

}
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *const std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
impl Default for __sbuf {
    fn default() -> Self {
        Self {
            _base: std::ptr::null_mut(),
            _size: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *const std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *const std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void) -> std::os::raw::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            _: *mut std::os::raw::c_void,
            _: *mut std::os::raw::c_char,
            _: std::os::raw::c_int,
        ) -> std::os::raw::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(
            _: *mut std::os::raw::c_void,
            _: fpos_t,
            _: std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            _: *mut std::os::raw::c_void,
            _: *const std::os::raw::c_char,
            _: std::os::raw::c_int,
        ) -> std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *const __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
impl Default for __sFILE {
    fn default() -> Self {
        Self {
            _p: std::ptr::null_mut(),
            _r: Default::default(),
            _w: Default::default(),
            _flags: Default::default(),
            _file: Default::default(),
            _bf: Default::default(),
            _lbfsize: Default::default(),
            _cookie: std::ptr::null_mut(),
            _close: Default::default(),
            _read: Default::default(),
            _seek: Default::default(),
            _write: Default::default(),
            _ub: Default::default(),
            _extra: std::ptr::null_mut(),
            _ur: Default::default(),
            _ubuf: Default::default(),
            _nbuf: Default::default(),
            _lb: Default::default(),
            _blksize: Default::default(),
            _offset: Default::default(),
        }
    }
}

pub type FILE = __sFILE;
pub type genann_actfun =
    Option<unsafe extern "C" fn(_: std::os::raw::c_double) -> std::os::raw::c_double>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1;
impl Default for ErasedByPreprocessor1 {
    fn default() -> Self {
        Self {}
    }
}

#[no_mangle]
pub static mut save_name: *const std::os::raw::c_char =
    b"example/xor.ann\x00" as *const u8 as *const std::os::raw::c_char;
unsafe fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *const *const std::os::raw::c_char,
) -> std::os::raw::c_int {
    printf(b"GENANN example 3.\n\x00" as *const u8 as *const std::os::raw::c_char);
    printf(
        b"Load a saved ANN to solve the XOR function.\n\x00" as *const u8
            as *const std::os::raw::c_char,
    );
    let mut saved: *mut FILE = fopen(
        save_name,
        b"r\x00" as *const u8 as *const std::os::raw::c_char,
    );
    if saved.is_null() {
        ();
        printf(
            b"Couldn\'t open file: %s\n\x00" as *const u8 as *const std::os::raw::c_char,
            save_name,
        );
        exit(1 as std::os::raw::c_int);
    }
    let mut ann: *mut crate::example1::genann = crate::genann::genann_read(saved);
    fclose(saved);
    if ann.is_null() {
        ();
        printf(
            b"Error loading ANN from file: %s.\x00" as *const u8 as *const std::os::raw::c_char,
            save_name,
        );
        exit(1 as std::os::raw::c_int);
    }
    /* Input data for the XOR function. */
    let input: [[std::os::raw::c_double; 2]; 4] = [
        [
            0 as std::os::raw::c_int as std::os::raw::c_double,
            0 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            0 as std::os::raw::c_int as std::os::raw::c_double,
            1 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            1 as std::os::raw::c_int as std::os::raw::c_double,
            0 as std::os::raw::c_int as std::os::raw::c_double,
        ],
        [
            1 as std::os::raw::c_int as std::os::raw::c_double,
            1 as std::os::raw::c_int as std::os::raw::c_double,
        ],
    ];
    /* Run the network and see what it predicts. */
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[0 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[0 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[0 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[1 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[1 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[1 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[2 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[2 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[2 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\x00" as *const u8 as *const std::os::raw::c_char,
        input[3 as std::os::raw::c_int as usize][0 as std::os::raw::c_int as usize],
        input[3 as std::os::raw::c_int as usize][1 as std::os::raw::c_int as usize],
        *crate::genann::genann_run(
            ann.as_mut(),
            input[3 as std::os::raw::c_int as usize].as_ptr(),
        ),
    );
    crate::genann::genann_free(Some(Box::from_raw(ann)));
    return 0 as std::os::raw::c_int;
}
// pub fn main() {
//     let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
//     };
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
//                                     args.as_mut_ptr() as
//                                         *mut *mut std::os::raw::c_char) as i32)
//     }
// }
