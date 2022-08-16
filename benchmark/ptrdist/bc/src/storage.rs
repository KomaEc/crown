use ::libc;
extern "C" {
    /* global variables for the bc machine. All will be dynamic in size.*/
    /* Function storage. main is (0) and functions (1-f_count) */
    #[no_mangle]
    static mut functions: *mut bc_function;
    #[no_mangle]
    static mut f_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut f_count: libc::c_int;
    /* Variable stoarge and reverse names. */
    #[no_mangle]
    static mut variables: *mut *mut bc_var;
    #[no_mangle]
    static mut v_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut v_count: libc::c_int;
    /* Array Variable storage and reverse names. */
    #[no_mangle]
    static mut arrays: *mut *mut bc_var_array;
    #[no_mangle]
    static mut a_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut a_count: libc::c_int;
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
    /* defined in number.c */
    #[no_mangle]
    static mut _zero_: bc_num;
    #[no_mangle]
    static mut _one_: bc_num;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn byte(pc: *mut program_counter) -> libc::c_uchar;
    #[no_mangle]
    fn free_args(args: *mut arg_list);
    #[no_mangle]
    fn rt_error(mesg: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn rt_warn(mesg: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn free_num(num: *mut bc_num);
    #[no_mangle]
    fn init_numbers();
    #[no_mangle]
    fn copy_num(num: bc_num) -> bc_num;
    #[no_mangle]
    fn init_num(num: *mut bc_num);
    #[no_mangle]
    fn int2num(num: *mut bc_num, val: libc::c_int);
    #[no_mangle]
    fn num2long(num: bc_num) -> libc::c_long;
    #[no_mangle]
    fn is_zero(num: bc_num) -> libc::c_char;
    #[no_mangle]
    fn is_neg(num: bc_num) -> libc::c_char;
    #[no_mangle]
    fn bc_add(n1: bc_num, n2: bc_num, result: *mut bc_num);
    #[no_mangle]
    fn bc_sub(n1: bc_num, n2: bc_num, result: *mut bc_num);
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
pub type bc_num = *mut bc_struct;
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
/* Variables are "pushable" (auto) and thus we need a stack mechanism.
This is built into the variable record. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var {
    pub v_value: bc_num,
    pub v_next: *mut bc_var,
}
/* bc arrays can also be "auto" variables and thus need the same
kind of stacking mechanisms. */
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
/* For the stacks, execution and function, we need records to allow
for arbitrary size. */
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
pub const BC_DIM_MAX: libc::c_int = 65535 as libc::c_int;
pub const BC_BASE_MAX: libc::c_int = 2147483647 as libc::c_int;
pub const BC_SCALE_MAX: libc::c_int = INT_MAX;
pub const INT_MAX: libc::c_int = 2147483647 as libc::c_int;
/* this should be NODE_SIZE^NODE_DEPTH-1 */
/* Must be a power of 2. */
pub const NODE_MASK: libc::c_int = 0xf as libc::c_int;
/* Must be NODE_SIZE-1. */
pub const NODE_SHIFT: libc::c_int = 4 as libc::c_int;
pub const NODE_SIZE: libc::c_int = 16 as libc::c_int;
/* Number of 1 bits in NODE_MASK. */
/* Other BC limits defined but not part of POSIX. */
/* Code segments. */
/* Maximum number of variables, arrays and functions and the
allocation increment for the dynamic arrays. */
/* Other interesting constants. */
pub const TRUE: libc::c_int = 1 as libc::c_int;
pub const BC_MAX_SEGS: libc::c_int = 16 as libc::c_int;
pub const STORE_INCR: libc::c_int = 32 as libc::c_int;
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
/* storage.c:  Code and data storage manipulations.  This includes labels. */
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
/* Initialize the storage at the beginning of the run. */
#[no_mangle]
pub unsafe extern "C" fn init_storage() {
    /* Functions: we start with none and ask for more. */
    f_count = 0 as libc::c_int;
    more_functions();
    let ref mut fresh0 = *f_names.offset(0 as libc::c_int as isize);
    *fresh0 = b"(main)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    /* Variables. */
    v_count = 0 as libc::c_int;
    more_variables();
    /* Arrays. */
    a_count = 0 as libc::c_int;
    more_arrays();
    /* Other things... */
    ex_stack = NULL as *mut estack_rec;
    fn_stack = NULL as *mut fstack_rec;
    i_base = 10 as libc::c_int;
    o_base = 10 as libc::c_int;
    scale = 0 as libc::c_int;
    c_code = FALSE as libc::c_char;
    init_numbers();
}
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
/* Three functions for increasing the number of functions, variables, or
arrays that are needed.  This adds another 32 of the requested object. */
#[no_mangle]
pub unsafe extern "C" fn more_functions() {
    let mut old_count: libc::c_int = 0;
    let mut indx1: libc::c_int = 0;
    let mut indx2: libc::c_int = 0;
    let mut old_f: *mut bc_function = 0 as *mut bc_function;
    let mut f: *mut bc_function = 0 as *mut bc_function;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Save old information. */
    old_count = f_count;
    old_f = functions;
    old_names = f_names;
    /* Add a fixed amount and allocate new space. */
    f_count += STORE_INCR;
    functions = malloc(
        (f_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<bc_function>() as libc::c_ulong),
    ) as *mut bc_function;
    f_names = malloc(
        (f_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    /* Copy old ones. */
    indx1 = 0 as libc::c_int;
    while indx1 < old_count {
        *functions.offset(indx1 as isize) = *old_f.offset(indx1 as isize);
        let ref mut fresh1 = *f_names.offset(indx1 as isize);
        *fresh1 = *old_names.offset(indx1 as isize);
        indx1 += 1
    }
    /* Initialize the new ones. */
    while indx1 < f_count {
        f = &mut *functions.offset(indx1 as isize) as *mut bc_function;
        (*f).f_defined = FALSE as libc::c_char;
        indx2 = 0 as libc::c_int;
        while indx2 < BC_MAX_SEGS {
            (*f).f_body[indx2 as usize] = NULL as *mut libc::c_char;
            indx2 += 1
        }
        (*f).f_code_size = 0 as libc::c_int;
        (*f).f_label = NULL as *mut bc_label_group;
        (*f).f_autos = NULL as *mut arg_list;
        (*f).f_params = NULL as *mut arg_list;
        indx1 += 1
    }
    /* Free the old elements. */
    if old_count != 0 as libc::c_int {
        free(old_f as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn more_variables() {
    let mut indx: libc::c_int = 0;
    let mut old_count: libc::c_int = 0;
    let mut old_var: *mut *mut bc_var = 0 as *mut *mut bc_var;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Save the old values. */
    old_count = v_count;
    old_var = variables;
    old_names = v_names;
    /* Increment by a fixed amount and allocate. */
    v_count += STORE_INCR;
    variables = malloc(
        (v_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut bc_var>() as libc::c_ulong),
    ) as *mut *mut bc_var;
    v_names = malloc(
        (v_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    /* Copy the old variables. */
    indx = 3 as libc::c_int;
    while indx < old_count {
        let ref mut fresh2 = *variables.offset(indx as isize);
        *fresh2 = *old_var.offset(indx as isize);
        indx += 1
    }
    /* Initialize the new elements. */
    while indx < v_count {
        let ref mut fresh3 = *variables.offset(indx as isize);
        *fresh3 = NULL as *mut bc_var;
        indx += 1
    }
    /* Free the old elements. */
    if old_count != 0 as libc::c_int {
        free(old_var as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn more_arrays() {
    let mut indx: libc::c_int = 0;
    let mut old_count: libc::c_int = 0;
    let mut old_ary: *mut *mut bc_var_array = 0 as *mut *mut bc_var_array;
    let mut old_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    /* Save the old values. */
    old_count = a_count;
    old_ary = arrays;
    old_names = a_names;
    /* Increment by a fixed amount and allocate. */
    a_count += STORE_INCR;
    arrays = malloc(
        (a_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut bc_var_array>() as libc::c_ulong),
    ) as *mut *mut bc_var_array;
    a_names = malloc(
        (a_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    /* Copy the old arrays. */
    indx = 1 as libc::c_int;
    while indx < old_count {
        let ref mut fresh4 = *arrays.offset(indx as isize);
        *fresh4 = *old_ary.offset(indx as isize);
        indx += 1
    }
    /* Initialize the new elements. */
    while indx < v_count {
        let ref mut fresh5 = *arrays.offset(indx as isize);
        *fresh5 = NULL as *mut bc_var_array;
        indx += 1
    }
    /* Free the old elements. */
    if old_count != 0 as libc::c_int {
        free(old_ary as *mut libc::c_void);
        free(old_names as *mut libc::c_void);
    };
}
/* clear_func clears out function FUNC and makes it ready to redefine. */
#[no_mangle]
pub unsafe extern "C" fn clear_func(mut func: libc::c_char) {
    let mut f: *mut bc_function = 0 as *mut bc_function;
    let mut indx: libc::c_int = 0;
    let mut lg: *mut bc_label_group = 0 as *mut bc_label_group;
    /* Set the pointer to the function. */
    f = &mut *functions.offset(func as isize) as *mut bc_function;
    (*f).f_defined = FALSE as libc::c_char;
    /* Clear the code segments. */
    indx = 0 as libc::c_int;
    while indx < BC_MAX_SEGS {
        if !(*f).f_body[indx as usize].is_null() {
            free((*f).f_body[indx as usize] as *mut libc::c_void);
            (*f).f_body[indx as usize] = NULL as *mut libc::c_char
        }
        indx += 1
    }
    (*f).f_code_size = 0 as libc::c_int;
    if !(*f).f_autos.is_null() {
        free_args((*f).f_autos);
        (*f).f_autos = NULL as *mut arg_list
    }
    if !(*f).f_params.is_null() {
        free_args((*f).f_params);
        (*f).f_params = NULL as *mut arg_list
    }
    while !(*f).f_label.is_null() {
        lg = (*(*f).f_label).l_next;
        free((*f).f_label as *mut libc::c_void);
        (*f).f_label = lg
    }
}
/*  Pop the function execution stack and return the top. */
#[no_mangle]
pub unsafe extern "C" fn fpop() -> libc::c_int {
    let mut temp: *mut fstack_rec = 0 as *mut fstack_rec;
    let mut retval: libc::c_int = 0;
    if !fn_stack.is_null() {
        temp = fn_stack;
        fn_stack = (*temp).s_next;
        retval = (*temp).s_val;
        free(temp as *mut libc::c_void);
    }
    return retval;
}
/* Push  on to the function stack. */
#[no_mangle]
pub unsafe extern "C" fn fpush(mut val: libc::c_int) {
    let mut temp: *mut fstack_rec = 0 as *mut fstack_rec;
    temp = malloc(::std::mem::size_of::<fstack_rec>() as libc::c_ulong) as *mut fstack_rec;
    (*temp).s_next = fn_stack;
    (*temp).s_val = val;
    fn_stack = temp;
}
/* Pop and discard the top element of the regular execution stack. */
#[no_mangle]
pub unsafe extern "C" fn pop() {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    if !ex_stack.is_null() {
        temp = ex_stack;
        ex_stack = (*temp).s_next;
        free_num(&mut (*temp).s_num);
        free(temp as *mut libc::c_void);
    };
}
/* Push a copy of NUM on to the regular execution stack. */
#[no_mangle]
pub unsafe extern "C" fn push_copy(mut num: bc_num) {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = malloc(::std::mem::size_of::<estack_rec>() as libc::c_ulong) as *mut estack_rec;
    (*temp).s_num = copy_num(num);
    (*temp).s_next = ex_stack;
    ex_stack = temp;
}
/* Push NUM on to the regular execution stack.  Do NOT push a copy. */
#[no_mangle]
pub unsafe extern "C" fn push_num(mut num: bc_num) {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = malloc(::std::mem::size_of::<estack_rec>() as libc::c_ulong) as *mut estack_rec;
    (*temp).s_num = num;
    (*temp).s_next = ex_stack;
    ex_stack = temp;
}
/* Make sure the ex_stack has at least DEPTH elements on it.
Return TRUE if it has at least DEPTH elements, otherwise
return FALSE. */
#[no_mangle]
pub unsafe extern "C" fn check_stack(mut depth: libc::c_int) -> libc::c_char {
    let mut temp: *mut estack_rec = 0 as *mut estack_rec;
    temp = ex_stack;
    while !temp.is_null() && depth > 0 as libc::c_int {
        temp = (*temp).s_next;
        depth -= 1
    }
    if depth > 0 as libc::c_int {
        rt_error(b"Stack error.\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return FALSE as libc::c_char;
    }
    return TRUE as libc::c_char;
}
/* The following routines manipulate simple variables and
array variables. */
/* get_var returns a pointer to the variable VAR_NAME.  If one does not
exist, one is created. */
#[no_mangle]
pub unsafe extern "C" fn get_var(mut var_name: libc::c_int) -> *mut bc_var {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    var_ptr = *variables.offset(var_name as isize);
    if var_ptr.is_null() {
        let ref mut fresh6 = *variables.offset(var_name as isize);
        *fresh6 = malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong) as *mut bc_var;
        var_ptr = *fresh6;
        init_num(&mut (*var_ptr).v_value);
    }
    return var_ptr;
}
/* get_array_num returns the address of the bc_num in the array
structure.  If more structure is requried to get to the index,
this routine does the work to create that structure. VAR_INDEX
is a zero based index into the arrays storage array. INDEX is
the index into the bc array. */
#[no_mangle]
pub unsafe extern "C" fn get_array_num(
    mut var_index: libc::c_int,
    mut index: libc::c_long,
) -> *mut bc_num {
    let mut ary_ptr: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut a_var: *mut bc_array = 0 as *mut bc_array;
    let mut temp: *mut bc_array_node = 0 as *mut bc_array_node;
    let mut log: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut ix1: libc::c_int = 0;
    let mut sub: [libc::c_int; 4] = [0; 4];
    /* Get the array entry. */
    ary_ptr = *arrays.offset(var_index as isize);
    if ary_ptr.is_null() {
        let ref mut fresh7 = *arrays.offset(var_index as isize);
        *fresh7 =
            malloc(::std::mem::size_of::<bc_var_array>() as libc::c_ulong) as *mut bc_var_array;
        ary_ptr = *fresh7;
        (*ary_ptr).a_value = NULL as *mut bc_array;
        (*ary_ptr).a_next = NULL as *mut bc_var_array;
        (*ary_ptr).a_param = FALSE as libc::c_char
    }
    a_var = (*ary_ptr).a_value;
    if a_var.is_null() {
        (*ary_ptr).a_value =
            malloc(::std::mem::size_of::<bc_array>() as libc::c_ulong) as *mut bc_array;
        a_var = (*ary_ptr).a_value;
        (*a_var).a_tree = NULL as *mut bc_array_node;
        (*a_var).a_depth = 0 as libc::c_int as libc::c_short
    }
    /* Get the index variable. */
    sub[0 as libc::c_int as usize] = (index & NODE_MASK as libc::c_long) as libc::c_int;
    ix = (index >> NODE_SHIFT) as libc::c_int;
    log = 1 as libc::c_int;
    while ix > 0 as libc::c_int || log < (*a_var).a_depth as libc::c_int {
        sub[log as usize] = ix & NODE_MASK;
        ix >>= NODE_SHIFT;
        log += 1
    }
    /* Build any tree that is necessary. */
    while log > (*a_var).a_depth as libc::c_int {
        temp =
            malloc(::std::mem::size_of::<bc_array_node>() as libc::c_ulong) as *mut bc_array_node;
        if (*a_var).a_depth as libc::c_int != 0 as libc::c_int {
            (*temp).n_down[0 as libc::c_int as usize] = (*a_var).a_tree;
            ix = 1 as libc::c_int;
            while ix < NODE_SIZE {
                (*temp).n_down[ix as usize] = NULL as *mut bc_array_node;
                ix += 1
            }
        } else {
            ix = 0 as libc::c_int;
            while ix < NODE_SIZE {
                (*temp).n_num[ix as usize] = copy_num(_zero_);
                ix += 1
            }
        }
        (*a_var).a_tree = temp;
        (*a_var).a_depth += 1
    }
    /* Find the indexed variable. */
    temp = (*a_var).a_tree;
    loop {
        let fresh8 = log;
        log = log - 1;
        if !(fresh8 > 1 as libc::c_int) {
            break;
        }
        ix1 = sub[log as usize];
        if (*temp).n_down[ix1 as usize].is_null() {
            (*temp).n_down[ix1 as usize] =
                malloc(::std::mem::size_of::<bc_array_node>() as libc::c_ulong)
                    as *mut bc_array_node;
            temp = (*temp).n_down[ix1 as usize];
            if log > 1 as libc::c_int {
                ix = 0 as libc::c_int;
                while ix < NODE_SIZE {
                    (*temp).n_down[ix as usize] = NULL as *mut bc_array_node;
                    ix += 1
                }
            } else {
                ix = 0 as libc::c_int;
                while ix < NODE_SIZE {
                    (*temp).n_num[ix as usize] = copy_num(_zero_);
                    ix += 1
                }
            }
        } else {
            temp = (*temp).n_down[ix1 as usize]
        }
    }
    /* Return the address of the indexed variable. */
    return &mut *(*temp)
        .n_num
        .as_mut_ptr()
        .offset(*sub.as_mut_ptr().offset(0 as libc::c_int as isize) as isize)
        as *mut bc_num;
}
/* Store the top of the execution stack into VAR_NAME.
This includes the special variables ibase, obase, and scale. */
#[no_mangle]
pub unsafe extern "C" fn store_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    let mut temp: libc::c_long = 0;
    let mut toobig: libc::c_char = 0;
    if var_name > 2 as libc::c_int {
        /* It is a simple variable. */
        var_ptr = get_var(var_name);
        if !var_ptr.is_null() {
            free_num(&mut (*var_ptr).v_value);
            (*var_ptr).v_value = copy_num((*ex_stack).s_num)
        }
    } else {
        /* It is a special variable... */
        toobig = FALSE as libc::c_char;
        if is_neg((*ex_stack).s_num) != 0 {
            match var_name {
                0 => {
                    rt_warn(
                        b"negative ibase, set to 2\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    temp = 2 as libc::c_int as libc::c_long
                }
                1 => {
                    rt_warn(
                        b"negative obase, set to 2\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    temp = 2 as libc::c_int as libc::c_long
                }
                2 => {
                    rt_warn(
                        b"negative scale, set to 0\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    temp = 0 as libc::c_int as libc::c_long
                }
                _ => {}
            }
        } else {
            temp = num2long((*ex_stack).s_num);
            if is_zero((*ex_stack).s_num) == 0 && temp == 0 as libc::c_int as libc::c_long {
                toobig = TRUE as libc::c_char
            }
        }
        match var_name {
            0 => {
                if temp < 2 as libc::c_int as libc::c_long && toobig == 0 {
                    i_base = 2 as libc::c_int;
                    rt_warn(
                        b"ibase too small, set to 2\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else if temp > 16 as libc::c_int as libc::c_long || toobig as libc::c_int != 0 {
                    i_base = 16 as libc::c_int;
                    rt_warn(
                        b"ibase too large, set to 16\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    i_base = temp as libc::c_int
                }
            }
            1 => {
                if temp < 2 as libc::c_int as libc::c_long && toobig == 0 {
                    o_base = 2 as libc::c_int;
                    rt_warn(
                        b"obase too small, set to 2\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else if temp > BC_BASE_MAX as libc::c_long || toobig as libc::c_int != 0 {
                    o_base = BC_BASE_MAX;
                    rt_warn(
                        b"obase too large, set to %d\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        BC_BASE_MAX,
                    );
                } else {
                    o_base = temp as libc::c_int
                }
            }
            2 => {
                /*  WARNING:  The following if statement may generate a compiler
                warning if INT_MAX == LONG_MAX.  This is NOT a problem. */
                if temp > BC_SCALE_MAX as libc::c_long || toobig as libc::c_int != 0 {
                    scale = BC_SCALE_MAX;
                    rt_warn(
                        b"scale too large, set to %d\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        BC_SCALE_MAX,
                    );
                } else {
                    scale = temp as libc::c_int
                }
            }
            _ => {}
        }
    };
}
/* Store the top of the execution stack into array VAR_NAME.
VAR_NAME is the name of an array, and the next to the top
of stack for the index into the array. */
#[no_mangle]
pub unsafe extern "C" fn store_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut index: libc::c_long = 0;
    if check_stack(2 as libc::c_int) == 0 {
        return;
    }
    index = num2long((*(*ex_stack).s_next).s_num);
    if index < 0 as libc::c_int as libc::c_long
        || index > BC_DIM_MAX as libc::c_long
        || index == 0 as libc::c_int as libc::c_long && is_zero((*(*ex_stack).s_next).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, index);
        if !num_ptr.is_null() {
            free_num(num_ptr);
            *num_ptr = copy_num((*ex_stack).s_num);
            free_num(&mut (*(*ex_stack).s_next).s_num);
            (*(*ex_stack).s_next).s_num = (*ex_stack).s_num;
            init_num(&mut (*ex_stack).s_num);
            pop();
        }
    };
}
/*  Load a copy of VAR_NAME on to the execution stack.  This includes
the special variables ibase, obase and scale.  */
#[no_mangle]
pub unsafe extern "C" fn load_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            /* Special variable ibase. */
            push_copy(_zero_);
            int2num(&mut (*ex_stack).s_num, i_base);
        }
        1 => {
            /* Special variable obase. */
            push_copy(_zero_);
            int2num(&mut (*ex_stack).s_num, o_base);
        }
        2 => {
            /* Special variable scale. */
            push_copy(_zero_);
            int2num(&mut (*ex_stack).s_num, scale);
        }
        _ => {
            /* It is a simple variable. */
            var_ptr = *variables.offset(var_name as isize);
            if !var_ptr.is_null() {
                push_copy((*var_ptr).v_value);
            } else {
                push_copy(_zero_);
            }
        }
    };
}
/*  Load a copy of VAR_NAME on to the execution stack.  This includes
the special variables ibase, obase and scale.  */
#[no_mangle]
pub unsafe extern "C" fn load_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut index: libc::c_long = 0;
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    index = num2long((*ex_stack).s_num);
    if index < 0 as libc::c_int as libc::c_long
        || index > BC_DIM_MAX as libc::c_long
        || index == 0 as libc::c_int as libc::c_long && is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, index);
        if !num_ptr.is_null() {
            pop();
            push_copy(*num_ptr);
        }
    };
}
/* Decrement VAR_NAME by one.  This includes the special variables
ibase, obase, and scale. */
#[no_mangle]
pub unsafe extern "C" fn decr_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            /* ibase */
            if i_base > 2 as libc::c_int {
                i_base -= 1
            } else {
                rt_warn(
                    b"ibase too small in --\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        1 => {
            /* obase */
            if o_base > 2 as libc::c_int {
                o_base -= 1
            } else {
                rt_warn(
                    b"obase too small in --\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        2 => {
            /* scale */
            if scale > 0 as libc::c_int {
                scale -= 1
            } else {
                rt_warn(
                    b"scale can not be negative in -- \x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        _ => {
            /* It is a simple variable. */
            var_ptr = get_var(var_name);
            if !var_ptr.is_null() {
                bc_sub((*var_ptr).v_value, _one_, &mut (*var_ptr).v_value);
            }
        }
    };
}
/* Decrement VAR_NAME by one.  VAR_NAME is an array, and the top of
the execution stack is the index and it is popped off the stack. */
#[no_mangle]
pub unsafe extern "C" fn decr_array(mut var_name: libc::c_char) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut index: libc::c_long = 0;
    /* It is an array variable. */
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    index = num2long((*ex_stack).s_num);
    if index < 0 as libc::c_int as libc::c_long
        || index > BC_DIM_MAX as libc::c_long
        || index == 0 as libc::c_int as libc::c_long && is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name as libc::c_int, index);
        if !num_ptr.is_null() {
            pop();
            bc_sub(*num_ptr, _one_, num_ptr);
        }
    };
}
/* Increment VAR_NAME by one.  This includes the special variables
ibase, obase, and scale. */
#[no_mangle]
pub unsafe extern "C" fn incr_var(mut var_name: libc::c_int) {
    let mut var_ptr: *mut bc_var = 0 as *mut bc_var;
    match var_name {
        0 => {
            /* ibase */
            if i_base < 16 as libc::c_int {
                i_base += 1
            } else {
                rt_warn(
                    b"ibase too big in ++\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        1 => {
            /* obase */
            if o_base < BC_BASE_MAX {
                o_base += 1
            } else {
                rt_warn(
                    b"obase too big in ++\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        2 => {
            if scale < BC_SCALE_MAX {
                scale += 1
            } else {
                rt_warn(
                    b"Scale too big in ++\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        _ => {
            /* It is a simple variable. */
            var_ptr = get_var(var_name);
            if !var_ptr.is_null() {
                bc_add((*var_ptr).v_value, _one_, &mut (*var_ptr).v_value);
            }
        }
    };
}
/* Increment VAR_NAME by one.  VAR_NAME is an array and top of
execution stack is the index and is popped off the stack. */
#[no_mangle]
pub unsafe extern "C" fn incr_array(mut var_name: libc::c_int) {
    let mut num_ptr: *mut bc_num = 0 as *mut bc_num;
    let mut index: libc::c_long = 0;
    if check_stack(1 as libc::c_int) == 0 {
        return;
    }
    index = num2long((*ex_stack).s_num);
    if index < 0 as libc::c_int as libc::c_long
        || index > BC_DIM_MAX as libc::c_long
        || index == 0 as libc::c_int as libc::c_long && is_zero((*ex_stack).s_num) == 0
    {
        rt_error(
            b"Array %s subscript out of bounds.\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *a_names.offset(var_name as isize),
        );
    } else {
        num_ptr = get_array_num(var_name, index);
        if !num_ptr.is_null() {
            pop();
            bc_add(*num_ptr, _one_, num_ptr);
        }
    };
}
/* Routines for processing autos variables and parameters. */
/* NAME is an auto variable that needs to be pushed on its stack. */
#[no_mangle]
pub unsafe extern "C" fn auto_var(mut name: libc::c_int) {
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_temp: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut ix: libc::c_int = 0;
    if name > 0 as libc::c_int {
        /* A simple variable. */
        ix = name;
        v_temp = malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong) as *mut bc_var;
        (*v_temp).v_next = *variables.offset(ix as isize);
        init_num(&mut (*v_temp).v_value);
        let ref mut fresh9 = *variables.offset(ix as isize);
        *fresh9 = v_temp
    } else {
        /* An array variable. */
        ix = -name;
        a_temp =
            malloc(::std::mem::size_of::<bc_var_array>() as libc::c_ulong) as *mut bc_var_array;
        (*a_temp).a_next = *arrays.offset(ix as isize);
        (*a_temp).a_value = NULL as *mut bc_array;
        (*a_temp).a_param = FALSE as libc::c_char;
        let ref mut fresh10 = *arrays.offset(ix as isize);
        *fresh10 = a_temp
    };
}
/* Free_a_tree frees everything associated with an array variable tree.
This is used when popping an array variable off its auto stack.  */
#[no_mangle]
pub unsafe extern "C" fn free_a_tree(mut root: *mut bc_array_node, mut depth: libc::c_int) {
    let mut ix: libc::c_int = 0;
    if !root.is_null() {
        if depth > 1 as libc::c_int {
            ix = 0 as libc::c_int;
            while ix < NODE_SIZE {
                free_a_tree((*root).n_down[ix as usize], depth - 1 as libc::c_int);
                ix += 1
            }
        } else {
            ix = 0 as libc::c_int;
            while ix < NODE_SIZE {
                free_num(&mut *(*root).n_num.as_mut_ptr().offset(ix as isize));
                ix += 1
            }
        }
        free(root as *mut libc::c_void);
    };
}
/* LIST is an NULL terminated list of varible names that need to be
popped off their auto stacks. */
#[no_mangle]
pub unsafe extern "C" fn pop_vars(mut list: *mut arg_list) {
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_temp: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut ix: libc::c_int = 0;
    while !list.is_null() {
        ix = (*list).av_name;
        if ix > 0 as libc::c_int {
            /* A simple variable. */
            v_temp = *variables.offset(ix as isize);
            if !v_temp.is_null() {
                let ref mut fresh11 = *variables.offset(ix as isize);
                *fresh11 = (*v_temp).v_next;
                free_num(&mut (*v_temp).v_value);
                free(v_temp as *mut libc::c_void);
            }
        } else {
            /* An array variable. */
            ix = -ix;
            a_temp = *arrays.offset(ix as isize);
            if !a_temp.is_null() {
                let ref mut fresh12 = *arrays.offset(ix as isize);
                *fresh12 = (*a_temp).a_next;
                if (*a_temp).a_param == 0 && !(*a_temp).a_value.is_null() {
                    free_a_tree(
                        (*(*a_temp).a_value).a_tree,
                        (*(*a_temp).a_value).a_depth as libc::c_int,
                    );
                    free((*a_temp).a_value as *mut libc::c_void);
                }
                free(a_temp as *mut libc::c_void);
            }
        }
        list = (*list).next
    }
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
/* From util.c */
/* From load.c */
/* From main.c */
/* From number.c */
/* From storage.c */
/* A call is being made to FUNC.  The call types are at PC.  Process
the parameters by doing an auto on the parameter variable and then
store the value at the new variable or put a pointer the the array
variable. */
#[no_mangle]
pub unsafe extern "C" fn process_params(mut pc: *mut program_counter, mut func: libc::c_int) {
    let mut ch: libc::c_char = 0;
    let mut params: *mut arg_list = 0 as *mut arg_list;
    let mut warned: libc::c_char = FALSE as libc::c_char;
    let mut ix: libc::c_int = 0;
    let mut ix1: libc::c_int = 0;
    let mut v_temp: *mut bc_var = 0 as *mut bc_var;
    let mut a_src: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut a_dest: *mut bc_var_array = 0 as *mut bc_var_array;
    let mut n_temp: *mut bc_num = 0 as *mut bc_num;
    /* Get the parameter names from the function. */
    params = (*functions.offset(func as isize)).f_params;
    loop {
        ch = byte(pc) as libc::c_char;
        if !(ch as libc::c_int != ':' as i32) {
            break;
        }
        if !params.is_null() {
            if ch as libc::c_int == '0' as i32 && (*params).av_name > 0 as libc::c_int {
                /* A simple variable. */
                ix = (*params).av_name;
                v_temp = malloc(::std::mem::size_of::<bc_var>() as libc::c_ulong) as *mut bc_var;
                (*v_temp).v_next = *variables.offset(ix as isize);
                (*v_temp).v_value = (*ex_stack).s_num;
                init_num(&mut (*ex_stack).s_num);
                let ref mut fresh13 = *variables.offset(ix as isize);
                *fresh13 = v_temp
            } else if ch as libc::c_int == '1' as i32 && (*params).av_name < 0 as libc::c_int {
                /* The variables is an array variable. */
                /* Compute source index and make sure some structure exists. */
                ix = num2long((*ex_stack).s_num) as libc::c_int;
                n_temp = get_array_num(ix, 0 as libc::c_int as libc::c_long);
                /* Push a new array and Compute Destination index */
                auto_var((*params).av_name);
                ix1 = -(*params).av_name;
                /* Set up the correct pointers in the structure. */
                if ix == ix1 {
                    a_src = (**arrays.offset(ix as isize)).a_next
                } else {
                    a_src = *arrays.offset(ix as isize)
                }
                a_dest = *arrays.offset(ix1 as isize);
                (*a_dest).a_param = TRUE as libc::c_char;
                (*a_dest).a_value = (*a_src).a_value
            } else {
                if (*params).av_name < 0 as libc::c_int {
                    rt_error(
                        b"Parameter type mismatch parameter %s.\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        *a_names.offset(-(*params).av_name as isize),
                    );
                } else {
                    rt_error(
                        b"Parameter type mismatch, parameter %s.\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        *v_names.offset((*params).av_name as isize),
                    );
                }
                params = params.offset(1)
            }
            pop();
        } else if warned == 0 {
            rt_error(
                b"Parameter number mismatch\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            warned = TRUE as libc::c_char
        }
        params = (*params).next
    }
    if !params.is_null() {
        rt_error(
            b"Parameter number mismatch\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    };
}
