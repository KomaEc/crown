use ::libc;
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
pub struct bc_var {
    pub v_value: bc_num,
    pub v_next: *mut bc_var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array_node {
    pub n_num: [bc_num; 16],
    pub n_down: [*mut bc_array_node; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array {
    pub a_tree: *mut bc_array_node,
    pub a_depth: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var_array {
    pub a_value: *mut bc_array,
    pub a_param: libc::c_char,
    pub a_next: *mut bc_var_array,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_rec {
    pub id: *mut libc::c_char,
    pub a_name: libc::c_int,
    pub f_name: libc::c_int,
    pub v_name: libc::c_int,
    pub balance: libc::c_short,
    pub left: *mut id_rec,
    pub right: *mut id_rec,
}
/* global.h:  The global variables for bc.  */
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
/* For the current "break level" and if statements. */
#[no_mangle]
pub static mut break_label: libc::c_int = 0;
#[no_mangle]
pub static mut if_label: libc::c_int = 0;
#[no_mangle]
pub static mut continue_label: libc::c_int = 0;
/* Label numbers. */
#[no_mangle]
pub static mut next_label: libc::c_int = 0;
/* Used for "code" generation. */
#[no_mangle]
pub static mut genstr: [libc::c_char; 80] = [0; 80];
#[no_mangle]
pub static mut out_count: libc::c_int = 0;
#[no_mangle]
pub static mut did_gen: libc::c_char = 0;
/* Interactive and other flags. */
#[no_mangle]
pub static mut interactive: libc::c_char = 0;
#[no_mangle]
pub static mut compile_only: libc::c_char = 0;
#[no_mangle]
pub static mut use_math: libc::c_char = 0;
#[no_mangle]
pub static mut warn_not_std: libc::c_char = 0;
#[no_mangle]
pub static mut std_only: libc::c_char = 0;
/* global variables for the bc machine. All will be dynamic in size.*/
/* Function storage. main is (0) and functions (1-f_count) */
#[no_mangle]
pub static mut functions: *mut bc_function = 0 as *const bc_function as *mut bc_function;
#[no_mangle]
pub static mut f_names: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut f_count: libc::c_int = 0;
/* Variable stoarge and reverse names. */
#[no_mangle]
pub static mut variables: *mut *mut bc_var = 0 as *const *mut bc_var as *mut *mut bc_var;
#[no_mangle]
pub static mut v_names: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut v_count: libc::c_int = 0;
/* Array Variable storage and reverse names. */
#[no_mangle]
pub static mut arrays: *mut *mut bc_var_array =
    0 as *const *mut bc_var_array as *mut *mut bc_var_array;
#[no_mangle]
pub static mut a_names: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut a_count: libc::c_int = 0;
/* Execution stack. */
#[no_mangle]
pub static mut ex_stack: *mut estack_rec = 0 as *const estack_rec as *mut estack_rec;
/* Function return stack. */
#[no_mangle]
pub static mut fn_stack: *mut fstack_rec = 0 as *const fstack_rec as *mut fstack_rec;
/* Other "storage". */
#[no_mangle]
pub static mut i_base: libc::c_int = 0;
#[no_mangle]
pub static mut o_base: libc::c_int = 0;
#[no_mangle]
pub static mut scale: libc::c_int = 0;
#[no_mangle]
pub static mut c_code: libc::c_char = 0;
#[no_mangle]
pub static mut out_col: libc::c_int = 0;
#[no_mangle]
pub static mut runtime_error: libc::c_char = 0;
#[no_mangle]
pub static mut pc: program_counter = program_counter {
    pc_func: 0,
    pc_addr: 0,
};
/* Input Line numbers and other error information. */
#[no_mangle]
pub static mut line_no: libc::c_int = 0;
#[no_mangle]
pub static mut had_error: libc::c_int = 0;
/* For larger identifiers, a tree, and how many "storage" locations
have been allocated. */
#[no_mangle]
pub static mut next_array: libc::c_int = 0;
#[no_mangle]
pub static mut next_func: libc::c_int = 0;
#[no_mangle]
pub static mut next_var: libc::c_int = 0;
#[no_mangle]
pub static mut name_tree: *mut id_rec = 0 as *const id_rec as *mut id_rec;
/* For error message production */
#[no_mangle]
pub static mut g_argv: *mut *mut libc::c_char =
    0 as *const *mut libc::c_char as *mut *mut libc::c_char;
#[no_mangle]
pub static mut g_argc: libc::c_int = 0;
#[no_mangle]
pub static mut is_std_in: libc::c_char = 0;
/* global.c:  This defines the global variables. */
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
/* Since we want to define them here, we use the following define. */
/* Define all the global variables for bc. */
#[no_mangle]
pub static mut libmath: [libc::c_char; 2140] = unsafe {
    *::std::mem::transmute::<&[u8; 2140],
                                 &[libc::c_char; 2140]>(b"@iK20:s2:p@r@iF1,4.5,6,7,8,9,10,11,12[l4:0<Z0:1s10:pl4:ns4:pN0:l2:s12:pK4:l12:+K.44:l4:*+s2:pN1:l4:1>Z2:l8:1+s8:pl4:K2:/s4:pJ1:N2:1l4:+s11:pl4:s5:p1s6:pK2:s9:pN4:1B5:J3:N6:l9:i9:pJ4:N5:l5:l4:*s5:l6:l9:*s6:/s7:pl7:0=Z7:l8:0>Z8:N9:l8:d8:Z10:l11:l11:*s11:pJ9:N10:N8:l12:s2:pl10:Z11:1l11:/RN11:l11:1/RN7:l11:l7:+s11:pJ6:N3:0R]@r@iF2,4.7,8,9,10,13,11,12[l4:0{Z0:1K10:l2:^-RN0:l2:s12:pl2:K4:+s2:pK2:s8:p0s9:pN1:l4:K2:}Z2:l8:K2:*s8:pl4:cRs4:pJ1:N2:N3:l4:K.5:{Z4:l8:K2:*s8:pl4:cRs4:pJ3:N4:l4:1-l4:1+/s13:s11:pl13:l13:*s10:pK3:s9:pN6:1B7:J5:N8:l9:K2:+s9:pJ6:N7:l13:l10:*s13:l9:/s7:pl7:0=Z9:l8:l11:*s11:pl12:s2:pl11:1/RN9:l11:l7:+s11:pJ8:N5:0R]@r@iF3,4.7,9,10,13,14,11,12[l2:s12:pK1.1:l12:*1+s2:p1C4,0:s11:pl4:0<Z0:1s10:pl4:ns4:pN0:0s2:pl4:l11:/K2:+K4:/s13:pl4:K4:l13:*l11:*-s4:pl13:K2:%Z1:l4:ns4:pN1:l12:K2:+s2:pl4:s7:s11:pl4:nl4:*s14:pK3:s9:pN3:1B4:J2:N5:l9:K2:+s9:pJ3:N4:l7:l14:l9:l9:1-*/*s7:pl7:0=Z6:l12:s2:pl10:Z7:l11:n1/RN7:l11:1/RN6:l11:l7:+s11:pJ5:N2:0R]@r@iF5,4.11[l2:1+s2:pl4:1C4,0:K2:*+C3,0:s11:pl2:1-s2:pl11:1/R0R]@r@iF4,4.5,7,8,9,10,13,14,11,12[l4:1=Z0:l2:K25:{Z1:K.7853981633974483096156608:1/RN1:l2:K40:{Z2:K.7853981633974483096156608458198757210492:1/RN2:l2:K60:{Z3:K.785398163397448309615660845819875721049292349843776455243736:1/RN3:N0:l4:K.2:=Z4:l2:K25:{Z5:K.1973955598498807583700497:1/RN5:l2:K40:{Z6:K.1973955598498807583700497651947902934475:1/RN6:l2:K60:{Z7:K.197395559849880758370049765194790293447585103787852101517688:1/RN7:N4:l4:0<Z8:1s10:pl4:ns4:pN8:l2:s12:pl4:K.2:>Z9:l12:K4:+s2:pK.2:C4,0:s5:pN9:l12:K2:+s2:pN10:l4:K.2:>Z11:l8:1+s8:pl4:K.2:-1l4:K.2:*+/s4:pJ10:N11:l4:s13:s11:pl4:nl4:*s14:pK3:s9:pN13:1B14:J12:N15:l9:K2:+s9:pJ13:N14:l13:l14:*s13:l9:/s7:pl7:0=Z16:l12:s2:pl10:Z17:l8:l5:*l11:+1n/RN17:l8:l5:*l11:+1/RN16:l11:l7:+s11:pJ15:N12:0R]@r@iF6,13,4.5,6,7,8,9,10,14,11,12[l2:s12:p0s2:pl13:1/s13:pl13:0<Z0:l13:ns13:pl13:K2:%1=Z1:1s10:pN1:N0:1s8:pK2:s9:pN3:l9:l13:{B4:J2:N5:l9:i9:pJ3:N4:l8:l9:*s8:pJ5:N2:K1.5:l12:*s2:pl4:l13:^K2:l13:^/l8:/s8:p1s7:s11:pl4:nl4:*K4:/s14:pK1.5:l12:*s2:p1s9:pN7:1B8:J6:N9:l9:i9:pJ7:N8:l7:l14:*l9:/l13:l9:+/s7:pl7:0=Z10:l12:s2:pl10:Z11:l8:nl11:*1/RN11:l8:l11:*1/RN10:l11:l7:+s11:pJ9:N6:0R]@r\x00")
};
