use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    
    
    
    
    
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor0 { dummy: () }
pub type _IO_lock_t = ();
pub type FILE = crate::src::lil::_IO_FILE;
pub type int64_t = __int64_t;
pub type lilint_t = int64_t;
pub type lil_value_t = *mut crate::src::lil::_lil_value_t;
pub type lil_var_t = *mut crate::src::lil::_lil_var_t;
pub type lil_list_t = *mut crate::src::lil::_lil_list_t;
pub type lil_t = *mut crate::src::lil::_lil_t;
pub type lil_func_proc_t = Option::<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>;
pub type lil_callback_proc_t = Option::<unsafe extern "C" fn() -> ()>;
static mut running: libc::c_int = 1 as libc::c_int;
static mut exit_code: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn do_exit(mut lil: *mut crate::src::lil::_lil_t, mut val: *mut crate::src::lil::_lil_value_t) {
    crate::src::main::running= 0 as libc::c_int;
    crate::src::main::exit_code= crate::src::lil::lil_to_integer(val) as libc::c_int;
}
unsafe extern "C" fn do_system(
    mut argc: libc::c_ulong,
    mut argv: *mut *mut libc::c_char,
) -> *mut /* owning */ libc::c_char {
    let mut cmd = 0 as *mut libc::c_char;
    let mut cmdlen = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut p = 0 as *mut FILE;
    i= 0 as libc::c_int as size_t;
    while i < argc {
        let mut len = strlen(*argv.offset(i as isize));
        if i != 0 as libc::c_int as libc::c_ulong {
            cmd= realloc(
                cmd as *mut libc::c_void,
                (cmdlen + 1 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            let fresh0 = cmdlen;
            cmdlen= cmdlen + 1;
            *cmd.offset(fresh0 as isize) = ' ' as i32 as libc::c_char;
        }
        cmd= realloc(
            cmd as *mut libc::c_void,
            (cmdlen as libc::c_ulong).wrapping_add(len),
        ) as *mut libc::c_char;
        memcpy(
            cmd.offset(cmdlen as isize) as *mut libc::c_void,
            *argv.offset(i as isize) as *const libc::c_void,
            len,
        );
        cmdlen= (cmdlen as libc::c_ulong).wrapping_add(len) as libc::c_int
            as libc::c_int;
        i= i.wrapping_add(1);
    }
    cmd= realloc(cmd as *mut libc::c_void, (cmdlen + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    *cmd.offset(cmdlen as isize) = 0 as libc::c_int as libc::c_char;
    p= popen(cmd as *const i8, b"r\0" as *const u8 as *const libc::c_char);
    free(cmd as *mut libc::c_void);
    if !p.is_null() {
        let mut retval = 0 as *mut libc::c_char;
        let mut size = 0 as libc::c_int as size_t;
        let mut buff: [libc::c_char; 1024] = [0; 1024];
        let mut bytes: ssize_t = 0;
        loop {
            bytes= fread(
                buff.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                1024 as libc::c_int as libc::c_ulong,
                p,
            ) as ssize_t;
            if !(bytes != 0) {
                break;
            }
            retval= realloc(
                retval as *mut libc::c_void,
                size.wrapping_add(bytes as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                retval.offset(size as isize) as *mut libc::c_void,
                buff.as_mut_ptr() as *const libc::c_void,
                bytes as libc::c_ulong,
            );
            size= (size as libc::c_ulong).wrapping_add(bytes as libc::c_ulong) as size_t
                as size_t;
        }
        retval= realloc(
            retval as *mut libc::c_void,
            size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        *retval.offset(size as isize) = 0 as libc::c_int as libc::c_char;
        pclose(p);
        return retval;
    } else {();
        return 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn fnc_writechar(
    mut lil: *mut crate::src::lil::_lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut crate::src::lil::_lil_value_t,
) -> *mut crate::src::lil::_lil_value_t {
    if argc == 0 {
        return 0 as lil_value_t;
    }
    printf(
        b"%c\0" as *const u8 as *const libc::c_char,
        crate::src::lil::lil_to_integer(*argv.offset(0 as libc::c_int as isize)) as libc::c_char
            as libc::c_int,
    );
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_system(
    mut lil: *mut crate::src::lil::_lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut crate::src::lil::_lil_value_t,
) -> *mut crate::src::lil::_lil_value_t {
    let mut sargv = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(argc.wrapping_add(1 as libc::c_int as libc::c_ulong)),
    ) as *mut *const libc::c_char;
    let mut r = 0 as lil_value_t;
    let mut rv = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    if argc == 0 as libc::c_int as libc::c_ulong {
        return 0 as lil_value_t;
    }
    i= 0 as libc::c_int as size_t;
    while i < argc {
        *sargv.offset(i as isize) = crate::src::lil::lil_to_string(*argv.offset(i as isize));
        i= i.wrapping_add(1);
    }
    *sargv.offset(argc as isize) = 0 as *const libc::c_char;
    rv= do_system(argc, sargv as *mut *mut libc::c_char);
    if !rv.is_null() {
        r= crate::src::lil::lil_alloc_string(rv);
        free(rv as *mut libc::c_void);
    }else { (); }
    free(sargv as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn fnc_readline(
    mut lil: *mut crate::src::lil::_lil_t,
    mut argc: libc::c_ulong,
    mut argv: *mut *mut crate::src::lil::_lil_value_t,
) -> *mut /* owning */ crate::src::lil::_lil_value_t {
    let mut len = 0 as libc::c_int as size_t;
    let mut size = 64 as libc::c_int as size_t;
    let mut buffer = malloc(size) as *mut libc::c_char;
    let mut ch: libc::c_schar = 0;
    let mut retval = 0 as *mut crate::src::lil::_lil_value_t;
    loop {
        ch= fgetc(crate::src::main::stdin) as libc::c_schar;
        if ch as libc::c_int == -(1 as libc::c_int) {
            break;
        }
        if ch as libc::c_int == '\r' as i32 {
            continue;
        }
        if ch as libc::c_int == '\n' as i32 {
            break;
        }
        if len < size {
            size= (size as libc::c_ulong)
                .wrapping_add(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
            buffer= realloc(buffer as *mut libc::c_void, size) as *mut libc::c_char;
        }
        let fresh3 = len;
        len= len.wrapping_add(1);
        *buffer.offset(fresh3 as isize) = ch as libc::c_char;
    }
    buffer= realloc(
        buffer as *mut libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *buffer.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    retval= crate::src::lil::lil_alloc_string(buffer as *const i8);
    free(buffer as *mut libc::c_void);
    return retval;
}
unsafe extern "C" fn repl() -> libc::c_int {
    let mut buffer: [libc::c_char; 16384] = [0; 16384];
    let mut lil = crate::src::lil::lil_new();
    crate::src::lil::lil_register(
        lil.as_mut(),
        b"writechar\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_writechar
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    crate::src::lil::lil_register(
        lil.as_mut(),
        b"system\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_system
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    crate::src::lil::lil_register(
        lil.as_mut(),
        b"readline\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_readline
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    printf(
        b"Little Interpreted Language Interactive Shell\n\0" as *const u8
            as *const libc::c_char,
    );
    crate::src::lil::lil_callback(
        lil.as_mut(),
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(lil_t, lil_value_t) -> ()>,
            lil_callback_proc_t,
        >(Some(do_exit as unsafe extern "C" fn(lil_t, lil_value_t) -> ())),
    );
    while crate::src::main::running != 0 {
        let mut result = 0 as *mut crate::src::lil::_lil_value_t;
        let mut strres = 0 as *const libc::c_char;
        let mut err_msg = 0 as *const libc::c_char;
        let mut pos: size_t = 0;
        buffer[0 as libc::c_int as usize]= 0 as libc::c_int as libc::c_char;
        printf(b"# \0" as *const u8 as *const libc::c_char);
        if (fgets(buffer.as_mut_ptr(), 16384 as libc::c_int, crate::src::main::stdin)).is_null() {();
            break;
        }
        result= crate::src::lil::lil_parse(
            lil,
            buffer.as_mut_ptr(),
            0 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        strres= crate::src::lil::lil_to_string(result);
        if *strres.offset(0 as libc::c_int as isize) != 0 {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, strres);
        }
        crate::src::lil::lil_free_value(result);
        if crate::src::lil::lil_error(lil.as_mut(), Some(&mut err_msg), Some(&mut pos)) != 0 {
            printf(
                b"error at %i: %s\n\0" as *const u8 as *const libc::c_char,
                pos as libc::c_int,
                err_msg,
            );
        }
    }
    crate::src::lil::lil_free(lil);
    return crate::src::main::exit_code;
}
unsafe extern "C" fn nonint(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut lil = crate::src::lil::lil_new();
    let mut filename = *argv.offset(1 as libc::c_int as isize);
    let mut err_msg = 0 as *const libc::c_char;
    let mut pos: size_t = 0;
    let mut arglist = crate::src::lil::lil_alloc_list();
    let mut args = 0 as *mut crate::src::lil::_lil_value_t;
    let mut result = 0 as *mut crate::src::lil::_lil_value_t;
    let mut tmpcode = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    crate::src::lil::lil_register(
        lil.as_mut(),
        b"writechar\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_writechar
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    crate::src::lil::lil_register(
        lil.as_mut(),
        b"system\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_system
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    i= 2 as libc::c_int;
    while i < argc {
        crate::src::lil::lil_list_append(arglist, crate::src::lil::lil_alloc_string(*argv.offset(i as isize)));
        i+= 1;
    }
    args= crate::src::lil::lil_list_to_value(arglist, 1 as libc::c_int);
    crate::src::lil::lil_free_list(arglist);
    crate::src::lil::lil_set_var(
        lil,
        b"argv\0" as *const u8 as *const libc::c_char,
        args,
        0 as libc::c_int,
    );
    crate::src::lil::lil_free_value(args);
    tmpcode= malloc(
        (strlen(filename)).wrapping_add(256 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        tmpcode,
        b"set __lilmain:code__ [read {%s}]\nif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\n\0"
            as *const u8 as *const libc::c_char,
        filename,
    );
    result= crate::src::lil::lil_parse(lil, tmpcode as *const i8, 0 as libc::c_int as size_t, 1 as libc::c_int);
    free(tmpcode as *mut libc::c_void);
    crate::src::lil::lil_free_value(result);
    if crate::src::lil::lil_error(lil.as_mut(), Some(&mut err_msg), Some(&mut pos)) != 0 {
        fprintf(
            crate::src::main::stderr,
            b"lil: error at %i: %s\n\0" as *const u8 as *const libc::c_char,
            pos as libc::c_int,
            err_msg,
        );
    }
    crate::src::lil::lil_free(lil);
    return crate::src::main::exit_code;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    if argc < 2 as libc::c_int { return repl() } else { return nonint(argc, argv) };
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
//                 args.as_mut_ptr() as *mut *const libc::c_char,
//             ) as i32,
//         )
//     }
// }
