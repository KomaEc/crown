#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn strerror(_: std::os::raw::c_int) -> *mut std::os::raw::c_char;
}
/*
 * Override strerror() to get consistent output across platforms.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub errno_value: std::os::raw::c_int,
    pub errno_str: *const std::os::raw::c_char,
}
// Initialized in run_static_initializers
static mut errno_list: [C2RustUnnamed; 36] =
    [C2RustUnnamed{errno_value: 0, errno_str: 0 as *const std::os::raw::c_char,}; 36];
// Enabled during tests
#[no_mangle]
pub static mut _json_c_strerror_enable: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut errno_buf: [std::os::raw::c_char; 128] =
    unsafe {
        *::std::mem::transmute::<&[u8; 128],
                                 &mut [std::os::raw::c_char; 128]>(b"ERRNO=\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn _json_c_strerror(mut errno_in: std::os::raw::c_int)
 -> *mut std::os::raw::c_char {
    let mut start_idx: std::os::raw::c_int = 0;
    let mut digbuf: [std::os::raw::c_char; 20] = [0; 20];
    let mut ii: std::os::raw::c_int = 0;
    let mut jj: std::os::raw::c_int = 0;
    if _json_c_strerror_enable == 0 { return strerror(errno_in) }
    // Avoid standard functions, so we don't need to include any
	// headers, or guess at signatures.
    ii = 0 as std::os::raw::c_int;
    while !errno_list[ii as usize].errno_str.is_null() {
        let mut errno_str: *const std::os::raw::c_char =
            errno_list[ii as usize].errno_str;
        if errno_list[ii as usize].errno_value != errno_in {
            ii += 1
        } else {
            start_idx =
                (::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
                     std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                     std::os::raw::c_ulong) as
                    std::os::raw::c_int;
            jj = 0 as std::os::raw::c_int;
            while *errno_str.offset(jj as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
                errno_buf[start_idx as usize] =
                    *errno_str.offset(jj as isize);
                jj += 1;
                start_idx += 1
            }
            errno_buf[start_idx as usize] = '\u{0}' as i32 as std::os::raw::c_char;
            return errno_buf.as_mut_ptr()
        }
    }
    // It's not one of the known errno values, return the numeric value.
    ii = 0 as std::os::raw::c_int;
    while errno_in > 10 as std::os::raw::c_int {
        digbuf[ii as usize] =
            (*::std::mem::transmute::<&[u8; 11],
                                      &[std::os::raw::c_char; 11]>(b"0123456789\x00"))[(errno_in
                                                                                    %
                                                                                    10
                                                                                        as
                                                                                        std::os::raw::c_int)
                                                                                   as
                                                                                   usize];
        errno_in /= 10 as std::os::raw::c_int;
        ii += 1
    }
    digbuf[ii as usize] =
        (*::std::mem::transmute::<&[u8; 11],
                                  &[std::os::raw::c_char; 11]>(b"0123456789\x00"))[(errno_in
                                                                                %
                                                                                10
                                                                                    as
                                                                                    std::os::raw::c_int)
                                                                               as
                                                                               usize];
    // Reverse the digits
    start_idx =
        (::std::mem::size_of::<[std::os::raw::c_char; 7]>() as
             std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong) as
            std::os::raw::c_int;
    while ii >= 0 as std::os::raw::c_int {
        errno_buf[start_idx as usize] = digbuf[ii as usize];
        ii -= 1;
        start_idx += 1
    }
    return errno_buf.as_mut_ptr();
}
unsafe extern "C" fn run_static_initializers() {
    errno_list =
        [{
             let mut init =
                 C2RustUnnamed{errno_value: 1 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EPERM\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 2 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOENT\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 3 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ESRCH\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 4 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EINTR\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 5 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EIO\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 6 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENXIO\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 7 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_E2BIG\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 8 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOEXEC\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 9 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EBADF\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 10 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ECHILD\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 11 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EDEADLK\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 12 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOMEM\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 13 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EACCES\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 14 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EFAULT\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 15 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOTBLK\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 16 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EBUSY\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 17 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EEXIST\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 18 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EXDEV\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 19 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENODEV\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 20 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOTDIR\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 21 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EISDIR\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 22 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EINVAL\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 23 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENFILE\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 24 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EMFILE\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 25 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOTTY\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 26 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ETXTBSY\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 27 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EFBIG\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 28 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ENOSPC\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 29 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ESPIPE\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 30 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EROFS\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 31 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EMLINK\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 32 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EPIPE\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 33 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EDOM\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 34 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_ERANGE\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 35 as std::os::raw::c_int,
                               errno_str:
                                   &*(b"undef_EAGAIN\x00" as *const u8 as
                                          *const std::os::raw::c_char).offset(6 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                       as *const std::os::raw::c_char,};
             init
         },
         {
             let mut init =
                 C2RustUnnamed{errno_value: 0 as std::os::raw::c_int,
                               errno_str: 0 as *mut std::os::raw::c_char,};
             init
         }]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
