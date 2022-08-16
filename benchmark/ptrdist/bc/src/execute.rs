use ::libc;
extern "C" {
    #[no_mangle]
    fn _IO_getc(__fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    /* Interactive and other flags. */
    #[no_mangle]
    static mut interactive: libc::c_char;
    /* global variables for the bc machine. All will be dynamic in size.*/
    /* Function storage. main is (0) and functions (1-f_count) */
    #[no_mangle]
    static mut functions: *mut bc_function;
    #[no_mangle]
    static mut f_names: *mut *mut libc::c_char;
    /* Execution stack. */
    #[no_mangle]
    static mut ex_stack: *mut estack_rec;
    /* Function return stack. */
    #[no_mangle]
    static mut fn_stack: *mut fstack_rec;
    /* Other "storage". */
    #[no_mangle]
    static mut i_base: libc::c_int;
    #[no_mangle]
    static mut o_base: libc::c_int;
    #[no_mangle]
    static mut scale: libc::c_int;
    #[no_mangle]
    static mut c_code: libc::c_char;
    #[no_mangle]
    static mut runtime_error: libc::c_char;
    #[no_mangle]
    static mut pc: program_counter;
    /* defined in number.c */
    #[no_mangle]
    static mut _zero_: bc_num;
    #[no_mangle]
    static mut _one_: bc_num;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn incr_array(var_name: libc::c_int);
    #[no_mangle]
    fn process_params(pc_0: *mut program_counter, func: libc::c_int);
    #[no_mangle]
    fn auto_var(name: libc::c_int);
    #[no_mangle]
    fn fpush(val: libc::c_int);
    #[no_mangle]
    fn new_num(length: libc::c_int, scale_0: libc::c_int) -> bc_num;
    #[no_mangle]
    fn load_array(var_name: libc::c_int);
    #[no_mangle]
    fn decr_array(var_name: libc::c_char);
    #[no_mangle]
    fn store_array(var_name: libc::c_int);
    #[no_mangle]
    fn out_num(
        num: bc_num,
        o_base_0: libc::c_int,
        out_char_0: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    );
    #[no_mangle]
    fn bc_sqrt(num: *mut bc_num, scale_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn int2num(num: *mut bc_num, val: libc::c_int);
    #[no_mangle]
    fn decr_var(var_name: libc::c_int);
    #[no_mangle]
    fn incr_var(var_name: libc::c_int);
    #[no_mangle]
    fn load_var(var_name: libc::c_int);
    #[no_mangle]
    fn store_var(var_name: libc::c_int);
    #[no_mangle]
    fn rt_error(mesg: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn out_char(ch: libc::c_char);
    #[no_mangle]
    fn push_copy(num: bc_num);
    #[no_mangle]
    fn bc_add(n1: bc_num, n2: bc_num, result: *mut bc_num);
    #[no_mangle]
    fn use_quit(_: libc::c_int);
    #[no_mangle]
    fn pop();
    #[no_mangle]
    fn fpop() -> libc::c_int;
    #[no_mangle]
    fn pop_vars(list: *mut arg_list);
    #[no_mangle]
    fn bc_sub(n1: bc_num, n2: bc_num, result: *mut bc_num);
    #[no_mangle]
    fn copy_num(num: bc_num) -> bc_num;
    #[no_mangle]
    fn free_num(num: *mut bc_num);
    #[no_mangle]
    fn bc_compare(n1: bc_num, n2: bc_num) -> libc::c_int;
    #[no_mangle]
    fn check_stack(depth: libc::c_int) -> libc::c_char;
    #[no_mangle]
    fn init_num(num: *mut bc_num);
    #[no_mangle]
    fn push_num(num: bc_num);
    #[no_mangle]
    fn is_neg(num: bc_num) -> libc::c_char;
    #[no_mangle]
    fn is_zero(num: bc_num) -> libc::c_char;
    #[no_mangle]
    fn bc_raise(num1: bc_num, num2: bc_num, result: *mut bc_num, scale_0: libc::c_int);
    #[no_mangle]
    fn bc_modulo(
        num1: bc_num,
        num2: bc_num,
        result: *mut bc_num,
        scale_0: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn bc_divide(n1: bc_num, n2: bc_num, quot: *mut bc_num, scale_0: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn bc_multiply(n1: bc_num, n2: bc_num, prod: *mut bc_num, scale_0: libc::c_int);
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
pub type sign = libc::c_uint;
pub const MINUS: sign = 1;
pub const PLUS: sign = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_struct {
    pub n_sign: sign,
    pub n_len: libc::c_int,
    pub n_scale: libc::c_int,
    pub n_refs: libc::c_int,
    pub n_value: [libc::c_char; 1024],
}
/* number.h: Arbitrary precision numbers header file. */
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
/* The number of digits before the decimal point. */
/* The number of digits after the decimal point. */
/* The number of pointers to this number. */
/* The storage. Not zero char terminated. It is
allocated with all other fields.  */
pub type bc_num = *mut bc_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_label_group {
    pub l_adrs: [libc::c_long; 64],
    pub l_next: *mut bc_label_group,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_int,
    pub pc_addr: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estack_rec {
    pub s_num: bc_num,
    pub s_next: *mut estack_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fstack_rec {
    pub s_val: libc::c_int,
    pub s_next: *mut fstack_rec,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
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
pub const BC_LABEL_LOG: libc::c_int = 6 as libc::c_int;
pub const BC_LABEL_GROUP: libc::c_int = 64 as libc::c_int;
/* Code segments. */
/* Maximum number of variables, arrays and functions and the
allocation increment for the dynamic arrays. */
/* Other interesting constants. */
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const BC_SEG_LOG: libc::c_int = 10 as libc::c_int;
pub const BC_SEG_SIZE: libc::c_int = 1024 as libc::c_int;
pub const TRUE: libc::c_int = 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return _IO_getc(stdin);
}
pub const SIGINT: libc::c_int = 2 as libc::c_int;
/* execute.c - run a bc program. */
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
/* The SIGINT interrupt handling routine. */
#[no_mangle]
pub static mut had_sigint: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn stop_execution(mut sig: libc::c_int) {
    had_sigint = TRUE;
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    rt_error(b"interrupted execution\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
/* Get the current byte and advance the PC counter. */
#[no_mangle]
pub unsafe extern "C" fn byte(mut pc_0: *mut program_counter) -> libc::c_uchar {
    let mut seg: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    seg = (*pc_0).pc_addr >> BC_SEG_LOG;
    let fresh0 = (*pc_0).pc_addr;
    (*pc_0).pc_addr = (*pc_0).pc_addr + 1;
    offset = fresh0 % BC_SEG_SIZE;
    return *(*functions.offset((*pc_0).pc_func as isize)).f_body[seg as usize]
        .offset(offset as isize) as libc::c_uchar;
}
/* The routine that actually runs the machine. */
#[no_mangle]
pub unsafe extern "C" fn execute() {
    let mut label_num: libc::c_int = 0;
    let mut l_gp: libc::c_int = 0;
    let mut l_off: libc::c_int = 0;
    let mut gp: *mut bc_label_group = 0 as *mut bc_label_group;
    let mut inst: libc::c_char = 0;
    let mut ch: libc::c_char = 0;
    let mut new_func: libc::c_int = 0;
    let mut var_name: libc::c_int = 0;
    let mut const_base: libc::c_int = 0;
    let mut temp_num: bc_num = 0 as *mut bc_struct;
    let mut auto_list: *mut arg_list = 0 as *mut arg_list;
    /* Initialize this run... */
    pc.pc_func = 0 as libc::c_int;
    pc.pc_addr = 0 as libc::c_int;
    runtime_error = FALSE as libc::c_char;
    init_num(&mut temp_num);
    /* Set up the interrupt mechanism for an interactive session. */
    if interactive != 0 {
        signal(
            SIGINT,
            Some(stop_execution as unsafe extern "C" fn(_: libc::c_int) -> ()),
        );
        had_sigint = FALSE
    }
    while pc.pc_addr < (*functions.offset(pc.pc_func as isize)).f_code_size && runtime_error == 0 {
        inst = byte(&mut pc) as libc::c_char;
        let mut current_block_215: u64;
        match inst as libc::c_int {
            65 => {
                /* increment array variable (Add one). */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                incr_array(var_name);
                current_block_215 = 613454377845503748;
            }
            66 | 90 => {
                /* Branch to a label if TOS != 0. Remove value on TOS. */
                /* Branch to a label if TOS == 0. Remove value on TOS. */
                c_code = (is_zero((*ex_stack).s_num) == 0) as libc::c_int as libc::c_char;
                pop();
                current_block_215 = 16102553208965294739;
            }
            74 => {
                current_block_215 = 16102553208965294739;
            }
            67 => {
                /* Call a function. */
                /* Get the function number. */
                new_func = byte(&mut pc) as libc::c_int;
                if new_func & 0x80 as libc::c_int != 0 as libc::c_int {
                    new_func = (new_func << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                /* Check to make sure it is defined. */
                if (*functions.offset(new_func as isize)).f_defined == 0 {
                    rt_error(
                        b"Function %s not defined.\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        *f_names.offset(new_func as isize),
                    );
                } else {
                    /* Check and push parameters. */
                    process_params(&mut pc, new_func);
                    /* Push auto variables. */
                    auto_list = (*functions.offset(new_func as isize)).f_autos;
                    while !auto_list.is_null() {
                        auto_var((*auto_list).av_name);
                        auto_list = (*auto_list).next
                    }
                    /* Push pc and ibase. */
                    fpush(pc.pc_func);
                    fpush(pc.pc_addr);
                    fpush(i_base);
                    /* Reset pc to start of function. */
                    pc.pc_func = new_func;
                    pc.pc_addr = 0 as libc::c_int
                }
                current_block_215 = 613454377845503748;
            }
            68 => {
                /* Duplicate top of stack */
                push_copy((*ex_stack).s_num);
                current_block_215 = 613454377845503748;
            }
            75 => {
                /* Push a constant */
                /* Get the input base and convert it to a bc number. */
                if pc.pc_func == 0 as libc::c_int {
                    const_base = i_base
                } else {
                    const_base = (*fn_stack).s_val
                }
                if const_base == 10 as libc::c_int {
                    push_b10_const(&mut pc);
                } else {
                    push_constant(
                        Some(prog_char as unsafe extern "C" fn() -> libc::c_char),
                        const_base,
                    );
                }
                current_block_215 = 613454377845503748;
            }
            76 => {
                /* load array variable */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                load_array(var_name);
                current_block_215 = 613454377845503748;
            }
            77 => {
                /* decrement array variable (Minus!) */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                decr_array(var_name as libc::c_char);
                current_block_215 = 613454377845503748;
            }
            79 => {
                loop
                /* Write a string to the output with processing. */
                {
                    ch = byte(&mut pc) as libc::c_char;
                    if !(ch as libc::c_int != '\"' as i32) {
                        break;
                    }
                    if ch as libc::c_int != '\\' as i32 {
                        out_char(ch);
                    } else {
                        ch = byte(&mut pc) as libc::c_char;
                        if ch as libc::c_int == '\"' as i32 {
                            break;
                        }
                        match ch as libc::c_int {
                            110 => {
                                out_char('\n' as i32 as libc::c_char);
                            }
                            116 => {
                                out_char('\t' as i32 as libc::c_char);
                            }
                            114 => {
                                out_char('\r' as i32 as libc::c_char);
                            }
                            98 => {
                                out_char(0o7 as libc::c_int as libc::c_char);
                            }
                            102 => {
                                out_char('\u{c}' as i32 as libc::c_char);
                            }
                            92 => {
                                out_char('\\' as i32 as libc::c_char);
                            }
                            _ => {}
                        }
                    }
                }
                if interactive != 0 {
                    fflush(stdout);
                }
                current_block_215 = 613454377845503748;
            }
            82 => {
                /* Return from function */
                if pc.pc_func != 0 as libc::c_int {
                    /* "Pop" autos and parameters. */
                    pop_vars((*functions.offset(pc.pc_func as isize)).f_autos);
                    pop_vars((*functions.offset(pc.pc_func as isize)).f_params);
                    /* reset the pc. */
                    fpop();
                    pc.pc_addr = fpop();
                    pc.pc_func = fpop()
                } else {
                    rt_error(
                        b"Return from main program.\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                current_block_215 = 613454377845503748;
            }
            83 => {
                /* store array variable */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                store_array(var_name);
                current_block_215 = 613454377845503748;
            }
            84 => {
                /* Test tos for zero */
                c_code = is_zero((*ex_stack).s_num);
                assign(c_code);
                current_block_215 = 613454377845503748;
            }
            87 | 80 => {
                /* Write the value on the top of the stack. */
                /* Write the value on the top of the stack.  No newline. */
                out_num(
                    (*ex_stack).s_num,
                    o_base,
                    ::std::mem::transmute::<
                        Option<unsafe extern "C" fn(_: libc::c_char) -> ()>,
                        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
                    >(Some(
                        out_char as unsafe extern "C" fn(_: libc::c_char) -> (),
                    )),
                ); /* Special variable "last". */
                if inst as libc::c_int == 'W' as i32 {
                    out_char('\n' as i32 as libc::c_char);
                }
                store_var(3 as libc::c_int);
                if interactive != 0 {
                    fflush(stdout);
                }
                current_block_215 = 613454377845503748;
            }
            99 => {
                /* Call special function. */
                new_func = byte(&mut pc) as libc::c_int;
                match new_func {
                    76 => {
                        /* Length function. */
                        /* For the number 0.xxxx,  0 is not significant. */
                        if (*(*ex_stack).s_num).n_len == 1 as libc::c_int
                            && (*(*ex_stack).s_num).n_scale != 0 as libc::c_int
                            && (*(*ex_stack).s_num).n_value[0 as libc::c_int as usize]
                                as libc::c_int
                                == 0 as libc::c_int
                        {
                            int2num(&mut (*ex_stack).s_num, (*(*ex_stack).s_num).n_scale);
                        } else {
                            int2num(
                                &mut (*ex_stack).s_num,
                                (*(*ex_stack).s_num).n_len + (*(*ex_stack).s_num).n_scale,
                            );
                        }
                    }
                    83 => {
                        /* Scale function. */
                        int2num(&mut (*ex_stack).s_num, (*(*ex_stack).s_num).n_scale);
                    }
                    82 => {
                        /* Square Root function. */
                        if bc_sqrt(&mut (*ex_stack).s_num, scale) == 0 {
                            rt_error(
                                b"Square root of a negative number\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                    }
                    73 => {
                        /* Read function. */
                        push_constant(
                            Some(input_char as unsafe extern "C" fn() -> libc::c_char),
                            i_base,
                        );
                    }
                    _ => {}
                }
                current_block_215 = 613454377845503748;
            }
            100 => {
                /* Decrement number */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                decr_var(var_name);
                current_block_215 = 613454377845503748;
            }
            104 => {
                /* Halt the machine. */
                exit(0 as libc::c_int);
            }
            105 => {
                /* increment number */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                incr_var(var_name);
                current_block_215 = 613454377845503748;
            }
            108 => {
                /* load variable */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                load_var(var_name);
                current_block_215 = 613454377845503748;
            }
            110 => {
                /* Negate top of stack. */
                bc_sub(_zero_, (*ex_stack).s_num, &mut (*ex_stack).s_num);
                current_block_215 = 613454377845503748;
            }
            112 => {
                /* Pop the execution stack. */
                pop();
                current_block_215 = 613454377845503748;
            }
            115 => {
                /* store variable */
                var_name = byte(&mut pc) as libc::c_int;
                if var_name & 0x80 as libc::c_int != 0 as libc::c_int {
                    var_name = (var_name << 8 as libc::c_int & 0x7f as libc::c_int)
                        + byte(&mut pc) as libc::c_int
                }
                store_var(var_name);
                current_block_215 = 613454377845503748;
            }
            119 => {
                loop
                /* Write a string to the output. */
                {
                    ch = byte(&mut pc) as libc::c_char;
                    if !(ch as libc::c_int != '\"' as i32) {
                        break;
                    }
                    out_char(ch);
                }
                if interactive != 0 {
                    fflush(stdout);
                }
                current_block_215 = 613454377845503748;
            }
            120 => {
                /* Exchange Top of Stack with the one under the tos. */
                if check_stack(2 as libc::c_int) != 0 {
                    let mut temp: bc_num = (*ex_stack).s_num;
                    (*ex_stack).s_num = (*(*ex_stack).s_next).s_num;
                    (*(*ex_stack).s_next).s_num = temp
                }
                current_block_215 = 613454377845503748;
            }
            48 => {
                /* Load Constant 0. */
                push_copy(_zero_);
                current_block_215 = 613454377845503748;
            }
            49 => {
                /* Load Constant 0. */
                push_copy(_one_);
                current_block_215 = 613454377845503748;
            }
            33 => {
                /* Negate the boolean value on top of the stack. */
                c_code = is_zero((*ex_stack).s_num);
                assign(c_code);
                current_block_215 = 613454377845503748;
            }
            38 => {
                /* compare greater than */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (is_zero((*(*ex_stack).s_next).s_num) == 0
                        && is_zero((*ex_stack).s_num) == 0)
                        as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            124 => {
                /* compare greater than */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (is_zero((*(*ex_stack).s_next).s_num) == 0
                        || is_zero((*ex_stack).s_num) == 0)
                        as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            43 => {
                /* add */
                if check_stack(2 as libc::c_int) != 0 {
                    bc_add(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    init_num(&mut temp_num);
                }
                current_block_215 = 613454377845503748;
            }
            45 => {
                /* subtract */
                if check_stack(2 as libc::c_int) != 0 {
                    bc_sub(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    init_num(&mut temp_num);
                }
                current_block_215 = 613454377845503748;
            }
            42 => {
                /* multiply */
                if check_stack(2 as libc::c_int) != 0 {
                    bc_multiply(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    init_num(&mut temp_num);
                }
                current_block_215 = 613454377845503748;
            }
            47 => {
                /* divide */
                if check_stack(2 as libc::c_int) != 0 {
                    if bc_divide(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    ) == 0 as libc::c_int
                    {
                        pop();
                        pop();
                        push_num(temp_num);
                        init_num(&mut temp_num);
                    } else {
                        rt_error(
                            b"Divide by zero\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
                current_block_215 = 613454377845503748;
            }
            37 => {
                /* remainder */
                if check_stack(2 as libc::c_int) != 0 {
                    if is_zero((*ex_stack).s_num) != 0 {
                        rt_error(
                            b"Modulo by zero\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    } else {
                        bc_modulo(
                            (*(*ex_stack).s_next).s_num,
                            (*ex_stack).s_num,
                            &mut temp_num,
                            scale,
                        );
                        pop();
                        pop();
                        push_num(temp_num);
                        init_num(&mut temp_num);
                    }
                }
                current_block_215 = 613454377845503748;
            }
            94 => {
                /* raise */
                if check_stack(2 as libc::c_int) != 0 {
                    bc_raise(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    );
                    if is_zero((*(*ex_stack).s_next).s_num) as libc::c_int != 0
                        && is_neg((*ex_stack).s_num) as libc::c_int != 0
                    {
                        rt_error(
                            b"divide by zero\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    pop();
                    pop();
                    push_num(temp_num);
                    init_num(&mut temp_num);
                }
                current_block_215 = 613454377845503748;
            }
            61 => {
                /* compare equal */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == 0 as libc::c_int) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            35 => {
                /* compare not equal */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        != 0 as libc::c_int) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            60 => {
                /* compare less than */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == -(1 as libc::c_int)) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            123 => {
                /* compare less than or equal */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        <= 0 as libc::c_int) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            62 => {
                /* compare greater than */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == 1 as libc::c_int) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            125 => {
                /* compare greater than or equal */
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        >= 0 as libc::c_int) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 613454377845503748;
            }
            _ => {
                /* error! */
                rt_error(
                    b"bad instruction: inst=%c\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    inst as libc::c_int,
                ); /* Low order bits first. */
                current_block_215 = 613454377845503748;
            }
        }
        match current_block_215 {
            16102553208965294739 =>
            /* Jump to a label. */
            {
                label_num = byte(&mut pc) as libc::c_int;
                label_num += (byte(&mut pc) as libc::c_int) << 8 as libc::c_int;
                if inst as libc::c_int == 'J' as i32
                    || inst as libc::c_int == 'B' as i32 && c_code as libc::c_int != 0
                    || inst as libc::c_int == 'Z' as i32 && c_code == 0
                {
                    gp = (*functions.offset(pc.pc_func as isize)).f_label;
                    l_gp = label_num >> BC_LABEL_LOG;
                    l_off = label_num % BC_LABEL_GROUP;
                    loop {
                        let fresh1 = l_gp;
                        l_gp = l_gp - 1;
                        if !(fresh1 > 0 as libc::c_int) {
                            break;
                        }
                        gp = (*gp).l_next
                    }
                    pc.pc_addr = (*gp).l_adrs[l_off as usize] as libc::c_int
                }
            }
            _ => {}
        }
    }
    /* Clean up the function stack and pop all autos/parameters. */
    while pc.pc_func != 0 as libc::c_int {
        pop_vars((*functions.offset(pc.pc_func as isize)).f_autos);
        pop_vars((*functions.offset(pc.pc_func as isize)).f_params);
        fpop();
        pc.pc_addr = fpop();
        pc.pc_func = fpop()
    }
    /* Clean up the execution stack. */
    while !ex_stack.is_null() {
        pop();
    }
    /* Clean up the interrupt stuff. */
    if interactive != 0 {
        signal(
            SIGINT,
            Some(use_quit as unsafe extern "C" fn(_: libc::c_int) -> ()),
        );
        if had_sigint != 0 {
            printf(b"Interruption completed.\n\x00" as *const u8 as *const libc::c_char);
        }
    };
}
/* Prog_char gets another byte from the program.  It is used for
conversion of text constants in the code to numbers. */
#[no_mangle]
pub unsafe extern "C" fn prog_char() -> libc::c_char {
    return byte(&mut pc) as libc::c_char;
}
/* Read a character from the standard input.  This function is used
by the "read" function. */
#[no_mangle]
pub unsafe extern "C" fn input_char() -> libc::c_char {
    let mut in_ch: libc::c_char = 0;
    /* Get a character from the standard input for the read function. */
    in_ch = getchar() as libc::c_char;
    /* Check for a \ quoted newline. */
    if in_ch as libc::c_int == '\\' as i32 {
        in_ch = getchar() as libc::c_char;
        if in_ch as libc::c_int == '\n' as i32 {
            in_ch = getchar() as libc::c_char
        }
    }
    /* Classify and preprocess the input character. */
    if *(*__ctype_b_loc()).offset(in_ch as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        != 0
    {
        return (in_ch as libc::c_int - '0' as i32) as libc::c_char;
    }
    if in_ch as libc::c_int >= 'A' as i32 && in_ch as libc::c_int <= 'F' as i32 {
        return (in_ch as libc::c_int + 10 as libc::c_int - 'A' as i32) as libc::c_char;
    }
    if in_ch as libc::c_int >= 'a' as i32 && in_ch as libc::c_int <= 'f' as i32 {
        return (in_ch as libc::c_int + 10 as libc::c_int - 'a' as i32) as libc::c_char;
    }
    if in_ch as libc::c_int == '.' as i32
        || in_ch as libc::c_int == '+' as i32
        || in_ch as libc::c_int == '-' as i32
    {
        return in_ch;
    }
    if in_ch as libc::c_int <= ' ' as i32 {
        return ' ' as i32 as libc::c_char;
    }
    return ':' as i32 as libc::c_char;
}
/* Push_constant converts a sequence of input characters as returned
by IN_CHAR into a number.  The number is pushed onto the execution
stack.  The number is converted as a number in base CONV_BASE. */
#[no_mangle]
pub unsafe extern "C" fn push_constant(
    mut in_char: Option<unsafe extern "C" fn() -> libc::c_char>,
    mut conv_base: libc::c_int,
) {
    let mut digits: libc::c_int = 0;
    let mut build: bc_num = 0 as *mut bc_struct;
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut result: bc_num = 0 as *mut bc_struct;
    let mut mult: bc_num = 0 as *mut bc_struct;
    let mut divisor: bc_num = 0 as *mut bc_struct;
    let mut in_ch: libc::c_char = 0;
    let mut first_ch: libc::c_char = 0;
    let mut negative: libc::c_char = 0;
    /* Initialize all bc numbers */
    init_num(&mut temp);
    init_num(&mut result);
    init_num(&mut mult);
    build = copy_num(_zero_);
    negative = FALSE as libc::c_char;
    /* The conversion base. */
    int2num(&mut mult, conv_base);
    /* Get things ready. */
    in_ch = in_char.expect("non-null function pointer")();
    while in_ch as libc::c_int == ' ' as i32 {
        in_ch = in_char.expect("non-null function pointer")()
    }
    if in_ch as libc::c_int == '+' as i32 {
        in_ch = in_char.expect("non-null function pointer")()
    } else if in_ch as libc::c_int == '-' as i32 {
        negative = TRUE as libc::c_char;
        in_ch = in_char.expect("non-null function pointer")()
    }
    /* Check for the special case of a single digit. */
    if (in_ch as libc::c_int) < 16 as libc::c_int {
        first_ch = in_ch;
        in_ch = in_char.expect("non-null function pointer")();
        if (in_ch as libc::c_int) < 16 as libc::c_int && first_ch as libc::c_int >= conv_base {
            first_ch = (conv_base - 1 as libc::c_int) as libc::c_char
        }
        int2num(&mut build, first_ch as libc::c_int);
    }
    /* Convert the integer part. */
    while (in_ch as libc::c_int) < 16 as libc::c_int {
        if (in_ch as libc::c_int) < 16 as libc::c_int && in_ch as libc::c_int >= conv_base {
            in_ch = (conv_base - 1 as libc::c_int) as libc::c_char
        }
        bc_multiply(build, mult, &mut result, 0 as libc::c_int);
        int2num(&mut temp, in_ch as libc::c_int);
        bc_add(result, temp, &mut build);
        in_ch = in_char.expect("non-null function pointer")()
    }
    if in_ch as libc::c_int == '.' as i32 {
        in_ch = in_char.expect("non-null function pointer")();
        if in_ch as libc::c_int >= conv_base {
            in_ch = (conv_base - 1 as libc::c_int) as libc::c_char
        }
        free_num(&mut result);
        free_num(&mut temp);
        divisor = copy_num(_one_);
        result = copy_num(_zero_);
        digits = 0 as libc::c_int;
        while (in_ch as libc::c_int) < 16 as libc::c_int {
            bc_multiply(result, mult, &mut result, 0 as libc::c_int);
            int2num(&mut temp, in_ch as libc::c_int);
            bc_add(result, temp, &mut result);
            bc_multiply(divisor, mult, &mut divisor, 0 as libc::c_int);
            digits += 1;
            in_ch = in_char.expect("non-null function pointer")();
            if (in_ch as libc::c_int) < 16 as libc::c_int && in_ch as libc::c_int >= conv_base {
                in_ch = (conv_base - 1 as libc::c_int) as libc::c_char
            }
        }
        bc_divide(result, divisor, &mut result, digits);
        bc_add(build, result, &mut build);
    }
    /* Final work.  */
    if negative != 0 {
        bc_sub(_zero_, build, &mut build);
    }
    push_num(build);
    free_num(&mut temp);
    free_num(&mut result);
    free_num(&mut mult);
}
/* When converting base 10 constants from the program, we use this
more efficient way to convert them to numbers.  PC tells where
the constant starts and is expected to be advanced to after
the constant. */
#[no_mangle]
pub unsafe extern "C" fn push_b10_const(mut pc_0: *mut program_counter) {
    let mut build: bc_num = 0 as *mut bc_struct;
    let mut look_pc: program_counter = program_counter {
        pc_func: 0,
        pc_addr: 0,
    };
    let mut kdigits: libc::c_int = 0;
    let mut kscale: libc::c_int = 0;
    let mut inchar: libc::c_char = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Count the digits and get things ready. */
    look_pc = *pc_0;
    kdigits = 0 as libc::c_int;
    kscale = 0 as libc::c_int;
    inchar = byte(&mut look_pc) as libc::c_char;
    while inchar as libc::c_int != '.' as i32 && inchar as libc::c_int != ':' as i32 {
        kdigits += 1;
        inchar = byte(&mut look_pc) as libc::c_char
    }
    if inchar as libc::c_int == '.' as i32 {
        inchar = byte(&mut look_pc) as libc::c_char;
        while inchar as libc::c_int != ':' as i32 {
            kscale += 1;
            inchar = byte(&mut look_pc) as libc::c_char
        }
    }
    /* Get the first character again and move the pc. */
    inchar = byte(pc_0) as libc::c_char;
    /* Secial cases of 0, 1, and A-F single inputs. */
    if kdigits == 1 as libc::c_int && kscale == 0 as libc::c_int {
        if inchar as libc::c_int == 0 as libc::c_int {
            push_copy(_zero_);
            inchar = byte(pc_0) as libc::c_char;
            return;
        }
        if inchar as libc::c_int == 1 as libc::c_int {
            push_copy(_one_);
            inchar = byte(pc_0) as libc::c_char;
            return;
        }
        if inchar as libc::c_int > 9 as libc::c_int {
            init_num(&mut build);
            int2num(&mut build, inchar as libc::c_int);
            push_num(build);
            inchar = byte(pc_0) as libc::c_char;
            return;
        }
    }
    /* Build the new number. */
    if kdigits == 0 as libc::c_int {
        build = new_num(1 as libc::c_int, kscale);
        ptr = &mut *(*build)
            .n_value
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char;
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = 0 as libc::c_int as libc::c_char
    } else {
        build = new_num(kdigits, kscale);
        ptr = &mut *(*build)
            .n_value
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char
    }
    while inchar as libc::c_int != ':' as i32 {
        if inchar as libc::c_int != '.' as i32 {
            if inchar as libc::c_int > 9 as libc::c_int {
                let fresh3 = ptr;
                ptr = ptr.offset(1);
                *fresh3 = 9 as libc::c_int as libc::c_char
            } else {
                let fresh4 = ptr;
                ptr = ptr.offset(1);
                *fresh4 = inchar
            }
        }
        inchar = byte(pc_0) as libc::c_char
    }
    push_num(build);
}
/* proto.h: Prototype function definitions for "external" functions. */
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
/* For the pc version using k&r ACK. (minix1.5 and earlier.) */
/* Include the standard library header files. */
/* Define the _PROTOTYPE macro if it is needed. */
/* From execute.c */
/* Put the correct value on the stack for C_CODE.  Frees TOS num. */
#[no_mangle]
pub unsafe extern "C" fn assign(mut c_code_0: libc::c_char) {
    free_num(&mut (*ex_stack).s_num);
    if c_code_0 != 0 {
        (*ex_stack).s_num = copy_num(_one_)
    } else {
        (*ex_stack).s_num = copy_num(_zero_)
    };
}
