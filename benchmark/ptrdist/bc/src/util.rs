use ::libc;
extern "C" {
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn _IO_putc(__c: libc::c_int, __fp: *mut _IO_FILE) -> libc::c_int;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    #[no_mangle]
    fn vsprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ::std::ffi::VaList)
        -> libc::c_int;
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
    static mut break_label: libc::c_int;
    #[no_mangle]
    static mut continue_label: libc::c_int;
    /* Label numbers. */
    #[no_mangle]
    static mut next_label: libc::c_int;
    #[no_mangle]
    static mut out_count: libc::c_int;
    #[no_mangle]
    static mut did_gen: libc::c_char;
    #[no_mangle]
    static mut compile_only: libc::c_char;
    #[no_mangle]
    static mut warn_not_std: libc::c_char;
    #[no_mangle]
    static mut std_only: libc::c_char;
    #[no_mangle]
    static mut f_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut f_count: libc::c_int;
    #[no_mangle]
    static mut v_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut v_count: libc::c_int;
    #[no_mangle]
    static mut a_names: *mut *mut libc::c_char;
    #[no_mangle]
    static mut a_count: libc::c_int;
    #[no_mangle]
    static mut out_col: libc::c_int;
    #[no_mangle]
    static mut runtime_error: libc::c_char;
    #[no_mangle]
    static mut pc: program_counter;
    /* Input Line numbers and other error information. */
    #[no_mangle]
    static mut line_no: libc::c_int;
    #[no_mangle]
    static mut had_error: libc::c_int;
    /* For larger identifiers, a tree, and how many "storage" locations
    have been allocated. */
    #[no_mangle]
    static mut next_array: libc::c_int;
    #[no_mangle]
    static mut next_func: libc::c_int;
    #[no_mangle]
    static mut next_var: libc::c_int;
    #[no_mangle]
    static mut name_tree: *mut id_rec;
    /* For error message production */
    #[no_mangle]
    static mut g_argv: *mut *mut libc::c_char;
    #[no_mangle]
    static mut is_std_in: libc::c_char;
    /* For use with getopt.  Do not declare them here.*/
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn execute();
    #[no_mangle]
    fn init_load();
    #[no_mangle]
    fn load_code(code: *mut libc::c_char);
    #[no_mangle]
    fn more_variables();
    #[no_mangle]
    fn more_functions();
    #[no_mangle]
    fn more_arrays();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_int,
    pub pc_addr: libc::c_int,
}
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
/* Each function has its own code segments and labels.  There can be
no jumps between functions so labels are unique to a function. */
/* Is this function defined yet. */
/* Code addresses. */
/* Variables are "pushable" (auto) and thus we need a stack mechanism.
This is built into the variable record. */
/* bc arrays can also be "auto" variables and thus need the same
kind of stacking mechanisms. */
/* For the stacks, execution and function, we need records to allow
for arbitrary size. */
/* The following are for the name tree. */
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return _IO_putc(__c, stdout);
}
pub const NULL: libc::c_int = 0 as libc::c_int;
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
pub const MAX_STORE: libc::c_int = 32767 as libc::c_int;
pub const SIMPLE: libc::c_int = 0 as libc::c_int;
pub const FUNCT: libc::c_int = 2 as libc::c_int;
pub const ARRAY: libc::c_int = 1 as libc::c_int;
pub const BC_VERSION: [libc::c_char; 76] = unsafe {
    *::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
        b"bc 1.02 (Mar 3, 92) Copyright (C) 1991, 1992 Free Software Foundation, Inc.\x00",
    )
};
pub const LONG_MAX: libc::c_long = __LONG_MAX__;
pub const BC_SEG_SIZE: libc::c_int = 1024 as libc::c_int;
pub const BC_MAX_SEGS: libc::c_int = 16 as libc::c_int;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const BC_STRING_MAX: libc::c_int = INT_MAX;
pub const BC_BASE_MAX: libc::c_int = INT_MAX;
pub const BC_SCALE_MAX: libc::c_int = INT_MAX;
pub const BC_DIM_MAX: libc::c_int = 65535 as libc::c_int;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
/* util.c: Utility routines for bc. */
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
/* strcopyof mallocs new memory and copies a string to to the new
memory. */
#[no_mangle]
pub unsafe extern "C" fn strcopyof(mut str: *mut libc::c_char) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    temp = malloc(strlen(str).wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    strcpy(temp, str);
    return temp;
}
pub const __LONG_MAX__: libc::c_long = 9223372036854775807 as libc::c_long;
/* nextarg adds another value to the list of arguments. */
#[no_mangle]
pub unsafe extern "C" fn nextarg(mut args: *mut arg_list, mut val: libc::c_char) -> *mut arg_list {
    let mut temp: *mut arg_list = 0 as *mut arg_list;
    temp = malloc(::std::mem::size_of::<arg_list>() as libc::c_ulong) as *mut arg_list;
    (*temp).av_name = val as libc::c_int;
    (*temp).next = args;
    return temp;
}
/* For generate, we must produce a string in the form
 "val,val,...,val".  We also need a couple of static variables
for retaining old generated strings.  It also uses a recursive
function that builds the string. */
static mut arglist1: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut arglist2: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* make_arg_str does the actual construction of the argument string.
ARGS is the pointer to the list and LEN is the maximum number of
characters needed.  1 char is the minimum needed. COMMAS tells
if each number should be seperated by commas.*/
unsafe extern "C" fn make_arg_str(
    mut args: *mut arg_list,
    mut len: libc::c_int,
    mut commas: libc::c_int,
) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sval: [libc::c_char; 20] = [0; 20];
    /* Recursive call. */
    if !args.is_null() {
        temp = make_arg_str((*args).next, len + 11 as libc::c_int, commas)
    } else {
        temp = malloc(len as libc::c_ulong) as *mut libc::c_char;
        *temp = 0 as libc::c_int as libc::c_char;
        return temp;
    }
    /* Add the current number to the end of the string. */
    if len != 1 as libc::c_int && commas != 0 {
        sprintf(
            sval.as_mut_ptr(),
            b"%d,\x00" as *const u8 as *const libc::c_char,
            (*args).av_name,
        );
    } else {
        sprintf(
            sval.as_mut_ptr(),
            b"%d\x00" as *const u8 as *const libc::c_char,
            (*args).av_name,
        );
    }
    /* XXX temp = */
    strcat(temp, sval.as_mut_ptr());
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn arg_str(
    mut args: *mut arg_list,
    mut commas: libc::c_int,
) -> *mut libc::c_char {
    if !arglist2.is_null() {
        free(arglist2 as *mut libc::c_void);
    }
    arglist2 = arglist1;
    arglist1 = make_arg_str(args, 1 as libc::c_int, commas);
    return arglist1;
}
/* free_args frees an argument list ARGS. */
#[no_mangle]
pub unsafe extern "C" fn free_args(mut args: *mut arg_list) {
    let mut temp: *mut arg_list = 0 as *mut arg_list;
    temp = args;
    while !temp.is_null() {
        args = (*args).next;
        free(temp as *mut libc::c_void);
        temp = args
    }
}
/* Check for valid parameter (PARAMS) and auto (AUTOS) lists.
There must be no duplicates any where.  Also, this is where
warnings are generated for array parameters. */
#[no_mangle]
pub unsafe extern "C" fn check_params(mut params: *mut arg_list, mut autos: *mut arg_list) {
    let mut tmp1: *mut arg_list = 0 as *mut arg_list;
    let mut tmp2: *mut arg_list = 0 as *mut arg_list;
    /* Check for duplicate parameters. */
    if !params.is_null() {
        tmp1 = params;
        while !tmp1.is_null() {
            tmp2 = (*tmp1).next;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"duplicate parameter names\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next
            }
            if (*tmp1).av_name < 0 as libc::c_int {
                warn(
                    b"Array parameter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            tmp1 = (*tmp1).next
        }
    }
    /* Check for duplicate autos. */
    if !autos.is_null() {
        tmp1 = autos;
        while !tmp1.is_null() {
            tmp2 = (*tmp1).next;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"duplicate auto variable names\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next
            }
            tmp1 = (*tmp1).next
        }
    }
    /* Check for duplicate between parameters and autos. */
    if !params.is_null() && !autos.is_null() {
        tmp1 = params;
        while !tmp1.is_null() {
            tmp2 = autos;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"variable in both parameter and auto lists\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next
            }
            tmp1 = (*tmp1).next
        }
    };
}
/* Initialize the code generator the parser. */
#[no_mangle]
pub unsafe extern "C" fn init_gen() {
    /* Get things ready. */
    break_label = 0 as libc::c_int;
    continue_label = 0 as libc::c_int;
    next_label = 1 as libc::c_int;
    out_count = 2 as libc::c_int;
    if compile_only != 0 {
        printf(b"@i\x00" as *const u8 as *const libc::c_char);
    } else {
        init_load();
    }
    had_error = FALSE;
    did_gen = FALSE as libc::c_char;
}
/* generate code STR for the machine. */
#[no_mangle]
pub unsafe extern "C" fn generate(mut str: *mut libc::c_char) {
    did_gen = TRUE as libc::c_char;
    if compile_only != 0 {
        printf(b"%s\x00" as *const u8 as *const libc::c_char, str);
        out_count =
            (out_count as libc::c_ulong).wrapping_add(strlen(str)) as libc::c_int as libc::c_int;
        if out_count > 60 as libc::c_int {
            printf(b"\n\x00" as *const u8 as *const libc::c_char);
            out_count = 0 as libc::c_int
        }
    } else {
        load_code(str);
    };
}
/* Execute the current code as loaded. */
#[no_mangle]
pub unsafe extern "C" fn run_code() {
    /* If no compile errors run the current code. */
    if had_error == 0 && did_gen as libc::c_int != 0 {
        if compile_only != 0 {
            printf(b"@r\n\x00" as *const u8 as *const libc::c_char);
            out_count = 0 as libc::c_int
        } else {
            execute();
        }
    }
    /* Reinitialize the code generation and machine. */
    if did_gen != 0 {
        init_gen();
    } else {
        had_error = FALSE
    };
}
/* Output routines: Write a character CH to the standard output.
It keeps track of the number of characters output and may
break the output with a "\<cr>". */
#[no_mangle]
pub unsafe extern "C" fn out_char(mut ch: libc::c_char) {
    if ch as libc::c_int == '\n' as i32 {
        out_col = 0 as libc::c_int;
        putchar('\n' as i32);
    } else {
        out_col += 1;
        if out_col == 70 as libc::c_int {
            putchar('\\' as i32);
            putchar('\n' as i32);
            out_col = 1 as libc::c_int
        }
        putchar(ch as libc::c_int);
    };
}
/* The following are "Symbol Table" routines for the parser. */
/*  find_id returns a pointer to node in TREE that has the correct
ID.  If there is no node in TREE with ID, NULL is returned. */
#[no_mangle]
pub unsafe extern "C" fn find_id(mut tree: *mut id_rec, mut id: *mut libc::c_char) -> *mut id_rec {
    let mut cmp_result: libc::c_int = 0;
    /* Check for an empty tree. */
    if tree.is_null() {
        return NULL as *mut id_rec;
    }
    /* Recursively search the tree. */
    cmp_result = strcmp(id, (*tree).id); /* This is the item. */
    if cmp_result == 0 as libc::c_int {
        return tree;
    } else if cmp_result < 0 as libc::c_int {
        return find_id((*tree).left, id);
    } else {
        return find_id((*tree).right, id);
    };
}
/* insert_id_rec inserts a NEW_ID rec into the tree whose ROOT is
provided.  insert_id_rec returns TRUE if the tree height from
ROOT down is increased otherwise it returns FALSE.  This is a
recursive balanced binary tree insertion algorithm. */
#[no_mangle]
pub unsafe extern "C" fn insert_id_rec(
    mut root: *mut *mut id_rec,
    mut new_id: *mut id_rec,
) -> libc::c_int {
    let mut A: *mut id_rec = 0 as *mut id_rec;
    let mut B: *mut id_rec = 0 as *mut id_rec;
    /* If root is NULL, this where it is to be inserted. */
    if (*root).is_null() {
        *root = new_id;
        (*new_id).left = NULL as *mut id_rec;
        (*new_id).right = NULL as *mut id_rec;
        (*new_id).balance = 0 as libc::c_int as libc::c_short;
        return 1 as libc::c_int;
    }
    /* We need to search for a leaf. */
    if strcmp((*new_id).id, (**root).id) < 0 as libc::c_int {
        /* Insert it on the left. */
        if insert_id_rec(&mut (**root).left, new_id) != 0 {
            /* The height increased. */
            (**root).balance -= 1;
            match (**root).balance as libc::c_int {
                0 => {
                    /* no height increase. */
                    return 0 as libc::c_int;
                }
                -1 => {
                    /* height increase. */
                    return 0 as libc::c_int;
                }
                -2 => {
                    /* we need to do a rebalancing act. */
                    A = *root;
                    B = (**root).left;
                    if (*B).balance as libc::c_int <= 0 as libc::c_int {
                        /* Single Rotate. */
                        (*A).left = (*B).right;
                        (*B).right = A;
                        *root = B;
                        (*A).balance = 0 as libc::c_int as libc::c_short;
                        (*B).balance = 0 as libc::c_int as libc::c_short
                    } else {
                        /* Double Rotate. */
                        *root = (*B).right;
                        (*B).right = (**root).left;
                        (*A).left = (**root).right;
                        (**root).left = B;
                        (**root).right = A;
                        match (**root).balance as libc::c_int {
                            -1 => {
                                (*A).balance = 1 as libc::c_int as libc::c_short;
                                (*B).balance = 0 as libc::c_int as libc::c_short
                            }
                            0 => {
                                (*A).balance = 0 as libc::c_int as libc::c_short;
                                (*B).balance = 0 as libc::c_int as libc::c_short
                            }
                            1 => {
                                (*A).balance = 0 as libc::c_int as libc::c_short;
                                (*B).balance = -(1 as libc::c_int) as libc::c_short
                            }
                            _ => {}
                        }
                        (**root).balance = 0 as libc::c_int as libc::c_short
                    }
                }
                _ => {}
            }
        }
    } else if insert_id_rec(&mut (**root).right, new_id) != 0 {
        /* Insert it on the right. */
        /* The height increased. */
        (**root).balance += 1;
        match (**root).balance as libc::c_int {
            0 => {
                /* no height increase. */
                return 0 as libc::c_int;
            }
            1 => {
                /* height increase. */
                return 0 as libc::c_int;
            }
            2 => {
                /* we need to do a rebalancing act. */
                A = *root;
                B = (**root).right;
                if (*B).balance as libc::c_int >= 0 as libc::c_int {
                    /* Single Rotate. */
                    (*A).right = (*B).left;
                    (*B).left = A;
                    *root = B;
                    (*A).balance = 0 as libc::c_int as libc::c_short;
                    (*B).balance = 0 as libc::c_int as libc::c_short
                } else {
                    /* Double Rotate. */
                    *root = (*B).left;
                    (*B).left = (**root).right;
                    (*A).right = (**root).left;
                    (**root).left = A;
                    (**root).right = B;
                    match (**root).balance as libc::c_int {
                        -1 => {
                            (*A).balance = 0 as libc::c_int as libc::c_short;
                            (*B).balance = 1 as libc::c_int as libc::c_short
                        }
                        0 => {
                            (*A).balance = 0 as libc::c_int as libc::c_short;
                            (*B).balance = 0 as libc::c_int as libc::c_short
                        }
                        1 => {
                            (*A).balance = -(1 as libc::c_int) as libc::c_short;
                            (*B).balance = 0 as libc::c_int as libc::c_short
                        }
                        _ => {}
                    }
                    (**root).balance = 0 as libc::c_int as libc::c_short
                }
            }
            _ => {}
        }
    }
    /* If we fall through to here, the tree did not grow in height. */
    return 0 as libc::c_int;
}
/* Initialize variables for the symbol table tree. */
#[no_mangle]
pub unsafe extern "C" fn init_tree() {
    name_tree = NULL as *mut id_rec;
    next_array = 1 as libc::c_int;
    next_func = 1 as libc::c_int;
    next_var = 4 as libc::c_int;
    /* 0 => ibase, 1 => obase, 2 => scale, 3 => last. */
}
/* Lookup routines for symbol table names. */
#[no_mangle]
pub unsafe extern "C" fn lookup(
    mut name: *mut libc::c_char,
    mut namekind: libc::c_int,
) -> libc::c_int {
    let mut id: *mut id_rec = 0 as *mut id_rec;
    /* Warn about non-standard name. */
    if strlen(name) != 1 as libc::c_int as libc::c_ulong {
        warn(
            b"multiple letter name - %s\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    /* Look for the id. */
    id = find_id(name_tree, name);
    if id.is_null() {
        /* We need to make a new item. */
        id = malloc(::std::mem::size_of::<id_rec>() as libc::c_ulong) as *mut id_rec;
        (*id).id = strcopyof(name);
        (*id).a_name = 0 as libc::c_int;
        (*id).f_name = 0 as libc::c_int;
        (*id).v_name = 0 as libc::c_int;
        insert_id_rec(&mut name_tree, id);
    }
    /* Return the correct value. */
    match namekind {
        ARRAY => {
            /* ARRAY variable numbers are returned as negative numbers. */
            if (*id).a_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return -(*id).a_name;
            }
            let fresh0 = next_array;
            next_array = next_array + 1;
            (*id).a_name = fresh0;
            let ref mut fresh1 = *a_names.offset((*id).a_name as isize);
            *fresh1 = name;
            if (*id).a_name < MAX_STORE {
                if (*id).a_name >= a_count {
                    more_arrays();
                }
                return -(*id).a_name;
            }
            yyerror(
                b"Too many array variables\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        FUNCT => {
            if (*id).f_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return (*id).f_name;
            }
            let fresh2 = next_func;
            next_func = next_func + 1;
            (*id).f_name = fresh2;
            let ref mut fresh3 = *f_names.offset((*id).f_name as isize);
            *fresh3 = name;
            if (*id).f_name < MAX_STORE {
                if (*id).f_name >= f_count {
                    more_functions();
                }
                return (*id).f_name;
            }
            yyerror(
                b"Too many functions\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        SIMPLE => {
            if (*id).v_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return (*id).v_name;
            }
            let fresh4 = next_var;
            next_var = next_var + 1;
            (*id).v_name = fresh4;
            let ref mut fresh5 = *v_names.offset(((*id).v_name - 1 as libc::c_int) as isize);
            *fresh5 = name;
            if (*id).v_name <= MAX_STORE {
                if (*id).v_name >= v_count {
                    more_variables();
                }
                return (*id).v_name;
            }
            yyerror(
                b"Too many variables\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
/* Print the welcome banner. */
#[no_mangle]
pub unsafe extern "C" fn welcome() {
    printf(
        b"This is free software with ABSOLUTELY NO WARRANTY.\n\x00" as *const u8
            as *const libc::c_char,
    );
    printf(b"For details type . \n\x00" as *const u8 as *const libc::c_char);
}
/* Print out the warranty information. */
#[no_mangle]
pub unsafe extern "C" fn warranty(mut prefix: *mut libc::c_char) {
    printf(
        b"\n%s%s\n\n\x00" as *const u8 as *const libc::c_char,
        prefix,
        BC_VERSION.as_ptr(),
    );
    printf(
        b"%s%s%s%s%s%s%s%s%s%s%s\x00" as *const u8 as *const libc::c_char,
        b"    This program is free software; you can redistribute it and/or modify\n\x00"
            as *const u8 as *const libc::c_char,
        b"    it under the terms of the GNU General Public License as published by\n\x00"
            as *const u8 as *const libc::c_char,
        b"    the Free Software Foundation; either version 2 of the License , or\n\x00" as *const u8
            as *const libc::c_char,
        b"    (at your option) any later version.\n\n\x00" as *const u8 as *const libc::c_char,
        b"    This program is distributed in the hope that it will be useful,\n\x00" as *const u8
            as *const libc::c_char,
        b"    but WITHOUT ANY WARRANTY; without even the implied warranty of\n\x00" as *const u8
            as *const libc::c_char,
        b"    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n\x00" as *const u8
            as *const libc::c_char,
        b"    GNU General Public License for more details.\n\n\x00" as *const u8
            as *const libc::c_char,
        b"    You should have received a copy of the GNU General Public License\n\x00" as *const u8
            as *const libc::c_char,
        b"    along with this program. If not, write to the Free Software\n\x00" as *const u8
            as *const libc::c_char,
        b"    Foundation, 675 Mass Ave, Cambridge, MA 02139, USA.\n\n\x00" as *const u8
            as *const libc::c_char,
    );
}
/* Print out the limits of this program. */
#[no_mangle]
pub unsafe extern "C" fn limits() {
    printf(
        b"BC_BASE_MAX     = %d\n\x00" as *const u8 as *const libc::c_char,
        BC_BASE_MAX,
    );
    printf(
        b"BC_DIM_MAX      = %ld\n\x00" as *const u8 as *const libc::c_char,
        BC_DIM_MAX as libc::c_long,
    );
    printf(
        b"BC_SCALE_MAX    = %d\n\x00" as *const u8 as *const libc::c_char,
        BC_SCALE_MAX,
    );
    printf(
        b"BC_STRING_MAX   = %d\n\x00" as *const u8 as *const libc::c_char,
        BC_STRING_MAX,
    );
    printf(
        b"MAX Exponent    = %ld\n\x00" as *const u8 as *const libc::c_char,
        LONG_MAX,
    );
    printf(
        b"MAX code        = %ld\n\x00" as *const u8 as *const libc::c_char,
        BC_MAX_SEGS as libc::c_long * BC_SEG_SIZE as libc::c_long,
    );
    printf(
        b"multiply digits = %ld\n\x00" as *const u8 as *const libc::c_char,
        LONG_MAX / 90 as libc::c_int as libc::c_long,
    );
    printf(
        b"Number of vars  = %ld\n\x00" as *const u8 as *const libc::c_char,
        MAX_STORE as libc::c_long,
    );
}
/* The standard yyerror routine.  Built with variable number of argumnets. */
#[no_mangle]
pub unsafe extern "C" fn yyerror(mut str: *mut libc::c_char, mut args: ...) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if is_std_in != 0 {
        name = b"(standard_in)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        name = *g_argv.offset((optind - 1 as libc::c_int) as isize)
    }
    fprintf(
        stderr,
        b"%s %d: \x00" as *const u8 as *const libc::c_char,
        name,
        line_no,
    );
    vfprintf(stderr, str, args_0.as_va_list());
    fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    had_error = TRUE;
}
/* The routine to produce warnings about non-standard features
found during parsing. */
#[no_mangle]
pub unsafe extern "C" fn warn(mut mesg: *mut libc::c_char, mut args: ...) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if std_only != 0 {
        if is_std_in != 0 {
            name = b"(standard_in)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            name = *g_argv.offset((optind - 1 as libc::c_int) as isize)
        }
        fprintf(
            stderr,
            b"%s %d: \x00" as *const u8 as *const libc::c_char,
            name,
            line_no,
        );
        vfprintf(stderr, mesg, args_0.as_va_list());
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        had_error = TRUE
    } else if warn_not_std != 0 {
        if is_std_in != 0 {
            name = b"(standard_in)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            name = *g_argv.offset((optind - 1 as libc::c_int) as isize)
        }
        fprintf(
            stderr,
            b"%s %d: (Warning) \x00" as *const u8 as *const libc::c_char,
            name,
            line_no,
        );
        vfprintf(stderr, mesg, args_0.as_va_list());
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
    };
}
/* Runtime error will  print a message and stop the machine. */
#[no_mangle]
pub unsafe extern "C" fn rt_error(mut mesg: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut error_mesg: [libc::c_char; 255] = [0; 255];
    args_0 = args.clone();
    vsprintf(error_mesg.as_mut_ptr(), mesg, args_0.as_va_list());
    fprintf(
        stderr,
        b"Runtime error (func=%s, adr=%d): %s\n\x00" as *const u8 as *const libc::c_char,
        *f_names.offset(pc.pc_func as isize),
        pc.pc_addr,
        error_mesg.as_mut_ptr(),
    );
    runtime_error = TRUE as libc::c_char;
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
/* A runtime warning tells of some action taken by the processor that
may change the program execution but was not enough of a problem
to stop the execution. */
#[no_mangle]
pub unsafe extern "C" fn rt_warn(mut mesg: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut error_mesg: [libc::c_char; 255] = [0; 255];
    args_0 = args.clone();
    vsprintf(error_mesg.as_mut_ptr(), mesg, args_0.as_va_list());
    fprintf(
        stderr,
        b"Runtime warning (func=%s, adr=%d): %s\n\x00" as *const u8 as *const libc::c_char,
        *f_names.offset(pc.pc_func as isize),
        pc.pc_addr,
        error_mesg.as_mut_ptr(),
    );
}
