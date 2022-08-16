use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn rt_error(mesg: *mut libc::c_char, _: ...);
    #[no_mangle]
    fn rt_warn(mesg: *mut libc::c_char, _: ...);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stk_rec {
    pub digit: libc::c_long,
    pub next: *mut stk_rec,
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
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const LONG_MAX: libc::c_long = __LONG_MAX__;
pub const NULL: libc::c_int = 0 as libc::c_int;
/* number.c: Implements arbitrary precision numbers. */
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
/* Storage used for special numbers. */
#[no_mangle]
pub static mut _zero_: bc_num = 0 as *const bc_struct as *mut bc_struct;
#[no_mangle]
pub static mut _one_: bc_num = 0 as *const bc_struct as *mut bc_struct;
#[no_mangle]
pub static mut _two_: bc_num = 0 as *const bc_struct as *mut bc_struct;
pub const __LONG_MAX__: libc::c_long = 9223372036854775807 as libc::c_long;
/* "Frees" a bc_num NUM.  Actually decreases reference count and only
frees the storage if reference count is zero. */
#[no_mangle]
pub unsafe extern "C" fn free_num(mut num: *mut bc_num) {
    if (*num).is_null() {
        return;
    }
    (**num).n_refs -= 1;
    if (**num).n_refs == 0 as libc::c_int {
        free(*num as *mut libc::c_void);
    }
    *num = NULL as bc_num;
}
/* new_num allocates a number and sets fields to known values. */
#[no_mangle]
pub unsafe extern "C" fn new_num(mut length: libc::c_int, mut scale: libc::c_int) -> bc_num {
    let mut temp: bc_num = 0 as *mut bc_struct;
    temp = malloc(
        (::std::mem::size_of::<bc_struct>() as libc::c_ulong)
            .wrapping_add(length as libc::c_ulong)
            .wrapping_add(scale as libc::c_ulong),
    ) as *mut bc_struct;
    /* XXX if (temp == NULL) out_of_memory (); */
    (*temp).n_sign = PLUS;
    (*temp).n_len = length;
    (*temp).n_scale = scale;
    (*temp).n_refs = 1 as libc::c_int;
    (*temp).n_value[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    return temp;
}
/* Intitialize the number package! */
#[no_mangle]
pub unsafe extern "C" fn init_numbers() {
    _zero_ = new_num(1 as libc::c_int, 0 as libc::c_int);
    _one_ = new_num(1 as libc::c_int, 0 as libc::c_int);
    (*_one_).n_value[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_char;
    _two_ = new_num(1 as libc::c_int, 0 as libc::c_int);
    (*_two_).n_value[0 as libc::c_int as usize] = 2 as libc::c_int as libc::c_char;
}
/* Make a copy of a number!  Just increments the reference count! */
#[no_mangle]
pub unsafe extern "C" fn copy_num(mut num: bc_num) -> bc_num {
    (*num).n_refs += 1;
    return num;
}
/* Initialize a number NUM by making it a copy of zero. */
#[no_mangle]
pub unsafe extern "C" fn init_num(mut num: *mut bc_num) {
    *num = copy_num(_zero_);
}
/* Convert an integer  to a bc number NUM. */
#[no_mangle]
pub unsafe extern "C" fn int2num(mut num: *mut bc_num, mut val: libc::c_int) {
    let mut buffer: [libc::c_char; 30] = [0; 30];
    let mut bptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ix: libc::c_int = 1 as libc::c_int;
    let mut neg: libc::c_char = 0 as libc::c_int as libc::c_char;
    /* Sign. */
    if val < 0 as libc::c_int {
        neg = 1 as libc::c_int as libc::c_char;
        val = -val
    }
    /* Get things going. */
    bptr = &mut *buffer.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;
    let fresh0 = bptr;
    bptr = bptr.offset(1);
    *fresh0 = (val % 10 as libc::c_int) as libc::c_char;
    val = val / 10 as libc::c_int;
    /* Extract remaining digits. */
    while val != 0 as libc::c_int {
        let fresh1 = bptr;
        bptr = bptr.offset(1);
        *fresh1 = (val % 10 as libc::c_int) as libc::c_char;
        val = val / 10 as libc::c_int;
        ix += 1
        /* Count the digits. */
    }
    /* Make the number. */
    free_num(num);
    *num = new_num(ix, 0 as libc::c_int);
    if neg != 0 {
        (**num).n_sign = MINUS
    }
    /* Assign the digits. */
    vptr = &mut *(**num)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    loop {
        let fresh2 = ix;
        ix = ix - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        bptr = bptr.offset(-1);
        let fresh3 = vptr;
        vptr = vptr.offset(1);
        *fresh3 = *bptr
    }
}
/* Convert a number NUM to a long.  The function returns only the integer
part of the number.  For numbers that are too large to represent as
a long, this function returns a zero.  This can be detected by checking
the NUM for zero after having a zero returned. */
#[no_mangle]
pub unsafe extern "C" fn num2long(mut num: bc_num) -> libc::c_long {
    let mut val: libc::c_long = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    /* Extract the int value, ignore the fraction. */
    val = 0 as libc::c_int as libc::c_long;
    nptr = &mut *(*num)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    index = (*num).n_len;
    while index > 0 as libc::c_int && val <= LONG_MAX / 10 as libc::c_int as libc::c_long {
        let fresh4 = nptr;
        nptr = nptr.offset(1);
        val = val * 10 as libc::c_int as libc::c_long + *fresh4 as libc::c_long;
        index -= 1
    }
    /* Check for overflow.  If overflow, return zero. */
    if index > 0 as libc::c_int {
        val = 0 as libc::c_int as libc::c_long
    }
    if val < 0 as libc::c_int as libc::c_long {
        val = 0 as libc::c_int as libc::c_long
    }
    /* Return the value. */
    if (*num).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
        return val;
    } else {
        return -val;
    };
}
/* The following are some math routines for numbers. */
/* Compare two bc numbers.  Return value is 0 if equal, -1 if N1 is less
than N2 and +1 if N1 is greater than N2.  If USE_SIGN is false, just
compare the magnitudes. */
unsafe extern "C" fn _do_compare(
    mut n1: bc_num,
    mut n2: bc_num,
    mut use_sign: libc::c_int,
    mut ignore_last: libc::c_int,
) -> libc::c_int {
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    /* First, compare signs. */
    if use_sign != 0 && (*n1).n_sign as libc::c_uint != (*n2).n_sign as libc::c_uint {
        if (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
            return 1 as libc::c_int;
        } else {
            return -(1 as libc::c_int);
        } /* Positive N1 > Negative N2 */
        /* Negative N1 < Positive N1 */
    }
    /* Now compare the magnitude. */
    if (*n1).n_len != (*n2).n_len {
        if (*n1).n_len > (*n2).n_len {
            /* Magnitude of n1 > n2. */
            if use_sign == 0 || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            } else {
                return -(1 as libc::c_int);
            }
        } else if use_sign == 0
            || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        } else {
            return 1 as libc::c_int;
        }
    }
    /* Magnitude of n1 < n2. */
    /* If we get here, they have the same number of integer digits.
    check the integer part and the equal length part of the fraction. */
    count = (*n1).n_len
        + (if (*n1).n_scale > (*n2).n_scale {
            (*n2).n_scale
        } else {
            (*n1).n_scale
        });
    n1ptr = &mut *(*n1).n_value.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;
    n2ptr = &mut *(*n2).n_value.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;
    while count > 0 as libc::c_int && *n1ptr as libc::c_int == *n2ptr as libc::c_int {
        n1ptr = n1ptr.offset(1);
        n2ptr = n2ptr.offset(1);
        count -= 1
    }
    if ignore_last != 0 && count == 1 as libc::c_int && (*n1).n_scale == (*n2).n_scale {
        return 0 as libc::c_int;
    }
    if count != 0 as libc::c_int {
        if *n1ptr as libc::c_int > *n2ptr as libc::c_int {
            /* Magnitude of n1 > n2. */
            if use_sign == 0 || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            } else {
                return -(1 as libc::c_int);
            }
        } else if use_sign == 0
            || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
        {
            return -(1 as libc::c_int);
        } else {
            return 1 as libc::c_int;
        }
    }
    /* Magnitude of n1 < n2. */
    /* They are equal up to the last part of the equal part of the fraction. */
    if (*n1).n_scale != (*n2).n_scale {
        if (*n1).n_scale > (*n2).n_scale {
            count = (*n1).n_scale - (*n2).n_scale;
            while count > 0 as libc::c_int {
                let fresh5 = n1ptr;
                n1ptr = n1ptr.offset(1);
                if *fresh5 as libc::c_int != 0 as libc::c_int {
                    /* Magnitude of n1 > n2. */
                    if use_sign == 0
                        || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
                    {
                        return 1 as libc::c_int;
                    } else {
                        return -(1 as libc::c_int);
                    }
                }
                count -= 1
            }
        } else {
            count = (*n2).n_scale - (*n1).n_scale;
            while count > 0 as libc::c_int {
                let fresh6 = n2ptr;
                n2ptr = n2ptr.offset(1);
                if *fresh6 as libc::c_int != 0 as libc::c_int {
                    /* Magnitude of n1 < n2. */
                    if use_sign == 0
                        || (*n1).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint
                    {
                        return -(1 as libc::c_int);
                    } else {
                        return 1 as libc::c_int;
                    }
                }
                count -= 1
            }
        }
    }
    /* They must be equal! */
    return 0 as libc::c_int;
}
/* This is the "user callable" routine to compare numbers N1 and N2. */
#[no_mangle]
pub unsafe extern "C" fn bc_compare(mut n1: bc_num, mut n2: bc_num) -> libc::c_int {
    return _do_compare(n1, n2, TRUE, FALSE);
}
/* In some places we need to check if the number NUM is zero. */
#[no_mangle]
pub unsafe extern "C" fn is_zero(mut num: bc_num) -> libc::c_char {
    let mut count: libc::c_int = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Quick check. */
    if num == _zero_ {
        return TRUE as libc::c_char;
    }
    /* Initialize */
    count = (*num).n_len + (*num).n_scale;
    nptr = &mut *(*num)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    /* The check */
    while count > 0 as libc::c_int && {
        let fresh7 = nptr;
        nptr = nptr.offset(1);
        (*fresh7 as libc::c_int) == 0 as libc::c_int
    } {
        count -= 1
    }
    if count != 0 as libc::c_int {
        return FALSE as libc::c_char;
    } else {
        return TRUE as libc::c_char;
    };
}
/* In some places we need to check if the number is negative. */
#[no_mangle]
pub unsafe extern "C" fn is_neg(mut num: bc_num) -> libc::c_char {
    return ((*num).n_sign as libc::c_uint == MINUS as libc::c_int as libc::c_uint) as libc::c_int
        as libc::c_char;
}
/* For many things, we may have leading zeros in a number NUM.
_rm_leading_zeros just moves the data to the correct
place and adjusts the length. */
unsafe extern "C" fn _rm_leading_zeros(mut num: bc_num) {
    let mut bytes: libc::c_int = 0;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    /* Do a quick check to see if we need to do it. */
    /* if ((*num)->n_value != (void *)0) return; */
    /* The first digit is 0, find the first non-zero digit in the 10's or
    greater place. */
    bytes = (*num).n_len;
    src = &mut *(*num)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    while bytes > 1 as libc::c_int && *src as libc::c_int == 0 as libc::c_int {
        src = src.offset(1);
        bytes -= 1
    }
    (*num).n_len = bytes;
    bytes += (*num).n_scale;
    dst = &mut *(*num)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char;
    loop {
        let fresh8 = bytes;
        bytes = bytes - 1;
        if !(fresh8 > 0 as libc::c_int) {
            break;
        }
        let fresh9 = src;
        src = src.offset(1);
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = *fresh9
    }
}
/* Perform addition: N1 is added to N2 and the value is
returned.  The signs of N1 and N2 are ignored. */
unsafe extern "C" fn _do_add(mut n1: bc_num, mut n2: bc_num) -> bc_num {
    let mut sum: bc_num = 0 as *mut bc_struct;
    let mut sum_scale: libc::c_int = 0;
    let mut sum_digits: libc::c_int = 0;
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sumptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut carry: libc::c_int = 0;
    let mut n1bytes: libc::c_int = 0;
    let mut n2bytes: libc::c_int = 0;
    /* Prepare sum. */
    sum_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n1).n_scale
    } else {
        (*n2).n_scale
    };
    sum_digits = (if (*n1).n_len > (*n2).n_len {
        (*n1).n_len
    } else {
        (*n2).n_len
    }) + 1 as libc::c_int;
    sum = new_num(sum_digits, sum_scale);
    /* Start with the fraction part.  Initialize the pointers. */
    n1bytes = (*n1).n_scale;
    n2bytes = (*n2).n_scale;
    n1ptr = (&mut *(*n1).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset((*n1).n_len as isize)
        .offset(n1bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    n2ptr = (&mut *(*n2).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset((*n2).n_len as isize)
        .offset(n2bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    sumptr = (&mut *(*sum)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char)
        .offset(sum_scale as isize)
        .offset(sum_digits as isize)
        .offset(-(1 as libc::c_int as isize));
    /* Add the fraction part.  First copy the longer fraction.*/
    if n1bytes != n2bytes {
        if n1bytes > n2bytes {
            while n1bytes > n2bytes {
                let fresh11 = n1ptr;
                n1ptr = n1ptr.offset(-1);
                let fresh12 = sumptr;
                sumptr = sumptr.offset(-1);
                *fresh12 = *fresh11;
                n1bytes -= 1
            }
        } else {
            while n2bytes > n1bytes {
                let fresh13 = n2ptr;
                n2ptr = n2ptr.offset(-1);
                let fresh14 = sumptr;
                sumptr = sumptr.offset(-1);
                *fresh14 = *fresh13;
                n2bytes -= 1
            }
        }
    }
    /* Now add the remaining fraction part and equal size integer parts. */
    n1bytes += (*n1).n_len;
    n2bytes += (*n2).n_len;
    carry = 0 as libc::c_int;
    while n1bytes > 0 as libc::c_int && n2bytes > 0 as libc::c_int {
        let fresh15 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        let fresh16 = n2ptr;
        n2ptr = n2ptr.offset(-1);
        *sumptr = (*fresh15 as libc::c_int + *fresh16 as libc::c_int + carry) as libc::c_char;
        if *sumptr as libc::c_int > 9 as libc::c_int {
            carry = 1 as libc::c_int;
            *sumptr = (*sumptr as libc::c_int - 10 as libc::c_int) as libc::c_char
        } else {
            carry = 0 as libc::c_int
        }
        sumptr = sumptr.offset(-1);
        n1bytes -= 1;
        n2bytes -= 1
    }
    /* Now add carry the longer integer part. */
    if n1bytes == 0 as libc::c_int {
        n1bytes = n2bytes;
        n1ptr = n2ptr
    }
    loop {
        let fresh17 = n1bytes;
        n1bytes = n1bytes - 1;
        if !(fresh17 > 0 as libc::c_int) {
            break;
        }
        let fresh18 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        *sumptr = (*fresh18 as libc::c_int + carry) as libc::c_char;
        if *sumptr as libc::c_int > 9 as libc::c_int {
            carry = 1 as libc::c_int;
            *sumptr = (*sumptr as libc::c_int - 10 as libc::c_int) as libc::c_char
        } else {
            carry = 0 as libc::c_int
        }
        sumptr = sumptr.offset(-1)
    }
    /* Set final carry. */
    if carry == 1 as libc::c_int {
        *sumptr = (*sumptr as libc::c_int + 1 as libc::c_int) as libc::c_char
    }
    /* Adjust sum and return. */
    _rm_leading_zeros(sum);
    return sum;
}
/* Perform subtraction: N2 is subtracted from N1 and the value is
returned.  The signs of N1 and N2 are ignored.  Also, N1 is
assumed to be larger than N2.  */
unsafe extern "C" fn _do_sub(mut n1: bc_num, mut n2: bc_num) -> bc_num {
    let mut diff: bc_num = 0 as *mut bc_struct;
    let mut diff_scale: libc::c_int = 0;
    let mut diff_len: libc::c_int = 0;
    let mut min_scale: libc::c_int = 0;
    let mut min_len: libc::c_int = 0;
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut diffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut borrow: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    /* Allocate temporary storage. */
    diff_len = if (*n1).n_len > (*n2).n_len {
        (*n1).n_len
    } else {
        (*n2).n_len
    };
    diff_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n1).n_scale
    } else {
        (*n2).n_scale
    };
    min_len = if (*n1).n_len > (*n2).n_len {
        (*n2).n_len
    } else {
        (*n1).n_len
    };
    min_scale = if (*n1).n_scale > (*n2).n_scale {
        (*n2).n_scale
    } else {
        (*n1).n_scale
    };
    diff = new_num(diff_len, diff_scale);
    /* Initialize the subtract. */
    n1ptr = (&mut *(*n1).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset((*n1).n_len as isize)
        .offset((*n1).n_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    n2ptr = (&mut *(*n2).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset((*n2).n_len as isize)
        .offset((*n2).n_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    diffptr = (&mut *(*diff)
        .n_value
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut libc::c_char)
        .offset(diff_len as isize)
        .offset(diff_scale as isize)
        .offset(-(1 as libc::c_int as isize));
    /* Subtract the numbers. */
    borrow = 0 as libc::c_int;
    /* Take care of the longer scaled number. */
    if (*n1).n_scale != min_scale {
        /* n1 has the longer scale */
        count = (*n1).n_scale - min_scale;
        while count > 0 as libc::c_int {
            let fresh19 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            let fresh20 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh20 = *fresh19;
            count -= 1
        }
    } else {
        /* n2 has the longer scale */
        count = (*n2).n_scale - min_scale;
        while count > 0 as libc::c_int {
            let fresh21 = n2ptr;
            n2ptr = n2ptr.offset(-1);
            val = -(*fresh21 as libc::c_int) - borrow;
            if val < 0 as libc::c_int {
                val += 10 as libc::c_int;
                borrow = 1 as libc::c_int
            } else {
                borrow = 0 as libc::c_int
            }
            let fresh22 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh22 = val as libc::c_char;
            count -= 1
        }
    }
    /* Now do the equal length scale and integer parts. */
    count = 0 as libc::c_int;
    while count < min_len + min_scale {
        let fresh23 = n1ptr;
        n1ptr = n1ptr.offset(-1);
        let fresh24 = n2ptr;
        n2ptr = n2ptr.offset(-1);
        val = *fresh23 as libc::c_int - *fresh24 as libc::c_int - borrow;
        if val < 0 as libc::c_int {
            val += 10 as libc::c_int;
            borrow = 1 as libc::c_int
        } else {
            borrow = 0 as libc::c_int
        }
        let fresh25 = diffptr;
        diffptr = diffptr.offset(-1);
        *fresh25 = val as libc::c_char;
        count += 1
    }
    /* If n1 has more digits then n2, we now do that subtract. */
    if diff_len != min_len {
        count = diff_len - min_len;
        while count > 0 as libc::c_int {
            let fresh26 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            val = *fresh26 as libc::c_int - borrow;
            if val < 0 as libc::c_int {
                val += 10 as libc::c_int;
                borrow = 1 as libc::c_int
            } else {
                borrow = 0 as libc::c_int
            }
            let fresh27 = diffptr;
            diffptr = diffptr.offset(-1);
            *fresh27 = val as libc::c_char;
            count -= 1
        }
    }
    /* Clean up and return. */
    _rm_leading_zeros(diff);
    return diff;
}
/* Here is the full add routine that takes care of negative numbers.
N1 is added to N2 and the result placed into RESULT. */
#[no_mangle]
pub unsafe extern "C" fn bc_add(mut n1: bc_num, mut n2: bc_num, mut result: *mut bc_num) {
    let mut sum: bc_num = 0 as *mut bc_struct;
    let mut cmp_res: libc::c_int = 0;
    if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        sum = _do_add(n1, n2);
        (*sum).n_sign = (*n1).n_sign
    } else {
        /* subtraction must be done. */
        cmp_res = _do_compare(n1, n2, FALSE, FALSE); /* Compare magnitudes. */
        match cmp_res {
            -1 => {
                /* n1 is less than n2, subtract n1 from n2. */
                sum = _do_sub(n2, n1);
                (*sum).n_sign = (*n2).n_sign
            }
            0 => {
                /* They are equal! return zero! */
                sum = copy_num(_zero_)
            }
            1 => {
                /* n2 is less than n1, subtract n2 from n1. */
                sum = _do_sub(n1, n2);
                (*sum).n_sign = (*n1).n_sign
            }
            _ => {}
        }
    }
    /* Clean up and return. */
    free_num(result);
    *result = sum;
}
/* Here is the full subtract routine that takes care of negative numbers.
N2 is subtracted from N1 and the result placed in RESULT. */
#[no_mangle]
pub unsafe extern "C" fn bc_sub(mut n1: bc_num, mut n2: bc_num, mut result: *mut bc_num) {
    let mut diff: bc_num = 0 as *mut bc_struct;
    let mut cmp_res: libc::c_int = 0;
    if (*n1).n_sign as libc::c_uint != (*n2).n_sign as libc::c_uint {
        diff = _do_add(n1, n2);
        (*diff).n_sign = (*n1).n_sign
    } else {
        /* subtraction must be done. */
        cmp_res = _do_compare(n1, n2, FALSE, FALSE); /* Compare magnitudes. */
        match cmp_res {
            -1 => {
                /* n1 is less than n2, subtract n1 from n2. */
                diff = _do_sub(n2, n1);
                (*diff).n_sign =
                    if (*n2).n_sign as libc::c_uint == PLUS as libc::c_int as libc::c_uint {
                        MINUS as libc::c_int
                    } else {
                        PLUS as libc::c_int
                    } as sign
            }
            0 => {
                /* They are equal! return zero! */
                diff = copy_num(_zero_)
            }
            1 => {
                /* n2 is less than n1, subtract n2 from n1. */
                diff = _do_sub(n1, n2);
                (*diff).n_sign = (*n1).n_sign
            }
            _ => {}
        }
    }
    /* Clean up and return. */
    free_num(result);
    *result = diff;
}
/* The multiply routine.  N2 time N1 is put int PROD with the scale of
the result being MIN(N2 scale+N1 scale, MAX (SCALE, N2 scale, N1 scale)).
*/
#[no_mangle]
pub unsafe extern "C" fn bc_multiply(
    mut n1: bc_num,
    mut n2: bc_num,
    mut prod: *mut bc_num,
    mut scale: libc::c_int,
) {
    let mut pval: bc_num = 0 as *mut bc_struct; /* For the working storage. */
    let mut n1ptr: *mut libc::c_char = 0 as *mut libc::c_char; /* Work pointers. */
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char; /* To the end of n1 and n2. */
    let mut pvptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n1end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut indx: libc::c_int = 0;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut total_digits: libc::c_int = 0;
    let mut sum: libc::c_long = 0;
    let mut full_scale: libc::c_int = 0;
    let mut prod_scale: libc::c_int = 0;
    let mut toss: libc::c_int = 0;
    let mut tmpI: libc::c_int = 0;
    /* Initialize things. */
    len1 = (*n1).n_len + (*n1).n_scale;
    len2 = (*n2).n_len + (*n2).n_scale;
    total_digits = len1 + len2;
    full_scale = (*n1).n_scale + (*n2).n_scale;
    prod_scale = if full_scale
        > (if scale
            > (if (*n1).n_scale > (*n2).n_scale {
                (*n1).n_scale
            } else {
                (*n2).n_scale
            })
        {
            scale
        } else {
            (if (*n1).n_scale > (*n2).n_scale {
                (*n1).n_scale
            } else {
                (*n2).n_scale
            })
        }) {
        if scale
            > (if (*n1).n_scale > (*n2).n_scale {
                (*n1).n_scale
            } else {
                (*n2).n_scale
            })
        {
            scale
        } else if (*n1).n_scale > (*n2).n_scale {
            (*n1).n_scale
        } else {
            (*n2).n_scale
        }
    } else {
        full_scale
    };
    toss = full_scale - prod_scale;
    pval = new_num(total_digits - full_scale, prod_scale);
    (*pval).n_sign = if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        PLUS as libc::c_int
    } else {
        MINUS as libc::c_int
    } as sign;
    n1end = (&mut *(*n1).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset(len1 as isize)
        .offset(-(1 as libc::c_int as isize));
    n2end = (&mut *(*n2).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset(len2 as isize)
        .offset(-(1 as libc::c_int as isize));
    tmpI = total_digits - toss - 1 as libc::c_int;
    pvptr = (*pval).n_value.as_mut_ptr().offset(tmpI as isize);
    sum = 0 as libc::c_int as libc::c_long;
    /* Here are the loops... */
    indx = 0 as libc::c_int;
    while indx < toss {
        n1ptr = n1end.offset(
            -((if 0 as libc::c_int > indx - len2 + 1 as libc::c_int {
                0 as libc::c_int
            } else {
                (indx - len2) + 1 as libc::c_int
            }) as isize),
        );
        n2ptr = n2end.offset(
            -((if indx > len2 - 1 as libc::c_int {
                (len2) - 1 as libc::c_int
            } else {
                indx
            }) as isize),
        );
        while n1ptr >= (*n1).n_value.as_mut_ptr() && n2ptr <= n2end {
            let fresh28 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            let fresh29 = n2ptr;
            n2ptr = n2ptr.offset(1);
            sum += (*fresh28 as libc::c_int * *fresh29 as libc::c_int) as libc::c_long
        }
        sum = sum / 10 as libc::c_int as libc::c_long;
        indx += 1
    }
    while indx < total_digits - 1 as libc::c_int {
        n1ptr = n1end.offset(
            -((if 0 as libc::c_int > indx - len2 + 1 as libc::c_int {
                0 as libc::c_int
            } else {
                (indx - len2) + 1 as libc::c_int
            }) as isize),
        );
        n2ptr = n2end.offset(
            -((if indx > len2 - 1 as libc::c_int {
                (len2) - 1 as libc::c_int
            } else {
                indx
            }) as isize),
        );
        while n1ptr >= (*n1).n_value.as_mut_ptr() && n2ptr <= n2end {
            let fresh30 = n1ptr;
            n1ptr = n1ptr.offset(-1);
            let fresh31 = n2ptr;
            n2ptr = n2ptr.offset(1);
            sum += (*fresh30 as libc::c_int * *fresh31 as libc::c_int) as libc::c_long
        }
        let fresh32 = pvptr;
        pvptr = pvptr.offset(-1);
        *fresh32 = (sum % 10 as libc::c_int as libc::c_long) as libc::c_char;
        sum = sum / 10 as libc::c_int as libc::c_long;
        indx += 1
    }
    let fresh33 = pvptr;
    pvptr = pvptr.offset(-1);
    *fresh33 = sum as libc::c_char;
    /* Assign to prod and clean up the number. */
    free_num(prod);
    *prod = pval;
    _rm_leading_zeros(*prod);
    if is_zero(*prod) != 0 {
        (**prod).n_sign = PLUS
    };
}
/* Some utility routines for the divide:  First a one digit multiply.
NUM (with SIZE digits) is multiplied by DIGIT and the result is
placed into RESULT.  It is written so that NUM and RESULT can be
the same pointers.  */
unsafe extern "C" fn _one_mult(
    mut num: *mut libc::c_char,
    mut size: libc::c_int,
    mut digit: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut carry: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if digit == 0 as libc::c_int {
        memset(
            result as *mut libc::c_void,
            0 as libc::c_int,
            size as libc::c_ulong,
        );
    } else if digit == 1 as libc::c_int {
        memcpy(
            result as *mut libc::c_void,
            num as *const libc::c_void,
            size as libc::c_ulong,
        );
    } else {
        /* Initialize */
        nptr = num
            .offset(size as isize)
            .offset(-(1 as libc::c_int as isize));
        rptr = result
            .offset(size as isize)
            .offset(-(1 as libc::c_int as isize));
        carry = 0 as libc::c_int;
        loop {
            let fresh34 = size;
            size = size - 1;
            if !(fresh34 > 0 as libc::c_int) {
                break;
            }
            let fresh35 = nptr;
            nptr = nptr.offset(-1);
            value = *fresh35 as libc::c_int * digit + carry;
            let fresh36 = rptr;
            rptr = rptr.offset(-1);
            *fresh36 = (value % 10 as libc::c_int) as libc::c_char;
            carry = value / 10 as libc::c_int
        }
        if carry != 0 as libc::c_int {
            *rptr = carry as libc::c_char
        }
    };
}
/* The full division routine. This computes N1 / N2.  It returns
0 if the division is ok and the result is in QUOT.  The number of
digits after the decimal point is SCALE. It returns -1 if division
by zero is tried.  The algorithm is found in Knuth Vol 2. p237. */
#[no_mangle]
pub unsafe extern "C" fn bc_divide(
    mut n1: bc_num,
    mut n2: bc_num,
    mut quot: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut qval: bc_num = 0 as *mut bc_struct;
    let mut num1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n2ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scale1: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut len1: libc::c_uint = 0;
    let mut len2: libc::c_uint = 0;
    let mut scale2: libc::c_uint = 0;
    let mut qdigits: libc::c_uint = 0;
    let mut extra: libc::c_uint = 0;
    let mut count: libc::c_uint = 0;
    let mut qdig: libc::c_uint = 0;
    let mut qguess: libc::c_uint = 0;
    let mut borrow: libc::c_uint = 0;
    let mut carry: libc::c_uint = 0;
    let mut mval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zero: libc::c_char = 0;
    let mut norm: libc::c_uint = 0;
    /* Test for divide by zero. */
    if is_zero(n2) != 0 {
        return -(1 as libc::c_int);
    }
    /* Test for divide by 1.  If it is we must truncate. */
    if (*n2).n_scale == 0 as libc::c_int {
        if (*n2).n_len == 1 as libc::c_int
            && (*n2).n_value[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int
        {
            qval = new_num((*n1).n_len, scale);
            (*qval).n_sign = if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
                PLUS as libc::c_int
            } else {
                MINUS as libc::c_int
            } as sign;
            memset(
                &mut *(*qval).n_value.as_mut_ptr().offset((*n1).n_len as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                0 as libc::c_int,
                scale as libc::c_ulong,
            );
            memcpy(
                (*qval).n_value.as_mut_ptr() as *mut libc::c_void,
                (*n1).n_value.as_mut_ptr() as *const libc::c_void,
                ((*n1).n_len
                    + (if (*n1).n_scale > scale {
                        scale
                    } else {
                        (*n1).n_scale
                    })) as libc::c_ulong,
            );
            free_num(quot);
            *quot = qval
        }
    }
    /* Set up the divide.  Move the decimal point on n1 by n2's scale.
    Remember, zeros on the end of num2 are wasted effort for dividing. */
    scale2 = (*n2).n_scale as libc::c_uint;
    n2ptr = (&mut *(*n2).n_value.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut libc::c_char)
        .offset((*n2).n_len as isize)
        .offset(scale2 as isize)
        .offset(-(1 as libc::c_int as isize));
    while scale2 > 0 as libc::c_int as libc::c_uint && {
        let fresh37 = n2ptr;
        n2ptr = n2ptr.offset(-1);
        (*fresh37 as libc::c_int) == 0 as libc::c_int
    } {
        scale2 = scale2.wrapping_sub(1)
    }
    len1 = ((*n1).n_len as libc::c_uint).wrapping_add(scale2);
    scale1 = ((*n1).n_scale as libc::c_uint).wrapping_sub(scale2) as libc::c_int;
    if scale1 < scale {
        extra = (scale - scale1) as libc::c_uint
    } else {
        extra = 0 as libc::c_int as libc::c_uint
    }
    num1 = malloc(
        (((*n1).n_len + (*n1).n_scale) as libc::c_uint)
            .wrapping_add(extra)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut libc::c_char;
    /* XXX if (num1 == NULL) out_of_memory(); */
    memset(
        num1 as *mut libc::c_void,
        0 as libc::c_int,
        (((*n1).n_len + (*n1).n_scale) as libc::c_uint)
            .wrapping_add(extra)
            .wrapping_add(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
    );
    memcpy(
        num1.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        (*n1).n_value.as_mut_ptr() as *const libc::c_void,
        ((*n1).n_len + (*n1).n_scale) as libc::c_ulong,
    );
    len2 = ((*n2).n_len as libc::c_uint).wrapping_add(scale2);
    num2 = malloc(len2.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as *mut libc::c_char;
    /* XXX if (num2 == NULL) out_of_memory(); */
    memcpy(
        num2 as *mut libc::c_void,
        (*n2).n_value.as_mut_ptr() as *const libc::c_void,
        len2 as libc::c_ulong,
    );
    *num2.offset(len2 as isize) = 0 as libc::c_int as libc::c_char;
    n2ptr = num2;
    while *n2ptr as libc::c_int == 0 as libc::c_int {
        n2ptr = n2ptr.offset(1);
        len2 = len2.wrapping_sub(1)
    }
    /* Calculate the number of quotient digits. */
    if len2 > len1.wrapping_add(scale as libc::c_uint) {
        qdigits = (scale + 1 as libc::c_int) as libc::c_uint; /* One for the zero integer part. */
        zero = TRUE as libc::c_char
    } else {
        zero = FALSE as libc::c_char;
        if len2 > len1 {
            qdigits = (scale + 1 as libc::c_int) as libc::c_uint
        } else {
            qdigits = len1
                .wrapping_sub(len2)
                .wrapping_add(scale as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }
    }
    /* Allocate and zero the storage for the quotient. */
    qval = new_num(
        qdigits.wrapping_sub(scale as libc::c_uint) as libc::c_int,
        scale,
    );
    memset(
        (*qval).n_value.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        qdigits as libc::c_ulong,
    );
    /* Allocate storage for the temporary storage mval. */
    mval = malloc(len2.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
        as *mut libc::c_char;
    /* XXX if (mval == NULL) out_of_memory (); */
    /* Now for the full divide algorithm. */
    if zero == 0 {
        /* Normalize */
        norm = (10 as libc::c_int / (*n2ptr as libc::c_int + 1 as libc::c_int)) as libc::c_uint;
        if norm != 1 as libc::c_int as libc::c_uint {
            _one_mult(
                num1,
                len1.wrapping_add(scale1 as libc::c_uint)
                    .wrapping_add(extra)
                    .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int,
                norm as libc::c_int,
                num1,
            );
            _one_mult(n2ptr, len2 as libc::c_int, norm as libc::c_int, n2ptr);
        }
        /* Initialize divide loop. */
        qdig = 0 as libc::c_int as libc::c_uint;
        if len2 > len1 {
            qptr = (&mut *(*qval)
                .n_value
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char)
                .offset(len2 as isize)
                .offset(-(len1 as isize))
        } else {
            qptr = &mut *(*qval)
                .n_value
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_char
        }
        /* Loop */
        while qdig <= len1.wrapping_add(scale as libc::c_uint).wrapping_sub(len2) {
            /* Calculate the quotient digit guess. */
            if *n2ptr as libc::c_int == *num1.offset(qdig as isize) as libc::c_int {
                qguess = 9 as libc::c_int as libc::c_uint
            } else {
                qguess = ((*num1.offset(qdig as isize) as libc::c_int * 10 as libc::c_int
                    + *num1.offset(qdig.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_int)
                    / *n2ptr as libc::c_int) as libc::c_uint
            }
            /* Test qguess. */
            if (*n2ptr.offset(1 as libc::c_int as isize) as libc::c_uint).wrapping_mul(qguess)
                > ((*num1.offset(qdig as isize) as libc::c_int * 10 as libc::c_int
                    + *num1.offset(qdig.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_int) as libc::c_uint)
                    .wrapping_sub((*n2ptr as libc::c_uint).wrapping_mul(qguess))
                    .wrapping_mul(10 as libc::c_int as libc::c_uint)
                    .wrapping_add(
                        *num1.offset(qdig.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_uint,
                    )
            {
                qguess = qguess.wrapping_sub(1);
                /* And again. */
                if (*n2ptr.offset(1 as libc::c_int as isize) as
                        libc::c_uint).wrapping_mul(qguess) >
                       ((*num1.offset(qdig as isize) as libc::c_int *
                             10 as libc::c_int +
                             *num1.offset(qdig.wrapping_add(1 as libc::c_int
                                                                as
                                                                libc::c_uint)
                                              as isize) as libc::c_int) as
                            libc::c_uint).wrapping_sub((*n2ptr as
                                                            libc::c_uint).wrapping_mul(qguess)).wrapping_mul(10
                                                                                                                 as
                                                                                                                 libc::c_int
                                                                                                                 as
                                                                                                                 libc::c_uint).wrapping_add(*num1.offset(qdig.wrapping_add(2
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_int
                                                                                                                                                                               as
                                                                                                                                                                               libc::c_uint)
                                                                                                                                                             as
                                                                                                                                                             isize)
                                                                                                                                                as
                                                                                                                                                libc::c_uint)
                   {
                    qguess = qguess.wrapping_sub(1)
                }
            }
            /* Multiply and subtract. */
            borrow = 0 as libc::c_int as libc::c_uint;
            if qguess != 0 as libc::c_int as libc::c_uint {
                *mval = 0 as libc::c_int as libc::c_char;
                _one_mult(
                    n2ptr,
                    len2 as libc::c_int,
                    qguess as libc::c_int,
                    mval.offset(1 as libc::c_int as isize),
                );
                ptr1 = num1.offset(qdig as isize).offset(len2 as isize);
                ptr2 = mval.offset(len2 as isize);
                count = 0 as libc::c_int as libc::c_uint;
                while count < len2.wrapping_add(1 as libc::c_int as libc::c_uint) {
                    let fresh38 = ptr2;
                    ptr2 = ptr2.offset(-1);
                    val = ((*ptr1 as libc::c_int - *fresh38 as libc::c_int) as libc::c_uint)
                        .wrapping_sub(borrow) as libc::c_int;
                    if val < 0 as libc::c_int {
                        val += 10 as libc::c_int;
                        borrow = 1 as libc::c_int as libc::c_uint
                    } else {
                        borrow = 0 as libc::c_int as libc::c_uint
                    }
                    let fresh39 = ptr1;
                    ptr1 = ptr1.offset(-1);
                    *fresh39 = val as libc::c_char;
                    count = count.wrapping_add(1)
                }
            }
            /* Test for negative result. */
            if borrow == 1 as libc::c_int as libc::c_uint {
                qguess = qguess.wrapping_sub(1);
                ptr1 = num1.offset(qdig as isize).offset(len2 as isize);
                ptr2 = n2ptr
                    .offset(len2 as isize)
                    .offset(-(1 as libc::c_int as isize));
                carry = 0 as libc::c_int as libc::c_uint;
                count = 0 as libc::c_int as libc::c_uint;
                while count < len2 {
                    let fresh40 = ptr2;
                    ptr2 = ptr2.offset(-1);
                    val = ((*ptr1 as libc::c_int + *fresh40 as libc::c_int) as libc::c_uint)
                        .wrapping_add(carry) as libc::c_int;
                    if val > 9 as libc::c_int {
                        val -= 10 as libc::c_int;
                        carry = 1 as libc::c_int as libc::c_uint
                    } else {
                        carry = 0 as libc::c_int as libc::c_uint
                    }
                    let fresh41 = ptr1;
                    ptr1 = ptr1.offset(-1);
                    *fresh41 = val as libc::c_char;
                    count = count.wrapping_add(1)
                }
                if carry == 1 as libc::c_int as libc::c_uint {
                    *ptr1 = ((*ptr1 as libc::c_int + 1 as libc::c_int) % 10 as libc::c_int)
                        as libc::c_char
                }
            }
            /* We now know the quotient digit. */
            let fresh42 = qptr;
            qptr = qptr.offset(1);
            *fresh42 = qguess as libc::c_char;
            qdig = qdig.wrapping_add(1)
        }
    }
    /* Clean up and return the number. */
    (*qval).n_sign = if (*n1).n_sign as libc::c_uint == (*n2).n_sign as libc::c_uint {
        PLUS as libc::c_int
    } else {
        MINUS as libc::c_int
    } as sign;
    if is_zero(qval) != 0 {
        (*qval).n_sign = PLUS
    }
    _rm_leading_zeros(qval);
    free_num(quot);
    *quot = qval;
    /* Clean up temporary storage. */
    free(mval as *mut libc::c_void);
    free(num1 as *mut libc::c_void);
    free(num2 as *mut libc::c_void);
    return 0 as libc::c_int;
    /* Everything is OK. */
}
/* Modulo for numbers.  This computes NUM1 % NUM2  and puts the
result in RESULT.   */
#[no_mangle]
pub unsafe extern "C" fn bc_modulo(
    mut num1: bc_num,
    mut num2: bc_num,
    mut result: *mut bc_num,
    mut scale: libc::c_int,
) -> libc::c_int {
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut rscale: libc::c_int = 0;
    /* Check for correct numbers. */
    if is_zero(num2) != 0 {
        return -(1 as libc::c_int);
    }
    /* Calculate final scale. */
    rscale = if (*num1).n_scale > (*num2).n_scale + scale {
        (*num1).n_scale
    } else {
        ((*num2).n_scale) + scale
    };
    init_num(&mut temp);
    /* Calculate it. */
    bc_divide(num1, num2, &mut temp, scale);
    bc_multiply(temp, num2, &mut temp, rscale);
    bc_sub(num1, temp, result);
    free_num(&mut temp);
    return 0 as libc::c_int;
    /* Everything is OK. */
}
/* Raise NUM1 to the NUM2 power.  The result is placed in RESULT.
Maximum exponent is LONG_MAX.  If a NUM2 is not an integer,
only the integer part is used.  */
#[no_mangle]
pub unsafe extern "C" fn bc_raise(
    mut num1: bc_num,
    mut num2: bc_num,
    mut result: *mut bc_num,
    mut scale: libc::c_int,
) {
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut power: bc_num = 0 as *mut bc_struct;
    let mut exponent: libc::c_long = 0;
    let mut rscale: libc::c_int = 0;
    let mut neg: libc::c_char = 0;
    /* Check the exponent for scale digits and convert to a long. */
    if (*num2).n_scale != 0 as libc::c_int {
        rt_warn(
            b"non-zero scale in exponent\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    exponent = num2long(num2);
    if exponent == 0 as libc::c_int as libc::c_long
        && ((*num2).n_len > 1 as libc::c_int
            || (*num2).n_value[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int)
    {
        rt_error(
            b"exponent too large in raise\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    /* Special case if exponent is a zero. */
    if exponent == 0 as libc::c_int as libc::c_long {
        free_num(result);
        *result = copy_num(_one_);
        return;
    }
    /* Other initializations. */
    if exponent < 0 as libc::c_int as libc::c_long {
        neg = TRUE as libc::c_char;
        exponent = -exponent;
        rscale = scale
    } else {
        neg = FALSE as libc::c_char;
        rscale = if (*num1).n_scale as libc::c_long * exponent
            > (if scale > (*num1).n_scale {
                scale
            } else {
                (*num1).n_scale
            }) as libc::c_long
        {
            (if scale > (*num1).n_scale {
                scale
            } else {
                (*num1).n_scale
            }) as libc::c_long
        } else {
            ((*num1).n_scale as libc::c_long) * exponent
        } as libc::c_int
    }
    temp = copy_num(_one_);
    power = copy_num(num1);
    /* Do the calculation. */
    while exponent != 0 as libc::c_int as libc::c_long {
        if exponent & (1 as libc::c_int != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            bc_multiply(temp, power, &mut temp, rscale);
        }
        bc_multiply(power, power, &mut power, rscale);
        exponent = exponent >> 1 as libc::c_int
    }
    /* Assign the value. */
    if neg != 0 {
        bc_divide(_one_, temp, result, rscale);
        free_num(&mut temp);
    } else {
        free_num(result);
        *result = temp
    }
    free_num(&mut power);
}
/* Take the square root NUM and return it in NUM with SCALE digits
after the decimal place. */
#[no_mangle]
pub unsafe extern "C" fn bc_sqrt(mut num: *mut bc_num, mut scale: libc::c_int) -> libc::c_int {
    let mut rscale: libc::c_int = 0;
    let mut cmp_res: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut cscale: libc::c_int = 0;
    let mut guess: bc_num = 0 as *mut bc_struct;
    let mut guess1: bc_num = 0 as *mut bc_struct;
    let mut point5: bc_num = 0 as *mut bc_struct;
    /* Initial checks. */
    cmp_res = bc_compare(*num, _zero_);
    if cmp_res < 0 as libc::c_int {
        return 0 as libc::c_int;
    /* error */
    } else {
        if cmp_res == 0 as libc::c_int {
            free_num(num);
            *num = copy_num(_zero_);
            return 1 as libc::c_int;
        }
    }
    cmp_res = bc_compare(*num, _one_);
    if cmp_res == 0 as libc::c_int {
        free_num(num);
        *num = copy_num(_one_);
        return 1 as libc::c_int;
    }
    /* Initialize the variables. */
    rscale = if scale > (**num).n_scale {
        scale
    } else {
        (**num).n_scale
    };
    cscale = rscale + 2 as libc::c_int;
    init_num(&mut guess);
    init_num(&mut guess1);
    point5 = new_num(1 as libc::c_int, 1 as libc::c_int);
    (*point5).n_value[1 as libc::c_int as usize] = 5 as libc::c_int as libc::c_char;
    /* Calculate the initial guess. */
    if cmp_res < 0 as libc::c_int {
        /* The number is between 0 and 1.  Guess should start at 1. */
        guess = copy_num(_one_)
    } else {
        /* The number is greater than 1.  Guess should start at 10^(exp/2). */
        int2num(&mut guess, 10 as libc::c_int);
        int2num(&mut guess1, (**num).n_len);
        bc_multiply(guess1, point5, &mut guess1, rscale);
        (*guess1).n_scale = 0 as libc::c_int;
        bc_raise(guess, guess1, &mut guess, rscale);
        free_num(&mut guess1);
    }
    /* Find the square root using Newton's algorithm. */
    done = FALSE;
    while done == 0 {
        free_num(&mut guess1);
        guess1 = copy_num(guess);
        bc_divide(*num, guess, &mut guess, cscale);
        bc_add(guess, guess1, &mut guess);
        bc_multiply(guess, point5, &mut guess, cscale);
        cmp_res = _do_compare(guess, guess1, FALSE, TRUE);
        if cmp_res == 0 as libc::c_int {
            done = TRUE
        }
    }
    /* Assign the number and clean up. */
    free_num(num);
    bc_divide(guess, _one_, num, rscale);
    free_num(&mut guess);
    free_num(&mut guess1);
    free_num(&mut point5);
    return 1 as libc::c_int;
}
/* The reference string for digits. */
#[no_mangle]
pub static mut ref_str: [libc::c_char; 17] = unsafe {
    *::std::mem::transmute::<&[u8; 17], &mut [libc::c_char; 17]>(b"0123456789ABCDEF\x00")
};
/* A special output routine for "multi-character digits."  Exactly
SIZE characters must be output for the value .  If SPACE is
non-zero, we must output one space before the number.  OUT_CHAR
is the actual routine for writing the characters. */
#[no_mangle]
pub unsafe extern "C" fn out_long(
    mut val: libc::c_long,
    mut size: libc::c_int,
    mut space: libc::c_int,
    mut out_char: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
    let mut digits: [libc::c_char; 40] = [0; 40];
    let mut len: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    if space != 0 {
        Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
            ' ' as i32,
        );
    }
    sprintf(
        digits.as_mut_ptr(),
        b"%ld\x00" as *const u8 as *const libc::c_char,
        val,
    );
    len = strlen(digits.as_mut_ptr()) as libc::c_int;
    while size > len {
        Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
            '0' as i32,
        );
        size -= 1
    }
    ix = 0 as libc::c_int;
    while ix < len {
        Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
            digits[ix as usize] as libc::c_int,
        );
        ix += 1
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
/* Output of a bcd number.  NUM is written in base O_BASE using OUT_CHAR
as the routine to do the actual output of the characters. */
#[no_mangle]
pub unsafe extern "C" fn out_num(
    mut num: bc_num,
    mut o_base: libc::c_int,
    mut out_char: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
) {
    let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    let mut fdigit: libc::c_int = 0;
    let mut pre_space: libc::c_int = 0;
    let mut digits: *mut stk_rec = 0 as *mut stk_rec;
    let mut temp: *mut stk_rec = 0 as *mut stk_rec;
    let mut int_part: bc_num = 0 as *mut bc_struct;
    let mut frac_part: bc_num = 0 as *mut bc_struct;
    let mut base: bc_num = 0 as *mut bc_struct;
    let mut cur_dig: bc_num = 0 as *mut bc_struct;
    let mut t_num: bc_num = 0 as *mut bc_struct;
    let mut max_o_digit: bc_num = 0 as *mut bc_struct;
    /* The negative sign if needed. */
    if (*num).n_sign as libc::c_uint == MINUS as libc::c_int as libc::c_uint {
        Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
            '-' as i32,
        );
    }
    /* Output the number. */
    if is_zero(num) != 0 {
        Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
            '0' as i32,
        );
    } else if o_base == 10 as libc::c_int {
        /* The number is in base 10, do it the fast way. */
        nptr = &mut *(*num)
            .n_value
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut libc::c_char;
        if (*num).n_len > 1 as libc::c_int || *nptr as libc::c_int != 0 as libc::c_int {
            index = (*num).n_len;
            while index > 0 as libc::c_int {
                let fresh43 = nptr;
                nptr = nptr.offset(1);
                Some(out_char.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    *fresh43 as libc::c_int + '0' as i32
                );
                index -= 1
            }
        } else {
            nptr = nptr.offset(1)
        }
        /* Now the fraction. */
        if (*num).n_scale > 0 as libc::c_int {
            Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
                '.' as i32,
            );
            index = 0 as libc::c_int;
            while index < (*num).n_scale {
                let fresh44 = nptr;
                nptr = nptr.offset(1);
                Some(out_char.expect("non-null function pointer"))
                    .expect("non-null function pointer")(
                    *fresh44 as libc::c_int + '0' as i32
                );
                index += 1
            }
        }
    } else {
        /* The number is some other base. */
        digits = NULL as *mut stk_rec;
        init_num(&mut int_part);
        bc_divide(num, _one_, &mut int_part, 0 as libc::c_int);
        init_num(&mut frac_part);
        init_num(&mut cur_dig);
        init_num(&mut base);
        bc_sub(num, int_part, &mut frac_part);
        int2num(&mut base, o_base);
        init_num(&mut max_o_digit);
        int2num(&mut max_o_digit, o_base - 1 as libc::c_int);
        /* Get the digits of the integer part and push them on a stack. */
        while is_zero(int_part) == 0 {
            bc_modulo(int_part, base, &mut cur_dig, 0 as libc::c_int);
            temp = malloc(::std::mem::size_of::<stk_rec>() as libc::c_ulong) as *mut stk_rec;
            /* XXX if (temp == NULL) out_of_memory(); */
            (*temp).digit = num2long(cur_dig);
            (*temp).next = digits;
            digits = temp;
            bc_divide(int_part, base, &mut int_part, 0 as libc::c_int);
        }
        /* Print the digits on the stack. */
        if !digits.is_null() {
            /* Output the digits. */
            while !digits.is_null() {
                temp = digits;
                digits = (*digits).next;
                if o_base <= 16 as libc::c_int {
                    Some(out_char.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        ref_str[(*temp).digit as libc::c_int as usize] as libc::c_int,
                    );
                } else {
                    out_long(
                        (*temp).digit,
                        (*max_o_digit).n_len,
                        1 as libc::c_int,
                        out_char,
                    );
                }
                free(temp as *mut libc::c_void);
            }
        }
        /* Get and print the digits of the fraction part. */
        if (*num).n_scale > 0 as libc::c_int {
            Some(out_char.expect("non-null function pointer")).expect("non-null function pointer")(
                '.' as i32,
            );
            pre_space = 0 as libc::c_int;
            t_num = copy_num(_one_);
            while (*t_num).n_len <= (*num).n_scale {
                bc_multiply(frac_part, base, &mut frac_part, (*num).n_scale);
                fdigit = num2long(frac_part) as libc::c_int;
                int2num(&mut int_part, fdigit);
                bc_sub(frac_part, int_part, &mut frac_part);
                if o_base <= 16 as libc::c_int {
                    Some(out_char.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        ref_str[fdigit as usize] as libc::c_int,
                    );
                } else {
                    out_long(
                        fdigit as libc::c_long,
                        (*max_o_digit).n_len,
                        pre_space,
                        out_char,
                    );
                    pre_space = 1 as libc::c_int
                }
                bc_multiply(t_num, base, &mut t_num, 0 as libc::c_int);
            }
        }
        /* Clean up. */
        free_num(&mut int_part);
        free_num(&mut frac_part);
        free_num(&mut base);
        free_num(&mut cur_dig);
    };
}
