use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int
    ) -> uintmax_t;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_s {
    pub payload: *mut libc::c_void,
    pub type_0: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_result_s {
    pub error: size_t,
    pub error_offset: size_t,
    pub error_line_no: size_t,
    pub error_row_no: size_t
}
pub type json_parse_flags_e = libc::c_uint;
pub const json_parse_flags_allow_json5: json_parse_flags_e = 16163;
pub const json_parse_flags_allow_simplified_json: json_parse_flags_e = 31;
pub const json_parse_flags_allow_multi_line_strings: json_parse_flags_e = 8192;
pub const json_parse_flags_allow_inf_and_nan: json_parse_flags_e = 4096;
pub const json_parse_flags_allow_leading_or_trailing_decimal_point:
    json_parse_flags_e = 2048;
pub const json_parse_flags_allow_leading_plus_sign: json_parse_flags_e = 1024;
pub const json_parse_flags_allow_hexadecimal_numbers: json_parse_flags_e = 512;
pub const json_parse_flags_allow_single_quoted_strings: json_parse_flags_e =
    256;
pub const json_parse_flags_allow_location_information: json_parse_flags_e = 128;
pub const json_parse_flags_deprecated: json_parse_flags_e = 64;
pub const json_parse_flags_allow_c_style_comments: json_parse_flags_e = 32;
pub const json_parse_flags_allow_no_commas: json_parse_flags_e = 16;
pub const json_parse_flags_allow_equals_in_object: json_parse_flags_e = 8;
pub const json_parse_flags_allow_global_object: json_parse_flags_e = 4;
pub const json_parse_flags_allow_unquoted_keys: json_parse_flags_e = 2;
pub const json_parse_flags_allow_trailing_comma: json_parse_flags_e = 1;
pub const json_parse_flags_default: json_parse_flags_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_state_s {
    pub src: *const libc::c_char,
    pub size: size_t,
    pub offset: size_t,
    pub flags_bitset: size_t,
    pub data: *mut libc::c_char,
    pub dom: *mut libc::c_char,
    pub dom_size: size_t,
    pub data_size: size_t,
    pub line_no: size_t,
    pub line_offset: size_t,
    pub error: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_number_s {
    pub number: *const libc::c_char,
    pub number_size: size_t
}
pub const json_type_number: json_type_e = 1;
pub const json_type_null: json_type_e = 6;
pub const json_type_false: json_type_e = 5;
pub const json_type_true: json_type_e = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_s {
    pub start: *mut json_array_element_s,
    pub length: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_element_s {
    pub value: *mut json_value_s,
    pub next: *mut json_array_element_s
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_ex_s {
    pub value: json_value_s,
    pub offset: size_t,
    pub line_no: size_t,
    pub row_no: size_t
}
pub const json_parse_error_premature_end_of_buffer: json_parse_error_e = 7;
pub const json_type_array: json_type_e = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_s {
    pub start: *mut json_object_element_s,
    pub length: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_element_s {
    pub name: *mut json_string_s,
    pub value: *mut json_value_s,
    pub next: *mut json_object_element_s
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_s {
    pub string: *const libc::c_char,
    pub string_size: size_t
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_ex_s {
    pub string: json_string_s,
    pub offset: size_t,
    pub line_no: size_t,
    pub row_no: size_t
}
pub const json_type_object: json_type_e = 2;
pub const json_type_string: json_type_e = 0;
pub const json_parse_error_allocator_failed: json_parse_error_e = 9;
pub const json_parse_error_unexpected_trailing_characters: json_parse_error_e =
    10;
pub const json_parse_error_invalid_value: json_parse_error_e = 6;
pub const json_parse_error_invalid_number_format: json_parse_error_e = 5;
pub const json_parse_error_expected_comma_or_closing_bracket:
    json_parse_error_e = 1;
pub const json_parse_error_unknown: json_parse_error_e = 11;
pub const json_parse_error_expected_colon: json_parse_error_e = 2;
pub const json_parse_error_invalid_string: json_parse_error_e = 8;
pub const json_parse_error_invalid_string_escape_sequence: json_parse_error_e =
    4;
pub const json_parse_error_expected_opening_quote: json_parse_error_e = 3;
pub const json_parse_error_none: json_parse_error_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_state_s {
    pub dom: *mut libc::c_char,
    pub data: *mut libc::c_char
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_result_s {
    pub dom_size: size_t,
    pub data_size: size_t
}
pub type uintmax_t = __uintmax_t;
pub type __uintmax_t = libc::c_ulong;
pub type json_type_e = libc::c_uint;
pub type json_parse_error_e = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn json_hexadecimal_digit(
    c: libc::c_char
) -> libc::c_int {
    if '0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32 {
        return c as libc::c_int - '0' as i32;
    }
    if 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'f' as i32 {
        return c as libc::c_int - 'a' as i32 + 10 as libc::c_int;
    }
    if 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'F' as i32 {
        return c as libc::c_int - 'A' as i32 + 10 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_hexadecimal_value(
    mut c: *const libc::c_char,
    size: libc::c_ulong,
    mut result: *mut libc::c_ulong
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut digit: libc::c_int = 0;
    if size
        > (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    *result = 0 as libc::c_int as libc::c_ulong;
    p = c;
    while (p.offset_from(c) as libc::c_long as libc::c_ulong) < size {
        *result <<= 4 as libc::c_int;
        digit = json_hexadecimal_digit(*p);
        if digit < 0 as libc::c_int || digit > 15 as libc::c_int {
            return 0 as libc::c_int;
        }
        *result |= digit as libc::c_uchar as libc::c_ulong;
        p = p.offset(1);
        p;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_whitespace(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    let mut offset: size_t = (*state).offset;
    let size: size_t = (*state).size;
    let src: *const libc::c_char = (*state).src;
    if offset >= (*state).size {
        return 0 as libc::c_int;
    }
    match *src.offset(offset as isize) as libc::c_int {
        32 | 13 | 9 | 10 => {}
        _ => return 0 as libc::c_int
    }
    loop {
        match *src.offset(offset as isize) as libc::c_int {
            32 | 13 | 9 => {}
            10 => {
                (*state).line_no = ((*state).line_no).wrapping_add(1);
                (*state).line_no;
                (*state).line_offset = offset;
            }
            _ => {
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
        }
        offset = offset.wrapping_add(1);
        offset;
        if !(offset < size) {
            break;
        }
    }
    (*state).offset = offset;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_c_style_comments(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    if ((*state).offset).wrapping_add(2 as libc::c_int as libc::c_ulong)
        > (*state).size
    {
        return 0 as libc::c_int;
    }
    if '/' as i32
        == *((*state).src).offset((*state).offset as isize) as libc::c_int
    {
        if '/' as i32
            == *((*state).src).offset(
                ((*state).offset)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as isize
            ) as libc::c_int
        {
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            while (*state).offset < (*state).size {
                match *((*state).src).offset((*state).offset as isize)
                    as libc::c_int
                {
                    10 => {
                        (*state).offset = ((*state).offset).wrapping_add(1);
                        (*state).offset;
                        (*state).line_no = ((*state).line_no).wrapping_add(1);
                        (*state).line_no;
                        (*state).line_offset = (*state).offset;
                        return 1 as libc::c_int;
                    }
                    _ => {
                        (*state).offset = ((*state).offset).wrapping_add(1);
                        (*state).offset;
                    }
                }
            }
            return 1 as libc::c_int;
        } else if '*' as i32
            == *((*state).src).offset(
                ((*state).offset)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as isize
            ) as libc::c_int
        {
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            while ((*state).offset)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                < (*state).size
            {
                if '*' as i32
                    == *((*state).src).offset((*state).offset as isize)
                        as libc::c_int
                    && '/' as i32
                        == *((*state).src).offset(
                            ((*state).offset)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                    return 1 as libc::c_int;
                } else if '\n' as i32
                    == *((*state).src).offset((*state).offset as isize)
                        as libc::c_int
                {
                    (*state).line_no = ((*state).line_no).wrapping_add(1);
                    (*state).line_no;
                    (*state).line_offset = (*state).offset;
                }
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
            }
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_all_skippables(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    let mut did_consume: libc::c_int = 0 as libc::c_int;
    let size: size_t = (*state).size;
    if json_parse_flags_allow_c_style_comments as libc::c_int as libc::c_ulong
        & (*state).flags_bitset
        != 0
    {
        loop {
            if (*state).offset == size {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            did_consume = json_skip_whitespace(state);
            if (*state).offset >= size {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            did_consume |= json_skip_c_style_comments(state);
            if !(0 as libc::c_int != did_consume) {
                break;
            }
        }
    } else {
        loop {
            if (*state).offset == size {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            did_consume = json_skip_whitespace(state);
            if !(0 as libc::c_int != did_consume) {
                break;
            }
        }
    }
    if (*state).offset == size {
        (*state).error =
            json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_string_size(
    mut state: *mut json_parse_state_s,
    mut is_key: size_t
) -> libc::c_int {
    let mut offset: size_t = (*state).offset;
    let size: size_t = (*state).size;
    let mut data_size: size_t = 0 as libc::c_int as size_t;
    let src: *const libc::c_char = (*state).src;
    let is_single_quote: libc::c_int = ('\'' as i32
        == *src.offset(offset as isize) as libc::c_int)
        as libc::c_int;
    let quote_to_use: libc::c_char = (if is_single_quote != 0 {
        '\'' as i32
    } else {
        '"' as i32
    }) as libc::c_char;
    let flags_bitset: size_t = (*state).flags_bitset;
    let mut codepoint: libc::c_ulong = 0;
    let mut high_surrogate: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if json_parse_flags_allow_location_information as libc::c_int
        as libc::c_ulong
        & flags_bitset
        != 0 as libc::c_int as libc::c_ulong
        && is_key != 0 as libc::c_int as libc::c_ulong
    {
        (*state).dom_size = ((*state).dom_size as libc::c_ulong).wrapping_add(
            ::core::mem::size_of::<json_string_ex_s>() as libc::c_ulong
        ) as size_t as size_t;
    } else {
        (*state).dom_size = ((*state).dom_size as libc::c_ulong).wrapping_add(
            ::core::mem::size_of::<json_string_s>() as libc::c_ulong
        ) as size_t as size_t;
    }
    if '"' as i32 != *src.offset(offset as isize) as libc::c_int {
        if !(json_parse_flags_allow_single_quoted_strings as libc::c_int
            as libc::c_ulong
            & flags_bitset
            != 0
            && is_single_quote != 0)
        {
            (*state).error = json_parse_error_expected_opening_quote
                as libc::c_int as size_t;
            (*state).offset = offset;
            return 1 as libc::c_int;
        }
    }
    offset = offset.wrapping_add(1);
    offset;
    while offset < size
        && quote_to_use as libc::c_int
            != *src.offset(offset as isize) as libc::c_int
    {
        data_size = data_size.wrapping_add(1);
        data_size;
        match *src.offset(offset as isize) as libc::c_int {
            0 | 9 => {
                (*state).error =
                    json_parse_error_invalid_string as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
            _ => {}
        }
        if '\\' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset = offset.wrapping_add(1);
            offset;
            if offset == size {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
            match *src.offset(offset as isize) as libc::c_int {
                34 | 92 | 47 | 98 | 102 | 110 | 114 | 116 => {
                    offset = offset.wrapping_add(1);
                    offset;
                }
                117 => {
                    if !(offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                        < size)
                    {
                        (*state).error =
                            json_parse_error_invalid_string_escape_sequence
                                as libc::c_int
                                as size_t;
                        (*state).offset = offset;
                        return 1 as libc::c_int;
                    }
                    codepoint = 0 as libc::c_int as libc::c_ulong;
                    if json_hexadecimal_value(
                        &*src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ),
                        4 as libc::c_int as libc::c_ulong,
                        &mut codepoint
                    ) == 0
                    {
                        (*state).error =
                            json_parse_error_invalid_string_escape_sequence
                                as libc::c_int
                                as size_t;
                        (*state).offset = offset;
                        return 1 as libc::c_int;
                    }
                    if high_surrogate != 0 as libc::c_int as libc::c_ulong {
                        if codepoint >= 0xdc00 as libc::c_int as libc::c_ulong
                            && codepoint
                                <= 0xdfff as libc::c_int as libc::c_ulong
                        {
                            data_size = (data_size as libc::c_ulong)
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as size_t
                                as size_t;
                            high_surrogate = 0 as libc::c_int as libc::c_ulong;
                        } else {
                            (*state).error =
                                json_parse_error_invalid_string_escape_sequence
                                    as libc::c_int
                                    as size_t;
                            (*state).offset = offset;
                            return 1 as libc::c_int;
                        }
                    } else if codepoint <= 0x7f as libc::c_int as libc::c_ulong
                    {
                        data_size = (data_size as libc::c_ulong)
                            .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            as size_t
                            as size_t;
                    } else if codepoint <= 0x7ff as libc::c_int as libc::c_ulong
                    {
                        data_size = (data_size as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as size_t
                            as size_t;
                    } else if codepoint
                        >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdbff as libc::c_int as libc::c_ulong
                    {
                        if offset
                            .wrapping_add(11 as libc::c_int as libc::c_ulong)
                            > size
                            || '\\' as i32
                                != *src.offset(offset.wrapping_add(
                                    5 as libc::c_int as libc::c_ulong
                                )
                                    as isize)
                                    as libc::c_int
                            || 'u' as i32
                                != *src.offset(offset.wrapping_add(
                                    6 as libc::c_int as libc::c_ulong
                                )
                                    as isize)
                                    as libc::c_int
                        {
                            (*state).error =
                                json_parse_error_invalid_string_escape_sequence
                                    as libc::c_int
                                    as size_t;
                            (*state).offset = offset;
                            return 1 as libc::c_int;
                        }
                        high_surrogate = codepoint;
                    } else if codepoint
                        >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdfff as libc::c_int as libc::c_ulong
                    {
                        (*state).error =
                            json_parse_error_invalid_string_escape_sequence
                                as libc::c_int
                                as size_t;
                        (*state).offset = offset;
                        return 1 as libc::c_int;
                    } else {
                        data_size = (data_size as libc::c_ulong)
                            .wrapping_add(2 as libc::c_int as libc::c_ulong)
                            as size_t
                            as size_t;
                    }
                    offset = (offset as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                }
                _ => {
                    (*state).error =
                        json_parse_error_invalid_string_escape_sequence
                            as libc::c_int as size_t;
                    (*state).offset = offset;
                    return 1 as libc::c_int;
                }
            }
        } else if '\r' as i32 == *src.offset(offset as isize) as libc::c_int
            || '\n' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            if json_parse_flags_allow_multi_line_strings as libc::c_int
                as libc::c_ulong
                & flags_bitset
                == 0
            {
                (*state).error = json_parse_error_invalid_string_escape_sequence
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
            offset = offset.wrapping_add(1);
            offset;
        } else {
            offset = offset.wrapping_add(1);
            offset;
        }
    }
    if offset == size {
        (*state).error =
            json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        (*state).offset =
            offset.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        return 1 as libc::c_int;
    }
    offset = offset.wrapping_add(1);
    offset;
    (*state).data_size = ((*state).data_size as libc::c_ulong)
        .wrapping_add(data_size) as size_t as size_t;
    (*state).data_size = ((*state).data_size).wrapping_add(1);
    (*state).data_size;
    (*state).offset = offset;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn is_valid_unquoted_key_char(
    c: libc::c_char
) -> libc::c_int {
    return ('0' as i32 <= c as libc::c_int && c as libc::c_int <= '9' as i32
        || 'a' as i32 <= c as libc::c_int && c as libc::c_int <= 'z' as i32
        || 'A' as i32 <= c as libc::c_int && c as libc::c_int <= 'Z' as i32
        || '_' as i32 == c as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_key_size(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    let flags_bitset: size_t = (*state).flags_bitset;
    if json_parse_flags_allow_unquoted_keys as libc::c_int as libc::c_ulong
        & flags_bitset
        != 0
    {
        let mut offset: size_t = (*state).offset;
        let size: size_t = (*state).size;
        let src: *const libc::c_char = (*state).src;
        let mut data_size: size_t = (*state).data_size;
        if '"' as i32 == *src.offset(offset as isize) as libc::c_int {
            return json_get_string_size(state, 1 as libc::c_int as size_t);
        } else if json_parse_flags_allow_single_quoted_strings as libc::c_int
            as libc::c_ulong
            & flags_bitset
            != 0
            && '\'' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            return json_get_string_size(state, 1 as libc::c_int as size_t);
        } else {
            while offset < size
                && is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0
            {
                offset = offset.wrapping_add(1);
                offset;
                data_size = data_size.wrapping_add(1);
                data_size;
            }
            data_size = data_size.wrapping_add(1);
            data_size;
            if json_parse_flags_allow_location_information as libc::c_int
                as libc::c_ulong
                & flags_bitset
                != 0
            {
                (*state).dom_size = ((*state).dom_size as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<json_string_ex_s>()
                        as libc::c_ulong)
                    as size_t as size_t;
            } else {
                (*state).dom_size = ((*state).dom_size as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<json_string_s>()
                        as libc::c_ulong)
                    as size_t as size_t;
            }
            (*state).offset = offset;
            (*state).data_size = data_size;
            return 0 as libc::c_int;
        }
    } else {
        return json_get_string_size(state, 1 as libc::c_int as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_get_object_size(
    mut state: *mut json_parse_state_s,
    mut is_global_object: libc::c_int
) -> libc::c_int {
    let flags_bitset: size_t = (*state).flags_bitset;
    let src: *const libc::c_char = (*state).src;
    let size: size_t = (*state).size;
    let mut elements: size_t = 0 as libc::c_int as size_t;
    let mut allow_comma: libc::c_int = 0 as libc::c_int;
    let mut found_closing_brace: libc::c_int = 0 as libc::c_int;
    if is_global_object != 0 {
        if json_skip_all_skippables(state) == 0
            && '{' as i32
                == *((*state).src).offset((*state).offset as isize)
                    as libc::c_int
        {
            is_global_object = 0 as libc::c_int;
        }
    }
    if is_global_object == 0 {
        if '{' as i32 != *src.offset((*state).offset as isize) as libc::c_int {
            (*state).error = json_parse_error_unknown as libc::c_int as size_t;
            return 1 as libc::c_int;
        }
        (*state).offset = ((*state).offset).wrapping_add(1);
        (*state).offset;
    }
    (*state).dom_size = ((*state).dom_size as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<json_object_s>() as libc::c_ulong)
        as size_t as size_t;
    if (*state).offset == size && is_global_object == 0 {
        (*state).error =
            json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    let mut current_block_66: u64;
    loop {
        if is_global_object == 0 {
            if json_skip_all_skippables(state) != 0 {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            if '}' as i32
                == *src.offset((*state).offset as isize) as libc::c_int
            {
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                found_closing_brace = 1 as libc::c_int;
                break;
            }
        } else if json_skip_all_skippables(state) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32
                == *src.offset((*state).offset as isize) as libc::c_int
            {
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                allow_comma = 0 as libc::c_int;
            } else if json_parse_flags_allow_no_commas as libc::c_int
                as libc::c_ulong
                & flags_bitset
                != 0
            {
                allow_comma = 0 as libc::c_int;
            } else {
                (*state).error =
                    json_parse_error_expected_comma_or_closing_bracket
                        as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            if json_parse_flags_allow_trailing_comma as libc::c_int
                as libc::c_ulong
                & flags_bitset
                != 0
            {
                current_block_66 = 17860125682698302841;
            } else {
                if json_skip_all_skippables(state) != 0 {
                    (*state).error = json_parse_error_premature_end_of_buffer
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                current_block_66 = 4090602189656566074;
            }
        } else {
            current_block_66 = 4090602189656566074;
        }
        match current_block_66 {
            4090602189656566074 => {
                if json_get_key_size(state) != 0 {
                    (*state).error = json_parse_error_invalid_string
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_skip_all_skippables(state) != 0 {
                    (*state).error = json_parse_error_premature_end_of_buffer
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_parse_flags_allow_equals_in_object as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                {
                    let current: libc::c_char =
                        *src.offset((*state).offset as isize);
                    if ':' as i32 != current as libc::c_int
                        && '=' as i32 != current as libc::c_int
                    {
                        (*state).error = json_parse_error_expected_colon
                            as libc::c_int
                            as size_t;
                        return 1 as libc::c_int;
                    }
                } else if ':' as i32
                    != *src.offset((*state).offset as isize) as libc::c_int
                {
                    (*state).error = json_parse_error_expected_colon
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                if json_skip_all_skippables(state) != 0 {
                    (*state).error = json_parse_error_premature_end_of_buffer
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
                if json_get_value_size(state, 0 as libc::c_int) != 0 {
                    return 1 as libc::c_int;
                }
                elements = elements.wrapping_add(1);
                elements;
                allow_comma = 1 as libc::c_int;
            }
            _ => {}
        }
        if !((*state).offset < size) {
            break;
        }
    }
    if (*state).offset == size
        && is_global_object == 0
        && found_closing_brace == 0
    {
        (*state).error =
            json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    (*state).dom_size = ((*state).dom_size as libc::c_ulong).wrapping_add(
        (::core::mem::size_of::<json_object_element_s>() as libc::c_ulong)
            .wrapping_mul(elements)
    ) as size_t as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_array_size(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    let flags_bitset: size_t = (*state).flags_bitset;
    let mut elements: size_t = 0 as libc::c_int as size_t;
    let mut allow_comma: libc::c_int = 0 as libc::c_int;
    let src: *const libc::c_char = (*state).src;
    let size: size_t = (*state).size;
    if '[' as i32 != *src.offset((*state).offset as isize) as libc::c_int {
        (*state).error = json_parse_error_unknown as libc::c_int as size_t;
        return 1 as libc::c_int;
    }
    (*state).offset = ((*state).offset).wrapping_add(1);
    (*state).offset;
    (*state).dom_size = ((*state).dom_size as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<json_array_s>() as libc::c_ulong)
        as size_t as size_t;
    while (*state).offset < size {
        if json_skip_all_skippables(state) != 0 {
            (*state).error = json_parse_error_premature_end_of_buffer
                as libc::c_int as size_t;
            return 1 as libc::c_int;
        }
        if ']' as i32 == *src.offset((*state).offset as isize) as libc::c_int {
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            (*state).dom_size = ((*state).dom_size as libc::c_ulong)
                .wrapping_add(
                    (::core::mem::size_of::<json_array_element_s>()
                        as libc::c_ulong)
                        .wrapping_mul(elements)
                ) as size_t as size_t;
            return 0 as libc::c_int;
        }
        if allow_comma != 0 {
            if ',' as i32
                == *src.offset((*state).offset as isize) as libc::c_int
            {
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                allow_comma = 0 as libc::c_int;
            } else if json_parse_flags_allow_no_commas as libc::c_int
                as libc::c_ulong
                & flags_bitset
                == 0
            {
                (*state).error =
                    json_parse_error_expected_comma_or_closing_bracket
                        as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
            if json_parse_flags_allow_trailing_comma as libc::c_int
                as libc::c_ulong
                & flags_bitset
                != 0
            {
                allow_comma = 0 as libc::c_int;
                continue;
            } else if json_skip_all_skippables(state) != 0 {
                (*state).error = json_parse_error_premature_end_of_buffer
                    as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
        }
        if json_get_value_size(state, 0 as libc::c_int) != 0 {
            return 1 as libc::c_int;
        }
        elements = elements.wrapping_add(1);
        elements;
        allow_comma = 1 as libc::c_int;
    }
    (*state).error =
        json_parse_error_premature_end_of_buffer as libc::c_int as size_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_number_size(
    mut state: *mut json_parse_state_s
) -> libc::c_int {
    let flags_bitset: size_t = (*state).flags_bitset;
    let mut offset: size_t = (*state).offset;
    let size: size_t = (*state).size;
    let mut had_leading_digits: libc::c_int = 0 as libc::c_int;
    let src: *const libc::c_char = (*state).src;
    (*state).dom_size = ((*state).dom_size as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<json_number_s>() as libc::c_ulong)
        as size_t as size_t;
    if json_parse_flags_allow_hexadecimal_numbers as libc::c_int
        as libc::c_ulong
        & flags_bitset
        != 0
        && offset.wrapping_add(1 as libc::c_int as libc::c_ulong) < size
        && '0' as i32 == *src.offset(offset as isize) as libc::c_int
        && ('x' as i32
            == *src
                .offset(offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as isize) as libc::c_int
            || 'X' as i32
                == *src.offset(
                    offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize
                ) as libc::c_int)
    {
        offset = (offset as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32
                || 'a' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= 'f' as i32
                || 'A' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= 'F' as i32)
        {
            offset = offset.wrapping_add(1);
            offset;
        }
    } else {
        let mut found_sign: libc::c_int = 0 as libc::c_int;
        let mut inf_or_nan: libc::c_int = 0 as libc::c_int;
        if offset < size
            && ('-' as i32 == *src.offset(offset as isize) as libc::c_int
                || json_parse_flags_allow_leading_plus_sign as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                    && '+' as i32
                        == *src.offset(offset as isize) as libc::c_int)
        {
            offset = offset.wrapping_add(1);
            offset;
            found_sign = 1 as libc::c_int;
        }
        if json_parse_flags_allow_inf_and_nan as libc::c_int as libc::c_ulong
            & flags_bitset
            != 0
        {
            let inf: [libc::c_char; 9] = *::core::mem::transmute::<
                &[u8; 9],
                &[libc::c_char; 9]
            >(b"Infinity\0");
            let inf_strlen: size_t =
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let nan: [libc::c_char; 4] = *::core::mem::transmute::<
                &[u8; 4],
                &[libc::c_char; 4]
            >(b"NaN\0");
            let nan_strlen: size_t =
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            if offset.wrapping_add(inf_strlen) < size {
                let mut found: libc::c_int = 1 as libc::c_int;
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                while i < inf_strlen {
                    if inf[i as usize] as libc::c_int
                        != *src.offset(offset.wrapping_add(i) as isize)
                            as libc::c_int
                    {
                        found = 0 as libc::c_int;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                if found != 0 {
                    offset = (offset as libc::c_ulong).wrapping_add(inf_strlen)
                        as size_t as size_t;
                    inf_or_nan = 1 as libc::c_int;
                }
            }
            if offset.wrapping_add(nan_strlen) < size {
                let mut found_0: libc::c_int = 1 as libc::c_int;
                let mut i_0: size_t = 0;
                i_0 = 0 as libc::c_int as size_t;
                while i_0 < nan_strlen {
                    if nan[i_0 as usize] as libc::c_int
                        != *src.offset(offset.wrapping_add(i_0) as isize)
                            as libc::c_int
                    {
                        found_0 = 0 as libc::c_int;
                        break;
                    } else {
                        i_0 = i_0.wrapping_add(1);
                        i_0;
                    }
                }
                if found_0 != 0 {
                    offset = (offset as libc::c_ulong).wrapping_add(nan_strlen)
                        as size_t as size_t;
                    inf_or_nan = 1 as libc::c_int;
                }
            }
            if inf_or_nan != 0 {
                if offset < size {
                    match *src.offset(offset as isize) as libc::c_int {
                        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57
                        | 101 | 69 => {
                            (*state).error =
                                json_parse_error_invalid_number_format
                                    as libc::c_int
                                    as size_t;
                            (*state).offset = offset;
                            return 1 as libc::c_int;
                        }
                        _ => {}
                    }
                }
            }
        }
        if found_sign != 0
            && inf_or_nan == 0
            && offset < size
            && !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
        {
            if json_parse_flags_allow_leading_or_trailing_decimal_point
                as libc::c_int as libc::c_ulong
                & flags_bitset
                == 0
                || '.' as i32 != *src.offset(offset as isize) as libc::c_int
            {
                (*state).error = json_parse_error_invalid_number_format
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
        }
        if offset < size
            && '0' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            offset = offset.wrapping_add(1);
            offset;
            had_leading_digits = 1 as libc::c_int;
            if offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= '9' as i32)
            {
                (*state).error = json_parse_error_invalid_number_format
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
        }
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                && *src.offset(offset as isize) as libc::c_int <= '9' as i32)
        {
            offset = offset.wrapping_add(1);
            offset;
            had_leading_digits = 1 as libc::c_int;
        }
        if offset < size
            && '.' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            offset = offset.wrapping_add(1);
            offset;
            if offset >= size
                || !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= '9' as i32)
            {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as libc::c_int as libc::c_ulong
                    & flags_bitset
                    == 0
                    || had_leading_digits == 0
                {
                    (*state).error = json_parse_error_invalid_number_format
                        as libc::c_int
                        as size_t;
                    (*state).offset = offset;
                    return 1 as libc::c_int;
                }
            }
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= '9' as i32)
            {
                offset = offset.wrapping_add(1);
                offset;
            }
        }
        if offset < size
            && ('e' as i32 == *src.offset(offset as isize) as libc::c_int
                || 'E' as i32 == *src.offset(offset as isize) as libc::c_int)
        {
            offset = offset.wrapping_add(1);
            offset;
            if offset < size
                && ('-' as i32 == *src.offset(offset as isize) as libc::c_int
                    || '+' as i32
                        == *src.offset(offset as isize) as libc::c_int)
            {
                offset = offset.wrapping_add(1);
                offset;
            }
            if offset < size
                && !('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= '9' as i32)
            {
                (*state).error = json_parse_error_invalid_number_format
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
            loop {
                offset = offset.wrapping_add(1);
                offset;
                if !(offset < size
                    && ('0' as i32
                        <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int
                            <= '9' as i32))
                {
                    break;
                }
            }
        }
    }
    if offset < size {
        match *src.offset(offset as isize) as libc::c_int {
            32 | 9 | 13 | 10 | 125 | 44 | 93 => {}
            61 => {
                if !(json_parse_flags_allow_equals_in_object as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0)
                {
                    (*state).error = json_parse_error_invalid_number_format
                        as libc::c_int
                        as size_t;
                    (*state).offset = offset;
                    return 1 as libc::c_int;
                }
            }
            _ => {
                (*state).error = json_parse_error_invalid_number_format
                    as libc::c_int as size_t;
                (*state).offset = offset;
                return 1 as libc::c_int;
            }
        }
    }
    (*state).data_size = ((*state).data_size as libc::c_ulong)
        .wrapping_add(offset.wrapping_sub((*state).offset))
        as size_t as size_t;
    (*state).data_size = ((*state).data_size).wrapping_add(1);
    (*state).data_size;
    (*state).offset = offset;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_value_size(
    mut state: *mut json_parse_state_s,
    mut is_global_object: libc::c_int
) -> libc::c_int {
    let flags_bitset: size_t = (*state).flags_bitset;
    let src: *const libc::c_char = (*state).src;
    let mut offset: size_t = 0;
    let size: size_t = (*state).size;
    if json_parse_flags_allow_location_information as libc::c_int
        as libc::c_ulong
        & flags_bitset
        != 0
    {
        (*state).dom_size = ((*state).dom_size as libc::c_ulong).wrapping_add(
            ::core::mem::size_of::<json_value_ex_s>() as libc::c_ulong
        ) as size_t as size_t;
    } else {
        (*state).dom_size = ((*state).dom_size as libc::c_ulong).wrapping_add(
            ::core::mem::size_of::<json_value_s>() as libc::c_ulong
        ) as size_t as size_t;
    }
    if is_global_object != 0 {
        return json_get_object_size(state, 1 as libc::c_int);
    } else {
        if json_skip_all_skippables(state) != 0 {
            (*state).error = json_parse_error_premature_end_of_buffer
                as libc::c_int as size_t;
            return 1 as libc::c_int;
        }
        offset = (*state).offset;
        match *src.offset(offset as isize) as libc::c_int {
            34 => {
                return json_get_string_size(state, 0 as libc::c_int as size_t)
            }
            39 => {
                if json_parse_flags_allow_single_quoted_strings as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                {
                    return json_get_string_size(
                        state,
                        0 as libc::c_int as size_t
                    );
                } else {
                    (*state).error =
                        json_parse_error_invalid_value as libc::c_int as size_t;
                    return 1 as libc::c_int;
                }
            }
            123 => return json_get_object_size(state, 0 as libc::c_int),
            91 => return json_get_array_size(state),
            45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return json_get_number_size(state);
            }
            43 => {
                if json_parse_flags_allow_leading_plus_sign as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                {
                    return json_get_number_size(state);
                } else {
                    (*state).error = json_parse_error_invalid_number_format
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
            }
            46 => {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as libc::c_int as libc::c_ulong
                    & flags_bitset
                    != 0
                {
                    return json_get_number_size(state);
                } else {
                    (*state).error = json_parse_error_invalid_number_format
                        as libc::c_int
                        as size_t;
                    return 1 as libc::c_int;
                }
            }
            _ => {
                if offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                    <= size
                    && 't' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'r' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'u' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'e' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                    return 0 as libc::c_int;
                } else if offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                    <= size
                    && 'f' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'a' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 's' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'e' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                    return 0 as libc::c_int;
                } else if offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                    <= size
                    && 'n' as i32
                        == *((*state).src).offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'u' as i32
                        == *((*state).src).offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *((*state).src).offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *((*state).src).offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                    return 0 as libc::c_int;
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                    && offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                        <= size
                    && 'N' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'a' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'N' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    return json_get_number_size(state);
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                    && offset.wrapping_add(8 as libc::c_int as libc::c_ulong)
                        <= size
                    && 'I' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'n' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'f' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'i' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'n' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'i' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 't' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'y' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    return json_get_number_size(state);
                }
                (*state).error =
                    json_parse_error_invalid_value as libc::c_int as size_t;
                return 1 as libc::c_int;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_string(
    mut state: *mut json_parse_state_s,
    mut string: *mut json_string_s
) {
    let mut offset: size_t = (*state).offset;
    let mut bytes_written: size_t = 0 as libc::c_int as size_t;
    let src: *const libc::c_char = (*state).src;
    let quote_to_use: libc::c_char =
        (if '\'' as i32 == *src.offset(offset as isize) as libc::c_int {
            '\'' as i32
        } else {
            '"' as i32
        }) as libc::c_char;
    let mut data: *mut libc::c_char = (*state).data;
    let mut high_surrogate: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut codepoint: libc::c_ulong = 0;
    (*string).string = data;
    offset = offset.wrapping_add(1);
    offset;
    while quote_to_use as libc::c_int
        != *src.offset(offset as isize) as libc::c_int
    {
        if '\\' as i32 == *src.offset(offset as isize) as libc::c_int {
            offset = offset.wrapping_add(1);
            offset;
            let fresh0 = offset;
            offset = offset.wrapping_add(1);
            match *src.offset(fresh0 as isize) as libc::c_int {
                117 => {
                    codepoint = 0 as libc::c_int as libc::c_ulong;
                    if json_hexadecimal_value(
                        &*src.offset(offset as isize),
                        4 as libc::c_int as libc::c_ulong,
                        &mut codepoint
                    ) == 0
                    {
                        return;
                    }
                    offset = (offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t as size_t;
                    if codepoint <= 0x7f as libc::c_uint as libc::c_ulong {
                        let fresh1 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh1 as isize) =
                            codepoint as libc::c_char;
                    } else if codepoint
                        <= 0x7ff as libc::c_uint as libc::c_ulong
                    {
                        let fresh2 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh2 as isize) = (0xc0 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 6 as libc::c_int)
                            as libc::c_char;
                        let fresh3 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh3 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    } else if codepoint
                        >= 0xd800 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdbff as libc::c_int as libc::c_ulong
                    {
                        high_surrogate = codepoint;
                    } else if codepoint
                        >= 0xdc00 as libc::c_int as libc::c_ulong
                        && codepoint <= 0xdfff as libc::c_int as libc::c_ulong
                    {
                        let surrogate_offset: libc::c_ulong = (0x10000
                            as libc::c_uint)
                            .wrapping_sub(
                                (0xd800 as libc::c_uint) << 10 as libc::c_int
                            )
                            .wrapping_sub(0xdc00 as libc::c_uint)
                            as libc::c_ulong;
                        codepoint = (high_surrogate << 10 as libc::c_int)
                            .wrapping_add(codepoint)
                            .wrapping_add(surrogate_offset);
                        high_surrogate = 0 as libc::c_int as libc::c_ulong;
                        let fresh4 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh4 as isize) = (0xf0 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 18 as libc::c_int)
                            as libc::c_char;
                        let fresh5 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh5 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 12 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                        let fresh6 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh6 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 6 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                        let fresh7 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh7 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    } else {
                        let fresh8 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh8 as isize) = (0xe0 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 12 as libc::c_int)
                            as libc::c_char;
                        let fresh9 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh9 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint >> 6 as libc::c_int
                                & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                        let fresh10 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh10 as isize) = (0x80 as libc::c_uint
                            as libc::c_ulong
                            | codepoint & 0x3f as libc::c_uint as libc::c_ulong)
                            as libc::c_char;
                    }
                }
                34 => {
                    let fresh11 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh11 as isize) = '"' as i32 as libc::c_char;
                }
                92 => {
                    let fresh12 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh12 as isize) =
                        '\\' as i32 as libc::c_char;
                }
                47 => {
                    let fresh13 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh13 as isize) = '/' as i32 as libc::c_char;
                }
                98 => {
                    let fresh14 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh14 as isize) =
                        '\u{8}' as i32 as libc::c_char;
                }
                102 => {
                    let fresh15 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh15 as isize) =
                        '\u{c}' as i32 as libc::c_char;
                }
                110 => {
                    let fresh16 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh16 as isize) =
                        '\n' as i32 as libc::c_char;
                }
                114 => {
                    let fresh17 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh17 as isize) =
                        '\r' as i32 as libc::c_char;
                }
                116 => {
                    let fresh18 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh18 as isize) =
                        '\t' as i32 as libc::c_char;
                }
                13 => {
                    let fresh19 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh19 as isize) =
                        '\r' as i32 as libc::c_char;
                    if '\n' as i32
                        == *src.offset(offset as isize) as libc::c_int
                    {
                        let fresh20 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh20 as isize) =
                            '\n' as i32 as libc::c_char;
                        offset = offset.wrapping_add(1);
                        offset;
                    }
                }
                10 => {
                    let fresh21 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh21 as isize) =
                        '\n' as i32 as libc::c_char;
                }
                _ => return
            }
        } else {
            let fresh22 = offset;
            offset = offset.wrapping_add(1);
            let fresh23 = bytes_written;
            bytes_written = bytes_written.wrapping_add(1);
            *data.offset(fresh23 as isize) = *src.offset(fresh22 as isize);
        }
    }
    offset = offset.wrapping_add(1);
    offset;
    (*string).string_size = bytes_written;
    let fresh24 = bytes_written;
    bytes_written = bytes_written.wrapping_add(1);
    *data.offset(fresh24 as isize) = '\0' as i32 as libc::c_char;
    (*state).data = ((*state).data).offset(bytes_written as isize);
    (*state).offset = offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_key(
    mut state: *mut json_parse_state_s,
    mut string: *mut json_string_s
) {
    if json_parse_flags_allow_unquoted_keys as libc::c_int as libc::c_ulong
        & (*state).flags_bitset
        != 0
    {
        let src: *const libc::c_char = (*state).src;
        let data: *mut libc::c_char = (*state).data;
        let mut offset: size_t = (*state).offset;
        if '"' as i32 == *src.offset(offset as isize) as libc::c_int
            || '\'' as i32 == *src.offset(offset as isize) as libc::c_int
        {
            json_parse_string(state, string);
        } else {
            let mut size: size_t = 0 as libc::c_int as size_t;
            (*string).string = (*state).data;
            while is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0
            {
                let fresh25 = offset;
                offset = offset.wrapping_add(1);
                let fresh26 = size;
                size = size.wrapping_add(1);
                *data.offset(fresh26 as isize) = *src.offset(fresh25 as isize);
            }
            *data.offset(size as isize) = '\0' as i32 as libc::c_char;
            let fresh27 = size;
            size = size.wrapping_add(1);
            (*string).string_size = fresh27;
            (*state).data = ((*state).data).offset(size as isize);
            (*state).offset = offset;
        }
    } else {
        json_parse_string(state, string);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_object(
    mut state: *mut json_parse_state_s,
    mut is_global_object: libc::c_int,
    mut object: *mut json_object_s
) {
    let flags_bitset: size_t = (*state).flags_bitset;
    let size: size_t = (*state).size;
    let src: *const libc::c_char = (*state).src;
    let mut elements: size_t = 0 as libc::c_int as size_t;
    let mut allow_comma: libc::c_int = 0 as libc::c_int;
    let mut previous: *mut json_object_element_s =
        0 as *mut json_object_element_s;
    if is_global_object != 0 {
        if '{' as i32 == *src.offset((*state).offset as isize) as libc::c_int {
            is_global_object = 0 as libc::c_int;
        }
    }
    if is_global_object == 0 {
        (*state).offset = ((*state).offset).wrapping_add(1);
        (*state).offset;
    }
    json_skip_all_skippables(state);
    elements = 0 as libc::c_int as size_t;
    while (*state).offset < size {
        let mut element: *mut json_object_element_s =
            0 as *mut json_object_element_s;
        let mut string: *mut json_string_s = 0 as *mut json_string_s;
        let mut value: *mut json_value_s = 0 as *mut json_value_s;
        if is_global_object == 0 {
            json_skip_all_skippables(state);
            if '}' as i32
                == *src.offset((*state).offset as isize) as libc::c_int
            {
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                break;
            }
        } else if json_skip_all_skippables(state) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32
                == *src.offset((*state).offset as isize) as libc::c_int
            {
                (*state).offset = ((*state).offset).wrapping_add(1);
                (*state).offset;
                allow_comma = 0 as libc::c_int;
                continue;
            }
        }
        element = (*state).dom as *mut json_object_element_s;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_object_element_s>()
                as libc::c_ulong as isize);
        if previous.is_null() {
            (*object).start = element;
        } else {
            (*previous).next = element;
        }
        previous = element;
        if json_parse_flags_allow_location_information as libc::c_int
            as libc::c_ulong
            & flags_bitset
            != 0
        {
            let mut string_ex: *mut json_string_ex_s =
                (*state).dom as *mut json_string_ex_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_string_ex_s>()
                        as libc::c_ulong as isize);
            (*string_ex).offset = (*state).offset;
            (*string_ex).line_no = (*state).line_no;
            (*string_ex).row_no =
                ((*state).offset).wrapping_sub((*state).line_offset);
            string = &mut (*string_ex).string;
        } else {
            string = (*state).dom as *mut json_string_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_string_s>()
                        as libc::c_ulong as isize);
        }
        (*element).name = string;
        json_parse_key(state, string);
        json_skip_all_skippables(state);
        (*state).offset = ((*state).offset).wrapping_add(1);
        (*state).offset;
        json_skip_all_skippables(state);
        if json_parse_flags_allow_location_information as libc::c_int
            as libc::c_ulong
            & flags_bitset
            != 0
        {
            let mut value_ex: *mut json_value_ex_s =
                (*state).dom as *mut json_value_ex_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_value_ex_s>()
                        as libc::c_ulong as isize);
            (*value_ex).offset = (*state).offset;
            (*value_ex).line_no = (*state).line_no;
            (*value_ex).row_no =
                ((*state).offset).wrapping_sub((*state).line_offset);
            value = &mut (*value_ex).value;
        } else {
            value = (*state).dom as *mut json_value_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_value_s>()
                        as libc::c_ulong as isize);
        }
        (*element).value = value;
        json_parse_value(state, 0 as libc::c_int, value);
        elements = elements.wrapping_add(1);
        elements;
        allow_comma = 1 as libc::c_int;
    }
    if !previous.is_null() {
        (*previous).next = 0 as *mut json_object_element_s;
    }
    if 0 as libc::c_int as libc::c_ulong == elements {
        (*object).start = 0 as *mut json_object_element_s;
    }
    (*object).length = elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_array(
    mut state: *mut json_parse_state_s,
    mut array: *mut json_array_s
) {
    let src: *const libc::c_char = (*state).src;
    let size: size_t = (*state).size;
    let mut elements: size_t = 0 as libc::c_int as size_t;
    let mut allow_comma: libc::c_int = 0 as libc::c_int;
    let mut previous: *mut json_array_element_s =
        0 as *mut json_array_element_s;
    (*state).offset = ((*state).offset).wrapping_add(1);
    (*state).offset;
    json_skip_all_skippables(state);
    elements = 0 as libc::c_int as size_t;
    let mut current_block_28: u64;
    loop {
        let mut element: *mut json_array_element_s =
            0 as *mut json_array_element_s;
        let mut value: *mut json_value_s = 0 as *mut json_value_s;
        json_skip_all_skippables(state);
        if ']' as i32 == *src.offset((*state).offset as isize) as libc::c_int {
            (*state).offset = ((*state).offset).wrapping_add(1);
            (*state).offset;
            break;
        } else {
            if allow_comma != 0 {
                if ',' as i32
                    == *src.offset((*state).offset as isize) as libc::c_int
                {
                    (*state).offset = ((*state).offset).wrapping_add(1);
                    (*state).offset;
                    allow_comma = 0 as libc::c_int;
                    current_block_28 = 17778012151635330486;
                } else {
                    current_block_28 = 12209867499936983673;
                }
            } else {
                current_block_28 = 12209867499936983673;
            }
            match current_block_28 {
                12209867499936983673 => {
                    element = (*state).dom as *mut json_array_element_s;
                    (*state).dom = ((*state).dom)
                        .offset(::core::mem::size_of::<json_array_element_s>()
                            as libc::c_ulong
                            as isize);
                    if previous.is_null() {
                        (*array).start = element;
                    } else {
                        (*previous).next = element;
                    }
                    previous = element;
                    if json_parse_flags_allow_location_information
                        as libc::c_int as libc::c_ulong
                        & (*state).flags_bitset
                        != 0
                    {
                        let mut value_ex: *mut json_value_ex_s =
                            (*state).dom as *mut json_value_ex_s;
                        (*state).dom =
                            ((*state).dom).offset(::core::mem::size_of::<
                                json_value_ex_s
                            >(
                            )
                                as libc::c_ulong
                                as isize);
                        (*value_ex).offset = (*state).offset;
                        (*value_ex).line_no = (*state).line_no;
                        (*value_ex).row_no = ((*state).offset)
                            .wrapping_sub((*state).line_offset);
                        value = &mut (*value_ex).value;
                    } else {
                        value = (*state).dom as *mut json_value_s;
                        (*state).dom =
                            ((*state).dom).offset(::core::mem::size_of::<
                                json_value_s
                            >(
                            )
                                as libc::c_ulong
                                as isize);
                    }
                    (*element).value = value;
                    json_parse_value(state, 0 as libc::c_int, value);
                    elements = elements.wrapping_add(1);
                    elements;
                    allow_comma = 1 as libc::c_int;
                }
                _ => {}
            }
            if !((*state).offset < size) {
                break;
            }
        }
    }
    if !previous.is_null() {
        (*previous).next = 0 as *mut json_array_element_s;
    }
    if 0 as libc::c_int as libc::c_ulong == elements {
        (*array).start = 0 as *mut json_array_element_s;
    }
    (*array).length = elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_number(
    mut state: *mut json_parse_state_s,
    mut number: *mut json_number_s
) {
    let flags_bitset: size_t = (*state).flags_bitset;
    let mut offset: size_t = (*state).offset;
    let size: size_t = (*state).size;
    let mut bytes_written: size_t = 0 as libc::c_int as size_t;
    let src: *const libc::c_char = (*state).src;
    let mut data: *mut libc::c_char = (*state).data;
    (*number).number = data;
    if json_parse_flags_allow_hexadecimal_numbers as libc::c_int
        as libc::c_ulong
        & flags_bitset
        != 0
    {
        if '0' as i32 == *src.offset(offset as isize) as libc::c_int
            && ('x' as i32
                == *src.offset(
                    offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as isize
                ) as libc::c_int
                || 'X' as i32
                    == *src.offset(
                        offset.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize
                    ) as libc::c_int)
        {
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as libc::c_int
                    && *src.offset(offset as isize) as libc::c_int
                        <= '9' as i32
                    || 'a' as i32
                        <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int
                            <= 'f' as i32
                    || 'A' as i32
                        <= *src.offset(offset as isize) as libc::c_int
                        && *src.offset(offset as isize) as libc::c_int
                            <= 'F' as i32
                    || 'x' as i32
                        == *src.offset(offset as isize) as libc::c_int
                    || 'X' as i32
                        == *src.offset(offset as isize) as libc::c_int)
            {
                let fresh28 = offset;
                offset = offset.wrapping_add(1);
                let fresh29 = bytes_written;
                bytes_written = bytes_written.wrapping_add(1);
                *data.offset(fresh29 as isize) = *src.offset(fresh28 as isize);
            }
        }
    }
    while offset < size {
        let mut end: libc::c_int = 0 as libc::c_int;
        match *src.offset(offset as isize) as libc::c_int {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 | 101 | 69
            | 43 | 45 => {
                let fresh30 = offset;
                offset = offset.wrapping_add(1);
                let fresh31 = bytes_written;
                bytes_written = bytes_written.wrapping_add(1);
                *data.offset(fresh31 as isize) = *src.offset(fresh30 as isize);
            }
            _ => {
                end = 1 as libc::c_int;
            }
        }
        if 0 as libc::c_int != end {
            break;
        }
    }
    if json_parse_flags_allow_inf_and_nan as libc::c_int as libc::c_ulong
        & flags_bitset
        != 0
    {
        let inf_strlen: size_t = 8 as libc::c_int as size_t;
        let nan_strlen: size_t = 3 as libc::c_int as size_t;
        if offset.wrapping_add(inf_strlen) < size {
            if 'I' as i32 == *src.offset(offset as isize) as libc::c_int {
                let mut i: size_t = 0;
                i = 0 as libc::c_int as size_t;
                while i < inf_strlen {
                    let fresh32 = offset;
                    offset = offset.wrapping_add(1);
                    let fresh33 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh33 as isize) =
                        *src.offset(fresh32 as isize);
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
        if offset.wrapping_add(nan_strlen) < size {
            if 'N' as i32 == *src.offset(offset as isize) as libc::c_int {
                let mut i_0: size_t = 0;
                i_0 = 0 as libc::c_int as size_t;
                while i_0 < nan_strlen {
                    let fresh34 = offset;
                    offset = offset.wrapping_add(1);
                    let fresh35 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh35 as isize) =
                        *src.offset(fresh34 as isize);
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                }
            }
        }
    }
    (*number).number_size = bytes_written;
    let fresh36 = bytes_written;
    bytes_written = bytes_written.wrapping_add(1);
    *data.offset(fresh36 as isize) = '\0' as i32 as libc::c_char;
    (*state).data = ((*state).data).offset(bytes_written as isize);
    (*state).offset = offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_value(
    mut state: *mut json_parse_state_s,
    mut is_global_object: libc::c_int,
    mut value: *mut json_value_s
) {
    let flags_bitset: size_t = (*state).flags_bitset;
    let src: *const libc::c_char = (*state).src;
    let size: size_t = (*state).size;
    let mut offset: size_t = 0;
    json_skip_all_skippables(state);
    offset = (*state).offset;
    if is_global_object != 0 {
        (*value).type_0 = json_type_object as libc::c_int as size_t;
        (*value).payload = (*state).dom as *mut libc::c_void;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_object_s>() as libc::c_ulong
                as isize);
        json_parse_object(
            state,
            1 as libc::c_int,
            (*value).payload as *mut json_object_s
        );
    } else {
        match *src.offset(offset as isize) as libc::c_int {
            34 | 39 => {
                (*value).type_0 = json_type_string as libc::c_int as size_t;
                (*value).payload = (*state).dom as *mut libc::c_void;
                (*state).dom = ((*state).dom).offset(::core::mem::size_of::<
                    json_string_s
                >()
                    as libc::c_ulong
                    as isize);
                json_parse_string(
                    state,
                    (*value).payload as *mut json_string_s
                );
            }
            123 => {
                (*value).type_0 = json_type_object as libc::c_int as size_t;
                (*value).payload = (*state).dom as *mut libc::c_void;
                (*state).dom = ((*state).dom).offset(::core::mem::size_of::<
                    json_object_s
                >()
                    as libc::c_ulong
                    as isize);
                json_parse_object(
                    state,
                    0 as libc::c_int,
                    (*value).payload as *mut json_object_s
                );
            }
            91 => {
                (*value).type_0 = json_type_array as libc::c_int as size_t;
                (*value).payload = (*state).dom as *mut libc::c_void;
                (*state).dom = ((*state).dom).offset(::core::mem::size_of::<
                    json_array_s
                >()
                    as libc::c_ulong
                    as isize);
                json_parse_array(state, (*value).payload as *mut json_array_s);
            }
            45 | 43 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 => {
                (*value).type_0 = json_type_number as libc::c_int as size_t;
                (*value).payload = (*state).dom as *mut libc::c_void;
                (*state).dom = ((*state).dom).offset(::core::mem::size_of::<
                    json_number_s
                >()
                    as libc::c_ulong
                    as isize);
                json_parse_number(
                    state,
                    (*value).payload as *mut json_number_s
                );
            }
            _ => {
                if offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                    <= size
                    && 't' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'r' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'u' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'e' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*value).type_0 = json_type_true as libc::c_int as size_t;
                    (*value).payload = 0 as *mut libc::c_void;
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                } else if offset.wrapping_add(5 as libc::c_int as libc::c_ulong)
                    <= size
                    && 'f' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'a' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 's' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'e' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*value).type_0 = json_type_false as libc::c_int as size_t;
                    (*value).payload = 0 as *mut libc::c_void;
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                } else if offset.wrapping_add(4 as libc::c_int as libc::c_ulong)
                    <= size
                    && 'n' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'u' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'l' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*value).type_0 = json_type_null as libc::c_int as size_t;
                    (*value).payload = 0 as *mut libc::c_void;
                    (*state).offset = ((*state).offset as libc::c_ulong)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        as size_t
                        as size_t;
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                    && offset.wrapping_add(3 as libc::c_int as libc::c_ulong)
                        <= size
                    && 'N' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'a' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'N' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*value).type_0 = json_type_number as libc::c_int as size_t;
                    (*value).payload = (*state).dom as *mut libc::c_void;
                    (*state).dom = ((*state).dom)
                        .offset(::core::mem::size_of::<json_number_s>()
                            as libc::c_ulong
                            as isize);
                    json_parse_number(
                        state,
                        (*value).payload as *mut json_number_s
                    );
                } else if json_parse_flags_allow_inf_and_nan as libc::c_int
                    as libc::c_ulong
                    & flags_bitset
                    != 0
                    && offset.wrapping_add(8 as libc::c_int as libc::c_ulong)
                        <= size
                    && 'I' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'n' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'f' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'i' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'n' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'i' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 't' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(6 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                    && 'y' as i32
                        == *src.offset(
                            offset
                                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                                as isize
                        ) as libc::c_int
                {
                    (*value).type_0 = json_type_number as libc::c_int as size_t;
                    (*value).payload = (*state).dom as *mut libc::c_void;
                    (*state).dom = ((*state).dom)
                        .offset(::core::mem::size_of::<json_number_s>()
                            as libc::c_ulong
                            as isize);
                    json_parse_number(
                        state,
                        (*value).payload as *mut json_number_s
                    );
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_ex(
    mut src: *const libc::c_void,
    mut src_size: size_t,
    mut flags_bitset: size_t,
    mut alloc_func_ptr: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void
    >,
    mut user_data: *mut libc::c_void,
    mut result: *mut json_parse_result_s
) -> *mut json_value_s {
    let mut state: json_parse_state_s = json_parse_state_s {
        src: 0 as *const libc::c_char,
        size: 0,
        offset: 0,
        flags_bitset: 0,
        data: 0 as *mut libc::c_char,
        dom: 0 as *mut libc::c_char,
        dom_size: 0,
        data_size: 0,
        line_no: 0,
        line_offset: 0,
        error: 0
    };
    let mut allocation: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut value: *mut json_value_s = 0 as *mut json_value_s;
    let mut total_size: size_t = 0;
    let mut input_error: libc::c_int = 0;
    if !result.is_null() {
        (*result).error = json_parse_error_none as libc::c_int as size_t;
        (*result).error_offset = 0 as libc::c_int as size_t;
        (*result).error_line_no = 0 as libc::c_int as size_t;
        (*result).error_row_no = 0 as libc::c_int as size_t;
    }
    if src.is_null() {
        return 0 as *mut json_value_s;
    }
    state.src = src as *const libc::c_char;
    state.size = src_size;
    state.offset = 0 as libc::c_int as size_t;
    state.line_no = 1 as libc::c_int as size_t;
    state.line_offset = 0 as libc::c_int as size_t;
    state.error = json_parse_error_none as libc::c_int as size_t;
    state.dom_size = 0 as libc::c_int as size_t;
    state.data_size = 0 as libc::c_int as size_t;
    state.flags_bitset = flags_bitset;
    input_error = json_get_value_size(
        &mut state,
        (json_parse_flags_allow_global_object as libc::c_int as libc::c_ulong
            & state.flags_bitset) as libc::c_int
    );
    if 0 as libc::c_int == input_error {
        json_skip_all_skippables(&mut state);
        if state.offset != state.size {
            state.error = json_parse_error_unexpected_trailing_characters
                as libc::c_int as size_t;
            input_error = 1 as libc::c_int;
        }
    }
    if input_error != 0 {
        if !result.is_null() {
            (*result).error = state.error;
            (*result).error_offset = state.offset;
            (*result).error_line_no = state.line_no;
            (*result).error_row_no =
                (state.offset).wrapping_sub(state.line_offset);
        }
        return 0 as *mut json_value_s;
    }
    total_size = (state.dom_size).wrapping_add(state.data_size);
    if alloc_func_ptr.is_none() {
        allocation = malloc(total_size);
    } else {
        allocation = alloc_func_ptr.expect("non-null function pointer")(
            user_data, total_size
        );
    }
    if allocation.is_null() {
        if !result.is_null() {
            (*result).error =
                json_parse_error_allocator_failed as libc::c_int as size_t;
            (*result).error_offset = 0 as libc::c_int as size_t;
            (*result).error_line_no = 0 as libc::c_int as size_t;
            (*result).error_row_no = 0 as libc::c_int as size_t;
        }
        return 0 as *mut json_value_s;
    }
    state.offset = 0 as libc::c_int as size_t;
    state.line_no = 1 as libc::c_int as size_t;
    state.line_offset = 0 as libc::c_int as size_t;
    state.dom = allocation as *mut libc::c_char;
    state.data = (state.dom).offset(state.dom_size as isize);
    if json_parse_flags_allow_location_information as libc::c_int
        as libc::c_ulong
        & state.flags_bitset
        != 0
    {
        let mut value_ex: *mut json_value_ex_s =
            state.dom as *mut json_value_ex_s;
        state.dom = (state.dom)
            .offset(::core::mem::size_of::<json_value_ex_s>() as libc::c_ulong
                as isize);
        (*value_ex).offset = state.offset;
        (*value_ex).line_no = state.line_no;
        (*value_ex).row_no = (state.offset).wrapping_sub(state.line_offset);
        value = &mut (*value_ex).value;
    } else {
        value = state.dom as *mut json_value_s;
        state.dom = (state.dom)
            .offset(::core::mem::size_of::<json_value_s>() as libc::c_ulong
                as isize);
    }
    json_parse_value(
        &mut state,
        (json_parse_flags_allow_global_object as libc::c_int as libc::c_ulong
            & state.flags_bitset) as libc::c_int,
        value
    );
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse(
    mut src: *const libc::c_void,
    mut src_size: size_t
) -> *mut json_value_s {
    return json_parse_ex(
        src,
        src_size,
        json_parse_flags_default as libc::c_int as size_t,
        None,
        0 as *mut libc::c_void,
        0 as *mut json_parse_result_s
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value(
    mut value: *const json_value_s
) -> *mut json_value_s {
    return json_extract_value_ex(value, None, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_number_size(
    number: *const json_number_s
) -> json_extract_result_s {
    let mut result: json_extract_result_s = json_extract_result_s {
        dom_size: 0,
        data_size: 0
    };
    result.dom_size = ::core::mem::size_of::<json_number_s>() as libc::c_ulong;
    result.data_size = (*number).number_size;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_string_size(
    string: *const json_string_s
) -> json_extract_result_s {
    let mut result: json_extract_result_s = json_extract_result_s {
        dom_size: 0,
        data_size: 0
    };
    result.dom_size = ::core::mem::size_of::<json_string_s>() as libc::c_ulong;
    result.data_size =
        ((*string).string_size).wrapping_add(1 as libc::c_int as libc::c_ulong);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_object_size(
    object: *const json_object_s
) -> json_extract_result_s {
    let mut result: json_extract_result_s = json_extract_result_s {
        dom_size: 0,
        data_size: 0
    };
    let mut i: size_t = 0;
    let mut element: *const json_object_element_s = (*object).start;
    result.dom_size = (::core::mem::size_of::<json_object_s>()
        as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<json_object_element_s>() as libc::c_ulong)
                .wrapping_mul((*object).length)
        );
    result.data_size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < (*object).length {
        let string_result: json_extract_result_s =
            json_extract_get_string_size((*element).name);
        let value_result: json_extract_result_s =
            json_extract_get_value_size((*element).value);
        result.dom_size = (result.dom_size as libc::c_ulong)
            .wrapping_add(string_result.dom_size)
            as size_t as size_t;
        result.data_size = (result.data_size as libc::c_ulong)
            .wrapping_add(string_result.data_size)
            as size_t as size_t;
        result.dom_size = (result.dom_size as libc::c_ulong)
            .wrapping_add(value_result.dom_size)
            as size_t as size_t;
        result.data_size = (result.data_size as libc::c_ulong)
            .wrapping_add(value_result.data_size)
            as size_t as size_t;
        element = (*element).next;
        i = i.wrapping_add(1);
        i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_array_size(
    array: *const json_array_s
) -> json_extract_result_s {
    let mut result: json_extract_result_s = json_extract_result_s {
        dom_size: 0,
        data_size: 0
    };
    let mut i: size_t = 0;
    let mut element: *const json_array_element_s = (*array).start;
    result.dom_size = (::core::mem::size_of::<json_array_s>() as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<json_array_element_s>() as libc::c_ulong)
                .wrapping_mul((*array).length)
        );
    result.data_size = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < (*array).length {
        let value_result: json_extract_result_s =
            json_extract_get_value_size((*element).value);
        result.dom_size = (result.dom_size as libc::c_ulong)
            .wrapping_add(value_result.dom_size)
            as size_t as size_t;
        result.data_size = (result.data_size as libc::c_ulong)
            .wrapping_add(value_result.data_size)
            as size_t as size_t;
        element = (*element).next;
        i = i.wrapping_add(1);
        i;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_value_size(
    value: *const json_value_s
) -> json_extract_result_s {
    let mut result: json_extract_result_s = {
        let mut init = json_extract_result_s {
            dom_size: 0 as libc::c_int as size_t,
            data_size: 0 as libc::c_int as size_t
        };
        init
    };
    match (*value).type_0 {
        2 => {
            result = json_extract_get_object_size(
                (*value).payload as *const json_object_s
            );
        }
        3 => {
            result = json_extract_get_array_size(
                (*value).payload as *const json_array_s
            );
        }
        1 => {
            result = json_extract_get_number_size(
                (*value).payload as *const json_number_s
            );
        }
        0 => {
            result = json_extract_get_string_size(
                (*value).payload as *const json_string_s
            );
        }
        _ => {}
    }
    result.dom_size = (result.dom_size as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<json_value_s>() as libc::c_ulong)
        as size_t as size_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_copy_value(
    state: *mut json_extract_state_s,
    value: *const json_value_s
) {
    let mut string: *mut json_string_s = 0 as *mut json_string_s;
    let mut number: *mut json_number_s = 0 as *mut json_number_s;
    let mut object: *mut json_object_s = 0 as *mut json_object_s;
    let mut array: *mut json_array_s = 0 as *mut json_array_s;
    let mut new_value: *mut json_value_s = 0 as *mut json_value_s;
    memcpy(
        (*state).dom as *mut libc::c_void,
        value as *const libc::c_void,
        ::core::mem::size_of::<json_value_s>() as libc::c_ulong
    );
    new_value = (*state).dom as *mut json_value_s;
    (*state).dom =
        ((*state).dom)
            .offset(::core::mem::size_of::<json_value_s>() as libc::c_ulong
                as isize);
    (*new_value).payload = (*state).dom as *mut libc::c_void;
    if json_type_string as libc::c_int as libc::c_ulong == (*value).type_0 {
        memcpy(
            (*state).dom as *mut libc::c_void,
            (*value).payload,
            ::core::mem::size_of::<json_string_s>() as libc::c_ulong
        );
        string = (*state).dom as *mut json_string_s;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_string_s>() as libc::c_ulong
                as isize);
        memcpy(
            (*state).data as *mut libc::c_void,
            (*string).string as *const libc::c_void,
            ((*string).string_size)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        );
        (*string).string = (*state).data;
        (*state).data = ((*state).data).offset(
            ((*string).string_size)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                as isize
        );
    } else if json_type_number as libc::c_int as libc::c_ulong
        == (*value).type_0
    {
        memcpy(
            (*state).dom as *mut libc::c_void,
            (*value).payload,
            ::core::mem::size_of::<json_number_s>() as libc::c_ulong
        );
        number = (*state).dom as *mut json_number_s;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_number_s>() as libc::c_ulong
                as isize);
        memcpy(
            (*state).data as *mut libc::c_void,
            (*number).number as *const libc::c_void,
            (*number).number_size
        );
        (*number).number = (*state).data;
        (*state).data = ((*state).data).offset((*number).number_size as isize);
    } else if json_type_object as libc::c_int as libc::c_ulong
        == (*value).type_0
    {
        let mut element: *mut json_object_element_s =
            0 as *mut json_object_element_s;
        let mut i: size_t = 0;
        memcpy(
            (*state).dom as *mut libc::c_void,
            (*value).payload,
            ::core::mem::size_of::<json_object_s>() as libc::c_ulong
        );
        object = (*state).dom as *mut json_object_s;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_object_s>() as libc::c_ulong
                as isize);
        element = (*object).start;
        (*object).start = (*state).dom as *mut json_object_element_s;
        i = 0 as libc::c_int as size_t;
        while i < (*object).length {
            let mut previous_value: *mut json_value_s = 0 as *mut json_value_s;
            let mut previous_element: *mut json_object_element_s =
                0 as *mut json_object_element_s;
            memcpy(
                (*state).dom as *mut libc::c_void,
                element as *const libc::c_void,
                ::core::mem::size_of::<json_object_element_s>()
                    as libc::c_ulong
            );
            element = (*state).dom as *mut json_object_element_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_object_element_s>()
                        as libc::c_ulong as isize);
            string = (*element).name;
            memcpy(
                (*state).dom as *mut libc::c_void,
                string as *const libc::c_void,
                ::core::mem::size_of::<json_string_s>() as libc::c_ulong
            );
            string = (*state).dom as *mut json_string_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_string_s>()
                        as libc::c_ulong as isize);
            (*element).name = string;
            memcpy(
                (*state).data as *mut libc::c_void,
                (*string).string as *const libc::c_void,
                ((*string).string_size)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            );
            (*string).string = (*state).data;
            (*state).data = ((*state).data).offset(
                ((*string).string_size)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as isize
            );
            previous_value = (*element).value;
            (*element).value = (*state).dom as *mut json_value_s;
            json_extract_copy_value(state, previous_value);
            previous_element = element;
            element = (*element).next;
            if !element.is_null() {
                (*previous_element).next =
                    (*state).dom as *mut json_object_element_s;
            }
            i = i.wrapping_add(1);
            i;
        }
    } else if json_type_array as libc::c_int as libc::c_ulong == (*value).type_0
    {
        let mut element_0: *mut json_array_element_s =
            0 as *mut json_array_element_s;
        let mut i_0: size_t = 0;
        memcpy(
            (*state).dom as *mut libc::c_void,
            (*value).payload,
            ::core::mem::size_of::<json_array_s>() as libc::c_ulong
        );
        array = (*state).dom as *mut json_array_s;
        (*state).dom = ((*state).dom)
            .offset(::core::mem::size_of::<json_array_s>() as libc::c_ulong
                as isize);
        element_0 = (*array).start;
        (*array).start = (*state).dom as *mut json_array_element_s;
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < (*array).length {
            let mut previous_value_0: *mut json_value_s =
                0 as *mut json_value_s;
            let mut previous_element_0: *mut json_array_element_s =
                0 as *mut json_array_element_s;
            memcpy(
                (*state).dom as *mut libc::c_void,
                element_0 as *const libc::c_void,
                ::core::mem::size_of::<json_array_element_s>() as libc::c_ulong
            );
            element_0 = (*state).dom as *mut json_array_element_s;
            (*state).dom =
                ((*state).dom)
                    .offset(::core::mem::size_of::<json_array_element_s>()
                        as libc::c_ulong as isize);
            previous_value_0 = (*element_0).value;
            (*element_0).value = (*state).dom as *mut json_value_s;
            json_extract_copy_value(state, previous_value_0);
            previous_element_0 = element_0;
            element_0 = (*element_0).next;
            if !element_0.is_null() {
                (*previous_element_0).next =
                    (*state).dom as *mut json_array_element_s;
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value_ex(
    mut value: *const json_value_s,
    mut alloc_func_ptr: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void
    >,
    mut user_data: *mut libc::c_void
) -> *mut json_value_s {
    let mut allocation: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut result: json_extract_result_s = json_extract_result_s {
        dom_size: 0,
        data_size: 0
    };
    let mut state: json_extract_state_s = json_extract_state_s {
        dom: 0 as *mut libc::c_char,
        data: 0 as *mut libc::c_char
    };
    let mut total_size: size_t = 0;
    if value.is_null() {
        return 0 as *mut json_value_s;
    }
    result = json_extract_get_value_size(value);
    total_size = (result.dom_size).wrapping_add(result.data_size);
    if alloc_func_ptr.is_none() {
        allocation = malloc(total_size);
    } else {
        allocation = alloc_func_ptr.expect("non-null function pointer")(
            user_data, total_size
        );
    }
    state.dom = allocation as *mut libc::c_char;
    state.data = (state.dom).offset(result.dom_size as isize);
    json_extract_copy_value(&mut state, value);
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_string(
    value: *mut json_value_s
) -> *mut json_string_s {
    if (*value).type_0 != json_type_string as libc::c_int as libc::c_ulong {
        return 0 as *mut json_string_s;
    }
    return (*value).payload as *mut json_string_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_number(
    value: *mut json_value_s
) -> *mut json_number_s {
    if (*value).type_0 != json_type_number as libc::c_int as libc::c_ulong {
        return 0 as *mut json_number_s;
    }
    return (*value).payload as *mut json_number_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_object(
    value: *mut json_value_s
) -> *mut json_object_s {
    if (*value).type_0 != json_type_object as libc::c_int as libc::c_ulong {
        return 0 as *mut json_object_s;
    }
    return (*value).payload as *mut json_object_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_array(
    value: *mut json_value_s
) -> *mut json_array_s {
    if (*value).type_0 != json_type_array as libc::c_int as libc::c_ulong {
        return 0 as *mut json_array_s;
    }
    return (*value).payload as *mut json_array_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_true(
    value: *const json_value_s
) -> libc::c_int {
    return ((*value).type_0 == json_type_true as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_false(
    value: *const json_value_s
) -> libc::c_int {
    return ((*value).type_0 == json_type_false as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_null(
    value: *const json_value_s
) -> libc::c_int {
    return ((*value).type_0 == json_type_null as libc::c_int as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_number_size(
    mut number: *const json_number_s,
    mut size: *mut size_t
) -> libc::c_int {
    let mut parsed_number: uintmax_t = 0;
    let mut i: size_t = 0;
    if (*number).number_size >= 2 as libc::c_int as libc::c_ulong {
        match *((*number).number).offset(1 as libc::c_int as isize)
            as libc::c_int
        {
            120 | 88 => {
                parsed_number = strtoumax(
                    (*number).number,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int
                );
                i = 0 as libc::c_int as size_t;
                while 0 as libc::c_int as libc::c_ulong != parsed_number {
                    parsed_number = (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong)
                        as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_add(1);
                    i;
                }
                *size = (*size as libc::c_ulong).wrapping_add(i) as size_t
                    as size_t;
                return 0 as libc::c_int;
            }
            _ => {}
        }
    }
    i = 0 as libc::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *((*number).number).offset(i as isize) as libc::c_int
            || '-' as i32
                == *((*number).number).offset(i as isize) as libc::c_int)
    {
        i = i.wrapping_add(1);
        i;
    }
    if i < (*number).number_size
        && 'I' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        let mut inf: *const libc::c_char =
            b"Infinity\0" as *const u8 as *const libc::c_char;
        let mut k: size_t = 0;
        k = i;
        while k < (*number).number_size {
            let fresh37 = inf;
            inf = inf.offset(1);
            let c: libc::c_char = *fresh37;
            if '\0' as i32 == c as libc::c_int {
                break;
            }
            if c as libc::c_int
                != *((*number).number).offset(k as isize) as libc::c_int
            {
                break;
            }
            k = k.wrapping_add(1);
            k;
        }
        if '\0' as i32 == *inf as libc::c_int {
            *size = (*size as libc::c_ulong)
                .wrapping_add(22 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            if '-' as i32
                == *((*number).number).offset(0 as libc::c_int as isize)
                    as libc::c_int
            {
                *size = (*size as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        return 0 as libc::c_int;
    }
    if i < (*number).number_size
        && 'N' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        let mut nan: *const libc::c_char =
            b"NaN\0" as *const u8 as *const libc::c_char;
        let mut k_0: size_t = 0;
        k_0 = i;
        while k_0 < (*number).number_size {
            let fresh38 = nan;
            nan = nan.offset(1);
            let c_0: libc::c_char = *fresh38;
            if '\0' as i32 == c_0 as libc::c_int {
                break;
            }
            if c_0 as libc::c_int
                != *((*number).number).offset(k_0 as isize) as libc::c_int
            {
                break;
            }
            k_0 = k_0.wrapping_add(1);
            k_0;
        }
        if '\0' as i32 == *nan as libc::c_int {
            *size = (*size as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        *size = (*size as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    } else {
        while i < (*number).number_size {
            let c_1: libc::c_char = *((*number).number).offset(i as isize);
            if !('0' as i32 <= c_1 as libc::c_int
                && c_1 as libc::c_int <= '9' as i32)
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i.wrapping_add(1 as libc::c_int as libc::c_ulong)
            == (*number).number_size
            && '.' as i32
                == *((*number).number).offset(i as isize) as libc::c_int
        {
            *size = (*size as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
    }
    *size = (*size as libc::c_ulong).wrapping_add((*number).number_size)
        as size_t as size_t;
    if '+' as i32
        == *((*number).number).offset(0 as libc::c_int as isize) as libc::c_int
    {
        *size = (*size as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_string_size(
    mut string: *const json_string_s,
    mut size: *mut size_t
) -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*string).string_size {
        match *((*string).string).offset(i as isize) as libc::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                *size = (*size as libc::c_ulong)
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
            _ => {
                *size = (*size as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *size = (*size as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_array_size(
    mut array: *const json_array_s,
    mut size: *mut size_t
) -> libc::c_int {
    let mut element: *mut json_array_element_s = 0 as *mut json_array_element_s;
    *size = (*size as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    if (1 as libc::c_int as libc::c_ulong) < (*array).length {
        *size = (*size as libc::c_ulong).wrapping_add(
            ((*array).length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        ) as size_t as size_t;
    }
    element = (*array).start;
    while !element.is_null() {
        if json_write_minified_get_value_size((*element).value, size) != 0 {
            return 1 as libc::c_int;
        }
        element = (*element).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_object_size(
    mut object: *const json_object_s,
    mut size: *mut size_t
) -> libc::c_int {
    let mut element: *mut json_object_element_s =
        0 as *mut json_object_element_s;
    *size = (*size as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    *size = (*size as libc::c_ulong).wrapping_add((*object).length) as size_t
        as size_t;
    if (1 as libc::c_int as libc::c_ulong) < (*object).length {
        *size = (*size as libc::c_ulong).wrapping_add(
            ((*object).length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        ) as size_t as size_t;
    }
    element = (*object).start;
    while !element.is_null() {
        if json_write_get_string_size((*element).name, size) != 0 {
            return 1 as libc::c_int;
        }
        if json_write_minified_get_value_size((*element).value, size) != 0 {
            return 1 as libc::c_int;
        }
        element = (*element).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_value_size(
    mut value: *const json_value_s,
    mut size: *mut size_t
) -> libc::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size
            );
        }
        3 => {
            return json_write_minified_get_array_size(
                (*value).payload as *mut json_array_s,
                size
            );
        }
        2 => {
            return json_write_minified_get_object_size(
                (*value).payload as *mut json_object_s,
                size
            );
        }
        4 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        5 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        6 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        _ => return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_number(
    mut number: *const json_number_s,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut parsed_number: uintmax_t = 0;
    let mut backup: uintmax_t = 0;
    let mut i: size_t = 0;
    if (*number).number_size >= 2 as libc::c_int as libc::c_ulong {
        match *((*number).number).offset(1 as libc::c_int as isize)
            as libc::c_int
        {
            120 | 88 => {
                parsed_number = strtoumax(
                    (*number).number,
                    0 as *mut *mut libc::c_char,
                    0 as libc::c_int
                );
                backup = parsed_number;
                i = 0 as libc::c_int as size_t;
                while 0 as libc::c_int as libc::c_ulong != parsed_number {
                    parsed_number = (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong)
                        as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_add(1);
                    i;
                }
                parsed_number = backup;
                backup = i;
                loop {
                    *data
                        .offset(i as isize)
                        .offset(-(1 as libc::c_int as isize)) = ('0' as i32
                        + parsed_number
                            .wrapping_rem(10 as libc::c_int as libc::c_ulong)
                            as libc::c_char
                            as libc::c_int)
                        as libc::c_char;
                    parsed_number = (parsed_number as libc::c_ulong)
                        .wrapping_div(10 as libc::c_int as libc::c_ulong)
                        as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_sub(1);
                    i;
                    if !(0 as libc::c_int as libc::c_ulong != parsed_number) {
                        break;
                    }
                }
                data = data.offset(backup as isize);
                return data;
            }
            _ => {}
        }
    }
    i = 0 as libc::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *((*number).number).offset(i as isize) as libc::c_int
            || '-' as i32
                == *((*number).number).offset(i as isize) as libc::c_int)
    {
        i = i.wrapping_add(1);
        i;
    }
    if i < (*number).number_size
        && 'I' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        let mut inf: *const libc::c_char =
            b"Infinity\0" as *const u8 as *const libc::c_char;
        let mut k: size_t = 0;
        k = i;
        while k < (*number).number_size {
            let fresh39 = inf;
            inf = inf.offset(1);
            let c: libc::c_char = *fresh39;
            if '\0' as i32 == c as libc::c_int {
                break;
            }
            if c as libc::c_int
                != *((*number).number).offset(k as isize) as libc::c_int
            {
                break;
            }
            k = k.wrapping_add(1);
            k;
        }
        let fresh40 = inf;
        inf = inf.offset(1);
        if '\0' as i32 == *fresh40 as libc::c_int {
            let mut dbl_max: *const libc::c_char = 0 as *const libc::c_char;
            if '-' as i32
                == *((*number).number).offset(0 as libc::c_int as isize)
                    as libc::c_int
            {
                let fresh41 = data;
                data = data.offset(1);
                *fresh41 = '-' as i32 as libc::c_char;
            }
            dbl_max =
                b"1.7976931348623158e308\0" as *const u8 as *const libc::c_char;
            while '\0' as i32 != *dbl_max as libc::c_int {
                let fresh42 = data;
                data = data.offset(1);
                *fresh42 = *dbl_max;
                dbl_max = dbl_max.offset(1);
                dbl_max;
            }
            return data;
        }
    }
    if i < (*number).number_size
        && 'N' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        let mut nan: *const libc::c_char =
            b"NaN\0" as *const u8 as *const libc::c_char;
        let mut k_0: size_t = 0;
        k_0 = i;
        while k_0 < (*number).number_size {
            let fresh43 = nan;
            nan = nan.offset(1);
            let c_0: libc::c_char = *fresh43;
            if '\0' as i32 == c_0 as libc::c_int {
                break;
            }
            if c_0 as libc::c_int
                != *((*number).number).offset(k_0 as isize) as libc::c_int
            {
                break;
            }
            k_0 = k_0.wrapping_add(1);
            k_0;
        }
        let fresh44 = nan;
        nan = nan.offset(1);
        if '\0' as i32 == *fresh44 as libc::c_int {
            let fresh45 = data;
            data = data.offset(1);
            *fresh45 = '0' as i32 as libc::c_char;
            return data;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        i = 0 as libc::c_int as size_t;
        if '+' as i32 == *((*number).number).offset(i as isize) as libc::c_int {
            i = i.wrapping_add(1);
            i;
        }
        if '-' as i32 == *((*number).number).offset(i as isize) as libc::c_int {
            let fresh46 = data;
            data = data.offset(1);
            *fresh46 = '-' as i32 as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        let fresh47 = data;
        data = data.offset(1);
        *fresh47 = '0' as i32 as libc::c_char;
        while i < (*number).number_size {
            let fresh48 = data;
            data = data.offset(1);
            *fresh48 = *((*number).number).offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        return data;
    }
    while i < (*number).number_size {
        let c_1: libc::c_char = *((*number).number).offset(i as isize);
        if !('0' as i32 <= c_1 as libc::c_int
            && c_1 as libc::c_int <= '9' as i32)
        {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i.wrapping_add(1 as libc::c_int as libc::c_ulong)
        == (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as libc::c_int
    {
        i = 0 as libc::c_int as size_t;
        if '+' as i32 == *((*number).number).offset(i as isize) as libc::c_int {
            i = i.wrapping_add(1);
            i;
        }
        if '-' as i32 == *((*number).number).offset(i as isize) as libc::c_int {
            let fresh49 = data;
            data = data.offset(1);
            *fresh49 = '-' as i32 as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        while i < (*number).number_size {
            let fresh50 = data;
            data = data.offset(1);
            *fresh50 = *((*number).number).offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        let fresh51 = data;
        data = data.offset(1);
        *fresh51 = '0' as i32 as libc::c_char;
        return data;
    }
    i = 0 as libc::c_int as size_t;
    if '+' as i32 == *((*number).number).offset(i as isize) as libc::c_int {
        i = i.wrapping_add(1);
        i;
    }
    while i < (*number).number_size {
        let fresh52 = data;
        data = data.offset(1);
        *fresh52 = *((*number).number).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_string(
    mut string: *const json_string_s,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let fresh53 = data;
    data = data.offset(1);
    *fresh53 = '"' as i32 as libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < (*string).string_size {
        match *((*string).string).offset(i as isize) as libc::c_int {
            34 => {
                let fresh54 = data;
                data = data.offset(1);
                *fresh54 = '\\' as i32 as libc::c_char;
                let fresh55 = data;
                data = data.offset(1);
                *fresh55 = '"' as i32 as libc::c_char;
            }
            92 => {
                let fresh56 = data;
                data = data.offset(1);
                *fresh56 = '\\' as i32 as libc::c_char;
                let fresh57 = data;
                data = data.offset(1);
                *fresh57 = '\\' as i32 as libc::c_char;
            }
            8 => {
                let fresh58 = data;
                data = data.offset(1);
                *fresh58 = '\\' as i32 as libc::c_char;
                let fresh59 = data;
                data = data.offset(1);
                *fresh59 = 'b' as i32 as libc::c_char;
            }
            12 => {
                let fresh60 = data;
                data = data.offset(1);
                *fresh60 = '\\' as i32 as libc::c_char;
                let fresh61 = data;
                data = data.offset(1);
                *fresh61 = 'f' as i32 as libc::c_char;
            }
            10 => {
                let fresh62 = data;
                data = data.offset(1);
                *fresh62 = '\\' as i32 as libc::c_char;
                let fresh63 = data;
                data = data.offset(1);
                *fresh63 = 'n' as i32 as libc::c_char;
            }
            13 => {
                let fresh64 = data;
                data = data.offset(1);
                *fresh64 = '\\' as i32 as libc::c_char;
                let fresh65 = data;
                data = data.offset(1);
                *fresh65 = 'r' as i32 as libc::c_char;
            }
            9 => {
                let fresh66 = data;
                data = data.offset(1);
                *fresh66 = '\\' as i32 as libc::c_char;
                let fresh67 = data;
                data = data.offset(1);
                *fresh67 = 't' as i32 as libc::c_char;
            }
            _ => {
                let fresh68 = data;
                data = data.offset(1);
                *fresh68 = *((*string).string).offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let fresh69 = data;
    data = data.offset(1);
    *fresh69 = '"' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_array(
    mut array: *const json_array_s,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut element: *mut json_array_element_s = 0 as *mut json_array_element_s;
    let fresh70 = data;
    data = data.offset(1);
    *fresh70 = '[' as i32 as libc::c_char;
    element = (*array).start;
    while !element.is_null() {
        if element != (*array).start {
            let fresh71 = data;
            data = data.offset(1);
            *fresh71 = ',' as i32 as libc::c_char;
        }
        data = json_write_minified_value((*element).value, data);
        if data.is_null() {
            return 0 as *mut libc::c_char;
        }
        element = (*element).next;
    }
    let fresh72 = data;
    data = data.offset(1);
    *fresh72 = ']' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_object(
    mut object: *const json_object_s,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut element: *mut json_object_element_s =
        0 as *mut json_object_element_s;
    let fresh73 = data;
    data = data.offset(1);
    *fresh73 = '{' as i32 as libc::c_char;
    element = (*object).start;
    while !element.is_null() {
        if element != (*object).start {
            let fresh74 = data;
            data = data.offset(1);
            *fresh74 = ',' as i32 as libc::c_char;
        }
        data = json_write_string((*element).name, data);
        if data.is_null() {
            return 0 as *mut libc::c_char;
        }
        let fresh75 = data;
        data = data.offset(1);
        *fresh75 = ':' as i32 as libc::c_char;
        data = json_write_minified_value((*element).value, data);
        if data.is_null() {
            return 0 as *mut libc::c_char;
        }
        element = (*element).next;
    }
    let fresh76 = data;
    data = data.offset(1);
    *fresh76 = '}' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_value(
    mut value: *const json_value_s,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    match (*value).type_0 {
        1 => {
            return json_write_number(
                (*value).payload as *mut json_number_s,
                data
            )
        }
        0 => {
            return json_write_string(
                (*value).payload as *mut json_string_s,
                data
            )
        }
        3 => {
            return json_write_minified_array(
                (*value).payload as *mut json_array_s,
                data
            );
        }
        2 => {
            return json_write_minified_object(
                (*value).payload as *mut json_object_s,
                data
            );
        }
        4 => {
            *data.offset(0 as libc::c_int as isize) =
                't' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'r' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'u' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                'e' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        5 => {
            *data.offset(0 as libc::c_int as isize) =
                'f' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'a' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                's' as i32 as libc::c_char;
            *data.offset(4 as libc::c_int as isize) =
                'e' as i32 as libc::c_char;
            return data.offset(5 as libc::c_int as isize);
        }
        6 => {
            *data.offset(0 as libc::c_int as isize) =
                'n' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'u' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        _ => return 0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified(
    mut value: *const json_value_s,
    mut out_size: *mut size_t
) -> *mut libc::c_void {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_end: *mut libc::c_char = 0 as *mut libc::c_char;
    if value.is_null() {
        return 0 as *mut libc::c_void;
    }
    if json_write_minified_get_value_size(value, &mut size) != 0 {
        return 0 as *mut libc::c_void;
    }
    size = (size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    data = malloc(size) as *mut libc::c_char;
    if data.is_null() {
        return 0 as *mut libc::c_void;
    }
    data_end = json_write_minified_value(value, data);
    if data_end.is_null() {
        free(data as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    *data_end = '\0' as i32 as libc::c_char;
    if !out_size.is_null() {
        *out_size = size;
    }
    return data as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_array_size(
    mut array: *const json_array_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: *mut size_t
) -> libc::c_int {
    let mut element: *mut json_array_element_s = 0 as *mut json_array_element_s;
    *size = (*size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    if (0 as libc::c_int as libc::c_ulong) < (*array).length {
        *size = (*size as libc::c_ulong).wrapping_add(newline_size) as size_t
            as size_t;
        *size = (*size as libc::c_ulong).wrapping_add(
            ((*array).length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        ) as size_t as size_t;
        element = (*array).start;
        while !element.is_null() {
            *size = (*size as libc::c_ulong).wrapping_add(
                depth
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(indent_size)
            ) as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent_size,
                newline_size,
                size
            ) != 0
            {
                return 1 as libc::c_int;
            }
            *size = (*size as libc::c_ulong).wrapping_add(newline_size)
                as size_t as size_t;
            element = (*element).next;
        }
        *size = (*size as libc::c_ulong)
            .wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size = (*size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_object_size(
    mut object: *const json_object_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: *mut size_t
) -> libc::c_int {
    let mut element: *mut json_object_element_s =
        0 as *mut json_object_element_s;
    *size = (*size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    if (0 as libc::c_int as libc::c_ulong) < (*object).length {
        *size = (*size as libc::c_ulong).wrapping_add(newline_size) as size_t
            as size_t;
        *size = (*size as libc::c_ulong).wrapping_add(
            ((*object).length).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        ) as size_t as size_t;
        element = (*object).start;
        while !element.is_null() {
            *size = (*size as libc::c_ulong).wrapping_add(
                depth
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(indent_size)
            ) as size_t as size_t;
            *size = (*size as libc::c_ulong).wrapping_add(newline_size)
                as size_t as size_t;
            if json_write_get_string_size((*element).name, size) != 0 {
                return 1 as libc::c_int;
            }
            *size = (*size as libc::c_ulong)
                .wrapping_add(3 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent_size,
                newline_size,
                size
            ) != 0
            {
                return 1 as libc::c_int;
            }
            element = (*element).next;
        }
        *size = (*size as libc::c_ulong)
            .wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size = (*size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_value_size(
    mut value: *const json_value_s,
    mut depth: size_t,
    mut indent_size: size_t,
    mut newline_size: size_t,
    mut size: *mut size_t
) -> libc::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size
            );
        }
        3 => {
            return json_write_pretty_get_array_size(
                (*value).payload as *mut json_array_s,
                depth,
                indent_size,
                newline_size,
                size
            );
        }
        2 => {
            return json_write_pretty_get_object_size(
                (*value).payload as *mut json_object_s,
                depth,
                indent_size,
                newline_size,
                size
            );
        }
        4 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        5 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(5 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        6 => {
            *size = (*size as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
            return 0 as libc::c_int;
        }
        _ => return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_array(
    mut array: *const json_array_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    let mut element: *mut json_array_element_s = 0 as *mut json_array_element_s;
    let fresh77 = data;
    data = data.offset(1);
    *fresh77 = '[' as i32 as libc::c_char;
    if (0 as libc::c_int as libc::c_ulong) < (*array).length {
        k = 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh78 = data;
            data = data.offset(1);
            *fresh78 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        element = (*array).start;
        while !element.is_null() {
            if element != (*array).start {
                let fresh79 = data;
                data = data.offset(1);
                *fresh79 = ',' as i32 as libc::c_char;
                k = 0 as libc::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as libc::c_int
                {
                    let fresh80 = data;
                    data = data.offset(1);
                    *fresh80 = *newline.offset(k as isize);
                    k = k.wrapping_add(1);
                    k;
                }
            }
            k = 0 as libc::c_int as size_t;
            while k < depth.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                m = 0 as libc::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                    let fresh81 = data;
                    data = data.offset(1);
                    *fresh81 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                    m;
                }
                k = k.wrapping_add(1);
                k;
            }
            data = json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent,
                newline,
                data
            );
            if data.is_null() {
                return 0 as *mut libc::c_char;
            }
            element = (*element).next;
        }
        k = 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh82 = data;
            data = data.offset(1);
            *fresh82 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        k = 0 as libc::c_int as size_t;
        while k < depth {
            m = 0 as libc::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                let fresh83 = data;
                data = data.offset(1);
                *fresh83 = *indent.offset(m as isize);
                m = m.wrapping_add(1);
                m;
            }
            k = k.wrapping_add(1);
            k;
        }
    }
    let fresh84 = data;
    data = data.offset(1);
    *fresh84 = ']' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_object(
    mut object: *const json_object_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    let mut k: size_t = 0;
    let mut m: size_t = 0;
    let mut element: *mut json_object_element_s =
        0 as *mut json_object_element_s;
    let fresh85 = data;
    data = data.offset(1);
    *fresh85 = '{' as i32 as libc::c_char;
    if (0 as libc::c_int as libc::c_ulong) < (*object).length {
        k = 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh86 = data;
            data = data.offset(1);
            *fresh86 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        element = (*object).start;
        while !element.is_null() {
            if element != (*object).start {
                let fresh87 = data;
                data = data.offset(1);
                *fresh87 = ',' as i32 as libc::c_char;
                k = 0 as libc::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as libc::c_int
                {
                    let fresh88 = data;
                    data = data.offset(1);
                    *fresh88 = *newline.offset(k as isize);
                    k = k.wrapping_add(1);
                    k;
                }
            }
            k = 0 as libc::c_int as size_t;
            while k < depth.wrapping_add(1 as libc::c_int as libc::c_ulong) {
                m = 0 as libc::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                    let fresh89 = data;
                    data = data.offset(1);
                    *fresh89 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                    m;
                }
                k = k.wrapping_add(1);
                k;
            }
            data = json_write_string((*element).name, data);
            if data.is_null() {
                return 0 as *mut libc::c_char;
            }
            let fresh90 = data;
            data = data.offset(1);
            *fresh90 = ' ' as i32 as libc::c_char;
            let fresh91 = data;
            data = data.offset(1);
            *fresh91 = ':' as i32 as libc::c_char;
            let fresh92 = data;
            data = data.offset(1);
            *fresh92 = ' ' as i32 as libc::c_char;
            data = json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as libc::c_int as libc::c_ulong),
                indent,
                newline,
                data
            );
            if data.is_null() {
                return 0 as *mut libc::c_char;
            }
            element = (*element).next;
        }
        k = 0 as libc::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as libc::c_int {
            let fresh93 = data;
            data = data.offset(1);
            *fresh93 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
            k;
        }
        k = 0 as libc::c_int as size_t;
        while k < depth {
            m = 0 as libc::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as libc::c_int {
                let fresh94 = data;
                data = data.offset(1);
                *fresh94 = *indent.offset(m as isize);
                m = m.wrapping_add(1);
                m;
            }
            k = k.wrapping_add(1);
            k;
        }
    }
    let fresh95 = data;
    data = data.offset(1);
    *fresh95 = '}' as i32 as libc::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_value(
    mut value: *const json_value_s,
    mut depth: size_t,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut data: *mut libc::c_char
) -> *mut libc::c_char {
    match (*value).type_0 {
        1 => {
            return json_write_number(
                (*value).payload as *mut json_number_s,
                data
            )
        }
        0 => {
            return json_write_string(
                (*value).payload as *mut json_string_s,
                data
            )
        }
        3 => {
            return json_write_pretty_array(
                (*value).payload as *mut json_array_s,
                depth,
                indent,
                newline,
                data
            );
        }
        2 => {
            return json_write_pretty_object(
                (*value).payload as *mut json_object_s,
                depth,
                indent,
                newline,
                data
            );
        }
        4 => {
            *data.offset(0 as libc::c_int as isize) =
                't' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'r' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'u' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                'e' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        5 => {
            *data.offset(0 as libc::c_int as isize) =
                'f' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'a' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                's' as i32 as libc::c_char;
            *data.offset(4 as libc::c_int as isize) =
                'e' as i32 as libc::c_char;
            return data.offset(5 as libc::c_int as isize);
        }
        6 => {
            *data.offset(0 as libc::c_int as isize) =
                'n' as i32 as libc::c_char;
            *data.offset(1 as libc::c_int as isize) =
                'u' as i32 as libc::c_char;
            *data.offset(2 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            *data.offset(3 as libc::c_int as isize) =
                'l' as i32 as libc::c_char;
            return data.offset(4 as libc::c_int as isize);
        }
        _ => return 0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty(
    mut value: *const json_value_s,
    mut indent: *const libc::c_char,
    mut newline: *const libc::c_char,
    mut out_size: *mut size_t
) -> *mut libc::c_void {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut indent_size: size_t = 0 as libc::c_int as size_t;
    let mut newline_size: size_t = 0 as libc::c_int as size_t;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_end: *mut libc::c_char = 0 as *mut libc::c_char;
    if value.is_null() {
        return 0 as *mut libc::c_void;
    }
    if indent.is_null() {
        indent = b"  \0" as *const u8 as *const libc::c_char;
    }
    if newline.is_null() {
        newline = b"\n\0" as *const u8 as *const libc::c_char;
    }
    while '\0' as i32 != *indent.offset(indent_size as isize) as libc::c_int {
        indent_size = indent_size.wrapping_add(1);
        indent_size;
    }
    while '\0' as i32 != *newline.offset(newline_size as isize) as libc::c_int {
        newline_size = newline_size.wrapping_add(1);
        newline_size;
    }
    if json_write_pretty_get_value_size(
        value,
        0 as libc::c_int as size_t,
        indent_size,
        newline_size,
        &mut size
    ) != 0
    {
        return 0 as *mut libc::c_void;
    }
    size = (size as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t
        as size_t;
    data = malloc(size) as *mut libc::c_char;
    if data.is_null() {
        return 0 as *mut libc::c_void;
    }
    data_end = json_write_pretty_value(
        value,
        0 as libc::c_int as size_t,
        indent,
        newline,
        data
    );
    if data_end.is_null() {
        free(data as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    *data_end = '\0' as i32 as libc::c_char;
    if !out_size.is_null() {
        *out_size = size;
    }
    return data as *mut libc::c_void;
}
