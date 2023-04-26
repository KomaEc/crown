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
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut crate::src::example4::_IO_codecvt,
    pub _wide_data: *mut crate::src::test::_IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type genann_actfun = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor1 { dummy: () }
#[no_mangle]
pub static mut save_name: *const libc::c_char = b"example/xor.ann\0" as *const u8
    as *const libc::c_char;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    printf(b"GENANN example 3.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Load a saved ANN to solve the XOR function.\n\0" as *const u8
            as *const libc::c_char,
    );
    let mut saved = fopen(crate::src::example3::save_name, b"r\0" as *const u8 as *const libc::c_char);
    if saved.is_null() {();
        printf(
            b"Couldn't open file: %s\n\0" as *const u8 as *const libc::c_char,
            crate::src::example3::save_name,
        );
        exit(1 as libc::c_int);
    }
    let mut ann = crate::src::genann::genann_read(saved);
    fclose(saved);
    if ann.is_null() {();
        printf(
            b"Error loading ANN from file: %s.\0" as *const u8 as *const libc::c_char,
            crate::src::example3::save_name,
        );
        exit(1 as libc::c_int);
    }
    let input: [[libc::c_double; 2]; 4] = [
        [0 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [0 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 0 as libc::c_int as libc::c_double],
        [1 as libc::c_int as libc::c_double, 1 as libc::c_int as libc::c_double],
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
    crate::src::genann::genann_free(ann);
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
