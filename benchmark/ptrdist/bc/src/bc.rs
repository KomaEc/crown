use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    static mut if_label: libc::c_int;
    #[no_mangle]
    static mut continue_label: libc::c_int;
    /* Label numbers. */
    #[no_mangle]
    static mut next_label: libc::c_int;
    /* Used for "code" generation. */
    #[no_mangle]
    static mut genstr: [libc::c_char; 80];
    /* Interactive and other flags. */
    #[no_mangle]
    static mut interactive: libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn yylex() -> libc::c_int;
    #[no_mangle]
    fn nextarg(args: *mut arg_list, val: libc::c_char) -> *mut arg_list;
    #[no_mangle]
    fn arg_str(args: *mut arg_list, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn free_args(args: *mut arg_list);
    #[no_mangle]
    fn check_params(params: *mut arg_list, autos: *mut arg_list);
    #[no_mangle]
    fn init_gen();
    #[no_mangle]
    fn generate(str: *mut libc::c_char);
    #[no_mangle]
    fn run_code();
    #[no_mangle]
    fn lookup(name: *mut libc::c_char, namekind: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn welcome();
    #[no_mangle]
    fn warranty(_: *mut libc::c_char);
    #[no_mangle]
    fn limits();
    #[no_mangle]
    fn yyerror(str: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn warn(mesg: *mut libc::c_char, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct YYSTYPE {
    pub s_value: *mut libc::c_char,
    pub c_value: libc::c_char,
    pub i_value: libc::c_int,
    pub a_value: *mut arg_list,
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
pub const ARRAY: libc::c_int = 1 as libc::c_int;
pub const SIMPLE: libc::c_int = 0 as libc::c_int;
pub const FUNCT: libc::c_int = 2 as libc::c_int;
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const TRUE: libc::c_int = 1 as libc::c_int;
pub const BC_VERSION: [libc::c_char; 76] = unsafe {
    *::std::mem::transmute::<&[u8; 76], &[libc::c_char; 76]>(
        b"bc 1.02 (Mar 3, 92) Copyright (C) 1991, 1992 Free Software Foundation, Inc.\x00",
    )
};
pub const YYMAXDEPTH: libc::c_int = 150 as libc::c_int;
#[no_mangle]
pub static mut yylval: YYSTYPE = YYSTYPE {
    s_value: 0 as *const libc::c_char as *mut libc::c_char,
    c_value: 0,
    i_value: 0,
    a_value: 0 as *const arg_list as *mut arg_list,
};
#[no_mangle]
pub static mut yyval: YYSTYPE = YYSTYPE {
    s_value: 0 as *const libc::c_char as *mut libc::c_char,
    c_value: 0,
    i_value: 0,
    a_value: 0 as *const arg_list as *mut arg_list,
};
pub const YYERRCODE: libc::c_int = 256 as libc::c_int;
#[no_mangle]
pub static mut yyexca: [libc::c_short; 10] = [
    -(1 as libc::c_int) as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    257 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    0 as libc::c_int as libc::c_short,
];
pub const YYLAST: libc::c_int = 706 as libc::c_int;
#[no_mangle]
pub static mut yyact: [libc::c_short; 706] = [
    27 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    149 as libc::c_int as libc::c_short,
    145 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    135 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    141 as libc::c_int as libc::c_short,
    109 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    134 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    159 as libc::c_int as libc::c_short,
    88 as libc::c_int as libc::c_short,
    104 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    150 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    136 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    142 as libc::c_int as libc::c_short,
    126 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    151 as libc::c_int as libc::c_short,
    127 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    118 as libc::c_int as libc::c_short,
    115 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    139 as libc::c_int as libc::c_short,
    124 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    117 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    102 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    76 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    73 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    163 as libc::c_int as libc::c_short,
    140 as libc::c_int as libc::c_short,
    119 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    153 as libc::c_int as libc::c_short,
    96 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    152 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    100 as libc::c_int as libc::c_short,
    101 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    129 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    106 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    144 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    128 as libc::c_int as libc::c_short,
    164 as libc::c_int as libc::c_short,
    113 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    155 as libc::c_int as libc::c_short,
    112 as libc::c_int as libc::c_short,
    137 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    95 as libc::c_int as libc::c_short,
    116 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    148 as libc::c_int as libc::c_short,
    103 as libc::c_int as libc::c_short,
    80 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    114 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    130 as libc::c_int as libc::c_short,
    131 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    143 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    98 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    157 as libc::c_int as libc::c_short,
    160 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    138 as libc::c_int as libc::c_short,
    154 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    110 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    113 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    146 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    156 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    147 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    161 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    165 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    133 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    158 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    90 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    122 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    121 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    120 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    132 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut yypact: [libc::c_short; 166] = [
    -(1000 as libc::c_int) as libc::c_short,
    -(7 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(43 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    58 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(250 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    423 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    29 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    28 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(259 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(3 as libc::c_int) as libc::c_short,
    -(261 as libc::c_int) as libc::c_short,
    27 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    26 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    21 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    -(45 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    87 as libc::c_int as libc::c_short,
    152 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    409 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    435 as libc::c_int as libc::c_short,
    245 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(50 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(252 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(36 as libc::c_int) as libc::c_short,
    -(90 as libc::c_int) as libc::c_short,
    -(90 as libc::c_int) as libc::c_short,
    -(63 as libc::c_int) as libc::c_short,
    -(63 as libc::c_int) as libc::c_short,
    13 as libc::c_int as libc::c_short,
    423 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    419 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    2 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    423 as libc::c_int as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    10 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    423 as libc::c_int as libc::c_short,
    -(4 as libc::c_int) as libc::c_short,
    364 as libc::c_int as libc::c_short,
    408 as libc::c_int as libc::c_short,
    405 as libc::c_int as libc::c_short,
    375 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    8 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    -(51 as libc::c_int) as libc::c_short,
    409 as libc::c_int as libc::c_short,
    439 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    423 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    423 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    152 as libc::c_int as libc::c_short,
    -(36 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    212 as libc::c_int as libc::c_short,
    126 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(97 as libc::c_int) as libc::c_short,
    -(251 as libc::c_int) as libc::c_short,
    -(59 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    87 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    423 as libc::c_int as libc::c_short,
    -(5 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(237 as libc::c_int) as libc::c_short,
    -(52 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(274 as libc::c_int) as libc::c_short,
    87 as libc::c_int as libc::c_short,
    116 as libc::c_int as libc::c_short,
    -(279 as libc::c_int) as libc::c_short,
    -(61 as libc::c_int) as libc::c_short,
    -(17 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(252 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    87 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    -(31 as libc::c_int) as libc::c_short,
    162 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(44 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    6 as libc::c_int as libc::c_short,
    -(40 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
    87 as libc::c_int as libc::c_short,
    -(1000 as libc::c_int) as libc::c_short,
];
#[no_mangle]
pub static mut yypgo: [libc::c_short; 33] = [
    0 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    124 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    122 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    121 as libc::c_int as libc::c_short,
    119 as libc::c_int as libc::c_short,
    118 as libc::c_int as libc::c_short,
    117 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    115 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    114 as libc::c_int as libc::c_short,
    113 as libc::c_int as libc::c_short,
    111 as libc::c_int as libc::c_short,
    108 as libc::c_int as libc::c_short,
    107 as libc::c_int as libc::c_short,
    105 as libc::c_int as libc::c_short,
    104 as libc::c_int as libc::c_short,
    99 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut yyr1: [libc::c_short; 98] = [
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    6 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut yyr2: [libc::c_short; 98] = [
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    2 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
    1 as libc::c_int as libc::c_short,
];
#[no_mangle]
pub static mut yychk: [libc::c_short; 166] = [
    -(1000 as libc::c_int) as libc::c_short,
    -(1 as libc::c_int) as libc::c_short,
    -(11 as libc::c_int) as libc::c_short,
    -(12 as libc::c_int) as libc::c_short,
    -(15 as libc::c_int) as libc::c_short,
    256 as libc::c_int as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    268 as libc::c_int as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    283 as libc::c_int as libc::c_short,
    288 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    261 as libc::c_int as libc::c_short,
    269 as libc::c_int as libc::c_short,
    286 as libc::c_int as libc::c_short,
    270 as libc::c_int as libc::c_short,
    284 as libc::c_int as libc::c_short,
    272 as libc::c_int as libc::c_short,
    273 as libc::c_int as libc::c_short,
    274 as libc::c_int as libc::c_short,
    275 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    287 as libc::c_int as libc::c_short,
    -(4 as libc::c_int) as libc::c_short,
    260 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    263 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    262 as libc::c_int as libc::c_short,
    267 as libc::c_int as libc::c_short,
    271 as libc::c_int as libc::c_short,
    276 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    282 as libc::c_int as libc::c_short,
    279 as libc::c_int as libc::c_short,
    280 as libc::c_int as libc::c_short,
    285 as libc::c_int as libc::c_short,
    257 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    257 as libc::c_int as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    262 as libc::c_int as libc::c_short,
    258 as libc::c_int as libc::c_short,
    259 as libc::c_int as libc::c_short,
    266 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    264 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    -(17 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    -(23 as libc::c_int) as libc::c_short,
    -(13 as libc::c_int) as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    256 as libc::c_int as libc::c_short,
    -(25 as libc::c_int) as libc::c_short,
    265 as libc::c_int as libc::c_short,
    267 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    -(4 as libc::c_int) as libc::c_short,
    262 as libc::c_int as libc::c_short,
    278 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    -(31 as libc::c_int) as libc::c_short,
    -(32 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(3 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    40 as libc::c_int as libc::c_short,
    257 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    -(26 as libc::c_int) as libc::c_short,
    -(27 as libc::c_int) as libc::c_short,
    261 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(30 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    -(9 as libc::c_int) as libc::c_short,
    -(10 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    262 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    -(6 as libc::c_int) as libc::c_short,
    -(8 as libc::c_int) as libc::c_short,
    262 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    -(5 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    -(16 as libc::c_int) as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    44 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    -(21 as libc::c_int) as libc::c_short,
    -(24 as libc::c_int) as libc::c_short,
    -(26 as libc::c_int) as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    262 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    123 as libc::c_int as libc::c_short,
    262 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    -(18 as libc::c_int) as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    41 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    257 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    -(5 as libc::c_int) as libc::c_short,
    -(22 as libc::c_int) as libc::c_short,
    277 as libc::c_int as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    93 as libc::c_int as libc::c_short,
    -(7 as libc::c_int) as libc::c_short,
    281 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    -(28 as libc::c_int) as libc::c_short,
    -(29 as libc::c_int) as libc::c_short,
    -(8 as libc::c_int) as libc::c_short,
    -(19 as libc::c_int) as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
    -(13 as libc::c_int) as libc::c_short,
    257 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    -(5 as libc::c_int) as libc::c_short,
    257 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    125 as libc::c_int as libc::c_short,
    -(20 as libc::c_int) as libc::c_short,
    -(14 as libc::c_int) as libc::c_short,
];
#[no_mangle]
pub static mut yydef: [libc::c_short; 166] = [
    1 as libc::c_int as libc::c_short,
    -(2 as libc::c_int) as libc::c_short,
    2 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    4 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    7 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    16 as libc::c_int as libc::c_short,
    18 as libc::c_int as libc::c_short,
    19 as libc::c_int as libc::c_short,
    20 as libc::c_int as libc::c_short,
    21 as libc::c_int as libc::c_short,
    22 as libc::c_int as libc::c_short,
    23 as libc::c_int as libc::c_short,
    24 as libc::c_int as libc::c_short,
    25 as libc::c_int as libc::c_short,
    26 as libc::c_int as libc::c_short,
    28 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    35 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    39 as libc::c_int as libc::c_short,
    82 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    83 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    96 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    94 as libc::c_int as libc::c_short,
    95 as libc::c_int as libc::c_short,
    97 as libc::c_int as libc::c_short,
    3 as libc::c_int as libc::c_short,
    9 as libc::c_int as libc::c_short,
    5 as libc::c_int as libc::c_short,
    17 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    71 as libc::c_int as libc::c_short,
    73 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    67 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    11 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    69 as libc::c_int as libc::c_short,
    87 as libc::c_int as libc::c_short,
    75 as libc::c_int as libc::c_short,
    81 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    59 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    86 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    96 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    8 as libc::c_int as libc::c_short,
    50 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    76 as libc::c_int as libc::c_short,
    77 as libc::c_int as libc::c_short,
    78 as libc::c_int as libc::c_short,
    79 as libc::c_int as libc::c_short,
    80 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    68 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    14 as libc::c_int as libc::c_short,
    38 as libc::c_int as libc::c_short,
    40 as libc::c_int as libc::c_short,
    41 as libc::c_int as libc::c_short,
    43 as libc::c_int as libc::c_short,
    44 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    84 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    60 as libc::c_int as libc::c_short,
    61 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    91 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    51 as libc::c_int as libc::c_short,
    55 as libc::c_int as libc::c_short,
    72 as libc::c_int as libc::c_short,
    74 as libc::c_int as libc::c_short,
    27 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    66 as libc::c_int as libc::c_short,
    33 as libc::c_int as libc::c_short,
    36 as libc::c_int as libc::c_short,
    13 as libc::c_int as libc::c_short,
    15 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    70 as libc::c_int as libc::c_short,
    85 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    93 as libc::c_int as libc::c_short,
    88 as libc::c_int as libc::c_short,
    89 as libc::c_int as libc::c_short,
    90 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    29 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    42 as libc::c_int as libc::c_short,
    63 as libc::c_int as libc::c_short,
    92 as libc::c_int as libc::c_short,
    62 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    57 as libc::c_int as libc::c_short,
    56 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    45 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    52 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    34 as libc::c_int as libc::c_short,
    46 as libc::c_int as libc::c_short,
    37 as libc::c_int as libc::c_short,
    64 as libc::c_int as libc::c_short,
    48 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    58 as libc::c_int as libc::c_short,
    30 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    10 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    65 as libc::c_int as libc::c_short,
    47 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    53 as libc::c_int as libc::c_short,
    54 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    12 as libc::c_int as libc::c_short,
    31 as libc::c_int as libc::c_short,
    49 as libc::c_int as libc::c_short,
    0 as libc::c_int as libc::c_short,
    32 as libc::c_int as libc::c_short,
];
pub const YYFLAG: libc::c_int = -(1000 as libc::c_int);
/*	parser for yacc output	*/
#[no_mangle]
pub static mut yyv: [YYSTYPE; 150] = [YYSTYPE {
    s_value: 0 as *const libc::c_char as *mut libc::c_char,
    c_value: 0,
    i_value: 0,
    a_value: 0 as *const arg_list as *mut arg_list,
}; 150];
/* where the values are stored */
#[no_mangle]
pub static mut yychar: libc::c_int = -(1 as libc::c_int);
/* current input token number */
#[no_mangle]
pub static mut yynerrs: libc::c_int = 0 as libc::c_int;
/* number of errors */
#[no_mangle]
pub static mut yyerrflag: libc::c_short = 0 as libc::c_int as libc::c_short;
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
/* For the scanner and parser.... */
/* error recovery flag */
#[no_mangle]
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yys: [libc::c_short; 150] = [0; 150];
    let mut yyj: libc::c_short = 0;
    let mut yym: libc::c_short = 0;
    let mut yypvt: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystate: libc::c_short = 0;
    let mut yyps: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut yyn: libc::c_short = 0;
    let mut yypv: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyxi: *mut libc::c_short = 0 as *mut libc::c_short;
    yystate = 0 as libc::c_int as libc::c_short;
    yychar = -(1 as libc::c_int);
    yynerrs = 0 as libc::c_int;
    yyerrflag = 0 as libc::c_int as libc::c_short;
    yyps = &mut *yys.as_mut_ptr().offset(-(1 as libc::c_int) as isize) as *mut libc::c_short;
    yypv = &mut *yyv.as_mut_ptr().offset(-(1 as libc::c_int) as isize) as *mut YYSTYPE;
    'c_3442: loop {
        /* put a state and value onto the stack */
        yyps = yyps.offset(1); /* simple state */
        if yyps > &mut *yys.as_mut_ptr().offset(YYMAXDEPTH as isize) as *mut libc::c_short {
            yyerror(
                b"yacc stack overflow\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return 1 as libc::c_int;
        }
        *yyps = yystate;
        yypv = yypv.offset(1);
        *yypv = yyval;
        loop {
            yyn = yypact[yystate as usize];
            if !(yyn as libc::c_int <= YYFLAG) {
                if yychar < 0 as libc::c_int {
                    yychar = yylex();
                    if yychar < 0 as libc::c_int {
                        yychar = 0 as libc::c_int
                    }
                }
                yyn = (yyn as libc::c_int + yychar) as libc::c_short;
                if !((yyn as libc::c_int) < 0 as libc::c_int || yyn as libc::c_int >= YYLAST) {
                    yyn = yyact[yyn as usize];
                    if yychk[yyn as usize] as libc::c_int == yychar {
                        /* valid shift */
                        yychar = -(1 as libc::c_int);
                        yyval = yylval;
                        yystate = yyn;
                        if yyerrflag as libc::c_int > 0 as libc::c_int {
                            yyerrflag -= 1
                        }
                        continue 'c_3442;
                    }
                }
            }
            /* default state action */
            yyn = yydef[yystate as usize];
            if yyn as libc::c_int == -(2 as libc::c_int) {
                if yychar < 0 as libc::c_int {
                    yychar = yylex();
                    if yychar < 0 as libc::c_int {
                        yychar = 0 as libc::c_int
                    }
                }
                /* accept */
                yyxi = yyexca.as_mut_ptr();
                while *yyxi as libc::c_int != -(1 as libc::c_int)
                    || *yyxi.offset(1 as libc::c_int as isize) as libc::c_int
                        != yystate as libc::c_int
                {
                    yyxi = yyxi.offset(2 as libc::c_int as isize)
                }
                loop {
                    yyxi = yyxi.offset(2 as libc::c_int as isize);
                    if !(*yyxi as libc::c_int >= 0 as libc::c_int) {
                        break;
                    }
                    if *yyxi as libc::c_int == yychar {
                        break;
                    }
                }
                yyn = *yyxi.offset(1 as libc::c_int as isize);
                if (yyn as libc::c_int) < 0 as libc::c_int {
                    return 0 as libc::c_int;
                }
            }
            if !(yyn as libc::c_int == 0 as libc::c_int) {
                current_block = 6528285054092551010;
                break;
            }
            /* look through exception table */
            /* VOID */
            /* error */
            /* error ... attempt to resume parsing */
            match yyerrflag as libc::c_int {
                0 => {
                    /* brand new error */
                    yyerror(
                        b"syntax error\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    yynerrs += 1;
                    current_block = 6521545907417739840;
                    break;
                    /* try again in the same state */
                }
                1 | 2 => {
                    current_block = 6521545907417739840;
                    break;
                }
                3 => {
                    /* no shift yet; clobber input char */
                    if yychar == 0 as libc::c_int {
                        break 'c_3442;
                    }
                    yychar = -(1 as libc::c_int)
                }
                _ => {
                    current_block = 6528285054092551010;
                    break;
                }
            }
        }
        match current_block {
            6528285054092551010 => {
                /* reduction by production yyn */
                yyps = yyps.offset(-(yyr2[yyn as usize] as libc::c_int as isize));
                yypvt = yypv;
                yypv = yypv.offset(-(yyr2[yyn as usize] as libc::c_int as isize));
                yyval = *yypv.offset(1 as libc::c_int as isize);
                yym = yyn;
                /* consult goto table to find next state */
                yyn = yyr1[yyn as usize];
                yyj = (yypgo[yyn as usize] as libc::c_int + *yyps as libc::c_int + 1 as libc::c_int)
                    as libc::c_short;
                if yyj as libc::c_int >= YYLAST || {
                    yystate = yyact[yyj as usize];
                    (yychk[yystate as usize] as libc::c_int) != -(yyn as libc::c_int)
                } {
                    yystate = yyact[yypgo[yyn as usize] as usize]
                }
                match yym as libc::c_int {
                    1 => {
                        yyval.i_value = 0 as libc::c_int;
                        if interactive != 0 {
                            printf(
                                b"%s\n\x00" as *const u8 as *const libc::c_char,
                                BC_VERSION.as_ptr(),
                            );
                            welcome();
                        }
                    }
                    3 => {
                        run_code();
                    }
                    4 => {
                        run_code();
                    }
                    5 => {
                        yyerrflag = 0 as libc::c_int as libc::c_short;
                        init_gen();
                    }
                    6 => yyval.i_value = 0 as libc::c_int,
                    10 => yyval.i_value = 0 as libc::c_int,
                    17 => yyval.i_value = (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                    18 => {
                        warranty(b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    19 => {
                        limits();
                    }
                    20 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value & 2 as libc::c_int
                            != 0
                        {
                            warn(
                                b"comparison in expression\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value & 1 as libc::c_int
                            != 0
                        {
                            generate(
                                b"W\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        } else {
                            generate(
                                b"p\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                    }
                    21 => {
                        yyval.i_value = 0 as libc::c_int;
                        generate(b"w\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        generate((*yypvt.offset(-(0 as libc::c_int) as isize)).s_value);
                        free(
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value
                                as *mut libc::c_void,
                        );
                    }
                    22 => {
                        if break_label == 0 as libc::c_int {
                            yyerror(
                                b"Break outside a for/while\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"J%1d:\x00" as *const u8 as *const libc::c_char,
                                break_label,
                            );
                            generate(genstr.as_mut_ptr());
                        }
                    }
                    23 => {
                        warn(
                            b"Continue statement\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        if continue_label == 0 as libc::c_int {
                            yyerror(
                                b"Continue outside a for\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"J%1d:\x00" as *const u8 as *const libc::c_char,
                                continue_label,
                            );
                            generate(genstr.as_mut_ptr());
                        }
                    }
                    24 => {
                        exit(0 as libc::c_int);
                    }
                    25 => {
                        generate(b"h\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    26 => {
                        generate(
                            b"0R\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    27 => {
                        generate(b"R\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    28 => {
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = break_label;
                        let fresh0 = next_label;
                        next_label = next_label + 1;
                        break_label = fresh0
                    }
                    29 => {
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"Comparison in first for expression\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        let fresh1 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value = fresh1;
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"N%1d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"pN%1d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                            );
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    30 => {
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            generate(
                                b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        let fresh2 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value = fresh2;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"B%1d:J%1d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                            break_label,
                        );
                        generate(genstr.as_mut_ptr());
                        yyval.i_value = continue_label;
                        let fresh3 = next_label;
                        next_label = next_label + 1;
                        continue_label = fresh3;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"N%1d:\x00" as *const u8 as *const libc::c_char,
                            continue_label,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    31 => {
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"Comparison in third for expression\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"J%1d:N%1d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(7 as libc::c_int) as isize)).i_value,
                                (*yypvt.offset(-(4 as libc::c_int) as isize)).i_value,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"pJ%1d:N%1d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(7 as libc::c_int) as isize)).i_value,
                                (*yypvt.offset(-(4 as libc::c_int) as isize)).i_value,
                            );
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    32 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"J%1d:N%1d:\x00" as *const u8 as *const libc::c_char,
                            continue_label,
                            break_label,
                        );
                        generate(genstr.as_mut_ptr());
                        break_label = (*yypvt.offset(-(12 as libc::c_int) as isize)).i_value;
                        continue_label = (*yypvt.offset(-(4 as libc::c_int) as isize)).i_value
                    }
                    33 => {
                        (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value = if_label;
                        let fresh4 = next_label;
                        next_label = next_label + 1;
                        if_label = fresh4;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"Z%1d:\x00" as *const u8 as *const libc::c_char,
                            if_label,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    34 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"N%1d:\x00" as *const u8 as *const libc::c_char,
                            if_label,
                        );
                        generate(genstr.as_mut_ptr());
                        if_label = (*yypvt.offset(-(4 as libc::c_int) as isize)).i_value
                    }
                    35 => {
                        let fresh5 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = fresh5;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"N%1d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    36 => {
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = break_label;
                        let fresh6 = next_label;
                        next_label = next_label + 1;
                        break_label = fresh6;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"Z%1d:\x00" as *const u8 as *const libc::c_char,
                            break_label,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    37 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"J%1d:N%1d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(6 as libc::c_int) as isize)).i_value,
                            break_label,
                        );
                        generate(genstr.as_mut_ptr());
                        break_label = (*yypvt.offset(-(3 as libc::c_int) as isize)).i_value
                    }
                    38 => yyval.i_value = 0 as libc::c_int,
                    39 => {
                        warn(
                            b"print statement\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    43 => {
                        generate(b"O\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        generate((*yypvt.offset(-(0 as libc::c_int) as isize)).s_value);
                        free(
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value
                                as *mut libc::c_void,
                        );
                    }
                    44 => {
                        generate(b"P\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    46 => {
                        warn(
                            b"else clause in if statement\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        let fresh7 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = fresh7;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"J%d:N%1d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                            if_label,
                        );
                        generate(genstr.as_mut_ptr());
                        if_label = (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    48 => {
                        /* Check auto list against parameter list? */
                        check_params(
                            (*yypvt.offset(-(4 as libc::c_int) as isize)).a_value,
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).a_value,
                        );
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"F%d,%s.%s[\x00" as *const u8 as *const libc::c_char,
                            lookup((*yypvt.offset(-(6 as libc::c_int) as isize)).s_value, FUNCT),
                            arg_str((*yypvt.offset(-(4 as libc::c_int) as isize)).a_value, TRUE),
                            arg_str((*yypvt.offset(-(0 as libc::c_int) as isize)).a_value, TRUE),
                        );
                        generate(genstr.as_mut_ptr());
                        free_args((*yypvt.offset(-(4 as libc::c_int) as isize)).a_value);
                        free_args((*yypvt.offset(-(0 as libc::c_int) as isize)).a_value);
                        (*yypvt.offset(-(7 as libc::c_int) as isize)).i_value = next_label;
                        next_label = 0 as libc::c_int
                    }
                    49 => {
                        generate(
                            b"0R]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        next_label = (*yypvt.offset(-(11 as libc::c_int) as isize)).i_value
                    }
                    50 => yyval.a_value = NULL as *mut arg_list,
                    52 => yyval.a_value = NULL as *mut arg_list,
                    53 => yyval.a_value = (*yypvt.offset(-(1 as libc::c_int) as isize)).a_value,
                    54 => yyval.a_value = (*yypvt.offset(-(1 as libc::c_int) as isize)).a_value,
                    55 => {
                        yyval.a_value = nextarg(
                            NULL as *mut arg_list,
                            lookup(
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value,
                                SIMPLE,
                            ) as libc::c_char,
                        )
                    }
                    56 => {
                        yyval.a_value = nextarg(
                            NULL as *mut arg_list,
                            lookup((*yypvt.offset(-(2 as libc::c_int) as isize)).s_value, ARRAY)
                                as libc::c_char,
                        )
                    }
                    57 => {
                        yyval.a_value = nextarg(
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).a_value,
                            lookup(
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value,
                                SIMPLE,
                            ) as libc::c_char,
                        )
                    }
                    58 => {
                        yyval.a_value = nextarg(
                            (*yypvt.offset(-(4 as libc::c_int) as isize)).a_value,
                            lookup((*yypvt.offset(-(2 as libc::c_int) as isize)).s_value, ARRAY)
                                as libc::c_char,
                        )
                    }
                    59 => yyval.a_value = NULL as *mut arg_list,
                    61 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"comparison in argument\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        yyval.a_value =
                            nextarg(NULL as *mut arg_list, 0 as libc::c_int as libc::c_char)
                    }
                    62 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"K%d:\x00" as *const u8 as *const libc::c_char,
                            -lookup((*yypvt.offset(-(2 as libc::c_int) as isize)).s_value, ARRAY),
                        );
                        generate(genstr.as_mut_ptr());
                        yyval.a_value =
                            nextarg(NULL as *mut arg_list, 1 as libc::c_int as libc::c_char)
                    }
                    63 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"comparison in argument\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        yyval.a_value = nextarg(
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).a_value,
                            0 as libc::c_int as libc::c_char,
                        )
                    }
                    64 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"K%d:\x00" as *const u8 as *const libc::c_char,
                            -lookup((*yypvt.offset(-(2 as libc::c_int) as isize)).s_value, ARRAY),
                        );
                        generate(genstr.as_mut_ptr());
                        yyval.a_value = nextarg(
                            (*yypvt.offset(-(4 as libc::c_int) as isize)).a_value,
                            1 as libc::c_int as libc::c_char,
                        )
                    }
                    65 => {
                        yyval.i_value = -(1 as libc::c_int);
                        warn(
                            b"Missing expression in for statement\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    67 => {
                        yyval.i_value = 0 as libc::c_int;
                        generate(b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    68 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"comparison in return expresion\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                    }
                    69 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).c_value as libc::c_int
                            != '=' as i32
                        {
                            if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value
                                < 0 as libc::c_int
                            {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"DL%d:\x00" as *const u8 as *const libc::c_char,
                                    -(*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"l%d:\x00" as *const u8 as *const libc::c_char,
                                    (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            }
                            generate(genstr.as_mut_ptr());
                        }
                    }
                    70 => {
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"comparison in assignment\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        if (*yypvt.offset(-(2 as libc::c_int) as isize)).c_value as libc::c_int
                            != '=' as i32
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"%c\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(2 as libc::c_int) as isize)).c_value
                                    as libc::c_int,
                            );
                            generate(genstr.as_mut_ptr());
                        }
                        if (*yypvt.offset(-(3 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"S%d:\x00" as *const u8 as *const libc::c_char,
                                -(*yypvt.offset(-(3 as libc::c_int) as isize)).i_value,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"s%d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(3 as libc::c_int) as isize)).i_value,
                            );
                        }
                        generate(genstr.as_mut_ptr());
                        yyval.i_value = 0 as libc::c_int
                    }
                    71 => {
                        warn(
                            b"&& operator\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        let fresh8 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = fresh8;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"DZ%d:p\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    72 => {
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"DZ%d:p1N%d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value,
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value,
                        );
                        generate(genstr.as_mut_ptr());
                        yyval.i_value = (*yypvt.offset(-(3 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    73 => {
                        warn(
                            b"|| operator\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        let fresh9 = next_label;
                        next_label = next_label + 1;
                        (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value = fresh9;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"B%d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                        );
                        generate(genstr.as_mut_ptr());
                    }
                    74 => {
                        let mut tmplab: libc::c_int = 0;
                        let fresh10 = next_label;
                        next_label = next_label + 1;
                        tmplab = fresh10;
                        sprintf(
                            genstr.as_mut_ptr(),
                            b"B%d:0J%d:N%d:1N%d:\x00" as *const u8 as *const libc::c_char,
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value,
                            tmplab,
                            (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value,
                            tmplab,
                        );
                        generate(genstr.as_mut_ptr());
                        yyval.i_value = (*yypvt.offset(-(3 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    75 => {
                        yyval.i_value = (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value;
                        warn(
                            b"! operator\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        generate(b"!\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                    }
                    76 => {
                        yyval.i_value = 3 as libc::c_int;
                        match *(*yypvt.offset(-(1 as libc::c_int) as isize)).s_value as libc::c_int
                        {
                            61 => {
                                generate(
                                    b"=\x00" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            33 => {
                                generate(
                                    b"#\x00" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                            60 => {
                                if *(*yypvt.offset(-(1 as libc::c_int) as isize))
                                    .s_value
                                    .offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == '=' as i32
                                {
                                    generate(
                                        b"{\x00" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                } else {
                                    generate(
                                        b"<\x00" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                            }
                            62 => {
                                if *(*yypvt.offset(-(1 as libc::c_int) as isize))
                                    .s_value
                                    .offset(1 as libc::c_int as isize)
                                    as libc::c_int
                                    == '=' as i32
                                {
                                    generate(
                                        b"}\x00" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                } else {
                                    generate(
                                        b">\x00" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    77 => {
                        generate(b"+\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        yyval.i_value = (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    78 => {
                        generate(b"-\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        yyval.i_value = (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    79 => {
                        genstr[0 as libc::c_int as usize] =
                            (*yypvt.offset(-(1 as libc::c_int) as isize)).c_value;
                        genstr[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                        generate(genstr.as_mut_ptr());
                        yyval.i_value = (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    80 => {
                        generate(b"^\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        yyval.i_value = (*yypvt.offset(-(2 as libc::c_int) as isize)).i_value
                            | (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    81 => {
                        generate(b"n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
                        yyval.i_value = (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value
                    }
                    82 => {
                        yyval.i_value = 1 as libc::c_int;
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"L%d:\x00" as *const u8 as *const libc::c_char,
                                -(*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"l%d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                            );
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    83 => {
                        let mut length: libc::c_int =
                            strlen((*yypvt.offset(-(0 as libc::c_int) as isize)).s_value)
                                as libc::c_int;
                        yyval.i_value = 1 as libc::c_int;
                        if length == 1 as libc::c_int
                            && *(*yypvt.offset(-(0 as libc::c_int) as isize)).s_value as libc::c_int
                                == '0' as i32
                        {
                            generate(
                                b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        } else if length == 1 as libc::c_int
                            && *(*yypvt.offset(-(0 as libc::c_int) as isize)).s_value as libc::c_int
                                == '1' as i32
                        {
                            generate(
                                b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        } else {
                            generate(
                                b"K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            generate((*yypvt.offset(-(0 as libc::c_int) as isize)).s_value);
                            generate(
                                b":\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        free(
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value
                                as *mut libc::c_void,
                        );
                    }
                    84 => {
                        yyval.i_value =
                            (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value | 1 as libc::c_int
                    }
                    85 => {
                        yyval.i_value = 1 as libc::c_int;
                        if !(*yypvt.offset(-(1 as libc::c_int) as isize))
                            .a_value
                            .is_null()
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"C%d,%s:\x00" as *const u8 as *const libc::c_char,
                                lookup(
                                    (*yypvt.offset(-(3 as libc::c_int) as isize)).s_value,
                                    FUNCT,
                                ),
                                arg_str(
                                    (*yypvt.offset(-(1 as libc::c_int) as isize)).a_value,
                                    FALSE,
                                ),
                            );
                            free_args((*yypvt.offset(-(1 as libc::c_int) as isize)).a_value);
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"C%d:\x00" as *const u8 as *const libc::c_char,
                                lookup(
                                    (*yypvt.offset(-(3 as libc::c_int) as isize)).s_value,
                                    FUNCT,
                                ),
                            );
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    86 => {
                        yyval.i_value = 1 as libc::c_int;
                        if (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            if (*yypvt.offset(-(1 as libc::c_int) as isize)).c_value as libc::c_int
                                == '+' as i32
                            {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"DA%d:L%d:\x00" as *const u8 as *const libc::c_char,
                                    -(*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                    -(*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"DM%d:L%d:\x00" as *const u8 as *const libc::c_char,
                                    -(*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                    -(*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                );
                            }
                        } else if (*yypvt.offset(-(1 as libc::c_int) as isize)).c_value
                            as libc::c_int
                            == '+' as i32
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"i%d:l%d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                            );
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"d%d:l%d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                                (*yypvt.offset(-(0 as libc::c_int) as isize)).i_value,
                            );
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    87 => {
                        yyval.i_value = 1 as libc::c_int;
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value < 0 as libc::c_int
                        {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"DL%d:x\x00" as *const u8 as *const libc::c_char,
                                -(*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                            );
                            generate(genstr.as_mut_ptr());
                            if (*yypvt.offset(-(0 as libc::c_int) as isize)).c_value as libc::c_int
                                == '+' as i32
                            {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"A%d:\x00" as *const u8 as *const libc::c_char,
                                    -(*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"M%d:\x00" as *const u8 as *const libc::c_char,
                                    -(*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            }
                        } else {
                            sprintf(
                                genstr.as_mut_ptr(),
                                b"l%d:\x00" as *const u8 as *const libc::c_char,
                                (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                            );
                            generate(genstr.as_mut_ptr());
                            if (*yypvt.offset(-(0 as libc::c_int) as isize)).c_value as libc::c_int
                                == '+' as i32
                            {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"i%d:\x00" as *const u8 as *const libc::c_char,
                                    (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                sprintf(
                                    genstr.as_mut_ptr(),
                                    b"d%d:\x00" as *const u8 as *const libc::c_char,
                                    (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                            }
                        }
                        generate(genstr.as_mut_ptr());
                    }
                    88 => {
                        generate(
                            b"cL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        yyval.i_value = 1 as libc::c_int
                    }
                    89 => {
                        generate(
                            b"cR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        yyval.i_value = 1 as libc::c_int
                    }
                    90 => {
                        generate(
                            b"cS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        yyval.i_value = 1 as libc::c_int
                    }
                    91 => {
                        warn(
                            b"read function\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        generate(
                            b"cI\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        yyval.i_value = 1 as libc::c_int
                    }
                    92 => {
                        yyval.i_value = lookup(
                            (*yypvt.offset(-(0 as libc::c_int) as isize)).s_value,
                            SIMPLE,
                        )
                    }
                    93 => {
                        if (*yypvt.offset(-(1 as libc::c_int) as isize)).i_value > 1 as libc::c_int
                        {
                            warn(
                                b"comparison in subscript\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        yyval.i_value =
                            lookup((*yypvt.offset(-(3 as libc::c_int) as isize)).s_value, ARRAY)
                    }
                    94 => yyval.i_value = 0 as libc::c_int,
                    95 => yyval.i_value = 1 as libc::c_int,
                    96 => yyval.i_value = 2 as libc::c_int,
                    97 => yyval.i_value = 3 as libc::c_int,
                    _ => {}
                }
            }
            _ => {
                /* incompletely recovered error ... try again */
                yyerrflag = 3 as libc::c_int as libc::c_short;
                loop
                /* find a state where "error" is a legal shift action */
                {
                    if !(yyps >= yys.as_mut_ptr()) {
                        break 'c_3442; /* simulate a shift of "error" */
                    }
                    yyn = (yypact[*yyps as usize] as libc::c_int + YYERRCODE) as libc::c_short;
                    if yyn as libc::c_int >= 0 as libc::c_int
                        && (yyn as libc::c_int) < YYLAST
                        && yychk[yyact[yyn as usize] as usize] as libc::c_int == YYERRCODE
                    {
                        yystate = yyact[yyn as usize];
                        break;
                    } else {
                        yyn = yypact[*yyps as usize];
                        /* the current yyps has no shift onn "error", pop stack */
                        yyps = yyps.offset(-1);
                        yypv = yypv.offset(-1)
                    }
                }
            }
        }
    }
    /* there is no state on the stack with an error shift ... abort */
    return 1 as libc::c_int;
    /* stack new state and value */
}
