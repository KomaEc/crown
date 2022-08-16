use ::libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    /* global variables for the bc machine. All will be dynamic in size.*/
    /* Function storage. main is (0) and functions (1-f_count) */
    #[no_mangle]
    static mut functions: *mut bc_function;
    #[no_mangle]
    static mut had_error: libc::c_int;
    #[no_mangle]
    fn execute();
    #[no_mangle]
    fn nextarg(args: *mut arg_list, val: libc::c_char) -> *mut arg_list;
    #[no_mangle]
    fn yyerror(str: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* From main.c */
    /* From number.c */
    /* From storage.c */
    #[no_mangle]
    fn clear_func(func: libc::c_char);
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/* bcdefs.h:  The single file to include all constants and type definitions. */
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
/* Include the configuration file. */
/* Standard includes for all files. */
/* Include the other definitions. */
/* These definitions define all the structures used in
code and data storage.  This includes the representation of
labels.   The "guiding" principle is to make structures that
take a minimum of space when unused but can be built to contain
the full structures.  */
/* Labels are first.  Labels are generated sequentially in functions
and full code.  They just "point" to a single bye in the code.  The
"address" is the byte number.  The byte number is used to get an
actual character pointer. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_label_group {
    pub l_adrs: [libc::c_long; 64],
    pub l_next: *mut bc_label_group,
}
/* Each function has its own code segments and labels.  There can be
no jumps between functions so labels are unique to a function. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_function {
    pub f_defined: libc::c_char,
    pub f_body: [*mut libc::c_char; 16],
    pub f_code_size: libc::c_int,
    pub f_label: *mut bc_label_group,
    pub f_params: *mut arg_list,
    pub f_autos: *mut arg_list,
}
/* Code addresses. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_int,
    pub pc_addr: libc::c_int,
}
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const BC_SEG_SIZE: libc::c_int = 1024 as libc::c_int;
pub const BC_MAX_SEGS: libc::c_int = 16 as libc::c_int;
pub const BC_SEG_LOG: libc::c_int = 10 as libc::c_int;
pub const BC_LABEL_GROUP: libc::c_int = 64 as libc::c_int;
pub const BC_LABEL_LOG: libc::c_int = 6 as libc::c_int;
/* load.c:  This code "loads" code into the code segments. */
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
/* Load variables. */
#[no_mangle]
pub static mut load_adr: program_counter = program_counter {
    pc_func: 0,
    pc_addr: 0,
};
#[no_mangle]
pub static mut load_str: libc::c_char = 0;
#[no_mangle]
pub static mut load_const: libc::c_char = 0;
/* Initialize the load sequence. */
#[no_mangle]
pub unsafe extern "C" fn init_load() {
    clear_func(0 as libc::c_int as libc::c_char);
    load_adr.pc_func = 0 as libc::c_int;
    load_adr.pc_addr = 0 as libc::c_int;
    load_str = FALSE as libc::c_char;
    load_const = FALSE as libc::c_char;
}
/* addbyte adds one BYTE to the current code segment. */
#[no_mangle]
pub unsafe extern "C" fn addbyte(mut byte: libc::c_int) {
    let mut seg: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut func: libc::c_int = 0;
    /* If there was an error, don't continue. */
    if had_error != 0 {
        return;
    }
    /* Calculate the segment and offset. */
    seg = load_adr.pc_addr >> BC_SEG_LOG;
    let fresh0 = load_adr.pc_addr;
    load_adr.pc_addr = load_adr.pc_addr + 1;
    offset = fresh0 % BC_SEG_SIZE;
    func = load_adr.pc_func;
    if seg >= BC_MAX_SEGS {
        yyerror(b"Function too big.\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return;
    }
    if (*functions.offset(func as isize)).f_body[seg as usize].is_null() {
        let ref mut fresh1 = (*functions.offset(func as isize)).f_body[seg as usize];
        *fresh1 = malloc(BC_SEG_SIZE as libc::c_ulong) as *mut libc::c_char
    }
    /* Store the byte. */
    *(*functions.offset(func as isize)).f_body[seg as usize].offset(offset as isize) =
        byte as libc::c_char;
    let ref mut fresh2 = (*functions.offset(func as isize)).f_code_size;
    *fresh2 += 1;
}
/* Define a label LAB to be the current program counter. */
#[no_mangle]
pub unsafe extern "C" fn def_label(mut lab: libc::c_long) {
    let mut temp: *mut bc_label_group = 0 as *mut bc_label_group;
    let mut group: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut func: libc::c_int = 0;
    /* Get things ready. */
    group = (lab >> BC_LABEL_LOG) as libc::c_int;
    offset = (lab % BC_LABEL_GROUP as libc::c_long) as libc::c_int;
    func = load_adr.pc_func;
    /* Make sure there is at least one label group. */
    if (*functions.offset(func as isize)).f_label.is_null() {
        let ref mut fresh3 = (*functions.offset(func as isize)).f_label;
        *fresh3 =
            malloc(::std::mem::size_of::<bc_label_group>() as libc::c_ulong) as *mut bc_label_group;
        let ref mut fresh4 = (*(*functions.offset(func as isize)).f_label).l_next;
        *fresh4 = NULL as *mut bc_label_group
    }
    /* Add the label group. */
    temp = (*functions.offset(func as isize)).f_label;
    while group > 0 as libc::c_int {
        if (*temp).l_next.is_null() {
            (*temp).l_next = malloc(::std::mem::size_of::<bc_label_group>() as libc::c_ulong)
                as *mut bc_label_group;
            (*(*temp).l_next).l_next = NULL as *mut bc_label_group
        }
        temp = (*temp).l_next;
        group -= 1
    }
    /* Define it! */
    (*temp).l_adrs[offset as usize] = load_adr.pc_addr as libc::c_long;
}
/* Several instructions have integers in the code.  They
are all known to be legal longs.  So, no error code
is added.  STR is the pointer to the load string and
must be moved to the last non-digit character. */
#[no_mangle]
pub unsafe extern "C" fn long_val(mut str: *mut *mut libc::c_char) -> libc::c_long {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut neg: libc::c_char = FALSE as libc::c_char;
    if **str as libc::c_int == '-' as i32 {
        neg = TRUE as libc::c_char;
        *str = (*str).offset(1)
    }
    while *(*__ctype_b_loc()).offset(**str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        let fresh5 = *str;
        *str = (*str).offset(1);
        val = val * 10 as libc::c_int + *fresh5 as libc::c_int - '0' as i32
    }
    if neg != 0 {
        return -val as libc::c_long;
    } else {
        return val as libc::c_long;
    };
}
/* From load.c */
/* load_code loads the CODE into the machine. */
#[no_mangle]
pub unsafe extern "C" fn load_code(mut code: *mut libc::c_char) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char; /* auto or parameter name. */
    let mut ap_name: libc::c_long = 0; /* variable, array or function number. */
    let mut label_no: libc::c_long = 0;
    let mut vaf_name: libc::c_long = 0;
    let mut func: libc::c_long = 0;
    let mut save_adr: program_counter = program_counter {
        pc_func: 0,
        pc_addr: 0,
    };
    /* Initialize. */
    str = code;
    /* Scan the code. */
    while *str as libc::c_int != 0 as libc::c_int {
        /* If there was an error, don't continue. */
        if had_error != 0 {
            return;
        }
        if load_str != 0 {
            if *str as libc::c_int == '\"' as i32 {
                load_str = FALSE as libc::c_char
            }
            let fresh6 = str;
            str = str.offset(1);
            addbyte(*fresh6 as libc::c_int);
        } else if load_const != 0 {
            if *str as libc::c_int == '\n' as i32 {
                str = str.offset(1)
            } else if *str as libc::c_int == ':' as i32 {
                load_const = FALSE as libc::c_char;
                let fresh7 = str;
                str = str.offset(1);
                addbyte(*fresh7 as libc::c_int);
            } else if *str as libc::c_int == '.' as i32 {
                let fresh8 = str;
                str = str.offset(1);
                addbyte(*fresh8 as libc::c_int);
            } else if *str as libc::c_int >= 'A' as i32 {
                let fresh9 = str;
                str = str.offset(1);
                addbyte(*fresh9 as libc::c_int + 10 as libc::c_int - 'A' as i32);
            } else {
                let fresh10 = str;
                str = str.offset(1);
                addbyte(*fresh10 as libc::c_int - '0' as i32);
            }
        } else {
            let mut current_block_79: u64;
            match *str as libc::c_int {
                34 => {
                    /* Starts a string. */
                    load_str = TRUE as libc::c_char;
                    current_block_79 = 11071260907632769126;
                }
                78 => {
                    /* A label */
                    str = str.offset(1);
                    label_no = long_val(&mut str);
                    def_label(label_no);
                    current_block_79 = 11071260907632769126;
                }
                66 => {
                    /* Branch to label. */
                    current_block_79 = 4003266481717755314;
                }
                74 | 90 => {
                    current_block_79 = 4003266481717755314;
                }
                70 => {
                    /* A function, get the name and initialize it. */
                    str = str.offset(1);
                    func = long_val(&mut str);
                    clear_func(func as libc::c_char);
                    loop
                    /* get the parameters */
                    {
                        let fresh12 = str;
                        str = str.offset(1);
                        if !(*fresh12 as libc::c_int != '.' as i32) {
                            break;
                        }
                        if *str as libc::c_int == '.' as i32 {
                            str = str.offset(1);
                            break;
                        } else {
                            ap_name = long_val(&mut str);
                            let ref mut fresh13 =
                                (*functions.offset(func as libc::c_int as isize)).f_params;
                            *fresh13 = nextarg(
                                (*functions.offset(func as libc::c_int as isize)).f_params,
                                ap_name as libc::c_char,
                            )
                        }
                    }
                    /* get the auto vars */
                    while *str as libc::c_int != '[' as i32 {
                        if *str as libc::c_int == ',' as i32 {
                            str = str.offset(1)
                        }
                        ap_name = long_val(&mut str);
                        let ref mut fresh14 =
                            (*functions.offset(func as libc::c_int as isize)).f_autos;
                        *fresh14 = nextarg(
                            (*functions.offset(func as libc::c_int as isize)).f_autos,
                            ap_name as libc::c_char,
                        )
                    }
                    save_adr = load_adr;
                    load_adr.pc_func = func as libc::c_int;
                    load_adr.pc_addr = 0 as libc::c_int;
                    current_block_79 = 11071260907632769126;
                }
                93 => {
                    /* A function end */
                    (*functions.offset(load_adr.pc_func as isize)).f_defined = TRUE as libc::c_char;
                    load_adr = save_adr;
                    current_block_79 = 11071260907632769126;
                }
                67 => {
                    /* Call a function. */
                    let fresh15 = str;
                    str = str.offset(1);
                    addbyte(*fresh15 as libc::c_int);
                    func = long_val(&mut str);
                    if func < 128 as libc::c_int as libc::c_long {
                        addbyte(func as libc::c_char as libc::c_int);
                    } else {
                        addbyte(
                            (func >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long
                                | 0x80 as libc::c_int as libc::c_long)
                                as libc::c_int,
                        );
                        addbyte((func & 0xff as libc::c_int as libc::c_long) as libc::c_int);
                    }
                    if *str as libc::c_int == ',' as i32 {
                        str = str.offset(1)
                    }
                    while *str as libc::c_int != ':' as i32 {
                        let fresh16 = str;
                        str = str.offset(1);
                        addbyte(*fresh16 as libc::c_int);
                    }
                    addbyte(':' as i32);
                    current_block_79 = 11071260907632769126;
                }
                99 => {
                    /* Call a special function. */
                    let fresh17 = str;
                    str = str.offset(1);
                    addbyte(*fresh17 as libc::c_int);
                    addbyte(*str as libc::c_int);
                    current_block_79 = 11071260907632769126;
                }
                75 => {
                    /* A constant.... may have an "F" in it. */
                    addbyte(*str as libc::c_int);
                    load_const = TRUE as libc::c_char;
                    current_block_79 = 11071260907632769126;
                }
                100 => {
                    /* Decrement. */
                    current_block_79 = 6153397765503504804;
                }
                105 => {
                    current_block_79 = 6153397765503504804;
                }
                108 => {
                    current_block_79 = 2977523333743079431;
                }
                115 => {
                    current_block_79 = 13620331185327884698;
                }
                65 => {
                    current_block_79 = 13839326668931527616;
                }
                77 => {
                    current_block_79 = 5113588946371423884;
                }
                76 | 83 => {
                    current_block_79 = 5892893968624351926;
                }
                64 => {
                    /* A command! */
                    str = str.offset(1);
                    match *str as libc::c_int {
                        105 => {
                            init_load();
                        }
                        114 => {
                            execute();
                        }
                        _ => {}
                    }
                    current_block_79 = 11071260907632769126;
                }
                10 => {
                    current_block_79 = 11071260907632769126;
                }
                _ => {
                    /* Anything else */
                    addbyte(*str as libc::c_int);
                    current_block_79 = 11071260907632769126;
                }
            }
            match current_block_79 {
                4003266481717755314 =>
                /* Jump to label. */
                /* Branch Zero to label. */
                {
                    let fresh11 = str;
                    str = str.offset(1);
                    addbyte(*fresh11 as libc::c_int);
                    label_no = long_val(&mut str);
                    if label_no > 65535 as libc::c_long {
                        /* Better message? */
                        fprintf(
                            stderr,
                            b"Program too big.\n\x00" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    addbyte(label_no as libc::c_char as libc::c_int & 0xff as libc::c_int);
                    addbyte(label_no as libc::c_char as libc::c_int >> 8 as libc::c_int);
                    current_block_79 = 11071260907632769126;
                }
                6153397765503504804 =>
                /* Increment. */
                {
                    current_block_79 = 2977523333743079431;
                }
                _ => {}
            }
            match current_block_79 {
                2977523333743079431 =>
                /* Load. */
                {
                    current_block_79 = 13620331185327884698;
                }
                _ => {}
            }
            match current_block_79 {
                13620331185327884698 =>
                /* Store. */
                {
                    current_block_79 = 13839326668931527616;
                }
                _ => {}
            }
            match current_block_79 {
                13839326668931527616 =>
                /* Array Increment */
                {
                    current_block_79 = 5113588946371423884;
                }
                _ => {}
            }
            match current_block_79 {
                5113588946371423884 =>
                /* Array Decrement */
                {
                    current_block_79 = 5892893968624351926;
                }
                _ => {}
            }
            match current_block_79 {
                5892893968624351926 =>
                /* Array Load */
                /* Array Store */
                {
                    let fresh18 = str;
                    str = str.offset(1);
                    addbyte(*fresh18 as libc::c_int);
                    vaf_name = long_val(&mut str);
                    if vaf_name < 128 as libc::c_int as libc::c_long {
                        addbyte(vaf_name as libc::c_int);
                    } else {
                        addbyte(
                            (vaf_name >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long
                                | 0x80 as libc::c_int as libc::c_long)
                                as libc::c_int,
                        );
                        addbyte((vaf_name & 0xff as libc::c_int as libc::c_long) as libc::c_int);
                    }
                }
                _ => {}
            }
            str = str.offset(1)
        }
    }
}
