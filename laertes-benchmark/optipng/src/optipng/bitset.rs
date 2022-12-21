
extern "C" {
    #[no_mangle]
    fn isspace(_: std::os::raw::c_int) -> std::os::raw::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut std::os::raw::c_int;
}
pub type opng_bitset_t = std::os::raw::c_uint;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const OPNG_BITSET_ELT_MAX: C2RustUnnamed = 31;
pub const OPNG_BITSET_ELT_MIN: C2RustUnnamed = 0;
/*
 * Spans the given pointer past the elements that satisfy the given predicate.
 * E.g., opng__SPAN__(str, isspace) moves str past the leading whitespace.
 */
/*
 * Counts the number of elements in a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_count(mut set: opng_bitset_t)
 -> std::os::raw::c_uint {
    let mut result: std::os::raw::c_uint = 0;
    /* Apply Wegner's method. */
    result = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    while set != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        set &= set.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint);
        result = result.wrapping_add(1)
    }
    return result;
}
/*
 * Finds the first element in a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_first(mut set: opng_bitset_t)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i <= OPNG_BITSET_ELT_MAX as std::os::raw::c_int {
        if set & (1 as std::os::raw::c_uint) << i != 0 as std::os::raw::c_int as std::os::raw::c_uint
           {
            return i
        }
        i += 1
    }
    return -(1 as std::os::raw::c_int);
}
/*
 * Finds the next element in a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_next(mut set: opng_bitset_t,
                                               mut elt: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i =
        (if elt > -(1 as std::os::raw::c_int) { elt } else { -(1 as std::os::raw::c_int) }) +
            1 as std::os::raw::c_int;
    while i <= OPNG_BITSET_ELT_MAX as std::os::raw::c_int {
        if set & (1 as std::os::raw::c_uint) << i != 0 as std::os::raw::c_int as std::os::raw::c_uint
           {
            return i
        }
        i += 1
    }
    return -(1 as std::os::raw::c_int);
}
/*
 * Finds the last element in a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_last(mut set: opng_bitset_t)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i = OPNG_BITSET_ELT_MAX as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        if set & (1 as std::os::raw::c_uint) << i != 0 as std::os::raw::c_int as std::os::raw::c_uint
           {
            return i
        }
        i -= 1
    }
    return -(1 as std::os::raw::c_int);
}
/*
 * Finds the previous element in a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_prev(mut set: opng_bitset_t,
                                               mut elt: std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut i: std::os::raw::c_int = 0;
    i =
        (if elt < OPNG_BITSET_ELT_MAX as std::os::raw::c_int + 1 as std::os::raw::c_int {
             elt
         } else { (OPNG_BITSET_ELT_MAX as std::os::raw::c_int) + 1 as std::os::raw::c_int }) -
            1 as std::os::raw::c_int;
    while i >= 0 as std::os::raw::c_int {
        if set & (1 as std::os::raw::c_uint) << i != 0 as std::os::raw::c_int as std::os::raw::c_uint
           {
            return i
        }
        i -= 1
    }
    return -(1 as std::os::raw::c_int);
}
/*
 * Parses a rangeset string and converts the result to a bitset.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_strparse_rangeset_to_bitset(mut out_set:
                                                              *mut opng_bitset_t,
                                                          mut rangeset_str:
                                                              *const std::os::raw::c_char,
                                                          mut mask_set:
                                                              opng_bitset_t)
 -> std::os::raw::c_int {
    let mut result: opng_bitset_t = 0;
    let mut ptr: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    let mut state: std::os::raw::c_int = 0;
    let mut num: std::os::raw::c_int = 0;
    let mut num1: std::os::raw::c_int = 0;
    let mut num2: std::os::raw::c_int = 0;
    let mut err_invalid: std::os::raw::c_int = 0;
    let mut err_range: std::os::raw::c_int = 0;
    result = 0 as std::os::raw::c_uint;
    ptr = rangeset_str;
    state = 0 as std::os::raw::c_int;
    err_range = 0 as std::os::raw::c_int;
    err_invalid = err_range;
    num2 = -(1 as std::os::raw::c_int);
    num1 = num2;
    let mut current_block_37: u64;
    loop  {
        while isspace(*ptr as std::os::raw::c_int) != 0 { ptr = ptr.offset(1) }
        match state {
            0 => {
                /* "" */
                current_block_37 = 6783031248595093306;
            }
            2 => { current_block_37 = 6783031248595093306; }
            1 => {
                /* "N" */
                /* Expecting range operator; go to next state. */
                if *ptr as std::os::raw::c_int == '-' as i32 {
                    ptr = ptr.offset(1);
                    num2 = OPNG_BITSET_ELT_MAX as std::os::raw::c_int;
                    state += 1;
                    continue ;
                } else { current_block_37 = 9520865839495247062; }
            }
            _ => { current_block_37 = 9520865839495247062; }
        }
        match current_block_37 {
            6783031248595093306 =>
            /* "N-" */
            /* Expecting number; go to next state. */
            {
                if *ptr as std::os::raw::c_int >= '0' as i32 &&
                       *ptr as std::os::raw::c_int <= '9' as i32 {
                    num = 0 as std::os::raw::c_int;
                    loop  {
                        num =
                            10 as std::os::raw::c_int * num +
                                (*ptr as std::os::raw::c_int - '0' as i32);
                        if num > OPNG_BITSET_ELT_MAX as std::os::raw::c_int {
                            num = OPNG_BITSET_ELT_MAX as std::os::raw::c_int;
                            err_range = 1 as std::os::raw::c_int
                        }
                        ptr = ptr.offset(1);
                        if !(*ptr as std::os::raw::c_int >= '0' as i32 &&
                                 *ptr as std::os::raw::c_int <= '9' as i32) {
                            break ;
                        }
                    }
                    if !(mask_set & (1 as std::os::raw::c_uint) << num !=
                             0 as std::os::raw::c_int as std::os::raw::c_uint) {
                        err_range = 1 as std::os::raw::c_int
                    }
                    if state == 0 as std::os::raw::c_int { num1 = num }
                    num2 = num;
                    state += 1;
                    continue ;
                }
            }
            _ => { }
        }
        if state > 0 as std::os::raw::c_int {
            /* "N", "N-" or "N-N" */
            /* Store the partial result; go to state 0. */
            if num1 <= num2 {
                result |=
                    if num1 <= num2 {
                        (((1 as std::os::raw::c_uint) << num2 - num1 <<
                              1 as
                                  std::os::raw::c_int).wrapping_sub(1 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint))
                            << num1
                    } else { 0 as std::os::raw::c_uint };
                result &= mask_set
            } else {
                /* Incorrect range operands. */
                err_range = 1 as std::os::raw::c_int
            }
            state = 0 as std::os::raw::c_int
        }
        if *ptr as std::os::raw::c_int == ',' as i32 ||
               *ptr as std::os::raw::c_int == ';' as i32 {
            /* Separator: continue the loop. */
            ptr = ptr.offset(1)
        } else {
            if !(*ptr as std::os::raw::c_int == '-' as i32) { break ; }
            /* Unexpected range operator: invalidate and exit the loop. */
            err_invalid = 1 as std::os::raw::c_int;
            break ;
        }
    }
    while isspace(*ptr as std::os::raw::c_int) != 0 { ptr = ptr.offset(1) }
    if *ptr as std::os::raw::c_int != '\u{0}' as i32 {
        /* Unexpected trailing character: invalidate. */
        err_invalid = 1 as std::os::raw::c_int
    }
    if err_invalid != 0 {
        /* Invalid input error. */
        *__errno_location() = 22 as std::os::raw::c_int;
        *out_set = 0 as std::os::raw::c_uint;
        return -(1 as std::os::raw::c_int)
    } else if err_range != 0 {
        /* Range error. */
        *__errno_location() = 34 as std::os::raw::c_int;
        *out_set = !(0 as std::os::raw::c_uint);
        return -(1 as std::os::raw::c_int)
    } else {
        /* Success. */
        *out_set = result;
        return 0 as std::os::raw::c_int
    };
}
/* TODO */
