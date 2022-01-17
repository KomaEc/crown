
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn fgets(__s: *mut std::os::raw::c_char, __n: std::os::raw::c_int, __stream: *mut FILE)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn memcmp(_: *const std::os::raw::c_void, _: *const std::os::raw::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strchr(_: *const std::os::raw::c_char, _: std::os::raw::c_int) -> *mut std::os::raw::c_char;
    #[no_mangle]
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/* ----------------------------------------------------------------------- */
/* with -D C2STR: convert tccdefs.h to C-strings */
/* replace native host macros by compile-time versions */
#[no_mangle]
pub static mut platform_macros: [*const std::os::raw::c_char; 31] =
    [b"__i386__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_I386\x00" as *const u8 as *const std::os::raw::c_char,
     b"__x86_64__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_X86_64\x00" as *const u8 as *const std::os::raw::c_char,
     b"_WIN32\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_PE\x00" as *const u8 as *const std::os::raw::c_char,
     b"__arm__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_ARM\x00" as *const u8 as *const std::os::raw::c_char,
     b"__ARM_EABI__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_ARM_EABI\x00" as *const u8 as *const std::os::raw::c_char,
     b"__aarch64__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_ARM64\x00" as *const u8 as *const std::os::raw::c_char,
     b"__riscv\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_RISCV64\x00" as *const u8 as *const std::os::raw::c_char,
     b"__APPLE__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TCC_TARGET_MACHO\x00" as *const u8 as *const std::os::raw::c_char,
     b"__FreeBSD__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TARGETOS_FreeBSD\x00" as *const u8 as *const std::os::raw::c_char,
     b"__FreeBSD_kernel__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TARGETOS_FreeBSD_kernel\x00" as *const u8 as *const std::os::raw::c_char,
     b"__OpenBSD__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TARGETOS_OpenBSD\x00" as *const u8 as *const std::os::raw::c_char,
     b"__NetBSD__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TARGETOS_NetBSD\x00" as *const u8 as *const std::os::raw::c_char,
     b"__linux__\x00" as *const u8 as *const std::os::raw::c_char,
     b"TARGETOS_Linux\x00" as *const u8 as *const std::os::raw::c_char,
     b"__SIZEOF_POINTER__\x00" as *const u8 as *const std::os::raw::c_char,
     b"PTR_SIZE\x00" as *const u8 as *const std::os::raw::c_char,
     b"__SIZEOF_LONG__\x00" as *const u8 as *const std::os::raw::c_char,
     b"LONG_SIZE\x00" as *const u8 as *const std::os::raw::c_char,
     0 as *const std::os::raw::c_char];
#[no_mangle]
pub unsafe extern "C" fn isid(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c >= 'a' as i32 && c <= 'z' as i32 ||
                c >= 'A' as i32 && c <= 'Z' as i32 ||
                c >= '0' as i32 && c <= '9' as i32 || c == '_' as i32) as
               std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isspc(mut c: std::os::raw::c_int) -> std::os::raw::c_int {
    return (c as std::os::raw::c_uchar as std::os::raw::c_int <= ' ' as i32 &&
                c != 0 as std::os::raw::c_int) as std::os::raw::c_int;
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut l: [std::os::raw::c_char; 1000] = [0; 1000];
    let mut l2: [std::os::raw::c_char; 1000] = [0; 1000];
    let mut p: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut q: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut p0: *mut std::os::raw::c_char = 0 as *mut std::os::raw::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut op: *mut FILE = 0 as *mut FILE;
    let mut c: std::os::raw::c_int = 0;
    let mut e: std::os::raw::c_int = 0;
    let mut f: std::os::raw::c_int = 0;
    let mut s: std::os::raw::c_int = 0;
    let mut cmt: std::os::raw::c_int = 0;
    let mut cmt_n: std::os::raw::c_int = 0;
    let mut r: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    if argc < 3 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    fp =
        fopen(*argv.offset(1 as std::os::raw::c_int as isize),
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    op =
        fopen(*argv.offset(2 as std::os::raw::c_int as isize),
              b"wb\x00" as *const u8 as *const std::os::raw::c_char);
    if fp.is_null() || op.is_null() {
        fprintf(stderr,
                b"c2str: file error\n\x00" as *const u8 as
                    *const std::os::raw::c_char);
        return 1 as std::os::raw::c_int
    }
    cmt_n = 0 as std::os::raw::c_int;
    cmt = cmt_n;
    's_55:
        loop  {
            p = l.as_mut_ptr();
            loop  {
                if !fgets(p,
                          (::std::mem::size_of::<[std::os::raw::c_char; 1000]>() as
                               std::os::raw::c_ulong).wrapping_sub(p.offset_from(l.as_mut_ptr())
                                                               as std::os::raw::c_long
                                                               as
                                                               std::os::raw::c_ulong)
                              as std::os::raw::c_int, fp).is_null() {
                    p = strchr(p, 0 as std::os::raw::c_int);
                    while p > l.as_mut_ptr() &&
                              isspc(*p.offset(-(1 as std::os::raw::c_int) as isize) as
                                        std::os::raw::c_int) != 0 {
                        p = p.offset(-1)
                    }
                    *p = 0 as std::os::raw::c_int as std::os::raw::c_char
                } else if p == l.as_mut_ptr() { break 's_55 ; }
                /* check for continuation */
                if !(p > l.as_mut_ptr() &&
                         *p.offset(-(1 as std::os::raw::c_int) as isize) as
                             std::os::raw::c_int == '\\' as i32) {
                    break ;
                }
                *p.offset(-(1 as std::os::raw::c_int) as isize) =
                    ' ' as i32 as std::os::raw::c_char
            }
            /* count & skip leading spaces */
            p = l.as_mut_ptr();
            q = l2.as_mut_ptr();
            f = 0 as std::os::raw::c_int;
            while *p as std::os::raw::c_int != 0 && isspc(*p as std::os::raw::c_int) != 0 {
                p = p.offset(1);
                f += 1
            }
            /* handle comments */
            if *p.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                   '/' as i32 && cmt == 0 as std::os::raw::c_int {
                if *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '*' as i32 {
                    cmt = 2 as std::os::raw::c_int
                }
                if *p.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                       '/' as i32 {
                    cmt = 1 as std::os::raw::c_int
                }
            }
            if cmt != 0 {
                fprintf(op, b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                        l.as_mut_ptr());
                cmt_n += 1;
                if cmt_n == 1 as std::os::raw::c_int {
                    fprintf(op,
                            b" (converted, do not edit this file)\x00" as
                                *const u8 as *const std::os::raw::c_char);
                }
                fprintf(op, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
                if cmt == 1 as std::os::raw::c_int { cmt = 0 as std::os::raw::c_int }
                if cmt == 2 as std::os::raw::c_int {
                    p = strchr(l.as_mut_ptr(), 0 as std::os::raw::c_int);
                    if p >= l.as_mut_ptr().offset(2 as std::os::raw::c_int as isize)
                           &&
                           *p.offset(-(1 as std::os::raw::c_int) as isize) as
                               std::os::raw::c_int == '/' as i32 &&
                           *p.offset(-(2 as std::os::raw::c_int) as isize) as
                               std::os::raw::c_int == '*' as i32 {
                        cmt = 0 as std::os::raw::c_int
                    }
                }
            } else if f < 4 as std::os::raw::c_int {
                loop  {
                    /* replace machine/os macros by compile-time counterparts */
                    f = 0 as std::os::raw::c_int;
                    e = f;
                    loop  {
                        r = platform_macros[f as usize];
                        if r.is_null() { break ; }
                        c = strlen(r) as std::os::raw::c_int;
                        /* remove 'defined' */
                    //e = memcmp(p, "defined ", 8) ? 0 : 8;
                        if 0 as std::os::raw::c_int ==
                               memcmp(p.offset(e as isize) as
                                          *const std::os::raw::c_void,
                                      r as *const std::os::raw::c_void,
                                      c as std::os::raw::c_ulong) {
                            p = p.offset((e + c) as isize);
                            q =
                                strchr(strcpy(q,
                                              platform_macros[(f +
                                                                   1 as
                                                                       std::os::raw::c_int)
                                                                  as usize]),
                                       0 as std::os::raw::c_int);
                            break ;
                        } else { f += 2 as std::os::raw::c_int }
                    }
                    !r.is_null();
                    let fresh0 = p;
                    p = p.offset(1);
                    let fresh1 = q;
                    q = q.offset(1);
                    *fresh1 = *fresh0;
                    if !(*fresh1 != 0) { break ; }
                }
                /* output as is */
                fprintf(op, b"%s\n\x00" as *const u8 as *const std::os::raw::c_char,
                        l2.as_mut_ptr()); /* trailing comment detected */
            } else {
                f = 0 as std::os::raw::c_int;
                e = f;
                s = e;
                p0 = p;
                loop  {
                    let fresh2 = p;
                    p = p.offset(1);
                    c = *fresh2 as std::os::raw::c_int;
                    if isspc(c) != 0 {
                        s = 1 as std::os::raw::c_int
                    } else {
                        if c == '/' as i32 &&
                               (*p.offset(0 as std::os::raw::c_int as isize) as
                                    std::os::raw::c_int == '/' as i32 ||
                                    *p.offset(0 as std::os::raw::c_int as isize) as
                                        std::os::raw::c_int == '*' as i32) {
                            c = 0 as std::os::raw::c_int
                        } else if s != 0 && q > l2.as_mut_ptr() &&
                                      (isid(*q.offset(-(1 as std::os::raw::c_int) as
                                                          isize) as
                                                std::os::raw::c_int) != 0 &&
                                           isid(c) != 0 ||
                                           q >=
                                               l2.as_mut_ptr().offset(2 as
                                                                          std::os::raw::c_int
                                                                          as
                                                                          isize)
                                               &&
                                               l2[0 as std::os::raw::c_int as usize]
                                                   as std::os::raw::c_int ==
                                                   '#' as i32 &&
                                               l2[1 as std::os::raw::c_int as usize]
                                                   as std::os::raw::c_int ==
                                                   'd' as i32 &&
                                               f < 2 as std::os::raw::c_int && e == 0)
                         {
                            let fresh3 = q;
                            q = q.offset(1);
                            *fresh3 = ' ' as i32 as std::os::raw::c_char;
                            f += 1
                        }
                        s = 0 as std::os::raw::c_int;
                        if c == '(' as i32 { e += 1 }
                        if c == ')' as i32 { e -= 1 }
                        if c == '\\' as i32 || c == '\"' as i32 {
                            let fresh4 = q;
                            q = q.offset(1);
                            *fresh4 = '\\' as i32 as std::os::raw::c_char
                        }
                        let fresh5 = q;
                        q = q.offset(1);
                        *fresh5 = c as std::os::raw::c_char;
                        if c == 0 as std::os::raw::c_int { break ; }
                        p0 = p
                    }
                }
                /* output with quotes */
                fprintf(op,
                        b"    \"%s\\n\"%s\n\x00" as *const u8 as
                            *const std::os::raw::c_char, l2.as_mut_ptr(), p0);
            }
        }
    fclose(fp);
    fclose(op);
    return 0 as std::os::raw::c_int;
}
#[main]
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
/* ----------------------------------------------------------------------- */
