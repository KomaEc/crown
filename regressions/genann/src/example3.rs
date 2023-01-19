use ::libc;
extern "C" {

    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;

}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *const libc::c_char,
    pub _IO_read_end: *const libc::c_char,
    pub _IO_read_base: *const libc::c_char,
    pub _IO_write_base: *const libc::c_char,
    pub _IO_write_ptr: *const libc::c_char,
    pub _IO_write_end: *const libc::c_char,
    pub _IO_buf_base: *const libc::c_char,
    pub _IO_buf_end: *const libc::c_char,
    pub _IO_save_base: *const libc::c_char,
    pub _IO_backup_base: *const libc::c_char,
    pub _IO_save_end: *const libc::c_char,
    pub _markers: *const _IO_marker,
    pub _chain: *const _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *const libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *const crate::src::example4::_IO_codecvt,
    pub _wide_data: *const crate::src::test::_IO_wide_data,
    pub _freeres_list: *const _IO_FILE,
    pub _freeres_buf: *const libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
impl Default for _IO_FILE {
    fn default() -> Self {
        Self {
            _flags: Default::default(),
            _IO_read_ptr: std::ptr::null_mut(),
            _IO_read_end: std::ptr::null_mut(),
            _IO_read_base: std::ptr::null_mut(),
            _IO_write_base: std::ptr::null_mut(),
            _IO_write_ptr: std::ptr::null_mut(),
            _IO_write_end: std::ptr::null_mut(),
            _IO_buf_base: std::ptr::null_mut(),
            _IO_buf_end: std::ptr::null_mut(),
            _IO_save_base: std::ptr::null_mut(),
            _IO_backup_base: std::ptr::null_mut(),
            _IO_save_end: std::ptr::null_mut(),
            _markers: std::ptr::null_mut(),
            _chain: std::ptr::null_mut(),
            _fileno: Default::default(),
            _flags2: Default::default(),
            _old_offset: Default::default(),
            _cur_column: Default::default(),
            _vtable_offset: Default::default(),
            _shortbuf: Default::default(),
            _lock: std::ptr::null_mut(),
            _offset: Default::default(),
            _codecvt: std::ptr::null_mut(),
            _wide_data: std::ptr::null_mut(),
            _freeres_list: std::ptr::null_mut(),
            _freeres_buf: std::ptr::null_mut(),
            __pad5: Default::default(),
            _mode: Default::default(),
            _unused2: Default::default(),
        }
    }
}

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type genann_actfun = Option<unsafe extern "C" fn(libc::c_double) -> libc::c_double>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1;
impl Default for ErasedByPreprocessor1 {
    fn default() -> Self {
        Self {}
    }
}

#[no_mangle]
pub static mut save_name: *const libc::c_char =
    b"example/xor.ann\0" as *const u8 as *const libc::c_char;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *const *const libc::c_char) -> libc::c_int {
    printf(b"GENANN example 3.\n\0" as *const u8 as *const libc::c_char);
    printf(b"Load a saved ANN to solve the XOR function.\n\0" as *const u8 as *const libc::c_char);
    let mut saved = fopen(save_name, b"r\0" as *const u8 as *const libc::c_char);
    if saved.is_null() {
        ();
        printf(
            b"Couldn't open file: %s\n\0" as *const u8 as *const libc::c_char,
            save_name,
        );
        exit(1 as libc::c_int);
    }
    let mut ann = crate::src::genann::genann_read(saved);
    fclose(saved);
    if ann.is_null() {
        ();
        printf(
            b"Error loading ANN from file: %s.\0" as *const u8 as *const libc::c_char,
            save_name,
        );
        exit(1 as libc::c_int);
    }
    let input: [[libc::c_double; 2]; 4] = [
        [
            0 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            0 as libc::c_int as libc::c_double,
        ],
        [
            1 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
        ],
    ];
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[0 as libc::c_int as usize][0 as libc::c_int as usize],
        input[0 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[0 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[1 as libc::c_int as usize][0 as libc::c_int as usize],
        input[1 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[1 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[2 as libc::c_int as usize][0 as libc::c_int as usize],
        input[2 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[2 as libc::c_int as usize]).as_ptr()),
    );
    printf(
        b"Output for [%1.f, %1.f] is %1.f.\n\0" as *const u8 as *const libc::c_char,
        input[3 as libc::c_int as usize][0 as libc::c_int as usize],
        input[3 as libc::c_int as usize][1 as libc::c_int as usize],
        *crate::src::genann::genann_run(ann.as_mut(), (input[3 as libc::c_int as usize]).as_ptr()),
    );
    crate::src::genann::genann_free(Some(Box::from_raw(ann)));
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
