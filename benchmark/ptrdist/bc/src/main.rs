use ::libc;
extern "C" {
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    /* Interactive and other flags. */
    #[no_mangle]
    static mut interactive: libc::c_char;
    #[no_mangle]
    static mut compile_only: libc::c_char;
    #[no_mangle]
    static mut use_math: libc::c_char;
    #[no_mangle]
    static mut warn_not_std: libc::c_char;
    #[no_mangle]
    static mut std_only: libc::c_char;
    /* Input Line numbers and other error information. */
    #[no_mangle]
    static mut line_no: libc::c_int;
    /* For error message production */
    #[no_mangle]
    static mut g_argv: *mut *mut libc::c_char;
    #[no_mangle]
    static mut g_argc: libc::c_int;
    #[no_mangle]
    static mut is_std_in: libc::c_char;
    /* For use with getopt.  Do not declare them here.*/
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn init_storage();
    #[no_mangle]
    fn yyparse() -> libc::c_int;
    #[no_mangle]
    fn load_code(code: *mut libc::c_char);
    #[no_mangle]
    fn init_gen();
    #[no_mangle]
    fn init_tree();
    #[no_mangle]
    fn lookup(name: *mut libc::c_char, namekind: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn init_load();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    static mut yyin: *mut FILE;
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const EOF: libc::c_int = -(1 as libc::c_int);
/* const.h: Constants for bc. */
/*  This file is part of bc written for MINIX.
    Copyright (C) 1991, 1992 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License , or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; see the file COPYING.  If not, write to
    the Free Software Foundation, 675 Mass Ave, Cambridge, MA 02139, USA.

    You may contact the author by:
       e-mail:  phil@cs.wwu.edu
      us-mail:  Philip A. Nelson
                Computer Science Department, 9062
                Western Washington University
                Bellingham, WA 98226-9062

*************************************************************************/
/* Define INT_MAX and LONG_MAX if not defined.  Assuming 32 bits... */
/* Define constants in some reasonable size.  The next 4 constants are
POSIX constants. */
/* Definitions for arrays. */
/* this should be NODE_SIZE^NODE_DEPTH-1 */
/* Must be a power of 2. */
/* Must be NODE_SIZE-1. */
/* Number of 1 bits in NODE_MASK. */
/* Other BC limits defined but not part of POSIX. */
/* Code segments. */
/* Maximum number of variables, arrays and functions and the
allocation increment for the dynamic arrays. */
/* Other interesting constants. */
pub const TRUE: libc::c_int = 1 as libc::c_int;
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const FUNCT: libc::c_int = 2 as libc::c_int;
pub const BC_VERSION: [libc::c_char; 76] = unsafe {
    *::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
        b"bc 1.02 (Mar 3, 92) Copyright (C) 1991, 1992 Free Software Foundation, Inc.\x00",
    )
};
pub const SIGINT: libc::c_int = 2 as libc::c_int;
/* main.c: The main program for bc.  */
/*  This file is part of bc written for MINIX.
    Copyright (C) 1991, 1992 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License , or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; see the file COPYING.  If not, write to
    the Free Software Foundation, 675 Mass Ave, Cambridge, MA 02139, USA.

    You may contact the author by:
       e-mail:  phil@cs.wwu.edu
      us-mail:  Philip A. Nelson
                Computer Science Department, 9062
                Western Washington University
                Bellingham, WA 98226-9062

*************************************************************************/
/* Variables for processing multiple files. */
#[no_mangle]
pub static mut first_file: libc::c_char = 0;
/* The main program for bc. */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    /* Initialize many variables. */
    compile_only = FALSE as libc::c_char;
    use_math = FALSE as libc::c_char;
    warn_not_std = FALSE as libc::c_char;
    std_only = FALSE as libc::c_char;
    if isatty(0 as libc::c_int) != 0 && isatty(1 as libc::c_int) != 0 {
        interactive = TRUE as libc::c_char
    } else {
        interactive = FALSE as libc::c_char
    }
    /* Parse the command line */
    ch = getopt(
        argc,
        argv,
        b"lcisvw\x00" as *const u8 as *const libc::c_char,
    );
    while ch != EOF {
        match ch {
            99 => {
                /* compile only */
                compile_only = TRUE as libc::c_char
            }
            108 => {
                /* math lib */
                use_math = TRUE as libc::c_char
            }
            105 => {
                /* force interactive */
                interactive = TRUE as libc::c_char
            }
            119 => {
                /* Non standard features give warnings. */
                warn_not_std = TRUE as libc::c_char
            }
            115 => {
                /* Non standard features give errors. */
                std_only = TRUE as libc::c_char
            }
            118 => {
                /* Print the version. */
                printf(
                    b"%s\n\x00" as *const u8 as *const libc::c_char,
                    BC_VERSION.as_ptr(),
                );
            }
            _ => {}
        }
        ch = getopt(
            argc,
            argv,
            b"lcisvw\x00" as *const u8 as *const libc::c_char,
        )
    }
    /* Initialize the machine.  */
    init_storage();
    init_load();
    /* Set up interrupts to print a message. */
    if interactive != 0 {
        signal(
            SIGINT,
            Some(use_quit as unsafe extern "C" fn(_: libc::c_int) -> ()),
        );
    }
    /* Initialize the front end. */
    init_tree();
    init_gen();
    g_argv = NULL as *mut *mut libc::c_char;
    g_argc = 0 as libc::c_int;
    is_std_in = FALSE as libc::c_char;
    first_file = TRUE as libc::c_char;
    if open_new_file() == 0 {
        exit(1 as libc::c_int);
    }
    /* Do the parse. */
    yyparse();
    /* End the compile only output with a newline. */
    if compile_only != 0 {
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
    }
    /* PLUS_STATS */
    exit(0 as libc::c_int);
}
/* This is the function that opens all the files.
It returns TRUE if the file was opened, otherwise
it returns FALSE. */
#[no_mangle]
pub unsafe extern "C" fn open_new_file() -> libc::c_int {
    let mut new_file: *mut FILE = 0 as *mut FILE;
    /* Set the line number. */
    line_no = 1 as libc::c_int;
    /* Check to see if we are done. */
    if is_std_in != 0 {
        return 0 as libc::c_int;
    }
    /* Open the other files. */
    if use_math as libc::c_int != 0 && first_file as libc::c_int != 0 {
        /* Load the code from a precompiled version of the math libarary. */
        extern "C" {
            #[no_mangle]
            static mut libmath: [libc::c_char; 0];
        }
        let mut tmp: libc::c_char = 0;
        /* These MUST be in the order of first mention of each function.
        That is why "a" comes before "c" even though "a" is defined after
        after "c".  "a" is used in "s"! */
        tmp = lookup(
            b"e\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        tmp = lookup(
            b"l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        tmp = lookup(
            b"s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        tmp = lookup(
            b"a\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        tmp = lookup(
            b"c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        tmp = lookup(
            b"j\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FUNCT,
        ) as libc::c_char;
        load_code(libmath.as_mut_ptr());
    }
    /* One of the argv values. */
    if optind < g_argc {
        new_file = fopen(
            *g_argv.offset(optind as isize),
            b"r\x00" as *const u8 as *const libc::c_char,
        );
        if !new_file.is_null() {
            new_yy_file(new_file);
            optind += 1;
            return TRUE;
        }
        let fresh0 = optind;
        optind = optind + 1;
        fprintf(
            stderr,
            b"File %s is unavailable.\n\x00" as *const u8 as *const libc::c_char,
            *g_argv.offset(fresh0 as isize),
        );
        exit(1 as libc::c_int);
    }
    /* If we fall through to here, we should return stdin. */
    new_yy_file(stdin);
    is_std_in = TRUE as libc::c_char;
    return TRUE;
}
/* Set yyin to the new file. */
#[no_mangle]
pub unsafe extern "C" fn new_yy_file(mut file: *mut FILE) {
    if first_file == 0 {
        fclose(yyin);
    }
    yyin = file;
    first_file = FALSE as libc::c_char;
}
/* From load.c */
/* From main.c */
/* Message to use quit.  */
#[no_mangle]
pub unsafe extern "C" fn use_quit(mut sig: libc::c_int) {
    printf(b"\n(interrupt) use quit to exit.\n\x00" as *const u8 as *const libc::c_char);
    signal(
        SIGINT,
        Some(use_quit as unsafe extern "C" fn(_: libc::c_int) -> ()),
    );
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
