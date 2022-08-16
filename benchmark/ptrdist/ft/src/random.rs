use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const LONG_MAX: libc::c_long = __LONG_MAX__;
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const __LONG_MAX__: libc::c_long = 9223372036854775807 as libc::c_long;
/*
 * Copyright (c) 1983 Regents of the University of California.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms are permitted
 * provided that the above copyright notice and this paragraph are
 * duplicated in all such forms and that any documentation,
 * advertising materials, and other materials related to such
 * distribution and use acknowledge that the software was developed
 * by the University of California, Berkeley.  The name of the
 * University may not be used to endorse or promote products derived
 * from this software without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 */
/*
 * This is derived from the Berkeley source:
 *	@(#)random.c	5.5 (Berkeley) 7/6/88
 * It was reworked for the GNU C Library by Roland McGrath.
 */
//#include <stdlib.h>
/* An improved random number generation package.  In addition to the standard
rand()/srand() like interface, this package also has a special state info
interface.  The initstate() routine is called with a seed, an array of
bytes, and a count of how many bytes are being passed in; this array is
then initialized to contain information for random number generation with
that much state information.  Good sizes for the amount of state
information are 32, 64, 128, and 256 bytes.  The state can be switched by
calling the setstate() function with the same array as was initiallized
with initstate().  By default, the package runs with 128 bytes of state
information and generates far better random numbers than a linear
congruential generator.  If the amount of state information is less than
32 bytes, a simple linear congruential R.N.G. is used.  Internally, the
state information is treated as an array of longs; the zeroeth element of
the array is the type of R.N.G. being used (small integer); the remainder
of the array is the state information for the R.N.G.  Thus, 32 bytes of
state information will give 7 longs worth of state information, which will
allow a degree seven polynomial.  (Note: The zeroeth word of state
information also has some other information stored in it; see setstate
for details).  The random number generation technique is a linear feedback
shift register approach, employing trinomials (since there are fewer terms
to sum up that way).  In this approach, the least significant bit of all
the numbers in the state table will act as a linear feedback shift register,
and will have period 2^deg - 1 (where deg is the degree of the polynomial
being used, assuming that the polynomial is irreducible and primitive).
The higher order bits will have longer periods, since their values are
also influenced by pseudo-random carries out of the lower bits.  The
total period of the generator is approximately deg*(2**deg - 1); thus
doubling the amount of state information has a vast influence on the
period of the generator.  Note: The deg*(2**deg - 1) is an approximation
only good for large deg, when the period of the shift register is the
dominant factor.  With deg equal to seven, the period is actually much
longer than the 7*(2**7 - 1) predicted by this formula.  */
/* For each of the currently supported random number generators, we have a
break value on the amount of state information (you need at least thi
bytes of state info to support this random number generator), a degree for
the polynomial (actually a trinomial) that the R.N.G. is based on, and
separation between the two lower order coefficients of the trinomial.  */
/* Linear congruential.  */
pub const TYPE_0: libc::c_int = 0 as libc::c_int;
pub const BREAK_0: libc::c_int = 8 as libc::c_int;
pub const DEG_0: libc::c_int = 0 as libc::c_int;
pub const SEP_0: libc::c_int = 0 as libc::c_int;
/* x**7 + x**3 + 1.  */
pub const TYPE_1: libc::c_int = 1 as libc::c_int;
pub const BREAK_1: libc::c_int = 32 as libc::c_int;
pub const DEG_1: libc::c_int = 7 as libc::c_int;
pub const SEP_1: libc::c_int = 3 as libc::c_int;
/* x**15 + x + 1.  */
pub const TYPE_2: libc::c_int = 2 as libc::c_int;
pub const BREAK_2: libc::c_int = 64 as libc::c_int;
pub const DEG_2: libc::c_int = 15 as libc::c_int;
pub const SEP_2: libc::c_int = 1 as libc::c_int;
/* x**31 + x**3 + 1.  */
pub const TYPE_3: libc::c_int = 3 as libc::c_int;
pub const BREAK_3: libc::c_int = 128 as libc::c_int;
pub const DEG_3: libc::c_int = 31 as libc::c_int;
pub const SEP_3: libc::c_int = 3 as libc::c_int;
/* x**63 + x + 1.  */
pub const TYPE_4: libc::c_int = 4 as libc::c_int;
pub const BREAK_4: libc::c_int = 256 as libc::c_int;
pub const DEG_4: libc::c_int = 63 as libc::c_int;
pub const SEP_4: libc::c_int = 1 as libc::c_int;
/* Array versions of the above information to make code run faster.
Relies on fact that TYPE_i == i.  */
pub const MAX_TYPES: libc::c_int = 5 as libc::c_int;
/* Max number of types above.  */
static mut degrees: [libc::c_int; 5] = [DEG_0, DEG_1, DEG_2, DEG_3, DEG_4];
static mut seps: [libc::c_int; 5] = [SEP_0, SEP_1, SEP_2, SEP_3, SEP_4];
/* Initially, everything is set up as if from:
 initstate(1, randtbl, 128);
Note that this initialization takes advantage of the fact that srandom
advances the front and rear pointers 10*rand_deg times, and hence the
rear pointer which starts at 0 will also end up at zero; thus the zeroeth
element of the state information, which contains info about the current
position of the rear pointer is just
 (MAX_TYPES * (rptr - state)) + TYPE_3 == TYPE_3.  */
static mut randtbl: [libc::c_long; 32] = [
    TYPE_3 as libc::c_long,
    -(851904987 as libc::c_int) as libc::c_long,
    -(43806228 as libc::c_int) as libc::c_long,
    -(2029755270 as libc::c_int) as libc::c_long,
    1390239686 as libc::c_int as libc::c_long,
    -(1912102820 as libc::c_int) as libc::c_long,
    -(485608943 as libc::c_int) as libc::c_long,
    1969813258 as libc::c_int as libc::c_long,
    -(1590463333 as libc::c_int) as libc::c_long,
    -(1944053249 as libc::c_int) as libc::c_long,
    455935928 as libc::c_int as libc::c_long,
    508023712 as libc::c_int as libc::c_long,
    -(1714531963 as libc::c_int) as libc::c_long,
    1800685987 as libc::c_int as libc::c_long,
    -(2015299881 as libc::c_int) as libc::c_long,
    654595283 as libc::c_int as libc::c_long,
    -(1149023258 as libc::c_int) as libc::c_long,
    -(1470005550 as libc::c_int) as libc::c_long,
    -(1143256056 as libc::c_int) as libc::c_long,
    -(1325577603 as libc::c_int) as libc::c_long,
    -(1568001885 as libc::c_int) as libc::c_long,
    1275120390 as libc::c_int as libc::c_long,
    -(607508183 as libc::c_int) as libc::c_long,
    -(205999574 as libc::c_int) as libc::c_long,
    -(1696891592 as libc::c_int) as libc::c_long,
    1492211999 as libc::c_int as libc::c_long,
    -(1528267240 as libc::c_int) as libc::c_long,
    -(952028296 as libc::c_int) as libc::c_long,
    -(189082757 as libc::c_int) as libc::c_long,
    362343714 as libc::c_int as libc::c_long,
    1424981831 as libc::c_int as libc::c_long,
    2039449641 as libc::c_int as libc::c_long,
];
/* FPTR and RPTR are two pointers into the state info, a front and a rear
pointer.  These two pointers are always rand_sep places aparts, as they
cycle through the state information.  (Yes, this does mean we could get
away with just one pointer, but the code for random is more efficient
this way).  The pointers are left positioned as they would be from the call:
 initstate(1, randtbl, 128);
(The position of the rear pointer, rptr, is really 0 (as explained above
in the initialization of randtbl) because the state table pointer is set
to point to randtbl[1] (as explained below).)  */
// Initialized in run_static_initializers
static mut fptr: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
// Initialized in run_static_initializers
static mut rptr: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
/* The following things are the pointer to the state information table,
the type of the current generator, the degree of the current polynomial
being used, and the separation between the two pointers.
Note that for efficiency of random, we remember the first location of
the state information, not the zeroeth.  Hence it is valid to access
state[-1], which is used to store the type of the R.N.G.
Also, we remember the last location, since this is more efficient than
indexing every time to find the address of the last element to see if
the front and rear pointers have wrapped.  */
// Initialized in run_static_initializers
static mut state: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
static mut rand_type: libc::c_int = TYPE_3;
static mut rand_deg: libc::c_int = DEG_3;
static mut rand_sep: libc::c_int = SEP_3;
// Initialized in run_static_initializers
static mut end_ptr: *mut libc::c_long = 0 as *const libc::c_long as *mut libc::c_long;
/* Initialize the random number generator based on the given seed.  If the
type is the trivial no-state-information type, just remember the seed.
Otherwise, initializes state[] based on the given "seed" via a linear
congruential generator.  Then, the pointers are set to known locations
that are exactly rand_sep places apart.  Lastly, it cycles the state
information a given number of times to get rid of any initial dependencies
introduced by the L.C.R.N.G.  Note that the initialization of randtbl[]
for default usage relies on values produced by this routine.  */
#[no_mangle]
pub unsafe extern "C" fn srandom(mut x: libc::c_uint) {
    /* Initialize the state information in the given array of N bytes for
    future random number generation.  Based on the number of bytes we
    are given, and the break values for the different R.N.G.'s, we choose
    the best (largest) one we can and set things up for it.  srandom is
    then called to initialize the state information.  Note that on return
    from srandom, we set state[-1] to be the type multiplexed with the current
    value of the rear pointer; this is so successive calls to initstate won't
    lose this information and will be able to restart with setstate.
    Note: The first thing we do is save the current state, if any, just like
    setstate so that it doesn't matter when initstate is called.
    Returns a pointer to the old state.  */
    /* First location.  */
    /* Must set END_PTR before srandom.  */
    /* Restore the state from the given state array.
    Note: It is important that we also remember the locations of the pointers
    in the current state information, and restore the locations of the pointers
    from the old state information.  This is done by multiplexing the pointer
    location into the zeroeth word of the state information. Note that due
    to the order in which things are done, it is OK to call setstate with the
    same state as the current state
    Returns a pointer to the old state information.  */
    /* State info munged.  */
    /* Set end_ptr too.  */
    /* If we are using the trivial TYPE_0 R.N.G., just do the old linear
    congruential bit.  Otherwise, we do our fancy trinomial stuff, which is the
    same in all ther other cases due to all the global variables that have been
    set up.  The basic operation is to add the number at the rear pointer into
    the one at the front pointer.  Then both pointers are advanced to the next
    location cyclically in the table.  The value returned is the sum generated,
    reduced to 31 bits by throwing away the "least random" low bit.
    Note: The code takes advantage of the fact that both the front and
    rear pointers can't wrap on the same call by not testing the rear
    pointer if the front one has wrapped.  Returns a 31-bit random number.  */
    #[export_name = "random"]
    pub unsafe extern "C" fn random_0() -> libc::c_long {
        if rand_type == TYPE_0 {
            *state.offset(0 as libc::c_int as isize) = *state.offset(0 as libc::c_int as isize)
                * 1103515245 as libc::c_int as libc::c_long
                + 12345 as libc::c_int as libc::c_long
                & LONG_MAX;
            return *state.offset(0 as libc::c_int as isize);
        } else {
            let mut i: libc::c_long = 0;
            *fptr += *rptr;
            /* Chucking least random bit.  */
            i = *fptr >> 1 as libc::c_int & LONG_MAX;
            fptr = fptr.offset(1);
            if fptr >= end_ptr {
                fptr = state;
                rptr = rptr.offset(1)
            } else {
                rptr = rptr.offset(1);
                if rptr >= end_ptr {
                    rptr = state
                }
            }
            return i;
        };
    }
    *state.offset(0 as libc::c_int as isize) = x as libc::c_long;
    if rand_type != TYPE_0 {
        let mut i: libc::c_long = 0;
        i = 1 as libc::c_int as libc::c_long;
        while i < rand_deg as libc::c_long {
            *state.offset(i as isize) = 1103515145 as libc::c_int as libc::c_long
                * *state.offset((i - 1 as libc::c_int as libc::c_long) as isize)
                + 12345 as libc::c_int as libc::c_long;
            i += 1
        }
        fptr = &mut *state.offset(rand_sep as isize) as *mut libc::c_long;
        rptr = &mut *state.offset(0 as libc::c_int as isize) as *mut libc::c_long;
        i = 0 as libc::c_int as libc::c_long;
        while i < (10 as libc::c_int * rand_deg) as libc::c_long {
            random_0();
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn initstate(
    mut seed: libc::c_uint,
    mut arg_state: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut ostate: *mut libc::c_void =
        &mut *state.offset(-(1 as libc::c_int) as isize) as *mut libc::c_long as *mut libc::c_void;
    if rand_type == TYPE_0 {
        *state.offset(-(1 as libc::c_int) as isize) = rand_type as libc::c_long
    } else {
        *state.offset(-(1 as libc::c_int) as isize) = MAX_TYPES as libc::c_long
            * rptr.wrapping_offset_from(state) as libc::c_long
            + rand_type as libc::c_long
    }
    if n < BREAK_1 as libc::c_ulong {
        if n < BREAK_0 as libc::c_ulong {
            errno = EINVAL;
            return NULL as *mut libc::c_void;
        }
        rand_type = TYPE_0;
        rand_deg = DEG_0;
        rand_sep = SEP_0
    } else if n < BREAK_2 as libc::c_ulong {
        rand_type = TYPE_1;
        rand_deg = DEG_1;
        rand_sep = SEP_1
    } else if n < BREAK_3 as libc::c_ulong {
        rand_type = TYPE_2;
        rand_deg = DEG_2;
        rand_sep = SEP_2
    } else if n < BREAK_4 as libc::c_ulong {
        rand_type = TYPE_3;
        rand_deg = DEG_3;
        rand_sep = SEP_3
    } else {
        rand_type = TYPE_4;
        rand_deg = DEG_4;
        rand_sep = SEP_4
    }
    state = &mut *(arg_state as *mut libc::c_long).offset(1 as libc::c_int as isize)
        as *mut libc::c_long;
    end_ptr = &mut *state.offset(rand_deg as isize) as *mut libc::c_long;
    srandom(seed);
    if rand_type == TYPE_0 {
        *state.offset(-(1 as libc::c_int) as isize) = rand_type as libc::c_long
    } else {
        *state.offset(-(1 as libc::c_int) as isize) = MAX_TYPES as libc::c_long
            * rptr.wrapping_offset_from(state) as libc::c_long
            + rand_type as libc::c_long
    }
    return ostate;
}
#[no_mangle]
pub unsafe extern "C" fn setstate(mut arg_state: *mut libc::c_void) -> *mut libc::c_void {
    let mut new_state: *mut libc::c_long = arg_state as *mut libc::c_long;
    let mut type_0: libc::c_int =
        (*new_state.offset(0 as libc::c_int as isize) % MAX_TYPES as libc::c_long) as libc::c_int;
    let mut rear: libc::c_int =
        (*new_state.offset(0 as libc::c_int as isize) / MAX_TYPES as libc::c_long) as libc::c_int;
    let mut ostate: *mut libc::c_void =
        &mut *state.offset(-(1 as libc::c_int) as isize) as *mut libc::c_long as *mut libc::c_void;
    if rand_type == TYPE_0 {
        *state.offset(-(1 as libc::c_int) as isize) = rand_type as libc::c_long
    } else {
        *state.offset(-(1 as libc::c_int) as isize) = MAX_TYPES as libc::c_long
            * rptr.wrapping_offset_from(state) as libc::c_long
            + rand_type as libc::c_long
    }
    match type_0 {
        TYPE_0 | TYPE_1 | TYPE_2 | TYPE_3 | TYPE_4 => {
            rand_type = type_0;
            rand_deg = degrees[type_0 as usize];
            rand_sep = seps[type_0 as usize]
        }
        _ => {
            errno = EINVAL;
            return NULL as *mut libc::c_void;
        }
    }
    state = &mut *new_state.offset(1 as libc::c_int as isize) as *mut libc::c_long;
    if rand_type != TYPE_0 {
        rptr = &mut *state.offset(rear as isize) as *mut libc::c_long;
        fptr = &mut *state.offset(((rear + rand_sep) % rand_deg) as isize) as *mut libc::c_long
    }
    end_ptr = &mut *state.offset(rand_deg as isize) as *mut libc::c_long;
    return ostate;
}
unsafe extern "C" fn run_static_initializers() {
    fptr = &mut *randtbl
        .as_mut_ptr()
        .offset((SEP_3 + 1 as libc::c_int) as isize) as *mut libc::c_long;
    rptr = &mut *randtbl.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_long;
    state = &mut *randtbl.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_long;
    end_ptr = &mut *randtbl.as_mut_ptr().offset(
        (::std::mem::size_of::<[libc::c_long; 32]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_long>() as libc::c_ulong) as isize,
    ) as *mut libc::c_long
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
