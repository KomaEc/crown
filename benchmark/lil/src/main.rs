use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _lil_value_t;
    pub type _lil_var_t;
    pub type _lil_list_t;
    pub type _lil_t;
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
    fn lil_new() -> lil_t;
    fn lil_free(lil: lil_t);
    fn lil_register(
        lil: lil_t,
        name: *const libc::c_char,
        proc_0: lil_func_proc_t,
    ) -> libc::c_int;
    fn lil_parse(
        lil: lil_t,
        code: *const libc::c_char,
        codelen: size_t,
        funclevel: libc::c_int,
    ) -> lil_value_t;
    fn lil_callback(lil: lil_t, cb: libc::c_int, proc_0: lil_callback_proc_t);
    fn lil_error(
        lil: lil_t,
        msg: *mut *const libc::c_char,
        pos: *mut size_t,
    ) -> libc::c_int;
    fn lil_to_string(val: lil_value_t) -> *const libc::c_char;
    fn lil_to_integer(val: lil_value_t) -> lilint_t;
    fn lil_alloc_string(str: *const libc::c_char) -> lil_value_t;
    fn lil_free_value(val: lil_value_t);
    fn lil_alloc_list() -> lil_list_t;
    fn lil_free_list(list: lil_list_t);
    fn lil_list_append(list: lil_list_t, val: lil_value_t);
    fn lil_list_to_value(list: lil_list_t, do_escape: libc::c_int) -> lil_value_t;
    fn lil_set_var(
        lil: lil_t,
        name: *const libc::c_char,
        val: lil_value_t,
        local: libc::c_int,
    ) -> lil_var_t;
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
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
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int64_t = __int64_t;
pub type lilint_t = int64_t;
pub type lil_value_t = *mut _lil_value_t;
pub type lil_var_t = *mut _lil_var_t;
pub type lil_list_t = *mut _lil_list_t;
pub type lil_t = *mut _lil_t;
pub type lil_func_proc_t = Option::<
    unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
>;
pub type lil_callback_proc_t = Option::<unsafe extern "C" fn() -> ()>;
static mut running: libc::c_int = 1 as libc::c_int;
static mut exit_code: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn do_exit(mut lil: lil_t, mut val: lil_value_t) {
    running = 0 as libc::c_int;
    exit_code = lil_to_integer(val) as libc::c_int;
}
unsafe extern "C" fn do_system(
    mut argc: size_t,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cmd = 0 as *mut libc::c_char;
    let mut cmdlen = 0 as libc::c_int;
    let mut i: size_t = 0;
    let mut p = 0 as *mut FILE;
    i = 0 as libc::c_int as size_t;
    while i < argc {
        let mut len = strlen(*argv.offset(i as isize));
        if i != 0 as libc::c_int as libc::c_ulong {
            cmd = realloc(
                cmd as *mut libc::c_void,
                (cmdlen + 1 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            let fresh0 = cmdlen;
            cmdlen = cmdlen + 1;
            *cmd.offset(fresh0 as isize) = ' ' as i32 as libc::c_char;
        }
        cmd = realloc(
            cmd as *mut libc::c_void,
            (cmdlen as libc::c_ulong).wrapping_add(len),
        ) as *mut libc::c_char;
        memcpy(
            cmd.offset(cmdlen as isize) as *mut libc::c_void,
            *argv.offset(i as isize) as *const libc::c_void,
            len,
        );
        cmdlen = (cmdlen as libc::c_ulong).wrapping_add(len) as libc::c_int
            as libc::c_int;
        i = i.wrapping_add(1);
    }
    cmd = realloc(cmd as *mut libc::c_void, (cmdlen + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    *cmd.offset(cmdlen as isize) = 0 as libc::c_int as libc::c_char;
    p = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
    free(cmd as *mut libc::c_void);
    if !p.is_null() {
        let mut retval = 0 as *mut libc::c_char;
        let mut size = 0 as libc::c_int as size_t;
        let mut buff: [libc::c_char; 1024] = [0; 1024];
        let mut bytes: ssize_t = 0;
        loop {
            bytes = fread(
                buff.as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                1024 as libc::c_int as libc::c_ulong,
                p,
            ) as ssize_t;
            if !(bytes != 0) {
                break;
            }
            retval = realloc(
                retval as *mut libc::c_void,
                size.wrapping_add(bytes as libc::c_ulong),
            ) as *mut libc::c_char;
            memcpy(
                retval.offset(size as isize) as *mut libc::c_void,
                buff.as_mut_ptr() as *const libc::c_void,
                bytes as libc::c_ulong,
            );
            size = (size as libc::c_ulong).wrapping_add(bytes as libc::c_ulong) as size_t
                as size_t;
        }
        retval = realloc(
            retval as *mut libc::c_void,
            size.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        *retval.offset(size as isize) = 0 as libc::c_int as libc::c_char;
        pclose(p);
        return retval;
    } else {
        return 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn fnc_writechar(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    if argc == 0 {
        return 0 as lil_value_t;
    }
    printf(
        b"%c\0" as *const u8 as *const libc::c_char,
        lil_to_integer(*argv.offset(0 as libc::c_int as isize)) as libc::c_char
            as libc::c_int,
    );
    return 0 as lil_value_t;
}
unsafe extern "C" fn fnc_system(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
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
    i = 0 as libc::c_int as size_t;
    while i < argc {
        let ref mut fresh1 = *sargv.offset(i as isize);
        *fresh1 = lil_to_string(*argv.offset(i as isize));
        i = i.wrapping_add(1);
    }
    let ref mut fresh2 = *sargv.offset(argc as isize);
    *fresh2 = 0 as *const libc::c_char;
    rv = do_system(argc, sargv as *mut *mut libc::c_char);
    if !rv.is_null() {
        r = lil_alloc_string(rv);
        free(rv as *mut libc::c_void);
    }
    free(sargv as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn fnc_readline(
    mut lil: lil_t,
    mut argc: size_t,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    let mut len = 0 as libc::c_int as size_t;
    let mut size = 64 as libc::c_int as size_t;
    let mut buffer = malloc(size) as *mut libc::c_char;
    let mut ch: libc::c_schar = 0;
    let mut retval = 0 as *mut _lil_value_t;
    loop {
        ch = fgetc(stdin) as libc::c_schar;
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
            size = (size as libc::c_ulong)
                .wrapping_add(64 as libc::c_int as libc::c_ulong) as size_t as size_t;
            buffer = realloc(buffer as *mut libc::c_void, size) as *mut libc::c_char;
        }
        let fresh3 = len;
        len = len.wrapping_add(1);
        *buffer.offset(fresh3 as isize) = ch as libc::c_char;
    }
    buffer = realloc(
        buffer as *mut libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    *buffer.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    retval = lil_alloc_string(buffer);
    free(buffer as *mut libc::c_void);
    return retval;
}
unsafe extern "C" fn repl() -> libc::c_int {
    let mut buffer: [libc::c_char; 16384] = [0; 16384];
    let mut lil = lil_new();
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_writechar
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_system
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
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
    lil_callback(
        lil,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(lil_t, lil_value_t) -> ()>,
            lil_callback_proc_t,
        >(Some(do_exit as unsafe extern "C" fn(lil_t, lil_value_t) -> ())),
    );
    while running != 0 {
        let mut result = 0 as *mut _lil_value_t;
        let mut strres = 0 as *const libc::c_char;
        let mut err_msg = 0 as *const libc::c_char;
        let mut pos: size_t = 0;
        buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        printf(b"# \0" as *const u8 as *const libc::c_char);
        if (fgets(buffer.as_mut_ptr(), 16384 as libc::c_int, stdin)).is_null() {
            break;
        }
        result = lil_parse(
            lil,
            buffer.as_mut_ptr(),
            0 as libc::c_int as size_t,
            0 as libc::c_int,
        );
        strres = lil_to_string(result);
        if *strres.offset(0 as libc::c_int as isize) != 0 {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, strres);
        }
        lil_free_value(result);
        if lil_error(lil, &mut err_msg, &mut pos) != 0 {
            printf(
                b"error at %i: %s\n\0" as *const u8 as *const libc::c_char,
                pos as libc::c_int,
                err_msg,
            );
        }
    }
    lil_free(lil);
    return exit_code;
}
unsafe extern "C" fn nonint(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut lil = lil_new();
    let mut filename = *argv.offset(1 as libc::c_int as isize);
    let mut err_msg = 0 as *const libc::c_char;
    let mut pos: size_t = 0;
    let mut arglist = lil_alloc_list();
    let mut args = 0 as *mut _lil_value_t;
    let mut result = 0 as *mut _lil_value_t;
    let mut tmpcode = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    lil_register(
        lil,
        b"writechar\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_writechar
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    lil_register(
        lil,
        b"system\0" as *const u8 as *const libc::c_char,
        Some(
            fnc_system
                as unsafe extern "C" fn(lil_t, size_t, *mut lil_value_t) -> lil_value_t,
        ),
    );
    i = 2 as libc::c_int;
    while i < argc {
        lil_list_append(arglist, lil_alloc_string(*argv.offset(i as isize)));
        i += 1;
    }
    args = lil_list_to_value(arglist, 1 as libc::c_int);
    lil_free_list(arglist);
    lil_set_var(
        lil,
        b"argv\0" as *const u8 as *const libc::c_char,
        args,
        0 as libc::c_int,
    );
    lil_free_value(args);
    tmpcode = malloc(
        (strlen(filename)).wrapping_add(256 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(
        tmpcode,
        b"set __lilmain:code__ [read {%s}]\nif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\n\0"
            as *const u8 as *const libc::c_char,
        filename,
    );
    result = lil_parse(lil, tmpcode, 0 as libc::c_int as size_t, 1 as libc::c_int);
    free(tmpcode as *mut libc::c_void);
    lil_free_value(result);
    if lil_error(lil, &mut err_msg, &mut pos) != 0 {
        fprintf(
            stderr,
            b"lil: error at %i: %s\n\0" as *const u8 as *const libc::c_char,
            pos as libc::c_int,
            err_msg,
        );
    }
    lil_free(lil);
    return exit_code;
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
