
extern "C" {
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: std::os::raw::c_ulong,
    ) -> * mut core::ffi::c_void;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(__ptr: * mut core::ffi::c_void);
    fn strtoumax(
        __nptr: * const std::os::raw::c_char,
        __endptr: * mut * mut std::os::raw::c_char,
        __base: std::os::raw::c_int,
    ) -> std::os::raw::c_ulong;
}
pub type size_t = std::os::raw::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_s {
    pub payload: * mut core::ffi::c_void,
    pub type_0: std::os::raw::c_ulong,
}
impl json_value_s {
    pub const fn new() -> Self {
        json_value_s {
        payload: (0 as * mut core::ffi::c_void),
        type_0: 0
        }
    }
}

impl std::default::Default for json_value_s {
    fn default() -> Self { json_value_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_result_s {
    pub error: std::os::raw::c_ulong,
    pub error_offset: std::os::raw::c_ulong,
    pub error_line_no: std::os::raw::c_ulong,
    pub error_row_no: std::os::raw::c_ulong,
}
impl json_parse_result_s {
    pub const fn new() -> Self {
        json_parse_result_s {
        error: 0,
        error_offset: 0,
        error_line_no: 0,
        error_row_no: 0
        }
    }
}

impl std::default::Default for json_parse_result_s {
    fn default() -> Self { json_parse_result_s::new() }
}

pub type json_parse_flags_e = std::os::raw::c_uint;
pub const json_parse_flags_allow_json5: json_parse_flags_e = 16163;
pub const json_parse_flags_allow_simplified_json: json_parse_flags_e = 31;
pub const json_parse_flags_allow_multi_line_strings: json_parse_flags_e = 8192;
pub const json_parse_flags_allow_inf_and_nan: json_parse_flags_e = 4096;
pub const json_parse_flags_allow_leading_or_trailing_decimal_point: json_parse_flags_e = 2048;
pub const json_parse_flags_allow_leading_plus_sign: json_parse_flags_e = 1024;
pub const json_parse_flags_allow_hexadecimal_numbers: json_parse_flags_e = 512;
pub const json_parse_flags_allow_single_quoted_strings: json_parse_flags_e = 256;
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
    pub src: * const std::os::raw::c_char,
    pub size: std::os::raw::c_ulong,
    pub offset: std::os::raw::c_ulong,
    pub flags_bitset: std::os::raw::c_ulong,
    pub data: * mut std::os::raw::c_char,
    pub dom: * mut std::os::raw::c_char,
    pub dom_size: std::os::raw::c_ulong,
    pub data_size: std::os::raw::c_ulong,
    pub line_no: std::os::raw::c_ulong,
    pub line_offset: std::os::raw::c_ulong,
    pub error: std::os::raw::c_ulong,
}
impl json_parse_state_s {
    pub const fn new() -> Self {
        json_parse_state_s {
        src: (0 as * const std::os::raw::c_char),
        size: 0,
        offset: 0,
        flags_bitset: 0,
        data: (0 as * mut std::os::raw::c_char),
        dom: (0 as * mut std::os::raw::c_char),
        dom_size: 0,
        data_size: 0,
        line_no: 0,
        line_offset: 0,
        error: 0
        }
    }
}

impl std::default::Default for json_parse_state_s {
    fn default() -> Self { json_parse_state_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_number_s {
    pub number: * const std::os::raw::c_char,
    pub number_size: std::os::raw::c_ulong,
}
impl json_number_s {
    pub const fn new() -> Self {
        json_number_s {
        number: (0 as * const std::os::raw::c_char),
        number_size: 0
        }
    }
}

impl std::default::Default for json_number_s {
    fn default() -> Self { json_number_s::new() }
}

pub const json_type_number: json_type_e = 1;
pub const json_type_null: json_type_e = 6;
pub const json_type_false: json_type_e = 5;
pub const json_type_true: json_type_e = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_s {
    pub start: * mut crate::src::json::json_array_element_s,
    pub length: std::os::raw::c_ulong,
}
impl json_array_s {
    pub const fn new() -> Self {
        json_array_s {
        start: (0 as * mut crate::src::json::json_array_element_s),
        length: 0
        }
    }
}

impl std::default::Default for json_array_s {
    fn default() -> Self { json_array_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_element_s {
    pub value: * mut crate::src::json::json_value_s,
    pub next: * mut crate::src::json::json_array_element_s,
}
impl json_array_element_s {
    pub const fn new() -> Self {
        json_array_element_s {
        value: (0 as * mut crate::src::json::json_value_s),
        next: (0 as * mut crate::src::json::json_array_element_s)
        }
    }
}

impl std::default::Default for json_array_element_s {
    fn default() -> Self { json_array_element_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_ex_s {
    pub value: crate::src::json::json_value_s,
    pub offset: std::os::raw::c_ulong,
    pub line_no: std::os::raw::c_ulong,
    pub row_no: std::os::raw::c_ulong,
}
impl json_value_ex_s {
    pub const fn new() -> Self {
        json_value_ex_s {
        value: crate::src::json::json_value_s::new(),
        offset: 0,
        line_no: 0,
        row_no: 0
        }
    }
}

impl std::default::Default for json_value_ex_s {
    fn default() -> Self { json_value_ex_s::new() }
}

pub const json_parse_error_premature_end_of_buffer: json_parse_error_e = 7;
pub const json_type_array: json_type_e = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_s {
    pub start: * mut crate::src::json::json_object_element_s,
    pub length: std::os::raw::c_ulong,
}
impl json_object_s {
    pub const fn new() -> Self {
        json_object_s {
        start: (0 as * mut crate::src::json::json_object_element_s),
        length: 0
        }
    }
}

impl std::default::Default for json_object_s {
    fn default() -> Self { json_object_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_element_s {
    pub name: * mut crate::src::json::json_string_s,
    pub value: * mut crate::src::json::json_value_s,
    pub next: * mut crate::src::json::json_object_element_s,
}
impl json_object_element_s {
    pub const fn new() -> Self {
        json_object_element_s {
        name: (0 as * mut crate::src::json::json_string_s),
        value: (0 as * mut crate::src::json::json_value_s),
        next: (0 as * mut crate::src::json::json_object_element_s)
        }
    }
}

impl std::default::Default for json_object_element_s {
    fn default() -> Self { json_object_element_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_s {
    pub string: * const std::os::raw::c_char,
    pub string_size: std::os::raw::c_ulong,
}
impl json_string_s {
    pub const fn new() -> Self {
        json_string_s {
        string: (0 as * const std::os::raw::c_char),
        string_size: 0
        }
    }
}

impl std::default::Default for json_string_s {
    fn default() -> Self { json_string_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_ex_s {
    pub string: crate::src::json::json_string_s,
    pub offset: std::os::raw::c_ulong,
    pub line_no: std::os::raw::c_ulong,
    pub row_no: std::os::raw::c_ulong,
}
impl json_string_ex_s {
    pub const fn new() -> Self {
        json_string_ex_s {
        string: crate::src::json::json_string_s::new(),
        offset: 0,
        line_no: 0,
        row_no: 0
        }
    }
}

impl std::default::Default for json_string_ex_s {
    fn default() -> Self { json_string_ex_s::new() }
}

pub const json_type_object: json_type_e = 2;
pub const json_type_string: json_type_e = 0;
pub const json_parse_error_allocator_failed: json_parse_error_e = 9;
pub const json_parse_error_unexpected_trailing_characters: json_parse_error_e = 10;
pub const json_parse_error_invalid_value: json_parse_error_e = 6;
pub const json_parse_error_invalid_number_format: json_parse_error_e = 5;
pub const json_parse_error_expected_comma_or_closing_bracket: json_parse_error_e = 1;
pub const json_parse_error_unknown: json_parse_error_e = 11;
pub const json_parse_error_expected_colon: json_parse_error_e = 2;
pub const json_parse_error_invalid_string: json_parse_error_e = 8;
pub const json_parse_error_invalid_string_escape_sequence: json_parse_error_e = 4;
pub const json_parse_error_expected_opening_quote: json_parse_error_e = 3;
pub const json_parse_error_none: json_parse_error_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_state_s {
    pub dom: * mut std::os::raw::c_char,
    pub data: * mut std::os::raw::c_char,
}
impl json_extract_state_s {
    pub const fn new() -> Self {
        json_extract_state_s {
        dom: (0 as * mut std::os::raw::c_char),
        data: (0 as * mut std::os::raw::c_char)
        }
    }
}

impl std::default::Default for json_extract_state_s {
    fn default() -> Self { json_extract_state_s::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_result_s {
    pub dom_size: std::os::raw::c_ulong,
    pub data_size: std::os::raw::c_ulong,
}
impl json_extract_result_s {
    pub const fn new() -> Self {
        json_extract_result_s {
        dom_size: 0,
        data_size: 0
        }
    }
}

impl std::default::Default for json_extract_result_s {
    fn default() -> Self { json_extract_result_s::new() }
}

pub type uintmax_t = std::os::raw::c_ulong;
pub type __uintmax_t = std::os::raw::c_ulong;
pub type json_type_e = std::os::raw::c_uint;
pub type json_parse_error_e = std::os::raw::c_uint;
#[no_mangle]
pub extern "C" fn json_hexadecimal_digit(c: std::os::raw::c_char) -> std::os::raw::c_int {
    if '0' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= '9' as i32 {
        return c as std::os::raw::c_int - '0' as i32;
    }
    if 'a' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= 'f' as i32 {
        return c as std::os::raw::c_int - 'a' as i32 + 10 as std::os::raw::c_int;
    }
    if 'A' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= 'F' as i32 {
        return c as std::os::raw::c_int - 'A' as i32 + 10 as std::os::raw::c_int;
    }
    return -(1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn json_hexadecimal_value<'a1>(
    mut c: * const std::os::raw::c_char,
    size: std::os::raw::c_ulong,
    mut result: Option<&'a1 mut std::os::raw::c_ulong>,
) -> std::os::raw::c_int {
    let mut p = (0 as * const i8);
    let mut digit: i32 = 0;
    if size
        > (::std::mem::size_of::<std::os::raw::c_ulong>() as std::os::raw::c_ulong)
            .wrapping_mul(2 as std::os::raw::c_int as std::os::raw::c_ulong)
    {
        return 0 as std::os::raw::c_int;
    }
    *(borrow_mut(&mut result)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    p = c;
    while (p.offset_from(c) as std::os::raw::c_long as std::os::raw::c_ulong) < size {
        *(borrow_mut(&mut result)).unwrap() <<= 4 as std::os::raw::c_int;
        digit = json_hexadecimal_digit(*p);
        if digit < 0 as std::os::raw::c_int || digit > 15 as std::os::raw::c_int {
            return 0 as std::os::raw::c_int;
        }
        *(borrow_mut(&mut result)).unwrap() |= digit as std::os::raw::c_uchar as std::os::raw::c_ulong;
        p = p.offset(1);
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_whitespace(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    let mut offset = (*state).offset;
    let size = (*state).size;
    let src = (*state).src;
    if offset >= (*state).size {
        return 0 as std::os::raw::c_int;
    }
    match *src.offset(offset as isize) as std::os::raw::c_int {
        32 | 13 | 9 | 10 => {}
        _ => return 0 as std::os::raw::c_int,
    }
    loop {
        match *src.offset(offset as isize) as std::os::raw::c_int {
            32 | 13 | 9 => {}
            10 => {
                let ref mut fresh0 = (*state).line_no;
                *fresh0 = (*fresh0).wrapping_add(1);
                (*state).line_offset = offset;
            }
            _ => {
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
        }
        offset = offset.wrapping_add(1);
        if !(offset < size) {
            break;
        }
    }
    (*state).offset = offset;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_c_style_comments(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    if ((*state).offset).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) > (*state).size
    {
        return 0 as std::os::raw::c_int;
    }
    if '/' as i32 == *((*state).src).offset((*state).offset as isize) as std::os::raw::c_int {
        if '/' as i32
            == *((*state).src)
                .offset(
                    ((*state).offset).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        as isize,
                ) as std::os::raw::c_int
        {
            let ref mut fresh1 = (*state).offset;
            *fresh1 = (*fresh1).wrapping_add(1);
            let ref mut fresh2 = (*state).offset;
            *fresh2 = (*fresh2).wrapping_add(1);
            while (*state).offset < (*state).size {
                match *((*state).src).offset((*state).offset as isize) as std::os::raw::c_int {
                    10 => {
                        let ref mut fresh4 = (*state).offset;
                        *fresh4 = (*fresh4).wrapping_add(1);
                        let ref mut fresh5 = (*state).line_no;
                        *fresh5 = (*fresh5).wrapping_add(1);
                        (*state).line_offset = (*state).offset;
                        return 1 as std::os::raw::c_int;
                    }
                    _ => {
                        let ref mut fresh3 = (*state).offset;
                        *fresh3 = (*fresh3).wrapping_add(1);
                    }
                }
            }
            return 1 as std::os::raw::c_int;
        } else {
            if '*' as i32
                == *((*state).src)
                    .offset(
                        ((*state).offset).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as isize,
                    ) as std::os::raw::c_int
            {
                let ref mut fresh6 = (*state).offset;
                *fresh6 = (*fresh6).wrapping_add(1);
                let ref mut fresh7 = (*state).offset;
                *fresh7 = (*fresh7).wrapping_add(1);
                while ((*state).offset).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    < (*state).size
                {
                    if '*' as i32
                        == *((*state).src).offset((*state).offset as isize)
                            as std::os::raw::c_int
                        && '/' as i32
                            == *((*state).src)
                                .offset(
                                    ((*state).offset)
                                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                                ) as std::os::raw::c_int
                    {
                        let ref mut fresh8 = (*state).offset;
                        *fresh8 = (*fresh8 as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        return 1 as std::os::raw::c_int;
                    } else {
                        if '\n' as i32
                            == *((*state).src).offset((*state).offset as isize)
                                as std::os::raw::c_int
                        {
                            let ref mut fresh9 = (*state).line_no;
                            *fresh9 = (*fresh9).wrapping_add(1);
                            (*state).line_offset = (*state).offset;
                        }
                    }
                    let ref mut fresh10 = (*state).offset;
                    *fresh10 = (*fresh10).wrapping_add(1);
                }
                return 1 as std::os::raw::c_int;
            }
        }
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_skip_all_skippables(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    let mut did_consume = 0 as std::os::raw::c_int;
    let size = (*state).size;
    if json_parse_flags_allow_c_style_comments as std::os::raw::c_int as std::os::raw::c_ulong
        & (*state).flags_bitset != 0
    {
        loop {
            if (*state).offset == size {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                return 1 as std::os::raw::c_int;
            }
            did_consume = json_skip_whitespace(state);
            if (*state).offset >= size {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                return 1 as std::os::raw::c_int;
            }
            did_consume |= json_skip_c_style_comments(state);
            if !(0 as std::os::raw::c_int != did_consume) {
                break;
            }
        }
    } else {
        loop {
            if (*state).offset == size {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                return 1 as std::os::raw::c_int;
            }
            did_consume = json_skip_whitespace(state);
            if !(0 as std::os::raw::c_int != did_consume) {
                break;
            }
        }
    }
    if (*state).offset == size {
        (*state)
            .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int as size_t;
        return 1 as std::os::raw::c_int;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_string_size(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut is_key: std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut offset = (*state).offset;
    let size = (*state).size;
    let mut data_size = 0 as std::os::raw::c_int as size_t;
    let src = (*state).src;
    let is_single_quote = ('\'' as i32 == *src.offset(offset as isize) as std::os::raw::c_int)
        as std::os::raw::c_int;
    let quote_to_use = (if is_single_quote != 0 { '\'' as i32 } else { '"' as i32 })
        as std::os::raw::c_char;
    let flags_bitset = (*state).flags_bitset;
    let mut codepoint: u64 = 0;
    let mut high_surrogate = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    if json_parse_flags_allow_location_information as std::os::raw::c_int as std::os::raw::c_ulong
        & flags_bitset != 0 as std::os::raw::c_int as std::os::raw::c_ulong
        && is_key != 0 as std::os::raw::c_int as std::os::raw::c_ulong
    {
        let ref mut fresh11 = (*state).dom_size;
        *fresh11 = (*fresh11 as std::os::raw::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_string_ex_s>() as std::os::raw::c_ulong)
            as size_t as size_t;
    } else {
        let ref mut fresh12 = (*state).dom_size;
        *fresh12 = (*fresh12 as std::os::raw::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    if '"' as i32 != *src.offset(offset as isize) as std::os::raw::c_int {
        if !(json_parse_flags_allow_single_quoted_strings as std::os::raw::c_int as std::os::raw::c_ulong
            & flags_bitset != 0 && is_single_quote != 0)
        {
            (*state)
                .error = json_parse_error_expected_opening_quote as std::os::raw::c_int
                as size_t;
            (*state).offset = offset;
            return 1 as std::os::raw::c_int;
        }
    }
    offset = offset.wrapping_add(1);
    while offset < size
        && quote_to_use as std::os::raw::c_int != *src.offset(offset as isize) as std::os::raw::c_int
    {
        data_size = data_size.wrapping_add(1);
        match *src.offset(offset as isize) as std::os::raw::c_int {
            0 | 9 => {
                (*state)
                    .error = json_parse_error_invalid_string as std::os::raw::c_int as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
            _ => {}
        }
        if '\\' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
            offset = offset.wrapping_add(1);
            if offset == size {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
            match *src.offset(offset as isize) as std::os::raw::c_int {
                34 | 92 | 47 | 98 | 102 | 110 | 114 | 116 => {
                    offset = offset.wrapping_add(1);
                }
                117 => {
                    if !(offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) < size) {
                        (*state)
                            .error = json_parse_error_invalid_string_escape_sequence
                            as std::os::raw::c_int as size_t;
                        (*state).offset = offset;
                        return 1 as std::os::raw::c_int;
                    }
                    codepoint = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
                    if json_hexadecimal_value(
                        &*src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ),
                        4 as std::os::raw::c_int as std::os::raw::c_ulong,
                        Some(&mut codepoint),
                    ) == 0
                    {
                        (*state)
                            .error = json_parse_error_invalid_string_escape_sequence
                            as std::os::raw::c_int as size_t;
                        (*state).offset = offset;
                        return 1 as std::os::raw::c_int;
                    }
                    if high_surrogate != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        if codepoint >= 0xdc00 as std::os::raw::c_int as std::os::raw::c_ulong
                            && codepoint <= 0xdfff as std::os::raw::c_int as std::os::raw::c_ulong
                        {
                            data_size = (data_size as std::os::raw::c_ulong)
                                .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                                as size_t;
                            high_surrogate = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
                        } else {
                            (*state)
                                .error = json_parse_error_invalid_string_escape_sequence
                                as std::os::raw::c_int as size_t;
                            (*state).offset = offset;
                            return 1 as std::os::raw::c_int;
                        }
                    } else if codepoint <= 0x7f as std::os::raw::c_int as std::os::raw::c_ulong {
                        data_size = (data_size as std::os::raw::c_ulong)
                            .wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    } else if codepoint <= 0x7ff as std::os::raw::c_int as std::os::raw::c_ulong {
                        data_size = (data_size as std::os::raw::c_ulong)
                            .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    } else if codepoint >= 0xd800 as std::os::raw::c_int as std::os::raw::c_ulong
                        && codepoint <= 0xdbff as std::os::raw::c_int as std::os::raw::c_ulong
                    {
                        if offset.wrapping_add(11 as std::os::raw::c_int as std::os::raw::c_ulong) > size
                            || '\\' as i32
                                != *src
                                    .offset(
                                        offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                            || 'u' as i32
                                != *src
                                    .offset(
                                        offset.wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                        {
                            (*state)
                                .error = json_parse_error_invalid_string_escape_sequence
                                as std::os::raw::c_int as size_t;
                            (*state).offset = offset;
                            return 1 as std::os::raw::c_int;
                        }
                        high_surrogate = codepoint;
                    } else if codepoint >= 0xd800 as std::os::raw::c_int as std::os::raw::c_ulong
                        && codepoint <= 0xdfff as std::os::raw::c_int as std::os::raw::c_ulong
                    {
                        (*state)
                            .error = json_parse_error_invalid_string_escape_sequence
                            as std::os::raw::c_int as size_t;
                        (*state).offset = offset;
                        return 1 as std::os::raw::c_int;
                    } else {
                        data_size = (data_size as std::os::raw::c_ulong)
                            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                    }
                    offset = (offset as std::os::raw::c_ulong)
                        .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                }
                _ => {
                    (*state)
                        .error = json_parse_error_invalid_string_escape_sequence
                        as std::os::raw::c_int as size_t;
                    (*state).offset = offset;
                    return 1 as std::os::raw::c_int;
                }
            }
        } else if '\r' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
            || '\n' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
        {
            if json_parse_flags_allow_multi_line_strings as std::os::raw::c_int as std::os::raw::c_ulong
                & flags_bitset == 0
            {
                (*state)
                    .error = json_parse_error_invalid_string_escape_sequence
                    as std::os::raw::c_int as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
            offset = offset.wrapping_add(1);
        } else {
            offset = offset.wrapping_add(1);
        }
    }
    if offset == size {
        (*state)
            .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int as size_t;
        (*state).offset = offset.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
        return 1 as std::os::raw::c_int;
    }
    offset = offset.wrapping_add(1);
    let ref mut fresh13 = (*state).data_size;
    *fresh13 = (*fresh13 as std::os::raw::c_ulong).wrapping_add(data_size) as size_t as size_t;
    let ref mut fresh14 = (*state).data_size;
    *fresh14 = (*fresh14).wrapping_add(1);
    (*state).offset = offset;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn is_valid_unquoted_key_char(c: std::os::raw::c_char) -> std::os::raw::c_int {
    return ('0' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= '9' as i32
        || 'a' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= 'z' as i32
        || 'A' as i32 <= c as std::os::raw::c_int && c as std::os::raw::c_int <= 'Z' as i32
        || '_' as i32 == c as std::os::raw::c_int) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_key_size(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    let flags_bitset = (*state).flags_bitset;
    if json_parse_flags_allow_unquoted_keys as std::os::raw::c_int as std::os::raw::c_ulong
        & flags_bitset != 0
    {
        let mut offset = (*state).offset;
        let size = (*state).size;
        let src = (*state).src;
        let mut data_size = (*state).data_size;
        if '"' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
            return json_get_string_size(state, 1 as std::os::raw::c_int as size_t)
        } else if json_parse_flags_allow_single_quoted_strings as std::os::raw::c_int
            as std::os::raw::c_ulong & flags_bitset != 0
            && '\'' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
        {
            return json_get_string_size(state, 1 as std::os::raw::c_int as size_t)
        } else {
            while offset < size
                && is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0
            {
                offset = offset.wrapping_add(1);
                data_size = data_size.wrapping_add(1);
            }
            data_size = data_size.wrapping_add(1);
            if json_parse_flags_allow_location_information as std::os::raw::c_int
                as std::os::raw::c_ulong & flags_bitset != 0
            {
                let ref mut fresh15 = (*state).dom_size;
                *fresh15 = (*fresh15 as std::os::raw::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<json_string_ex_s>() as std::os::raw::c_ulong,
                    ) as size_t as size_t;
            } else {
                let ref mut fresh16 = (*state).dom_size;
                *fresh16 = (*fresh16 as std::os::raw::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong,
                    ) as size_t as size_t;
            }
            (*state).offset = offset;
            (*state).data_size = data_size;
            return 0 as std::os::raw::c_int;
        }
    } else {
        return json_get_string_size(state, 1 as std::os::raw::c_int as size_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_get_object_size(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut is_global_object: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let flags_bitset = (*state).flags_bitset;
    let src = (*state).src;
    let size = (*state).size;
    let mut elements = 0 as std::os::raw::c_int as size_t;
    let mut allow_comma = 0 as std::os::raw::c_int;
    let mut found_closing_brace = 0 as std::os::raw::c_int;
    if is_global_object != 0 {
        if json_skip_all_skippables(state) == 0
            && '{' as i32
                == *((*state).src).offset((*state).offset as isize) as std::os::raw::c_int
        {
            is_global_object = 0 as std::os::raw::c_int;
        }
    }
    if is_global_object == 0 {
        if '{' as i32 != *src.offset((*state).offset as isize) as std::os::raw::c_int {
            (*state).error = json_parse_error_unknown as std::os::raw::c_int as size_t;
            return 1 as std::os::raw::c_int;
        }
        let ref mut fresh17 = (*state).offset;
        *fresh17 = (*fresh17).wrapping_add(1);
    }
    let ref mut fresh18 = (*state).dom_size;
    *fresh18 = (*fresh18 as std::os::raw::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong) as size_t
        as size_t;
    if (*state).offset == size && is_global_object == 0 {
        (*state)
            .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int as size_t;
        return 1 as std::os::raw::c_int;
    }
    let mut current_block_66: u64;
    loop {
        if is_global_object == 0 {
            if json_skip_all_skippables(state) != 0 {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                return 1 as std::os::raw::c_int;
            }
            if '}' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                let ref mut fresh19 = (*state).offset;
                *fresh19 = (*fresh19).wrapping_add(1);
                found_closing_brace = 1 as std::os::raw::c_int;
                break;
            }
        } else if json_skip_all_skippables(state) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                let ref mut fresh20 = (*state).offset;
                *fresh20 = (*fresh20).wrapping_add(1);
                allow_comma = 0 as std::os::raw::c_int;
            } else if json_parse_flags_allow_no_commas as std::os::raw::c_int as std::os::raw::c_ulong
                & flags_bitset != 0
            {
                allow_comma = 0 as std::os::raw::c_int;
            } else {
                (*state)
                    .error = json_parse_error_expected_comma_or_closing_bracket
                    as std::os::raw::c_int as size_t;
                return 1 as std::os::raw::c_int;
            }
            if json_parse_flags_allow_trailing_comma as std::os::raw::c_int as std::os::raw::c_ulong
                & flags_bitset != 0
            {
                current_block_66 = 6057473163062296781;
            } else {
                if json_skip_all_skippables(state) != 0 {
                    (*state)
                        .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
                current_block_66 = 2122094917359643297;
            }
        } else {
            current_block_66 = 2122094917359643297;
        }
        match current_block_66 {
            2122094917359643297 => {
                if json_get_key_size(state) != 0 {
                    (*state)
                        .error = json_parse_error_invalid_string as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
                if json_skip_all_skippables(state) != 0 {
                    (*state)
                        .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
                if json_parse_flags_allow_equals_in_object as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                {
                    let current = *src.offset((*state).offset as isize);
                    if ':' as i32 != current as std::os::raw::c_int
                        && '=' as i32 != current as std::os::raw::c_int
                    {
                        (*state)
                            .error = json_parse_error_expected_colon as std::os::raw::c_int
                            as size_t;
                        return 1 as std::os::raw::c_int;
                    }
                } else if ':' as i32
                    != *src.offset((*state).offset as isize) as std::os::raw::c_int
                {
                    (*state)
                        .error = json_parse_error_expected_colon as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
                let ref mut fresh21 = (*state).offset;
                *fresh21 = (*fresh21).wrapping_add(1);
                if json_skip_all_skippables(state) != 0 {
                    (*state)
                        .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
                if json_get_value_size(state, 0 as std::os::raw::c_int) != 0 {
                    return 1 as std::os::raw::c_int;
                }
                elements = elements.wrapping_add(1);
                allow_comma = 1 as std::os::raw::c_int;
            }
            _ => {}
        }
        if !((*state).offset < size) {
            break;
        }
    }
    if (*state).offset == size && is_global_object == 0 && found_closing_brace == 0 {
        (*state)
            .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int as size_t;
        return 1 as std::os::raw::c_int;
    }
    let ref mut fresh22 = (*state).dom_size;
    *fresh22 = (*fresh22 as std::os::raw::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as std::os::raw::c_ulong)
                .wrapping_mul(elements),
        ) as size_t as size_t;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_array_size(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    let flags_bitset = (*state).flags_bitset;
    let mut elements = 0 as std::os::raw::c_int as size_t;
    let mut allow_comma = 0 as std::os::raw::c_int;
    let src = (*state).src;
    let size = (*state).size;
    if '[' as i32 != *src.offset((*state).offset as isize) as std::os::raw::c_int {
        (*state).error = json_parse_error_unknown as std::os::raw::c_int as size_t;
        return 1 as std::os::raw::c_int;
    }
    let ref mut fresh23 = (*state).offset;
    *fresh23 = (*fresh23).wrapping_add(1);
    let ref mut fresh24 = (*state).dom_size;
    *fresh24 = (*fresh24 as std::os::raw::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_array_s>() as std::os::raw::c_ulong) as size_t
        as size_t;
    while (*state).offset < size {
        if json_skip_all_skippables(state) != 0 {
            (*state)
                .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                as size_t;
            return 1 as std::os::raw::c_int;
        }
        if ']' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
            let ref mut fresh25 = (*state).offset;
            *fresh25 = (*fresh25).wrapping_add(1);
            let ref mut fresh26 = (*state).dom_size;
            *fresh26 = (*fresh26 as std::os::raw::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<json_array_element_s>() as std::os::raw::c_ulong)
                        .wrapping_mul(elements),
                ) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                let ref mut fresh27 = (*state).offset;
                *fresh27 = (*fresh27).wrapping_add(1);
                allow_comma = 0 as std::os::raw::c_int;
            } else if json_parse_flags_allow_no_commas as std::os::raw::c_int as std::os::raw::c_ulong
                & flags_bitset == 0
            {
                (*state)
                    .error = json_parse_error_expected_comma_or_closing_bracket
                    as std::os::raw::c_int as size_t;
                return 1 as std::os::raw::c_int;
            }
            if json_parse_flags_allow_trailing_comma as std::os::raw::c_int as std::os::raw::c_ulong
                & flags_bitset != 0
            {
                allow_comma = 0 as std::os::raw::c_int;
                continue;
            } else if json_skip_all_skippables(state) != 0 {
                (*state)
                    .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                    as size_t;
                return 1 as std::os::raw::c_int;
            }
        }
        if json_get_value_size(state, 0 as std::os::raw::c_int) != 0 {
            return 1 as std::os::raw::c_int;
        }
        elements = elements.wrapping_add(1);
        allow_comma = 1 as std::os::raw::c_int;
    }
    (*state).error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int as size_t;
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_number_size(
    mut state: * mut crate::src::json::json_parse_state_s,
) -> std::os::raw::c_int {
    let flags_bitset = (*state).flags_bitset;
    let mut offset = (*state).offset;
    let size = (*state).size;
    let mut had_leading_digits = 0 as std::os::raw::c_int;
    let src = (*state).src;
    let ref mut fresh28 = (*state).dom_size;
    *fresh28 = (*fresh28 as std::os::raw::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong) as size_t
        as size_t;
    if json_parse_flags_allow_hexadecimal_numbers as std::os::raw::c_int as std::os::raw::c_ulong
        & flags_bitset != 0
        && offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) < size
        && '0' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
        && ('x' as i32
            == *src
                .offset(offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize)
                as std::os::raw::c_int
            || 'X' as i32
                == *src
                    .offset(
                        offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_int)
    {
        offset = (offset as std::os::raw::c_ulong)
            .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32
                || 'a' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= 'f' as i32
                || 'A' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= 'F' as i32)
        {
            offset = offset.wrapping_add(1);
        }
    } else {
        let mut found_sign = 0 as std::os::raw::c_int;
        let mut inf_or_nan = 0 as std::os::raw::c_int;
        if offset < size
            && ('-' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
                || json_parse_flags_allow_leading_plus_sign as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                    && '+' as i32 == *src.offset(offset as isize) as std::os::raw::c_int)
        {
            offset = offset.wrapping_add(1);
            found_sign = 1 as std::os::raw::c_int;
        }
        if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int as std::os::raw::c_ulong
            & flags_bitset != 0
        {
            let inf: [i8; 9] = *core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"Infinity\0");
            let inf_strlen = (::std::mem::size_of::<[std::os::raw::c_char; 9]>()
                as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            let nan: [i8; 4] = *core::intrinsics::transmute::<&'_ [u8; 4], &'_ [i8; 4]>(b"NaN\0");
            let nan_strlen = (::std::mem::size_of::<[std::os::raw::c_char; 4]>()
                as std::os::raw::c_ulong)
                .wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            if offset.wrapping_add(inf_strlen) < size {
                let mut found = 1 as std::os::raw::c_int;
                let mut i: u64 = 0;
                i = 0 as std::os::raw::c_int as size_t;
                while i < inf_strlen {
                    if inf[i as usize] as std::os::raw::c_int
                        != *src.offset(offset.wrapping_add(i) as isize) as std::os::raw::c_int
                    {
                        found = 0 as std::os::raw::c_int;
                        break;
                    } else {
                        i = i.wrapping_add(1);
                    }
                }
                if found != 0 {
                    offset = (offset as std::os::raw::c_ulong).wrapping_add(inf_strlen) as size_t
                        as size_t;
                    inf_or_nan = 1 as std::os::raw::c_int;
                }
            }
            if offset.wrapping_add(nan_strlen) < size {
                let mut found_0 = 1 as std::os::raw::c_int;
                let mut i_0: u64 = 0;
                i_0 = 0 as std::os::raw::c_int as size_t;
                while i_0 < nan_strlen {
                    if nan[i_0 as usize] as std::os::raw::c_int
                        != *src.offset(offset.wrapping_add(i_0) as isize) as std::os::raw::c_int
                    {
                        found_0 = 0 as std::os::raw::c_int;
                        break;
                    } else {
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                if found_0 != 0 {
                    offset = (offset as std::os::raw::c_ulong).wrapping_add(nan_strlen) as size_t
                        as size_t;
                    inf_or_nan = 1 as std::os::raw::c_int;
                }
            }
            if inf_or_nan != 0 {
                if offset < size {
                    match *src.offset(offset as isize) as std::os::raw::c_int {
                        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 101 | 69 => {
                            (*state)
                                .error = json_parse_error_invalid_number_format
                                as std::os::raw::c_int as size_t;
                            (*state).offset = offset;
                            return 1 as std::os::raw::c_int;
                        }
                        _ => {}
                    }
                }
            }
        }
        if found_sign != 0 && inf_or_nan == 0 && offset < size
            && !('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
        {
            if json_parse_flags_allow_leading_or_trailing_decimal_point as std::os::raw::c_int
                as std::os::raw::c_ulong & flags_bitset == 0
                || '.' as i32 != *src.offset(offset as isize) as std::os::raw::c_int
            {
                (*state)
                    .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                    as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
        }
        if offset < size && '0' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
            offset = offset.wrapping_add(1);
            had_leading_digits = 1 as std::os::raw::c_int;
            if offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
            {
                (*state)
                    .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                    as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
        }
        while offset < size
            && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
        {
            offset = offset.wrapping_add(1);
            had_leading_digits = 1 as std::os::raw::c_int;
        }
        if offset < size && '.' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
            offset = offset.wrapping_add(1);
            if offset >= size
                || !('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
            {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as std::os::raw::c_int as std::os::raw::c_ulong & flags_bitset == 0
                    || had_leading_digits == 0
                {
                    (*state)
                        .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                        as size_t;
                    (*state).offset = offset;
                    return 1 as std::os::raw::c_int;
                }
            }
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
            {
                offset = offset.wrapping_add(1);
            }
        }
        if offset < size
            && ('e' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
                || 'E' as i32 == *src.offset(offset as isize) as std::os::raw::c_int)
        {
            offset = offset.wrapping_add(1);
            if offset < size
                && ('-' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
                    || '+' as i32 == *src.offset(offset as isize) as std::os::raw::c_int)
            {
                offset = offset.wrapping_add(1);
            }
            if offset < size
                && !('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32)
            {
                (*state)
                    .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                    as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
            loop {
                offset = offset.wrapping_add(1);
                if !(offset < size
                    && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                        && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32))
                {
                    break;
                }
            }
        }
    }
    if offset < size {
        match *src.offset(offset as isize) as std::os::raw::c_int {
            32 | 9 | 13 | 10 | 125 | 44 | 93 => {}
            61 => {
                if !(json_parse_flags_allow_equals_in_object as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0)
                {
                    (*state)
                        .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                        as size_t;
                    (*state).offset = offset;
                    return 1 as std::os::raw::c_int;
                }
            }
            _ => {
                (*state)
                    .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                    as size_t;
                (*state).offset = offset;
                return 1 as std::os::raw::c_int;
            }
        }
    }
    let ref mut fresh29 = (*state).data_size;
    *fresh29 = (*fresh29 as std::os::raw::c_ulong)
        .wrapping_add(offset.wrapping_sub((*state).offset)) as size_t as size_t;
    let ref mut fresh30 = (*state).data_size;
    *fresh30 = (*fresh30).wrapping_add(1);
    (*state).offset = offset;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_get_value_size(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut is_global_object: std::os::raw::c_int,
) -> std::os::raw::c_int {
    let flags_bitset = (*state).flags_bitset;
    let src = (*state).src;
    let mut offset: u64 = 0;
    let size = (*state).size;
    if json_parse_flags_allow_location_information as std::os::raw::c_int as std::os::raw::c_ulong
        & flags_bitset != 0
    {
        let ref mut fresh31 = (*state).dom_size;
        *fresh31 = (*fresh31 as std::os::raw::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_value_ex_s>() as std::os::raw::c_ulong)
            as size_t as size_t;
    } else {
        let ref mut fresh32 = (*state).dom_size;
        *fresh32 = (*fresh32 as std::os::raw::c_ulong)
            .wrapping_add(::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    if is_global_object != 0 {
        return json_get_object_size(state, 1 as std::os::raw::c_int)
    } else {
        if json_skip_all_skippables(state) != 0 {
            (*state)
                .error = json_parse_error_premature_end_of_buffer as std::os::raw::c_int
                as size_t;
            return 1 as std::os::raw::c_int;
        }
        offset = (*state).offset;
        match *src.offset(offset as isize) as std::os::raw::c_int {
            34 => return json_get_string_size(state, 0 as std::os::raw::c_int as size_t),
            39 => {
                if json_parse_flags_allow_single_quoted_strings as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                {
                    return json_get_string_size(state, 0 as std::os::raw::c_int as size_t)
                } else {
                    (*state)
                        .error = json_parse_error_invalid_value as std::os::raw::c_int as size_t;
                    return 1 as std::os::raw::c_int;
                }
            }
            123 => return json_get_object_size(state, 0 as std::os::raw::c_int),
            91 => return json_get_array_size(state),
            45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                return json_get_number_size(state);
            }
            43 => {
                if json_parse_flags_allow_leading_plus_sign as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                {
                    return json_get_number_size(state)
                } else {
                    (*state)
                        .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
            }
            46 => {
                if json_parse_flags_allow_leading_or_trailing_decimal_point
                    as std::os::raw::c_int as std::os::raw::c_ulong & flags_bitset != 0
                {
                    return json_get_number_size(state)
                } else {
                    (*state)
                        .error = json_parse_error_invalid_number_format as std::os::raw::c_int
                        as size_t;
                    return 1 as std::os::raw::c_int;
                }
            }
            _ => {
                if offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'r' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    let ref mut fresh33 = (*state).offset;
                    *fresh33 = (*fresh33 as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                    return 0 as std::os::raw::c_int;
                } else {
                    if offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                        && 'f' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as isize,
                                ) as std::os::raw::c_int
                        && 'a' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as isize,
                                ) as std::os::raw::c_int
                        && 'l' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as isize,
                                ) as std::os::raw::c_int
                        && 's' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as isize,
                                ) as std::os::raw::c_int
                        && 'e' as i32
                            == *src
                                .offset(
                                    offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        as isize,
                                ) as std::os::raw::c_int
                    {
                        let ref mut fresh34 = (*state).offset;
                        *fresh34 = (*fresh34 as std::os::raw::c_ulong)
                            .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                            as size_t;
                        return 0 as std::os::raw::c_int;
                    } else {
                        if offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                            && 'n' as i32
                                == *((*state).src)
                                    .offset(
                                        offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                            && 'u' as i32
                                == *((*state).src)
                                    .offset(
                                        offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                            && 'l' as i32
                                == *((*state).src)
                                    .offset(
                                        offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                            && 'l' as i32
                                == *((*state).src)
                                    .offset(
                                        offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                            as isize,
                                    ) as std::os::raw::c_int
                        {
                            let ref mut fresh35 = (*state).offset;
                            *fresh35 = (*fresh35 as std::os::raw::c_ulong)
                                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                                as size_t;
                            return 0 as std::os::raw::c_int;
                        } else {
                            if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int
                                as std::os::raw::c_ulong & flags_bitset != 0
                                && offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    <= size
                                && 'N' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                as isize,
                                        ) as std::os::raw::c_int
                                && 'a' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                as isize,
                                        ) as std::os::raw::c_int
                                && 'N' as i32
                                    == *src
                                        .offset(
                                            offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                as isize,
                                        ) as std::os::raw::c_int
                            {
                                return json_get_number_size(state)
                            } else {
                                if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int
                                    as std::os::raw::c_ulong & flags_bitset != 0
                                    && offset.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong)
                                        <= size
                                    && 'I' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'n' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'f' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'i' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'n' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'i' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 't' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                    && 'y' as i32
                                        == *src
                                            .offset(
                                                offset.wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong)
                                                    as isize,
                                            ) as std::os::raw::c_int
                                {
                                    return json_get_number_size(state);
                                }
                            }
                        }
                    }
                }
                (*state).error = json_parse_error_invalid_value as std::os::raw::c_int as size_t;
                return 1 as std::os::raw::c_int;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_string(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut string: * mut crate::src::json::json_string_s,
) {
    let mut offset = (*state).offset;
    let mut bytes_written = 0 as std::os::raw::c_int as size_t;
    let src = (*state).src;
    let quote_to_use = (if '\'' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
        '\'' as i32
    } else {
        '"' as i32
    }) as std::os::raw::c_char;
    let mut data = (*state).data;
    let mut high_surrogate = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
    let mut codepoint: u64 = 0;
    let ref mut fresh36 = (*string).string;
    *fresh36 = data;
    offset = offset.wrapping_add(1);
    while quote_to_use as std::os::raw::c_int != *src.offset(offset as isize) as std::os::raw::c_int {
        if '\\' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
            offset = offset.wrapping_add(1);
            let mut fresh37 = offset;
            offset = offset.wrapping_add(1);
            match *src.offset(fresh37 as isize) as std::os::raw::c_int {
                117 => {
                    codepoint = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
                    if json_hexadecimal_value(
                        &*src.offset(offset as isize),
                        4 as std::os::raw::c_int as std::os::raw::c_ulong,
                        Some(&mut codepoint),
                    ) == 0
                    {
                        return;
                    }
                    offset = (offset as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                    if codepoint <= 0x7f as std::os::raw::c_uint as std::os::raw::c_ulong {
                        let mut fresh38 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh38 as isize) = codepoint as std::os::raw::c_char;
                    } else if codepoint <= 0x7ff as std::os::raw::c_uint as std::os::raw::c_ulong {
                        let mut fresh39 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh39 as isize,
                            ) = (0xc0 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 6 as std::os::raw::c_int) as std::os::raw::c_char;
                        let mut fresh40 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh40 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong)
                            as std::os::raw::c_char;
                    } else if codepoint >= 0xd800 as std::os::raw::c_int as std::os::raw::c_ulong
                        && codepoint <= 0xdbff as std::os::raw::c_int as std::os::raw::c_ulong
                    {
                        high_surrogate = codepoint;
                    } else if codepoint >= 0xdc00 as std::os::raw::c_int as std::os::raw::c_ulong
                        && codepoint <= 0xdfff as std::os::raw::c_int as std::os::raw::c_ulong
                    {
                        let surrogate_offset = (0x10000 as std::os::raw::c_uint)
                            .wrapping_sub((0xd800 as std::os::raw::c_uint) << 10 as std::os::raw::c_int)
                            .wrapping_sub(0xdc00 as std::os::raw::c_uint) as std::os::raw::c_ulong;
                        codepoint = (high_surrogate << 10 as std::os::raw::c_int)
                            .wrapping_add(codepoint)
                            .wrapping_add(surrogate_offset);
                        high_surrogate = 0 as std::os::raw::c_int as std::os::raw::c_ulong;
                        let mut fresh41 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh41 as isize,
                            ) = (0xf0 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 18 as std::os::raw::c_int) as std::os::raw::c_char;
                        let mut fresh42 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh42 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 12 as std::os::raw::c_int
                                & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong) as std::os::raw::c_char;
                        let mut fresh43 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh43 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 6 as std::os::raw::c_int
                                & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong) as std::os::raw::c_char;
                        let mut fresh44 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh44 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong)
                            as std::os::raw::c_char;
                    } else {
                        let mut fresh45 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh45 as isize,
                            ) = (0xe0 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 12 as std::os::raw::c_int) as std::os::raw::c_char;
                        let mut fresh46 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh46 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint >> 6 as std::os::raw::c_int
                                & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong) as std::os::raw::c_char;
                        let mut fresh47 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data
                            .offset(
                                fresh47 as isize,
                            ) = (0x80 as std::os::raw::c_uint as std::os::raw::c_ulong
                            | codepoint & 0x3f as std::os::raw::c_uint as std::os::raw::c_ulong)
                            as std::os::raw::c_char;
                    }
                }
                34 => {
                    let mut fresh48 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh48 as isize) = '"' as i32 as std::os::raw::c_char;
                }
                92 => {
                    let mut fresh49 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh49 as isize) = '\\' as i32 as std::os::raw::c_char;
                }
                47 => {
                    let mut fresh50 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh50 as isize) = '/' as i32 as std::os::raw::c_char;
                }
                98 => {
                    let mut fresh51 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh51 as isize) = '\u{8}' as i32 as std::os::raw::c_char;
                }
                102 => {
                    let mut fresh52 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh52 as isize) = '\u{c}' as i32 as std::os::raw::c_char;
                }
                110 => {
                    let mut fresh53 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh53 as isize) = '\n' as i32 as std::os::raw::c_char;
                }
                114 => {
                    let mut fresh54 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh54 as isize) = '\r' as i32 as std::os::raw::c_char;
                }
                116 => {
                    let mut fresh55 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh55 as isize) = '\t' as i32 as std::os::raw::c_char;
                }
                13 => {
                    let mut fresh56 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh56 as isize) = '\r' as i32 as std::os::raw::c_char;
                    if '\n' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
                        let mut fresh57 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh57 as isize) = '\n' as i32 as std::os::raw::c_char;
                        offset = offset.wrapping_add(1);
                    }
                }
                10 => {
                    let mut fresh58 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh58 as isize) = '\n' as i32 as std::os::raw::c_char;
                }
                _ => return,
            }
        } else {
            let mut fresh59 = offset;
            offset = offset.wrapping_add(1);
            let mut fresh60 = bytes_written;
            bytes_written = bytes_written.wrapping_add(1);
            *data.offset(fresh60 as isize) = *src.offset(fresh59 as isize);
        }
    }
    offset = offset.wrapping_add(1);
    (*string).string_size = bytes_written;
    let mut fresh61 = bytes_written;
    bytes_written = bytes_written.wrapping_add(1);
    *data.offset(fresh61 as isize) = '\0' as i32 as std::os::raw::c_char;
    let ref mut fresh62 = (*state).data;
    *fresh62 = (*fresh62).offset(bytes_written as isize);
    (*state).offset = offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_key(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut string: * mut crate::src::json::json_string_s,
) {
    if json_parse_flags_allow_unquoted_keys as std::os::raw::c_int as std::os::raw::c_ulong
        & (*state).flags_bitset != 0
    {
        let src = (*state).src;
        let data = (*state).data;
        let mut offset = (*state).offset;
        if '"' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
            || '\'' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
        {
            json_parse_string(state, string);
        } else {
            let mut size = 0 as std::os::raw::c_int as size_t;
            let ref mut fresh63 = (*string).string;
            *fresh63 = (*state).data;
            while is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0 {
                let mut fresh64 = offset;
                offset = offset.wrapping_add(1);
                let mut fresh65 = size;
                size = size.wrapping_add(1);
                *data.offset(fresh65 as isize) = *src.offset(fresh64 as isize);
            }
            *data.offset(size as isize) = '\0' as i32 as std::os::raw::c_char;
            let mut fresh66 = size;
            size = size.wrapping_add(1);
            (*string).string_size = fresh66;
            let ref mut fresh67 = (*state).data;
            *fresh67 = (*fresh67).offset(size as isize);
            (*state).offset = offset;
        }
    } else {
        json_parse_string(state, string);
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_object(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut is_global_object: std::os::raw::c_int,
    mut object: * mut crate::src::json::json_object_s,
) {
    let flags_bitset = (*state).flags_bitset;
    let size = (*state).size;
    let src = (*state).src;
    let mut elements = 0 as std::os::raw::c_int as size_t;
    let mut allow_comma = 0 as std::os::raw::c_int;
    let mut previous = 0 as *mut json_object_element_s;
    if is_global_object != 0 {
        if '{' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
            is_global_object = 0 as std::os::raw::c_int;
        }
    }
    if is_global_object == 0 {
        let ref mut fresh68 = (*state).offset;
        *fresh68 = (*fresh68).wrapping_add(1);
    }
    json_skip_all_skippables(state);
    elements = 0 as std::os::raw::c_int as size_t;
    while (*state).offset < size {
        let mut element = 0 as *mut json_object_element_s;
        let mut string = 0 as *mut json_string_s;
        let mut value = 0 as *mut json_value_s;
        if is_global_object == 0 {
            json_skip_all_skippables(state);
            if '}' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                let ref mut fresh69 = (*state).offset;
                *fresh69 = (*fresh69).wrapping_add(1);
                break;
            }
        } else if json_skip_all_skippables(state) != 0 {
            break;
        }
        if allow_comma != 0 {
            if ',' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                let ref mut fresh70 = (*state).offset;
                *fresh70 = (*fresh70).wrapping_add(1);
                allow_comma = 0 as std::os::raw::c_int;
                continue;
            }
        }
        element = (*state).dom as *mut json_object_element_s;
        let ref mut fresh71 = (*state).dom;
        *fresh71 = (*fresh71)
            .offset(
                ::std::mem::size_of::<json_object_element_s>() as std::os::raw::c_ulong as isize,
            );
        if previous.is_null() {
            let ref mut fresh72 = (*object).start;
            *fresh72 = element;
        } else {
            let ref mut fresh73 = (*previous).next;
            *fresh73 = element;
        }
        previous = element;
        if json_parse_flags_allow_location_information as std::os::raw::c_int as std::os::raw::c_ulong
            & flags_bitset != 0
        {
            let mut string_ex = (*state).dom as *mut json_string_ex_s;
            let ref mut fresh74 = (*state).dom;
            *fresh74 = (*fresh74)
                .offset(
                    ::std::mem::size_of::<json_string_ex_s>() as std::os::raw::c_ulong as isize,
                );
            (*string_ex).offset = (*state).offset;
            (*string_ex).line_no = (*state).line_no;
            (*string_ex).row_no = ((*state).offset).wrapping_sub((*state).line_offset);
            string = &mut (*string_ex).string;
        } else {
            string = (*state).dom as *mut json_string_s;
            let ref mut fresh75 = (*state).dom;
            *fresh75 = (*fresh75)
                .offset(
                    ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong as isize,
                );
        }
        let ref mut fresh76 = (*element).name;
        *fresh76 = string;
        json_parse_key(state, string);
        json_skip_all_skippables(state);
        let ref mut fresh77 = (*state).offset;
        *fresh77 = (*fresh77).wrapping_add(1);
        json_skip_all_skippables(state);
        if json_parse_flags_allow_location_information as std::os::raw::c_int as std::os::raw::c_ulong
            & flags_bitset != 0
        {
            let mut value_ex = (*state).dom as *mut json_value_ex_s;
            let ref mut fresh78 = (*state).dom;
            *fresh78 = (*fresh78)
                .offset(
                    ::std::mem::size_of::<json_value_ex_s>() as std::os::raw::c_ulong as isize,
                );
            (*value_ex).offset = (*state).offset;
            (*value_ex).line_no = (*state).line_no;
            (*value_ex).row_no = ((*state).offset).wrapping_sub((*state).line_offset);
            value = &mut (*value_ex).value;
        } else {
            value = (*state).dom as *mut json_value_s;
            let ref mut fresh79 = (*state).dom;
            *fresh79 = (*fresh79)
                .offset(::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong as isize);
        }
        let ref mut fresh80 = (*element).value;
        *fresh80 = value;
        json_parse_value(state, 0 as std::os::raw::c_int, value);
        elements = elements.wrapping_add(1);
        allow_comma = 1 as std::os::raw::c_int;
    }
    if !previous.is_null() {
        let ref mut fresh81 = (*previous).next;
        *fresh81 = 0 as *mut json_object_element_s;
    }
    if 0 as std::os::raw::c_int as std::os::raw::c_ulong == elements {
        let ref mut fresh82 = (*object).start;
        *fresh82 = 0 as *mut json_object_element_s;
    }
    (*object).length = elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_array(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut array: * mut crate::src::json::json_array_s,
) {
    let src = (*state).src;
    let size = (*state).size;
    let mut elements = 0 as std::os::raw::c_int as size_t;
    let mut allow_comma = 0 as std::os::raw::c_int;
    let mut previous = 0 as *mut json_array_element_s;
    let ref mut fresh83 = (*state).offset;
    *fresh83 = (*fresh83).wrapping_add(1);
    json_skip_all_skippables(state);
    elements = 0 as std::os::raw::c_int as size_t;
    let mut current_block_28: u64;
    loop {
        let mut element = 0 as *mut json_array_element_s;
        let mut value = 0 as *mut json_value_s;
        json_skip_all_skippables(state);
        if ']' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
            let ref mut fresh84 = (*state).offset;
            *fresh84 = (*fresh84).wrapping_add(1);
            break;
        } else {
            if allow_comma != 0 {
                if ',' as i32 == *src.offset((*state).offset as isize) as std::os::raw::c_int {
                    let ref mut fresh85 = (*state).offset;
                    *fresh85 = (*fresh85).wrapping_add(1);
                    allow_comma = 0 as std::os::raw::c_int;
                    current_block_28 = 6873731126896040597;
                } else {
                    current_block_28 = 13056961889198038528;
                }
            } else {
                current_block_28 = 13056961889198038528;
            }
            match current_block_28 {
                13056961889198038528 => {
                    element = (*state).dom as *mut json_array_element_s;
                    let ref mut fresh86 = (*state).dom;
                    *fresh86 = (*fresh86)
                        .offset(
                            ::std::mem::size_of::<json_array_element_s>()
                                as std::os::raw::c_ulong as isize,
                        );
                    if previous.is_null() {
                        let ref mut fresh87 = (*array).start;
                        *fresh87 = element;
                    } else {
                        let ref mut fresh88 = (*previous).next;
                        *fresh88 = element;
                    }
                    previous = element;
                    if json_parse_flags_allow_location_information as std::os::raw::c_int
                        as std::os::raw::c_ulong & (*state).flags_bitset != 0
                    {
                        let mut value_ex = (*state).dom as *mut json_value_ex_s;
                        let ref mut fresh89 = (*state).dom;
                        *fresh89 = (*fresh89)
                            .offset(
                                ::std::mem::size_of::<json_value_ex_s>() as std::os::raw::c_ulong
                                    as isize,
                            );
                        (*value_ex).offset = (*state).offset;
                        (*value_ex).line_no = (*state).line_no;
                        (*value_ex)
                            .row_no = ((*state).offset)
                            .wrapping_sub((*state).line_offset);
                        value = &mut (*value_ex).value;
                    } else {
                        value = (*state).dom as *mut json_value_s;
                        let ref mut fresh90 = (*state).dom;
                        *fresh90 = (*fresh90)
                            .offset(
                                ::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong
                                    as isize,
                            );
                    }
                    let ref mut fresh91 = (*element).value;
                    *fresh91 = value;
                    json_parse_value(state, 0 as std::os::raw::c_int, value);
                    elements = elements.wrapping_add(1);
                    allow_comma = 1 as std::os::raw::c_int;
                }
                _ => {}
            }
            if !((*state).offset < size) {
                break;
            }
        }
    }
    if !previous.is_null() {
        let ref mut fresh92 = (*previous).next;
        *fresh92 = 0 as *mut json_array_element_s;
    }
    if 0 as std::os::raw::c_int as std::os::raw::c_ulong == elements {
        let ref mut fresh93 = (*array).start;
        *fresh93 = 0 as *mut json_array_element_s;
    }
    (*array).length = elements;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_number(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut number: * mut crate::src::json::json_number_s,
) {
    let flags_bitset = (*state).flags_bitset;
    let mut offset = (*state).offset;
    let size = (*state).size;
    let mut bytes_written = 0 as std::os::raw::c_int as size_t;
    let src = (*state).src;
    let mut data = (*state).data;
    let ref mut fresh94 = (*number).number;
    *fresh94 = data;
    if json_parse_flags_allow_hexadecimal_numbers as std::os::raw::c_int as std::os::raw::c_ulong
        & flags_bitset != 0
    {
        if '0' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
            && ('x' as i32
                == *src
                    .offset(
                        offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                    ) as std::os::raw::c_int
                || 'X' as i32
                    == *src
                        .offset(
                            offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                as isize,
                        ) as std::os::raw::c_int)
        {
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                    && *src.offset(offset as isize) as std::os::raw::c_int <= '9' as i32
                    || 'a' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                        && *src.offset(offset as isize) as std::os::raw::c_int <= 'f' as i32
                    || 'A' as i32 <= *src.offset(offset as isize) as std::os::raw::c_int
                        && *src.offset(offset as isize) as std::os::raw::c_int <= 'F' as i32
                    || 'x' as i32 == *src.offset(offset as isize) as std::os::raw::c_int
                    || 'X' as i32 == *src.offset(offset as isize) as std::os::raw::c_int)
            {
                let mut fresh95 = offset;
                offset = offset.wrapping_add(1);
                let mut fresh96 = bytes_written;
                bytes_written = bytes_written.wrapping_add(1);
                *data.offset(fresh96 as isize) = *src.offset(fresh95 as isize);
            }
        }
    }
    while offset < size {
        let mut end = 0 as std::os::raw::c_int;
        match *src.offset(offset as isize) as std::os::raw::c_int {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 | 101 | 69 | 43
            | 45 => {
                let mut fresh97 = offset;
                offset = offset.wrapping_add(1);
                let mut fresh98 = bytes_written;
                bytes_written = bytes_written.wrapping_add(1);
                *data.offset(fresh98 as isize) = *src.offset(fresh97 as isize);
            }
            _ => {
                end = 1 as std::os::raw::c_int;
            }
        }
        if 0 as std::os::raw::c_int != end {
            break;
        }
    }
    if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int as std::os::raw::c_ulong & flags_bitset
        != 0
    {
        let inf_strlen = 8 as std::os::raw::c_int as size_t;
        let nan_strlen = 3 as std::os::raw::c_int as size_t;
        if offset.wrapping_add(inf_strlen) < size {
            if 'I' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
                let mut i: u64 = 0;
                i = 0 as std::os::raw::c_int as size_t;
                while i < inf_strlen {
                    let mut fresh99 = offset;
                    offset = offset.wrapping_add(1);
                    let mut fresh100 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh100 as isize) = *src.offset(fresh99 as isize);
                    i = i.wrapping_add(1);
                }
            }
        }
        if offset.wrapping_add(nan_strlen) < size {
            if 'N' as i32 == *src.offset(offset as isize) as std::os::raw::c_int {
                let mut i_0: u64 = 0;
                i_0 = 0 as std::os::raw::c_int as size_t;
                while i_0 < nan_strlen {
                    let mut fresh101 = offset;
                    offset = offset.wrapping_add(1);
                    let mut fresh102 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh102 as isize) = *src.offset(fresh101 as isize);
                    i_0 = i_0.wrapping_add(1);
                }
            }
        }
    }
    (*number).number_size = bytes_written;
    let mut fresh103 = bytes_written;
    bytes_written = bytes_written.wrapping_add(1);
    *data.offset(fresh103 as isize) = '\0' as i32 as std::os::raw::c_char;
    let ref mut fresh104 = (*state).data;
    *fresh104 = (*fresh104).offset(bytes_written as isize);
    (*state).offset = offset;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_value(
    mut state: * mut crate::src::json::json_parse_state_s,
    mut is_global_object: std::os::raw::c_int,
    mut value: * mut crate::src::json::json_value_s,
) {
    let flags_bitset = (*state).flags_bitset;
    let src = (*state).src;
    let size = (*state).size;
    let mut offset: u64 = 0;
    json_skip_all_skippables(state);
    offset = (*state).offset;
    if is_global_object != 0 {
        (*value).type_0 = json_type_object as std::os::raw::c_int as size_t;
        let ref mut fresh105 = (*value).payload;
        *fresh105 = (*state).dom as *mut std::os::raw::c_void;
        let ref mut fresh106 = (*state).dom;
        *fresh106 = (*fresh106)
            .offset(::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong as isize);
        json_parse_object(
            state,
            1 as std::os::raw::c_int,
            (*value).payload as *mut json_object_s,
        );
    } else {
        match *src.offset(offset as isize) as std::os::raw::c_int {
            34 | 39 => {
                (*value).type_0 = json_type_string as std::os::raw::c_int as size_t;
                let ref mut fresh107 = (*value).payload;
                *fresh107 = (*state).dom as *mut std::os::raw::c_void;
                let ref mut fresh108 = (*state).dom;
                *fresh108 = (*fresh108)
                    .offset(
                        ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong as isize,
                    );
                json_parse_string(state, (*value).payload as *mut json_string_s);
            }
            123 => {
                (*value).type_0 = json_type_object as std::os::raw::c_int as size_t;
                let ref mut fresh109 = (*value).payload;
                *fresh109 = (*state).dom as *mut std::os::raw::c_void;
                let ref mut fresh110 = (*state).dom;
                *fresh110 = (*fresh110)
                    .offset(
                        ::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong as isize,
                    );
                json_parse_object(
                    state,
                    0 as std::os::raw::c_int,
                    (*value).payload as *mut json_object_s,
                );
            }
            91 => {
                (*value).type_0 = json_type_array as std::os::raw::c_int as size_t;
                let ref mut fresh111 = (*value).payload;
                *fresh111 = (*state).dom as *mut std::os::raw::c_void;
                let ref mut fresh112 = (*state).dom;
                *fresh112 = (*fresh112)
                    .offset(
                        ::std::mem::size_of::<json_array_s>() as std::os::raw::c_ulong as isize,
                    );
                json_parse_array(state, (*value).payload as *mut json_array_s);
            }
            45 | 43 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 => {
                (*value).type_0 = json_type_number as std::os::raw::c_int as size_t;
                let ref mut fresh113 = (*value).payload;
                *fresh113 = (*state).dom as *mut std::os::raw::c_void;
                let ref mut fresh114 = (*state).dom;
                *fresh114 = (*fresh114)
                    .offset(
                        ::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong as isize,
                    );
                json_parse_number(state, (*value).payload as *mut json_number_s);
            }
            _ => {
                if offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'r' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    (*value).type_0 = json_type_true as std::os::raw::c_int as size_t;
                    let ref mut fresh115 = (*value).payload;
                    *fresh115 = 0 as *mut std::os::raw::c_void;
                    let ref mut fresh116 = (*state).offset;
                    *fresh116 = (*fresh116 as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                } else if offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 'f' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'a' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 's' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'e' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    (*value).type_0 = json_type_false as std::os::raw::c_int as size_t;
                    let ref mut fresh117 = (*value).payload;
                    *fresh117 = 0 as *mut std::os::raw::c_void;
                    let ref mut fresh118 = (*state).offset;
                    *fresh118 = (*fresh118 as std::os::raw::c_ulong)
                        .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                } else if offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'u' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'l' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    (*value).type_0 = json_type_null as std::os::raw::c_int as size_t;
                    let ref mut fresh119 = (*value).payload;
                    *fresh119 = 0 as *mut std::os::raw::c_void;
                    let ref mut fresh120 = (*state).offset;
                    *fresh120 = (*fresh120 as std::os::raw::c_ulong)
                        .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t
                        as size_t;
                } else if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                    && offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 'N' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'a' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'N' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    (*value).type_0 = json_type_number as std::os::raw::c_int as size_t;
                    let ref mut fresh121 = (*value).payload;
                    *fresh121 = (*state).dom as *mut std::os::raw::c_void;
                    let ref mut fresh122 = (*state).dom;
                    *fresh122 = (*fresh122)
                        .offset(
                            ::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong
                                as isize,
                        );
                    json_parse_number(state, (*value).payload as *mut json_number_s);
                } else if json_parse_flags_allow_inf_and_nan as std::os::raw::c_int
                    as std::os::raw::c_ulong & flags_bitset != 0
                    && offset.wrapping_add(8 as std::os::raw::c_int as std::os::raw::c_ulong) <= size
                    && 'I' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(0 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'f' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'i' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'n' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'i' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 't' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(6 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                    && 'y' as i32
                        == *src
                            .offset(
                                offset.wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_ulong)
                                    as isize,
                            ) as std::os::raw::c_int
                {
                    (*value).type_0 = json_type_number as std::os::raw::c_int as size_t;
                    let ref mut fresh123 = (*value).payload;
                    *fresh123 = (*state).dom as *mut std::os::raw::c_void;
                    let ref mut fresh124 = (*state).dom;
                    *fresh124 = (*fresh124)
                        .offset(
                            ::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong
                                as isize,
                        );
                    json_parse_number(state, (*value).payload as *mut json_number_s);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_parse_ex(
    mut src: * const core::ffi::c_void,
    mut src_size: std::os::raw::c_ulong,
    mut flags_bitset: std::os::raw::c_ulong,
    mut alloc_func_ptr: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void>,
    mut user_data: * mut core::ffi::c_void,
    mut result: * mut crate::src::json::json_parse_result_s,
) -> * mut crate::src::json::json_value_s {
    let mut state = json_parse_state_s {
        src: (0 as * const i8),
        size: 0,
        offset: 0,
        flags_bitset: 0,
        data: 0 as *mut std::os::raw::c_char,
        dom: 0 as *mut std::os::raw::c_char,
        dom_size: 0,
        data_size: 0,
        line_no: 0,
        line_offset: 0,
        error: 0,
    };
    let mut allocation = (0 as * mut core::ffi::c_void);
    let mut value = 0 as *mut json_value_s;
    let mut total_size: u64 = 0;
    let mut input_error: i32 = 0;
    if !result.is_null() {
        (*result).error = json_parse_error_none as std::os::raw::c_int as size_t;
        (*result).error_offset = 0 as std::os::raw::c_int as size_t;
        (*result).error_line_no = 0 as std::os::raw::c_int as size_t;
        (*result).error_row_no = 0 as std::os::raw::c_int as size_t;
    }
    if src.is_null() {
        return (0 as * mut crate::src::json::json_value_s);
    }
    state.src = src as *const std::os::raw::c_char;
    state.size = src_size;
    state.offset = 0 as std::os::raw::c_int as size_t;
    state.line_no = 1 as std::os::raw::c_int as size_t;
    state.line_offset = 0 as std::os::raw::c_int as size_t;
    state.error = json_parse_error_none as std::os::raw::c_int as size_t;
    state.dom_size = 0 as std::os::raw::c_int as size_t;
    state.data_size = 0 as std::os::raw::c_int as size_t;
    state.flags_bitset = flags_bitset;
    input_error = json_get_value_size(
        &mut state,
        (json_parse_flags_allow_global_object as std::os::raw::c_int as std::os::raw::c_ulong
            & state.flags_bitset) as std::os::raw::c_int,
    );
    if 0 as std::os::raw::c_int == input_error {
        json_skip_all_skippables(&mut state);
        if state.offset != state.size {
            state
                .error = json_parse_error_unexpected_trailing_characters as std::os::raw::c_int
                as size_t;
            input_error = 1 as std::os::raw::c_int;
        }
    }
    if input_error != 0 {
        if !result.is_null() {
            (*result).error = state.error;
            (*result).error_offset = state.offset;
            (*result).error_line_no = state.line_no;
            (*result).error_row_no = (state.offset).wrapping_sub(state.line_offset);
        }
        return (0 as * mut crate::src::json::json_value_s);
    }
    total_size = (state.dom_size).wrapping_add(state.data_size);
    if alloc_func_ptr.is_none() {
        allocation = malloc(total_size);
    } else {
        allocation = alloc_func_ptr
            .expect("non-null function pointer")(user_data, total_size);
    }
    if allocation.is_null() {
        if !result.is_null() {
            (*result).error = json_parse_error_allocator_failed as std::os::raw::c_int as size_t;
            (*result).error_offset = 0 as std::os::raw::c_int as size_t;
            (*result).error_line_no = 0 as std::os::raw::c_int as size_t;
            (*result).error_row_no = 0 as std::os::raw::c_int as size_t;
        }
        return (0 as * mut crate::src::json::json_value_s);
    }
    state.offset = 0 as std::os::raw::c_int as size_t;
    state.line_no = 1 as std::os::raw::c_int as size_t;
    state.line_offset = 0 as std::os::raw::c_int as size_t;
    state.dom = allocation as *mut std::os::raw::c_char;
    state.data = (state.dom).offset(state.dom_size as isize);
    if json_parse_flags_allow_location_information as std::os::raw::c_int as std::os::raw::c_ulong
        & state.flags_bitset != 0
    {
        let mut value_ex = state.dom as *mut json_value_ex_s;
        state
            .dom = (state.dom)
            .offset(::std::mem::size_of::<json_value_ex_s>() as std::os::raw::c_ulong as isize);
        (*value_ex).offset = state.offset;
        (*value_ex).line_no = state.line_no;
        (*value_ex).row_no = (state.offset).wrapping_sub(state.line_offset);
        value = &mut (*value_ex).value;
    } else {
        value = state.dom as *mut json_value_s;
        state
            .dom = (state.dom)
            .offset(::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong as isize);
    }
    json_parse_value(
        &mut state,
        (json_parse_flags_allow_global_object as std::os::raw::c_int as std::os::raw::c_ulong
            & state.flags_bitset) as std::os::raw::c_int,
        value,
    );
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_parse(
    mut src: * const core::ffi::c_void,
    mut src_size: std::os::raw::c_ulong,
) -> * mut crate::src::json::json_value_s {
    return json_parse_ex(
        src,
        src_size,
        json_parse_flags_default as std::os::raw::c_int as size_t,
        None,
        (0 as * mut core::ffi::c_void),
        (0 as * mut crate::src::json::json_parse_result_s),
    );
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value(
    mut value: * const crate::src::json::json_value_s,
) -> * mut crate::src::json::json_value_s {
    return json_extract_value_ex(value, None, (0 as * mut core::ffi::c_void));
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_number_size(
    number: * const crate::src::json::json_number_s,
) -> crate::src::json::json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    result.dom_size = ::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong;
    result.data_size = (*number).number_size;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_string_size(
    string: * const crate::src::json::json_string_s,
) -> crate::src::json::json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    result.dom_size = ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong;
    result
        .data_size = ((*string).string_size)
        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_object_size(
    object: * const crate::src::json::json_object_s,
) -> crate::src::json::json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut i: u64 = 0;
    let mut element: * const crate::src::json::json_object_element_s = (*object).start;
    result
        .dom_size = (::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as std::os::raw::c_ulong)
                .wrapping_mul((*object).length),
        );
    result.data_size = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*object).length {
        let string_result = json_extract_get_string_size((*element).name);
        let value_result = json_extract_get_value_size((*element).value);
        result
            .dom_size = (result.dom_size as std::os::raw::c_ulong)
            .wrapping_add(string_result.dom_size) as size_t as size_t;
        result
            .data_size = (result.data_size as std::os::raw::c_ulong)
            .wrapping_add(string_result.data_size) as size_t as size_t;
        result
            .dom_size = (result.dom_size as std::os::raw::c_ulong)
            .wrapping_add(value_result.dom_size) as size_t as size_t;
        result
            .data_size = (result.data_size as std::os::raw::c_ulong)
            .wrapping_add(value_result.data_size) as size_t as size_t;
        element = (*element).next;
        i = i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_array_size(
    array: * const crate::src::json::json_array_s,
) -> crate::src::json::json_extract_result_s {
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut i: u64 = 0;
    let mut element: * const crate::src::json::json_array_element_s = (*array).start;
    result
        .dom_size = (::std::mem::size_of::<json_array_s>() as std::os::raw::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<json_array_element_s>() as std::os::raw::c_ulong)
                .wrapping_mul((*array).length),
        );
    result.data_size = 0 as std::os::raw::c_int as size_t;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*array).length {
        let value_result = json_extract_get_value_size((*element).value);
        result
            .dom_size = (result.dom_size as std::os::raw::c_ulong)
            .wrapping_add(value_result.dom_size) as size_t as size_t;
        result
            .data_size = (result.data_size as std::os::raw::c_ulong)
            .wrapping_add(value_result.data_size) as size_t as size_t;
        element = (*element).next;
        i = i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_get_value_size(
    value: * const crate::src::json::json_value_s,
) -> crate::src::json::json_extract_result_s {
    let mut result = {
        let mut init = json_extract_result_s {
            dom_size: 0 as std::os::raw::c_int as size_t,
            data_size: 0 as std::os::raw::c_int as size_t,
        };
        init
    };
    match (*value).type_0 {
        2 => {
            result = json_extract_get_object_size(
                (*value).payload as *const json_object_s,
            );
        }
        3 => {
            result = json_extract_get_array_size(
                (*value).payload as *const json_array_s,
            );
        }
        1 => {
            result = json_extract_get_number_size(
                (*value).payload as *const json_number_s,
            );
        }
        0 => {
            result = json_extract_get_string_size(
                (*value).payload as *const json_string_s,
            );
        }
        _ => {}
    }
    result
        .dom_size = (result.dom_size as std::os::raw::c_ulong)
        .wrapping_add(::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong) as size_t
        as size_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_copy_value(
    mut state: * mut crate::src::json::json_extract_state_s,
    value: * const crate::src::json::json_value_s,
) {
    let mut string = 0 as *mut json_string_s;
    let mut number = 0 as *mut json_number_s;
    let mut object = 0 as *mut json_object_s;
    let mut array = 0 as *mut json_array_s;
    let mut new_value = 0 as *mut json_value_s;
    memcpy(
        (*state).dom as *mut std::os::raw::c_void,
        value as *const std::os::raw::c_void,
        ::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong,
    );
    new_value = (*state).dom as *mut json_value_s;
    let ref mut fresh125 = (*state).dom;
    *fresh125 = (*fresh125)
        .offset(::std::mem::size_of::<json_value_s>() as std::os::raw::c_ulong as isize);
    let ref mut fresh126 = (*new_value).payload;
    *fresh126 = (*state).dom as *mut std::os::raw::c_void;
    if json_type_string as std::os::raw::c_int as std::os::raw::c_ulong == (*value).type_0 {
        memcpy(
            (*state).dom as *mut std::os::raw::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong,
        );
        string = (*state).dom as *mut json_string_s;
        let ref mut fresh127 = (*state).dom;
        *fresh127 = (*fresh127)
            .offset(::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong as isize);
        memcpy(
            (*state).data as *mut std::os::raw::c_void,
            (*string).string as *const std::os::raw::c_void,
            ((*string).string_size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
        );
        let ref mut fresh128 = (*string).string;
        *fresh128 = (*state).data;
        let ref mut fresh129 = (*state).data;
        *fresh129 = (*fresh129)
            .offset(
                ((*string).string_size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                    as isize,
            );
    } else if json_type_number as std::os::raw::c_int as std::os::raw::c_ulong == (*value).type_0 {
        memcpy(
            (*state).dom as *mut std::os::raw::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong,
        );
        number = (*state).dom as *mut json_number_s;
        let ref mut fresh130 = (*state).dom;
        *fresh130 = (*fresh130)
            .offset(::std::mem::size_of::<json_number_s>() as std::os::raw::c_ulong as isize);
        memcpy(
            (*state).data as *mut std::os::raw::c_void,
            (*number).number as *const std::os::raw::c_void,
            (*number).number_size,
        );
        let ref mut fresh131 = (*number).number;
        *fresh131 = (*state).data;
        let ref mut fresh132 = (*state).data;
        *fresh132 = (*fresh132).offset((*number).number_size as isize);
    } else if json_type_object as std::os::raw::c_int as std::os::raw::c_ulong == (*value).type_0 {
        let mut element = 0 as *mut json_object_element_s;
        let mut i: u64 = 0;
        memcpy(
            (*state).dom as *mut std::os::raw::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong,
        );
        object = (*state).dom as *mut json_object_s;
        let ref mut fresh133 = (*state).dom;
        *fresh133 = (*fresh133)
            .offset(::std::mem::size_of::<json_object_s>() as std::os::raw::c_ulong as isize);
        element = (*object).start;
        let ref mut fresh134 = (*object).start;
        *fresh134 = (*state).dom as *mut json_object_element_s;
        i = 0 as std::os::raw::c_int as size_t;
        while i < (*object).length {
            let mut previous_value = 0 as *mut json_value_s;
            let mut previous_element = 0 as *mut json_object_element_s;
            memcpy(
                (*state).dom as *mut std::os::raw::c_void,
                element as *const std::os::raw::c_void,
                ::std::mem::size_of::<json_object_element_s>() as std::os::raw::c_ulong,
            );
            element = (*state).dom as *mut json_object_element_s;
            let ref mut fresh135 = (*state).dom;
            *fresh135 = (*fresh135)
                .offset(
                    ::std::mem::size_of::<json_object_element_s>() as std::os::raw::c_ulong
                        as isize,
                );
            string = (*element).name;
            memcpy(
                (*state).dom as *mut std::os::raw::c_void,
                string as *const std::os::raw::c_void,
                ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong,
            );
            string = (*state).dom as *mut json_string_s;
            let ref mut fresh136 = (*state).dom;
            *fresh136 = (*fresh136)
                .offset(
                    ::std::mem::size_of::<json_string_s>() as std::os::raw::c_ulong as isize,
                );
            let ref mut fresh137 = (*element).name;
            *fresh137 = string;
            memcpy(
                (*state).data as *mut std::os::raw::c_void,
                (*string).string as *const std::os::raw::c_void,
                ((*string).string_size).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            );
            let ref mut fresh138 = (*string).string;
            *fresh138 = (*state).data;
            let ref mut fresh139 = (*state).data;
            *fresh139 = (*fresh139)
                .offset(
                    ((*string).string_size)
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as isize,
                );
            previous_value = (*element).value;
            let ref mut fresh140 = (*element).value;
            *fresh140 = (*state).dom as *mut json_value_s;
            json_extract_copy_value(state, previous_value);
            previous_element = element;
            element = (*element).next;
            if !element.is_null() {
                let ref mut fresh141 = (*previous_element).next;
                *fresh141 = (*state).dom as *mut json_object_element_s;
            }
            i = i.wrapping_add(1);
        }
    } else if json_type_array as std::os::raw::c_int as std::os::raw::c_ulong == (*value).type_0 {
        let mut element_0 = 0 as *mut json_array_element_s;
        let mut i_0: u64 = 0;
        memcpy(
            (*state).dom as *mut std::os::raw::c_void,
            (*value).payload,
            ::std::mem::size_of::<json_array_s>() as std::os::raw::c_ulong,
        );
        array = (*state).dom as *mut json_array_s;
        let ref mut fresh142 = (*state).dom;
        *fresh142 = (*fresh142)
            .offset(::std::mem::size_of::<json_array_s>() as std::os::raw::c_ulong as isize);
        element_0 = (*array).start;
        let ref mut fresh143 = (*array).start;
        *fresh143 = (*state).dom as *mut json_array_element_s;
        i_0 = 0 as std::os::raw::c_int as size_t;
        while i_0 < (*array).length {
            let mut previous_value_0 = 0 as *mut json_value_s;
            let mut previous_element_0 = 0 as *mut json_array_element_s;
            memcpy(
                (*state).dom as *mut std::os::raw::c_void,
                element_0 as *const std::os::raw::c_void,
                ::std::mem::size_of::<json_array_element_s>() as std::os::raw::c_ulong,
            );
            element_0 = (*state).dom as *mut json_array_element_s;
            let ref mut fresh144 = (*state).dom;
            *fresh144 = (*fresh144)
                .offset(
                    ::std::mem::size_of::<json_array_element_s>() as std::os::raw::c_ulong
                        as isize,
                );
            previous_value_0 = (*element_0).value;
            let ref mut fresh145 = (*element_0).value;
            *fresh145 = (*state).dom as *mut json_value_s;
            json_extract_copy_value(state, previous_value_0);
            previous_element_0 = element_0;
            element_0 = (*element_0).next;
            if !element_0.is_null() {
                let ref mut fresh146 = (*previous_element_0).next;
                *fresh146 = (*state).dom as *mut json_array_element_s;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn json_extract_value_ex(
    mut value: * const crate::src::json::json_value_s,
    mut alloc_func_ptr: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void>,
    mut user_data: * mut core::ffi::c_void,
) -> * mut crate::src::json::json_value_s {
    let mut allocation = (0 as * mut core::ffi::c_void);
    let mut result = json_extract_result_s {
        dom_size: 0,
        data_size: 0,
    };
    let mut state = json_extract_state_s {
        dom: 0 as *mut std::os::raw::c_char,
        data: 0 as *mut std::os::raw::c_char,
    };
    let mut total_size: u64 = 0;
    if value.is_null() {
        return (0 as * mut crate::src::json::json_value_s);
    }
    result = json_extract_get_value_size(value);
    total_size = (result.dom_size).wrapping_add(result.data_size);
    if alloc_func_ptr.is_none() {
        allocation = malloc(total_size);
    } else {
        allocation = alloc_func_ptr
            .expect("non-null function pointer")(user_data, total_size);
    }
    state.dom = allocation as *mut std::os::raw::c_char;
    state.data = (state.dom).offset(result.dom_size as isize);
    json_extract_copy_value(&mut state, value);
    return allocation as *mut json_value_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_string(
    value: * mut crate::src::json::json_value_s,
) -> * mut crate::src::json::json_string_s {
    if (*value).type_0 != json_type_string as std::os::raw::c_int as std::os::raw::c_ulong {
        return (0 as * mut crate::src::json::json_string_s);
    }
    return (*value).payload as *mut json_string_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_number(
    value: * mut crate::src::json::json_value_s,
) -> * mut crate::src::json::json_number_s {
    if (*value).type_0 != json_type_number as std::os::raw::c_int as std::os::raw::c_ulong {
        return (0 as * mut crate::src::json::json_number_s);
    }
    return (*value).payload as *mut json_number_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_object(
    value: * mut crate::src::json::json_value_s,
) -> * mut crate::src::json::json_object_s {
    if (*value).type_0 != json_type_object as std::os::raw::c_int as std::os::raw::c_ulong {
        return (0 as * mut crate::src::json::json_object_s);
    }
    return (*value).payload as *mut json_object_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_as_array(
    value: * mut crate::src::json::json_value_s,
) -> * mut crate::src::json::json_array_s {
    if (*value).type_0 != json_type_array as std::os::raw::c_int as std::os::raw::c_ulong {
        return (0 as * mut crate::src::json::json_array_s);
    }
    return (*value).payload as *mut json_array_s;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_true<'a1>(value: Option<&'a1 crate::src::json::json_value_s>) -> std::os::raw::c_int {
    return ((*((value).clone()).unwrap()).type_0 == json_type_true as std::os::raw::c_int as std::os::raw::c_ulong)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_false<'a1>(value: Option<&'a1 crate::src::json::json_value_s>) -> std::os::raw::c_int {
    return ((*((value).clone()).unwrap()).type_0 == json_type_false as std::os::raw::c_int as std::os::raw::c_ulong)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_value_is_null<'a1>(value: Option<&'a1 crate::src::json::json_value_s>) -> std::os::raw::c_int {
    return ((*((value).clone()).unwrap()).type_0 == json_type_null as std::os::raw::c_int as std::os::raw::c_ulong)
        as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_number_size(
    mut number: * const crate::src::json::json_number_s,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut parsed_number: u64 = 0;
    let mut i: u64 = 0;
    if (*number).number_size >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        match *((*number).number).offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int {
            120 | 88 => {
                parsed_number = strtoumax(
                    (*number).number,
                    0 as *mut *mut std::os::raw::c_char,
                    0 as std::os::raw::c_int,
                );
                i = 0 as std::os::raw::c_int as size_t;
                while 0 as std::os::raw::c_int as std::os::raw::c_ulong != parsed_number {
                    parsed_number = (parsed_number as std::os::raw::c_ulong)
                        .wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_ulong) as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_add(1);
                }
                *size = (*size as std::os::raw::c_ulong).wrapping_add(i) as size_t as size_t;
                return 0 as std::os::raw::c_int;
            }
            _ => {}
        }
    }
    i = 0 as std::os::raw::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
            || '-' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int)
    {
        i = i.wrapping_add(1);
    }
    if i < (*number).number_size
        && 'I' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        let mut inf = b"Infinity\0" as *const u8 as *const std::os::raw::c_char;
        let mut k: u64 = 0;
        k = i;
        while k < (*number).number_size {
            let mut fresh147 = inf;
            inf = inf.offset(1);
            let c = *fresh147;
            if '\0' as i32 == c as std::os::raw::c_int {
                break;
            }
            if c as std::os::raw::c_int != *((*number).number).offset(k as isize) as std::os::raw::c_int
            {
                break;
            }
            k = k.wrapping_add(1);
        }
        if '\0' as i32 == *inf as std::os::raw::c_int {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(22 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            if '-' as i32
                == *((*number).number).offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            {
                *size = (*size as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            }
        }
        return 0 as std::os::raw::c_int;
    }
    if i < (*number).number_size
        && 'N' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        let mut nan = b"NaN\0" as *const u8 as *const std::os::raw::c_char;
        let mut k_0: u64 = 0;
        k_0 = i;
        while k_0 < (*number).number_size {
            let mut fresh148 = nan;
            nan = nan.offset(1);
            let c_0 = *fresh148;
            if '\0' as i32 == c_0 as std::os::raw::c_int {
                break;
            }
            if c_0 as std::os::raw::c_int
                != *((*number).number).offset(k_0 as isize) as std::os::raw::c_int
            {
                break;
            }
            k_0 = k_0.wrapping_add(1);
        }
        if '\0' as i32 == *nan as std::os::raw::c_int {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        *size = (*size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
    } else {
        while i < (*number).number_size {
            let c_1 = *((*number).number).offset(i as isize);
            if !('0' as i32 <= c_1 as std::os::raw::c_int && c_1 as std::os::raw::c_int <= '9' as i32) {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) == (*number).number_size
            && '.' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
        {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
        }
    }
    *size = (*size as std::os::raw::c_ulong).wrapping_add((*number).number_size) as size_t
        as size_t;
    if '+' as i32 == *((*number).number).offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
    {
        *size = (*size as std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong)
            as size_t as size_t;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_get_string_size(
    mut string: * const crate::src::json::json_string_s,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut i: u64 = 0;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*string).string_size {
        match *((*string).string).offset(i as isize) as std::os::raw::c_int {
            34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                *size = (*size as std::os::raw::c_ulong)
                    .wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            }
            _ => {
                *size = (*size as std::os::raw::c_ulong)
                    .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            }
        }
        i = i.wrapping_add(1);
    }
    *size = (*size as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_array_size(
    mut array: * const crate::src::json::json_array_s,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut element = 0 as *mut json_array_element_s;
    *size = (*size as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    if (1 as std::os::raw::c_int as std::os::raw::c_ulong) < (*array).length {
        *size = (*size as std::os::raw::c_ulong)
            .wrapping_add(
                ((*array).length).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) as size_t as size_t;
    }
    element = (*array).start;
    while !element.is_null() {
        if json_write_minified_get_value_size((*element).value, size) != 0 {
            return 1 as std::os::raw::c_int;
        }
        element = (*element).next;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_object_size(
    mut object: * const crate::src::json::json_object_s,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut element = 0 as *mut json_object_element_s;
    *size = (*size as std::os::raw::c_ulong).wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    *size = (*size as std::os::raw::c_ulong).wrapping_add((*object).length) as size_t as size_t;
    if (1 as std::os::raw::c_int as std::os::raw::c_ulong) < (*object).length {
        *size = (*size as std::os::raw::c_ulong)
            .wrapping_add(
                ((*object).length).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) as size_t as size_t;
    }
    element = (*object).start;
    while !element.is_null() {
        if json_write_get_string_size((*element).name, size) != 0 {
            return 1 as std::os::raw::c_int;
        }
        if json_write_minified_get_value_size((*element).value, size) != 0 {
            return 1 as std::os::raw::c_int;
        }
        element = (*element).next;
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_get_value_size(
    mut value: * const crate::src::json::json_value_s,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size,
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size,
            );
        }
        3 => {
            return json_write_minified_get_array_size(
                (*value).payload as *mut json_array_s,
                size,
            );
        }
        2 => {
            return json_write_minified_get_object_size(
                (*value).payload as *mut json_object_s,
                size,
            );
        }
        4 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        5 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        6 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        _ => return 1 as std::os::raw::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_number(
    mut number: * const crate::src::json::json_number_s,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut parsed_number: u64 = 0;
    let mut backup: u64 = 0;
    let mut i: u64 = 0;
    if (*number).number_size >= 2 as std::os::raw::c_int as std::os::raw::c_ulong {
        match *((*number).number).offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int {
            120 | 88 => {
                parsed_number = strtoumax(
                    (*number).number,
                    0 as *mut *mut std::os::raw::c_char,
                    0 as std::os::raw::c_int,
                );
                backup = parsed_number;
                i = 0 as std::os::raw::c_int as size_t;
                while 0 as std::os::raw::c_int as std::os::raw::c_ulong != parsed_number {
                    parsed_number = (parsed_number as std::os::raw::c_ulong)
                        .wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_ulong) as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_add(1);
                }
                parsed_number = backup;
                backup = i;
                loop {
                    *data
                        .offset(i as isize)
                        .offset(
                            -(1 as std::os::raw::c_int as isize),
                        ) = ('0' as i32
                        + parsed_number.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_ulong)
                            as std::os::raw::c_char as std::os::raw::c_int) as std::os::raw::c_char;
                    parsed_number = (parsed_number as std::os::raw::c_ulong)
                        .wrapping_div(10 as std::os::raw::c_int as std::os::raw::c_ulong) as uintmax_t
                        as uintmax_t;
                    i = i.wrapping_sub(1);
                    if !(0 as std::os::raw::c_int as std::os::raw::c_ulong != parsed_number) {
                        break;
                    }
                }
                data = data.offset(backup as isize);
                return data;
            }
            _ => {}
        }
    }
    i = 0 as std::os::raw::c_int as size_t;
    if i < (*number).number_size
        && ('+' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
            || '-' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int)
    {
        i = i.wrapping_add(1);
    }
    if i < (*number).number_size
        && 'I' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        let mut inf = b"Infinity\0" as *const u8 as *const std::os::raw::c_char;
        let mut k: u64 = 0;
        k = i;
        while k < (*number).number_size {
            let mut fresh149 = inf;
            inf = inf.offset(1);
            let c = *fresh149;
            if '\0' as i32 == c as std::os::raw::c_int {
                break;
            }
            if c as std::os::raw::c_int != *((*number).number).offset(k as isize) as std::os::raw::c_int
            {
                break;
            }
            k = k.wrapping_add(1);
        }
        let mut fresh150 = inf;
        inf = inf.offset(1);
        if '\0' as i32 == *fresh150 as std::os::raw::c_int {
            let mut dbl_max = (0 as * const i8);
            if '-' as i32
                == *((*number).number).offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            {
                let mut fresh151 = data;
                data = data.offset(1);
                *fresh151 = '-' as i32 as std::os::raw::c_char;
            }
            dbl_max = b"1.7976931348623158e308\0" as *const u8 as *const std::os::raw::c_char;
            while '\0' as i32 != *dbl_max as std::os::raw::c_int {
                let mut fresh152 = data;
                data = data.offset(1);
                *fresh152 = *dbl_max;
                dbl_max = dbl_max.offset(1);
            }
            return data;
        }
    }
    if i < (*number).number_size
        && 'N' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        let mut nan = b"NaN\0" as *const u8 as *const std::os::raw::c_char;
        let mut k_0: u64 = 0;
        k_0 = i;
        while k_0 < (*number).number_size {
            let mut fresh153 = nan;
            nan = nan.offset(1);
            let c_0 = *fresh153;
            if '\0' as i32 == c_0 as std::os::raw::c_int {
                break;
            }
            if c_0 as std::os::raw::c_int
                != *((*number).number).offset(k_0 as isize) as std::os::raw::c_int
            {
                break;
            }
            k_0 = k_0.wrapping_add(1);
        }
        let mut fresh154 = nan;
        nan = nan.offset(1);
        if '\0' as i32 == *fresh154 as std::os::raw::c_int {
            let mut fresh155 = data;
            data = data.offset(1);
            *fresh155 = '0' as i32 as std::os::raw::c_char;
            return data;
        }
    }
    if i < (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        i = 0 as std::os::raw::c_int as size_t;
        if '+' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int {
            i = i.wrapping_add(1);
        }
        if '-' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int {
            let mut fresh156 = data;
            data = data.offset(1);
            *fresh156 = '-' as i32 as std::os::raw::c_char;
            i = i.wrapping_add(1);
        }
        let mut fresh157 = data;
        data = data.offset(1);
        *fresh157 = '0' as i32 as std::os::raw::c_char;
        while i < (*number).number_size {
            let mut fresh158 = data;
            data = data.offset(1);
            *fresh158 = *((*number).number).offset(i as isize);
            i = i.wrapping_add(1);
        }
        return data;
    }
    while i < (*number).number_size {
        let c_1 = *((*number).number).offset(i as isize);
        if !('0' as i32 <= c_1 as std::os::raw::c_int && c_1 as std::os::raw::c_int <= '9' as i32) {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) == (*number).number_size
        && '.' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int
    {
        i = 0 as std::os::raw::c_int as size_t;
        if '+' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int {
            i = i.wrapping_add(1);
        }
        if '-' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int {
            let mut fresh159 = data;
            data = data.offset(1);
            *fresh159 = '-' as i32 as std::os::raw::c_char;
            i = i.wrapping_add(1);
        }
        while i < (*number).number_size {
            let mut fresh160 = data;
            data = data.offset(1);
            *fresh160 = *((*number).number).offset(i as isize);
            i = i.wrapping_add(1);
        }
        let mut fresh161 = data;
        data = data.offset(1);
        *fresh161 = '0' as i32 as std::os::raw::c_char;
        return data;
    }
    i = 0 as std::os::raw::c_int as size_t;
    if '+' as i32 == *((*number).number).offset(i as isize) as std::os::raw::c_int {
        i = i.wrapping_add(1);
    }
    while i < (*number).number_size {
        let mut fresh162 = data;
        data = data.offset(1);
        *fresh162 = *((*number).number).offset(i as isize);
        i = i.wrapping_add(1);
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_string(
    mut string: * const crate::src::json::json_string_s,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut i: u64 = 0;
    let mut fresh163 = data;
    data = data.offset(1);
    *fresh163 = '"' as i32 as std::os::raw::c_char;
    i = 0 as std::os::raw::c_int as size_t;
    while i < (*string).string_size {
        match *((*string).string).offset(i as isize) as std::os::raw::c_int {
            34 => {
                let mut fresh164 = data;
                data = data.offset(1);
                *fresh164 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh165 = data;
                data = data.offset(1);
                *fresh165 = '"' as i32 as std::os::raw::c_char;
            }
            92 => {
                let mut fresh166 = data;
                data = data.offset(1);
                *fresh166 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh167 = data;
                data = data.offset(1);
                *fresh167 = '\\' as i32 as std::os::raw::c_char;
            }
            8 => {
                let mut fresh168 = data;
                data = data.offset(1);
                *fresh168 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh169 = data;
                data = data.offset(1);
                *fresh169 = 'b' as i32 as std::os::raw::c_char;
            }
            12 => {
                let mut fresh170 = data;
                data = data.offset(1);
                *fresh170 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh171 = data;
                data = data.offset(1);
                *fresh171 = 'f' as i32 as std::os::raw::c_char;
            }
            10 => {
                let mut fresh172 = data;
                data = data.offset(1);
                *fresh172 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh173 = data;
                data = data.offset(1);
                *fresh173 = 'n' as i32 as std::os::raw::c_char;
            }
            13 => {
                let mut fresh174 = data;
                data = data.offset(1);
                *fresh174 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh175 = data;
                data = data.offset(1);
                *fresh175 = 'r' as i32 as std::os::raw::c_char;
            }
            9 => {
                let mut fresh176 = data;
                data = data.offset(1);
                *fresh176 = '\\' as i32 as std::os::raw::c_char;
                let mut fresh177 = data;
                data = data.offset(1);
                *fresh177 = 't' as i32 as std::os::raw::c_char;
            }
            _ => {
                let mut fresh178 = data;
                data = data.offset(1);
                *fresh178 = *((*string).string).offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    let mut fresh179 = data;
    data = data.offset(1);
    *fresh179 = '"' as i32 as std::os::raw::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_array(
    mut array: * const crate::src::json::json_array_s,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut element = 0 as *mut json_array_element_s;
    let mut fresh180 = data;
    data = data.offset(1);
    *fresh180 = '[' as i32 as std::os::raw::c_char;
    element = (*array).start;
    while !element.is_null() {
        if element != (*array).start {
            let mut fresh181 = data;
            data = data.offset(1);
            *fresh181 = ',' as i32 as std::os::raw::c_char;
        }
        data = json_write_minified_value((*element).value, data);
        if data.is_null() {
            return 0 as *mut std::os::raw::c_char;
        }
        element = (*element).next;
    }
    let mut fresh182 = data;
    data = data.offset(1);
    *fresh182 = ']' as i32 as std::os::raw::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_object(
    mut object: * const crate::src::json::json_object_s,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut element = 0 as *mut json_object_element_s;
    let mut fresh183 = data;
    data = data.offset(1);
    *fresh183 = '{' as i32 as std::os::raw::c_char;
    element = (*object).start;
    while !element.is_null() {
        if element != (*object).start {
            let mut fresh184 = data;
            data = data.offset(1);
            *fresh184 = ',' as i32 as std::os::raw::c_char;
        }
        data = json_write_string((*element).name, data);
        if data.is_null() {
            return 0 as *mut std::os::raw::c_char;
        }
        let mut fresh185 = data;
        data = data.offset(1);
        *fresh185 = ':' as i32 as std::os::raw::c_char;
        data = json_write_minified_value((*element).value, data);
        if data.is_null() {
            return 0 as *mut std::os::raw::c_char;
        }
        element = (*element).next;
    }
    let mut fresh186 = data;
    data = data.offset(1);
    *fresh186 = '}' as i32 as std::os::raw::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified_value(
    mut value: * const crate::src::json::json_value_s,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    match (*value).type_0 {
        1 => return json_write_number((*value).payload as *mut json_number_s, data),
        0 => return json_write_string((*value).payload as *mut json_string_s, data),
        3 => {
            return json_write_minified_array((*value).payload as *mut json_array_s, data);
        }
        2 => {
            return json_write_minified_object(
                (*value).payload as *mut json_object_s,
                data,
            );
        }
        4 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 't' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'r' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'u' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 'e' as i32 as std::os::raw::c_char;
            return data.offset(4 as std::os::raw::c_int as isize);
        }
        5 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 'f' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'a' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 's' as i32 as std::os::raw::c_char;
            *data.offset(4 as std::os::raw::c_int as isize) = 'e' as i32 as std::os::raw::c_char;
            return data.offset(5 as std::os::raw::c_int as isize);
        }
        6 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 'n' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'u' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            return data.offset(4 as std::os::raw::c_int as isize);
        }
        _ => return 0 as *mut std::os::raw::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_minified<'a1>(
    mut value: * const crate::src::json::json_value_s,
    mut out_size: Option<&'a1 mut std::os::raw::c_ulong>,
) -> * mut core::ffi::c_void {
    let mut size = 0 as std::os::raw::c_int as size_t;
    let mut data = 0 as *mut std::os::raw::c_char;
    let mut data_end = 0 as *mut std::os::raw::c_char;
    if value.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    if json_write_minified_get_value_size(value, &mut size) != 0 {
        return 0 as *mut std::os::raw::c_void;
    }
    size = (size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    data = malloc(size) as *mut std::os::raw::c_char;
    if data.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    data_end = json_write_minified_value(value, data);
    if data_end.is_null() {
        free(data as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void;
    }
    *data_end = '\0' as i32 as std::os::raw::c_char;
    if !borrow(& out_size).is_none() {
        *(borrow_mut(&mut out_size)).unwrap() = size;
    }
    return data as *mut std::os::raw::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_array_size(
    mut array: * const crate::src::json::json_array_s,
    mut depth: std::os::raw::c_ulong,
    mut indent_size: std::os::raw::c_ulong,
    mut newline_size: std::os::raw::c_ulong,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut element = 0 as *mut json_array_element_s;
    *size = (*size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    if (0 as std::os::raw::c_int as std::os::raw::c_ulong) < (*array).length {
        *size = (*size as std::os::raw::c_ulong).wrapping_add(newline_size) as size_t as size_t;
        *size = (*size as std::os::raw::c_ulong)
            .wrapping_add(
                ((*array).length).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) as size_t as size_t;
        element = (*array).start;
        while !element.is_null() {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(
                    depth
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(indent_size),
                ) as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                indent_size,
                newline_size,
                size,
            ) != 0
            {
                return 1 as std::os::raw::c_int;
            }
            *size = (*size as std::os::raw::c_ulong).wrapping_add(newline_size) as size_t
                as size_t;
            element = (*element).next;
        }
        *size = (*size as std::os::raw::c_ulong).wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size = (*size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_object_size(
    mut object: * const crate::src::json::json_object_s,
    mut depth: std::os::raw::c_ulong,
    mut indent_size: std::os::raw::c_ulong,
    mut newline_size: std::os::raw::c_ulong,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    let mut element = 0 as *mut json_object_element_s;
    *size = (*size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    if (0 as std::os::raw::c_int as std::os::raw::c_ulong) < (*object).length {
        *size = (*size as std::os::raw::c_ulong).wrapping_add(newline_size) as size_t as size_t;
        *size = (*size as std::os::raw::c_ulong)
            .wrapping_add(
                ((*object).length).wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_ulong),
            ) as size_t as size_t;
        element = (*object).start;
        while !element.is_null() {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(
                    depth
                        .wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
                        .wrapping_mul(indent_size),
                ) as size_t as size_t;
            *size = (*size as std::os::raw::c_ulong).wrapping_add(newline_size) as size_t
                as size_t;
            if json_write_get_string_size((*element).name, size) != 0 {
                return 1 as std::os::raw::c_int;
            }
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(3 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            if json_write_pretty_get_value_size(
                (*element).value,
                depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                indent_size,
                newline_size,
                size,
            ) != 0
            {
                return 1 as std::os::raw::c_int;
            }
            element = (*element).next;
        }
        *size = (*size as std::os::raw::c_ulong).wrapping_add(depth.wrapping_mul(indent_size))
            as size_t as size_t;
    }
    *size = (*size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_get_value_size(
    mut value: * const crate::src::json::json_value_s,
    mut depth: std::os::raw::c_ulong,
    mut indent_size: std::os::raw::c_ulong,
    mut newline_size: std::os::raw::c_ulong,
    mut size: * mut std::os::raw::c_ulong,
) -> std::os::raw::c_int {
    match (*value).type_0 {
        1 => {
            return json_write_get_number_size(
                (*value).payload as *mut json_number_s,
                size,
            );
        }
        0 => {
            return json_write_get_string_size(
                (*value).payload as *mut json_string_s,
                size,
            );
        }
        3 => {
            return json_write_pretty_get_array_size(
                (*value).payload as *mut json_array_s,
                depth,
                indent_size,
                newline_size,
                size,
            );
        }
        2 => {
            return json_write_pretty_get_object_size(
                (*value).payload as *mut json_object_s,
                depth,
                indent_size,
                newline_size,
                size,
            );
        }
        4 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        5 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(5 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        6 => {
            *size = (*size as std::os::raw::c_ulong)
                .wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_ulong) as size_t as size_t;
            return 0 as std::os::raw::c_int;
        }
        _ => return 1 as std::os::raw::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_array(
    mut array: * const crate::src::json::json_array_s,
    mut depth: std::os::raw::c_ulong,
    mut indent: * const std::os::raw::c_char,
    mut newline: * const std::os::raw::c_char,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut k: u64 = 0;
    let mut m: u64 = 0;
    let mut element = 0 as *mut json_array_element_s;
    let mut fresh187 = data;
    data = data.offset(1);
    *fresh187 = '[' as i32 as std::os::raw::c_char;
    if (0 as std::os::raw::c_int as std::os::raw::c_ulong) < (*array).length {
        k = 0 as std::os::raw::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
            let mut fresh188 = data;
            data = data.offset(1);
            *fresh188 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
        }
        element = (*array).start;
        while !element.is_null() {
            if element != (*array).start {
                let mut fresh189 = data;
                data = data.offset(1);
                *fresh189 = ',' as i32 as std::os::raw::c_char;
                k = 0 as std::os::raw::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
                    let mut fresh190 = data;
                    data = data.offset(1);
                    *fresh190 = *newline.offset(k as isize);
                    k = k.wrapping_add(1);
                }
            }
            k = 0 as std::os::raw::c_int as size_t;
            while k < depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
                m = 0 as std::os::raw::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as std::os::raw::c_int {
                    let mut fresh191 = data;
                    data = data.offset(1);
                    *fresh191 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                }
                k = k.wrapping_add(1);
            }
            data = json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                indent,
                newline,
                data,
            );
            if data.is_null() {
                return 0 as *mut std::os::raw::c_char;
            }
            element = (*element).next;
        }
        k = 0 as std::os::raw::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
            let mut fresh192 = data;
            data = data.offset(1);
            *fresh192 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
        }
        k = 0 as std::os::raw::c_int as size_t;
        while k < depth {
            m = 0 as std::os::raw::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as std::os::raw::c_int {
                let mut fresh193 = data;
                data = data.offset(1);
                *fresh193 = *indent.offset(m as isize);
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
    let mut fresh194 = data;
    data = data.offset(1);
    *fresh194 = ']' as i32 as std::os::raw::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_object(
    mut object: * const crate::src::json::json_object_s,
    mut depth: std::os::raw::c_ulong,
    mut indent: * const std::os::raw::c_char,
    mut newline: * const std::os::raw::c_char,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    let mut k: u64 = 0;
    let mut m: u64 = 0;
    let mut element = 0 as *mut json_object_element_s;
    let mut fresh195 = data;
    data = data.offset(1);
    *fresh195 = '{' as i32 as std::os::raw::c_char;
    if (0 as std::os::raw::c_int as std::os::raw::c_ulong) < (*object).length {
        k = 0 as std::os::raw::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
            let mut fresh196 = data;
            data = data.offset(1);
            *fresh196 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
        }
        element = (*object).start;
        while !element.is_null() {
            if element != (*object).start {
                let mut fresh197 = data;
                data = data.offset(1);
                *fresh197 = ',' as i32 as std::os::raw::c_char;
                k = 0 as std::os::raw::c_int as size_t;
                while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
                    let mut fresh198 = data;
                    data = data.offset(1);
                    *fresh198 = *newline.offset(k as isize);
                    k = k.wrapping_add(1);
                }
            }
            k = 0 as std::os::raw::c_int as size_t;
            while k < depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong) {
                m = 0 as std::os::raw::c_int as size_t;
                while '\0' as i32 != *indent.offset(m as isize) as std::os::raw::c_int {
                    let mut fresh199 = data;
                    data = data.offset(1);
                    *fresh199 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                }
                k = k.wrapping_add(1);
            }
            data = json_write_string((*element).name, data);
            if data.is_null() {
                return 0 as *mut std::os::raw::c_char;
            }
            let mut fresh200 = data;
            data = data.offset(1);
            *fresh200 = ' ' as i32 as std::os::raw::c_char;
            let mut fresh201 = data;
            data = data.offset(1);
            *fresh201 = ':' as i32 as std::os::raw::c_char;
            let mut fresh202 = data;
            data = data.offset(1);
            *fresh202 = ' ' as i32 as std::os::raw::c_char;
            data = json_write_pretty_value(
                (*element).value,
                depth.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong),
                indent,
                newline,
                data,
            );
            if data.is_null() {
                return 0 as *mut std::os::raw::c_char;
            }
            element = (*element).next;
        }
        k = 0 as std::os::raw::c_int as size_t;
        while '\0' as i32 != *newline.offset(k as isize) as std::os::raw::c_int {
            let mut fresh203 = data;
            data = data.offset(1);
            *fresh203 = *newline.offset(k as isize);
            k = k.wrapping_add(1);
        }
        k = 0 as std::os::raw::c_int as size_t;
        while k < depth {
            m = 0 as std::os::raw::c_int as size_t;
            while '\0' as i32 != *indent.offset(m as isize) as std::os::raw::c_int {
                let mut fresh204 = data;
                data = data.offset(1);
                *fresh204 = *indent.offset(m as isize);
                m = m.wrapping_add(1);
            }
            k = k.wrapping_add(1);
        }
    }
    let mut fresh205 = data;
    data = data.offset(1);
    *fresh205 = '}' as i32 as std::os::raw::c_char;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty_value(
    mut value: * const crate::src::json::json_value_s,
    mut depth: std::os::raw::c_ulong,
    mut indent: * const std::os::raw::c_char,
    mut newline: * const std::os::raw::c_char,
    mut data: * mut std::os::raw::c_char,
) -> * mut std::os::raw::c_char {
    match (*value).type_0 {
        1 => return json_write_number((*value).payload as *mut json_number_s, data),
        0 => return json_write_string((*value).payload as *mut json_string_s, data),
        3 => {
            return json_write_pretty_array(
                (*value).payload as *mut json_array_s,
                depth,
                indent,
                newline,
                data,
            );
        }
        2 => {
            return json_write_pretty_object(
                (*value).payload as *mut json_object_s,
                depth,
                indent,
                newline,
                data,
            );
        }
        4 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 't' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'r' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'u' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 'e' as i32 as std::os::raw::c_char;
            return data.offset(4 as std::os::raw::c_int as isize);
        }
        5 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 'f' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'a' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 's' as i32 as std::os::raw::c_char;
            *data.offset(4 as std::os::raw::c_int as isize) = 'e' as i32 as std::os::raw::c_char;
            return data.offset(5 as std::os::raw::c_int as isize);
        }
        6 => {
            *data.offset(0 as std::os::raw::c_int as isize) = 'n' as i32 as std::os::raw::c_char;
            *data.offset(1 as std::os::raw::c_int as isize) = 'u' as i32 as std::os::raw::c_char;
            *data.offset(2 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            *data.offset(3 as std::os::raw::c_int as isize) = 'l' as i32 as std::os::raw::c_char;
            return data.offset(4 as std::os::raw::c_int as isize);
        }
        _ => return 0 as *mut std::os::raw::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn json_write_pretty<'a1>(
    mut value: * const crate::src::json::json_value_s,
    mut indent: * const std::os::raw::c_char,
    mut newline: * const std::os::raw::c_char,
    mut out_size: Option<&'a1 mut std::os::raw::c_ulong>,
) -> * mut core::ffi::c_void {
    let mut size = 0 as std::os::raw::c_int as size_t;
    let mut indent_size = 0 as std::os::raw::c_int as size_t;
    let mut newline_size = 0 as std::os::raw::c_int as size_t;
    let mut data = 0 as *mut std::os::raw::c_char;
    let mut data_end = 0 as *mut std::os::raw::c_char;
    if value.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    if indent.is_null() {
        indent = b"  \0" as *const u8 as *const std::os::raw::c_char;
    }
    if newline.is_null() {
        newline = b"\n\0" as *const u8 as *const std::os::raw::c_char;
    }
    while '\0' as i32 != *indent.offset(indent_size as isize) as std::os::raw::c_int {
        indent_size = indent_size.wrapping_add(1);
    }
    while '\0' as i32 != *newline.offset(newline_size as isize) as std::os::raw::c_int {
        newline_size = newline_size.wrapping_add(1);
    }
    if json_write_pretty_get_value_size(
        value,
        0 as std::os::raw::c_int as size_t,
        indent_size,
        newline_size,
        &mut size,
    ) != 0
    {
        return 0 as *mut std::os::raw::c_void;
    }
    size = (size as std::os::raw::c_ulong).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
        as size_t as size_t;
    data = malloc(size) as *mut std::os::raw::c_char;
    if data.is_null() {
        return 0 as *mut std::os::raw::c_void;
    }
    data_end = json_write_pretty_value(
        value,
        0 as std::os::raw::c_int as size_t,
        indent,
        newline,
        data,
    );
    if data_end.is_null() {
        free(data as *mut std::os::raw::c_void);
        return 0 as *mut std::os::raw::c_void;
    }
    *data_end = '\0' as i32 as std::os::raw::c_char;
    if !borrow(& out_size).is_none() {
        *(borrow_mut(&mut out_size)).unwrap() = size;
    }
    return data as *mut std::os::raw::c_void;
}
use crate::laertes_rt::*;
