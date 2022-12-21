
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type internal_state;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn abort() -> !;
}
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::os::raw::c_int,
    pub _IO_read_ptr: *mut std::os::raw::c_char,
    pub _IO_read_end: *mut std::os::raw::c_char,
    pub _IO_read_base: *mut std::os::raw::c_char,
    pub _IO_write_base: *mut std::os::raw::c_char,
    pub _IO_write_ptr: *mut std::os::raw::c_char,
    pub _IO_write_end: *mut std::os::raw::c_char,
    pub _IO_buf_base: *mut std::os::raw::c_char,
    pub _IO_buf_end: *mut std::os::raw::c_char,
    pub _IO_save_base: *mut std::os::raw::c_char,
    pub _IO_backup_base: *mut std::os::raw::c_char,
    pub _IO_save_end: *mut std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::os::raw::c_int,
    pub _flags2: std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::os::raw::c_ushort,
    pub _vtable_offset: std::os::raw::c_schar,
    pub _shortbuf: [std::os::raw::c_char; 1],
    pub _lock: *mut std::os::raw::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: std::os::raw::c_int,
    pub _unused2: [std::os::raw::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_int_32 = std::os::raw::c_int;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = size_t;
pub type png_alloc_size_t = png_size_t;
pub type png_fixed_point = png_int_32;
pub type png_voidp = *mut std::os::raw::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_charp = *mut std::os::raw::c_char;
pub type png_const_charp = *const std::os::raw::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_struct_def {
    pub error_fn: png_error_ptr,
    pub warning_fn: png_error_ptr,
    pub error_ptr: png_voidp,
    pub write_data_fn: png_rw_ptr,
    pub read_data_fn: png_rw_ptr,
    pub io_ptr: png_voidp,
    pub mode: png_uint_32,
    pub flags: png_uint_32,
    pub transformations: png_uint_32,
    pub zowner: png_uint_32,
    pub zstream: z_stream,
    pub zbuffer_list: png_compression_bufferp,
    pub zbuffer_size: uInt,
    pub zlib_level: std::os::raw::c_int,
    pub zlib_method: std::os::raw::c_int,
    pub zlib_window_bits: std::os::raw::c_int,
    pub zlib_mem_level: std::os::raw::c_int,
    pub zlib_strategy: std::os::raw::c_int,
    pub zlib_set_level: std::os::raw::c_int,
    pub zlib_set_method: std::os::raw::c_int,
    pub zlib_set_window_bits: std::os::raw::c_int,
    pub zlib_set_mem_level: std::os::raw::c_int,
    pub zlib_set_strategy: std::os::raw::c_int,
    pub width: png_uint_32,
    pub height: png_uint_32,
    pub num_rows: png_uint_32,
    pub usr_width: png_uint_32,
    pub rowbytes: png_size_t,
    pub iwidth: png_uint_32,
    pub row_number: png_uint_32,
    pub chunk_name: png_uint_32,
    pub prev_row: png_bytep,
    pub row_buf: png_bytep,
    pub try_row: png_bytep,
    pub tst_row: png_bytep,
    pub info_rowbytes: png_size_t,
    pub idat_size: png_uint_32,
    pub crc: png_uint_32,
    pub palette: png_colorp,
    pub num_palette: png_uint_16,
    pub num_palette_max: std::os::raw::c_int,
    pub num_trans: png_uint_16,
    pub compression: png_byte,
    pub filter: png_byte,
    pub interlaced: png_byte,
    pub pass: png_byte,
    pub do_filter: png_byte,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub usr_bit_depth: png_byte,
    pub pixel_depth: png_byte,
    pub channels: png_byte,
    pub usr_channels: png_byte,
    pub sig_bytes: png_byte,
    pub maximum_pixel_depth: png_byte,
    pub transformed_pixel_depth: png_byte,
    pub zstream_start: png_byte,
    pub background_gamma_type: png_byte,
    pub background_gamma: png_fixed_point,
    pub background: png_color_16,
    pub output_flush_fn: png_flush_ptr,
    pub flush_dist: png_uint_32,
    pub flush_rows: png_uint_32,
    pub sig_bit: png_color_8,
    pub trans_alpha: png_bytep,
    pub trans_color: png_color_16,
    pub read_row_fn: png_read_status_ptr,
    pub write_row_fn: png_write_status_ptr,
    pub free_me: png_uint_32,
    pub unknown_default: std::os::raw::c_int,
    pub num_chunk_list: std::os::raw::c_uint,
    pub chunk_list: png_bytep,
    pub big_row_buf: png_bytep,
    pub compression_type: png_byte,
    pub user_width_max: png_uint_32,
    pub user_height_max: png_uint_32,
    pub user_chunk_cache_max: png_uint_32,
    pub user_chunk_malloc_max: png_alloc_size_t,
    pub unknown_chunk: png_unknown_chunk,
    pub old_big_row_buf_size: png_size_t,
    pub read_buffer: png_bytep,
    pub read_buffer_size: png_alloc_size_t,
    pub IDAT_read_size: uInt,
    pub io_state: png_uint_32,
    pub big_prev_row: png_bytep,
    pub read_filter: [Option<unsafe extern "C" fn(_: png_row_infop,
                                                  _: png_bytep,
                                                  _: png_const_bytep)
                                 -> ()>; 4],
}
pub type png_row_infop = *mut png_row_info;
pub type png_row_info = png_row_info_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: png_uint_32,
    pub rowbytes: png_size_t,
    pub color_type: png_byte,
    pub bit_depth: png_byte,
    pub channels: png_byte,
    pub pixel_depth: png_byte,
}
pub type uInt = std::os::raw::c_uint;
pub type png_unknown_chunk = png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [png_byte; 5],
    pub data: *mut png_byte,
    pub size: png_size_t,
    pub location: png_byte,
}
pub type png_write_status_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_uint_32,
                                _: std::os::raw::c_int) -> ()>;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_read_status_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_uint_32,
                                _: std::os::raw::c_int) -> ()>;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_8 = png_color_8_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
    pub gray: png_byte,
    pub alpha: png_byte,
}
pub type png_flush_ptr = Option<unsafe extern "C" fn(_: png_structp) -> ()>;
pub type png_colorp = *mut png_color;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_compression_bufferp = *mut png_compression_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_compression_buffer {
    pub next: *mut png_compression_buffer,
    pub output: [png_byte; 1],
}
pub type z_stream = z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const std::os::raw::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: std::os::raw::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type uLong = std::os::raw::c_ulong;
pub type voidpf = *mut std::os::raw::c_void;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type Bytef = Byte;
pub type Byte = std::os::raw::c_uchar;
pub type png_rw_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_bytep, _: png_size_t)
               -> ()>;
pub type png_error_ptr
    =
    Option<unsafe extern "C" fn(_: png_structp, _: png_const_charp) -> ()>;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_ptruint = std::os::raw::c_ulong;
/* WARNINGS */
/* This function is called whenever there is a fatal error.  This function
 * should not be changed.  If there is a need to handle errors differently,
 * you should supply a replacement error function and use png_set_error_fn()
 * to replace the error function at run-time.
 */
#[no_mangle]
pub unsafe extern "C" fn png_error(mut png_ptr: png_const_structrp,
                                   mut error_message: png_const_charp) -> ! {
    if !png_ptr.is_null() && (*png_ptr).error_fn.is_some() {
        Some((*png_ptr).error_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr
                                                                                                              as
                                                                                                              *const std::os::raw::c_void
                                                                                                              as
                                                                                                              png_ptruint
                                                                                                              as
                                                                                                              *mut png_struct,
                                                                                                          error_message);
    }
    /* If the custom handler doesn't exist, or if it returns,
      use the default handler, which will not return. */
    png_default_error(png_ptr, error_message);
}
/* ERROR_TEXT */
/* Utility to safely appends strings to a buffer.  This never errors out so
 * error checking is not required in the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn png_safecat(mut buffer: png_charp,
                                     mut bufsize: size_t, mut pos: size_t,
                                     mut string: png_const_charp) -> size_t {
    if !buffer.is_null() && pos < bufsize {
        if !string.is_null() {
            while *string as std::os::raw::c_int != '\u{0}' as i32 &&
                      pos <
                          bufsize.wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) {
                let fresh0 = string;
                string = string.offset(1);
                let fresh1 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh1 as isize) = *fresh0
            }
        }
        *buffer.offset(pos as isize) = '\u{0}' as i32 as std::os::raw::c_char
    }
    return pos;
}
/* Utility to dump an unsigned value into a buffer, given a start pointer and
 * and end pointer (which should point just *beyond* the end of the buffer!)
 * Returns the pointer to the start of the formatted string.
 */
#[no_mangle]
pub unsafe extern "C" fn png_format_number(mut start: png_const_charp,
                                           mut end: png_charp,
                                           mut format: std::os::raw::c_int,
                                           mut number: png_alloc_size_t)
 -> png_charp {
    let mut count: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* number of digits output */
    let mut mincount: std::os::raw::c_int =
        1 as std::os::raw::c_int; /* minimum number required */
    let mut output: std::os::raw::c_int =
        0 as std::os::raw::c_int; /* digit output (for the fixed point format) */
    end = end.offset(-1);
    *end = '\u{0}' as i32 as std::os::raw::c_char;
    /* This is written so that the loop always runs at least once, even with
    * number zero.
    */
    while end > start as png_charp &&
              (number != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                   count < mincount) {
        static mut digits: [std::os::raw::c_char; 17] =
            unsafe {
                *::std::mem::transmute::<&[u8; 17],
                                         &[std::os::raw::c_char; 17]>(b"0123456789ABCDEF\x00")
            };
        let mut current_block_13: u64;
        match format {
            5 => {
                /* Needs five digits (the fraction) */
                mincount = 5 as std::os::raw::c_int;
                if output != 0 as std::os::raw::c_int ||
                       number.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_ulong)
                           != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    end = end.offset(-1);
                    *end =
                        digits[number.wrapping_rem(10 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                                   usize];
                    output = 1 as std::os::raw::c_int
                }
                number =
                    (number as
                         std::os::raw::c_ulong).wrapping_div(10 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        png_alloc_size_t as png_alloc_size_t;
                current_block_13 = 13797916685926291137;
            }
            2 => {
                /* Expects at least 2 digits. */
                mincount = 2 as std::os::raw::c_int;
                current_block_13 = 8801382743313005935;
            }
            1 => { current_block_13 = 8801382743313005935; }
            4 => {
                /* This format expects at least two digits */
                mincount = 2 as std::os::raw::c_int;
                current_block_13 = 361767692244619964;
            }
            3 => { current_block_13 = 361767692244619964; }
            _ => {
                /* an error */
                number = 0 as std::os::raw::c_int as png_alloc_size_t;
                current_block_13 = 13797916685926291137;
            }
        }
        match current_block_13 {
            361767692244619964 =>
            /* FALLTHROUGH */
            {
                end = end.offset(-1);
                *end =
                    digits[(number & 0xf as std::os::raw::c_int as std::os::raw::c_ulong) as
                               usize];
                number >>= 4 as std::os::raw::c_int
            }
            8801382743313005935 =>
            /* FALLTHROUGH */
            {
                end = end.offset(-1);
                *end =
                    digits[number.wrapping_rem(10 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) as usize];
                number =
                    (number as
                         std::os::raw::c_ulong).wrapping_div(10 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        png_alloc_size_t as png_alloc_size_t
            }
            _ => { }
        }
        /* Keep track of the number of digits added */
        count += 1;
        /* Float a fixed number here: */
        if format == 5 as std::os::raw::c_int && count == 5 as std::os::raw::c_int &&
               end > start as png_charp {
            /* End of the fraction, but maybe nothing was output?  In that case
          * drop the decimal point.  If the number is a true zero handle that
          * here.
          */
            if output != 0 as std::os::raw::c_int {
                end = end.offset(-1);
                *end = '.' as i32 as std::os::raw::c_char
            } else if number == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* and !output */
                end = end.offset(-1);
                *end = '0' as i32 as std::os::raw::c_char
            }
        }
    }
    return end;
}
/* This function is called whenever there is a non-fatal error.  This function
 * should not be changed.  If there is a need to handle warnings differently,
 * you should supply a replacement warning function and use
 * png_set_error_fn() to replace the warning function at run-time.
 */
#[no_mangle]
pub unsafe extern "C" fn png_warning(mut png_ptr: png_const_structrp,
                                     mut warning_message: png_const_charp) {
    let mut offset: std::os::raw::c_int = 0 as std::os::raw::c_int;
    if !png_ptr.is_null() {
        if *warning_message as std::os::raw::c_int == 0x23 as std::os::raw::c_int {
            offset = 1 as std::os::raw::c_int;
            while offset < 15 as std::os::raw::c_int {
                if *warning_message.offset(offset as isize) as std::os::raw::c_int ==
                       ' ' as i32 {
                    break ;
                }
                offset += 1
            }
        }
    }
    if !png_ptr.is_null() && (*png_ptr).warning_fn.is_some() {
        Some((*png_ptr).warning_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr
                                                                                                                as
                                                                                                                *const std::os::raw::c_void
                                                                                                                as
                                                                                                                png_ptruint
                                                                                                                as
                                                                                                                *mut png_struct,
                                                                                                            warning_message.offset(offset
                                                                                                                                       as
                                                                                                                                       isize));
    } else {
        png_default_warning(png_ptr, warning_message.offset(offset as isize));
    };
}
/* These functions support 'formatted' warning messages with up to
 * PNG_WARNING_PARAMETER_COUNT parameters.  In the format string the parameter
 * is introduced by @<number>, where 'number' starts at 1.  This follows the
 * standard established by X/Open for internationalizable error messages.
 */
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter(mut p: *mut [std::os::raw::c_char; 32],
                                               mut number: std::os::raw::c_int,
                                               mut string: png_const_charp) {
    if number > 0 as std::os::raw::c_int && number <= 8 as std::os::raw::c_int {
        png_safecat((*p.offset((number - 1 as std::os::raw::c_int) as
                                   isize)).as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 32]>() as
                        std::os::raw::c_ulong, 0 as std::os::raw::c_int as size_t, string);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter_unsigned(mut p:
                                                            *mut [std::os::raw::c_char; 32],
                                                        mut number:
                                                            std::os::raw::c_int,
                                                        mut format:
                                                            std::os::raw::c_int,
                                                        mut value:
                                                            png_alloc_size_t) {
    let mut buffer: [std::os::raw::c_char; 24] = [0; 24];
    png_warning_parameter(p, number,
                          png_format_number(buffer.as_mut_ptr() as
                                                png_const_charp,
                                            buffer.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 24]>()
                                                                           as
                                                                           std::os::raw::c_ulong
                                                                           as
                                                                           isize),
                                            format, value) as
                              png_const_charp);
}
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter_signed(mut p:
                                                          *mut [std::os::raw::c_char; 32],
                                                      mut number: std::os::raw::c_int,
                                                      mut format: std::os::raw::c_int,
                                                      mut value: png_int_32) {
    let mut u: png_alloc_size_t = 0;
    let mut str: png_charp = 0 as *mut std::os::raw::c_char;
    let mut buffer: [std::os::raw::c_char; 24] = [0; 24];
    /* Avoid overflow by doing the negate in a png_alloc_size_t: */
    u = value as png_alloc_size_t;
    if value < 0 as std::os::raw::c_int {
        u = (!u).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    str =
        png_format_number(buffer.as_mut_ptr() as png_const_charp,
                          buffer.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 24]>()
                                                         as std::os::raw::c_ulong as
                                                         isize), format, u);
    if value < 0 as std::os::raw::c_int && str > buffer.as_mut_ptr() {
        str = str.offset(-1);
        *str = '-' as i32 as std::os::raw::c_char
    }
    png_warning_parameter(p, number, str as png_const_charp);
}
#[no_mangle]
pub unsafe extern "C" fn png_formatted_warning(mut png_ptr:
                                                   png_const_structrp,
                                               mut p: *mut [std::os::raw::c_char; 32],
                                               mut message: png_const_charp) {
    /* The internal buffer is just 192 bytes - enough for all our messages,
    * overflow doesn't happen because this code checks!  If someone figures
    * out how to send us a message longer than 192 bytes, all that will
    * happen is that the message will be truncated appropriately.
    */
    let mut i: size_t =
        0 as std::os::raw::c_int as size_t; /* Index in the msg[] buffer: */
    let mut msg: [std::os::raw::c_char; 192] = [0; 192];
    /* Each iteration through the following loop writes at most one character
    * to msg[i++] then returns here to validate that there is still space for
    * the trailing '\0'.  It may (in the case of a parameter) read more than
    * one character from message[]; it must check for '\0' and continue to the
    * test if it finds the end of string.
    */
    while i <
              (::std::mem::size_of::<[std::os::raw::c_char; 192]>() as
                   std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) &&
              *message as std::os::raw::c_int != '\u{0}' as i32 {
        /* '@' at end of string is now just printed (previously it was skipped);
       * it is an error in the calling code to terminate the string with @.
       */
        if !p.is_null() && *message as std::os::raw::c_int == '@' as i32 &&
               *message.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '\u{0}' as i32 {
            message = message.offset(1); /* Consume the '@' */
            let mut parameter_char: std::os::raw::c_int = *message as std::os::raw::c_int;
            static mut valid_parameters: [std::os::raw::c_char; 10] =
                unsafe {
                    *::std::mem::transmute::<&[u8; 10],
                                             &[std::os::raw::c_char; 10]>(b"123456789\x00")
                };
            let mut parameter: std::os::raw::c_int = 0 as std::os::raw::c_int;
            /* Search for the parameter digit, the index in the string is the
          * parameter to use.
          */
            while valid_parameters[parameter as usize] as std::os::raw::c_int !=
                      parameter_char &&
                      valid_parameters[parameter as usize] as std::os::raw::c_int !=
                          '\u{0}' as i32 {
                parameter += 1
            }
            /* If the parameter digit is out of range it will just get printed. */
            if parameter < 8 as std::os::raw::c_int {
                /* Append this parameter */
                let mut parm: png_const_charp =
                    (*p.offset(parameter as isize)).as_mut_ptr() as
                        png_const_charp;
                let mut pend: png_const_charp =
                    (*p.offset(parameter as
                                   isize)).as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                   as
                                                                   std::os::raw::c_ulong
                                                                   as isize)
                        as png_const_charp;
                /* No need to copy the trailing '\0' here, but there is no guarantee
             * that parm[] has been initialized, so there is no guarantee of a
             * trailing '\0':
             */
                while i <
                          (::std::mem::size_of::<[std::os::raw::c_char; 192]>() as
                               std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong)
                          && *parm as std::os::raw::c_int != '\u{0}' as i32 &&
                          parm < pend {
                    let fresh2 = parm;
                    parm = parm.offset(1);
                    let fresh3 = i;
                    i = i.wrapping_add(1);
                    msg[fresh3 as usize] = *fresh2
                }
                /* Consume the parameter digit too: */
                message = message.offset(1);
                continue ;
            }
            /* else not a parameter and there is a character after the @ sign; just
          * copy that.  This is known not to be '\0' because of the test above.
          */
        }
        /* At this point *message can't be '\0', even in the bad parameter case
       * above where there is a lone '@' at the end of the message string.
       */
        let fresh4 = message;
        message = message.offset(1);
        let fresh5 = i;
        i = i.wrapping_add(1);
        msg[fresh5 as usize] = *fresh4
    }
    /* i is always less than (sizeof msg), so: */
    msg[i as usize] = '\u{0}' as i32 as std::os::raw::c_char;
    /* And this is the formatted message. It may be larger than
    * PNG_MAX_ERROR_TEXT, but that is only used for 'chunk' errors and these
    * are not (currently) formatted.
    */
    png_warning(png_ptr, msg.as_mut_ptr() as png_const_charp);
}
/* WARNINGS */
#[no_mangle]
pub unsafe extern "C" fn png_benign_error(mut png_ptr: png_const_structrp,
                                          mut error_message:
                                              png_const_charp) {
    if (*png_ptr).flags & 0x100000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint &&
               (*png_ptr).chunk_name != 0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_chunk_warning(png_ptr, error_message);
        } else { png_warning(png_ptr, error_message); }
    } else if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint &&
                  (*png_ptr).chunk_name != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_chunk_error(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
#[no_mangle]
pub unsafe extern "C" fn png_app_warning(mut png_ptr: png_const_structrp,
                                         mut error_message: png_const_charp) {
    if (*png_ptr).flags & 0x200000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
#[no_mangle]
pub unsafe extern "C" fn png_app_error(mut png_ptr: png_const_structrp,
                                       mut error_message: png_const_charp) {
    if (*png_ptr).flags & 0x400000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
static mut png_digit: [std::os::raw::c_char; 16] =
    ['0' as i32 as std::os::raw::c_char, '1' as i32 as std::os::raw::c_char,
     '2' as i32 as std::os::raw::c_char, '3' as i32 as std::os::raw::c_char,
     '4' as i32 as std::os::raw::c_char, '5' as i32 as std::os::raw::c_char,
     '6' as i32 as std::os::raw::c_char, '7' as i32 as std::os::raw::c_char,
     '8' as i32 as std::os::raw::c_char, '9' as i32 as std::os::raw::c_char,
     'A' as i32 as std::os::raw::c_char, 'B' as i32 as std::os::raw::c_char,
     'C' as i32 as std::os::raw::c_char, 'D' as i32 as std::os::raw::c_char,
     'E' as i32 as std::os::raw::c_char, 'F' as i32 as std::os::raw::c_char];
unsafe extern "C" fn png_format_buffer(mut png_ptr: png_const_structrp,
                                       mut buffer: png_charp,
                                       mut error_message: png_const_charp) {
    let mut chunk_name: png_uint_32 = (*png_ptr).chunk_name;
    let mut iout: std::os::raw::c_int = 0 as std::os::raw::c_int;
    let mut ishift: std::os::raw::c_int = 24 as std::os::raw::c_int;
    while ishift >= 0 as std::os::raw::c_int {
        let mut c: std::os::raw::c_int =
            (chunk_name >> ishift) as std::os::raw::c_int & 0xff as std::os::raw::c_int;
        ishift -= 8 as std::os::raw::c_int;
        if (c < 65 as std::os::raw::c_int || c > 122 as std::os::raw::c_int ||
                c > 90 as std::os::raw::c_int && c < 97 as std::os::raw::c_int) as std::os::raw::c_int
               != 0 as std::os::raw::c_int {
            let fresh6 = iout;
            iout = iout + 1;
            *buffer.offset(fresh6 as isize) =
                0x5b as std::os::raw::c_int as std::os::raw::c_char;
            let fresh7 = iout;
            iout = iout + 1;
            *buffer.offset(fresh7 as isize) =
                png_digit[((c & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int) as
                              usize];
            let fresh8 = iout;
            iout = iout + 1;
            *buffer.offset(fresh8 as isize) =
                png_digit[(c & 0xf as std::os::raw::c_int) as usize];
            let fresh9 = iout;
            iout = iout + 1;
            *buffer.offset(fresh9 as isize) =
                0x5d as std::os::raw::c_int as std::os::raw::c_char
        } else {
            let fresh10 = iout;
            iout = iout + 1;
            *buffer.offset(fresh10 as isize) = c as std::os::raw::c_char
        }
    }
    if error_message.is_null() {
        *buffer.offset(iout as isize) = '\u{0}' as i32 as std::os::raw::c_char
    } else {
        let mut iin: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let fresh11 = iout;
        iout = iout + 1;
        *buffer.offset(fresh11 as isize) = ':' as i32 as std::os::raw::c_char;
        let fresh12 = iout;
        iout = iout + 1;
        *buffer.offset(fresh12 as isize) = ' ' as i32 as std::os::raw::c_char;
        while iin < 196 as std::os::raw::c_int - 1 as std::os::raw::c_int &&
                  *error_message.offset(iin as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
            let fresh13 = iin;
            iin = iin + 1;
            let fresh14 = iout;
            iout = iout + 1;
            *buffer.offset(fresh14 as isize) =
                *error_message.offset(fresh13 as isize)
        }
        /* iin < PNG_MAX_ERROR_TEXT, so the following is safe: */
        *buffer.offset(iout as isize) = '\u{0}' as i32 as std::os::raw::c_char
    };
}
/* WARNINGS || ERROR_TEXT */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_error(mut png_ptr: png_const_structrp,
                                         mut error_message: png_const_charp)
 -> ! {
    let mut msg: [std::os::raw::c_char; 214] = [0; 214];
    if png_ptr.is_null() {
        png_error(png_ptr, error_message);
    } else {
        png_format_buffer(png_ptr, msg.as_mut_ptr(), error_message);
        png_error(png_ptr, msg.as_mut_ptr() as png_const_charp);
    };
}
/* READ && ERROR_TEXT */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_warning(mut png_ptr: png_const_structrp,
                                           mut warning_message:
                                               png_const_charp) {
    let mut msg: [std::os::raw::c_char; 214] = [0; 214];
    if png_ptr.is_null() {
        png_warning(png_ptr, warning_message);
    } else {
        png_format_buffer(png_ptr, msg.as_mut_ptr(), warning_message);
        png_warning(png_ptr, msg.as_mut_ptr() as png_const_charp);
    };
}
/* WARNINGS */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_benign_error(mut png_ptr:
                                                    png_const_structrp,
                                                mut error_message:
                                                    png_const_charp) {
    if (*png_ptr).flags & 0x100000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_chunk_warning(png_ptr, error_message);
    } else { png_chunk_error(png_ptr, error_message); };
}
/* READ */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_report(mut png_ptr: png_const_structrp,
                                          mut message: png_const_charp,
                                          mut error: std::os::raw::c_int) {
    /* This is always supported, but for just read or just write it
    * unconditionally does the right thing.
    */
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if error < 2 as std::os::raw::c_int {
            png_chunk_warning(png_ptr, message);
        } else { png_chunk_benign_error(png_ptr, message); }
    } else if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint ==
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        if error < 1 as std::os::raw::c_int {
            png_app_warning(png_ptr, message);
        } else { png_app_error(png_ptr, message); }
    };
}
/* pngerror.c - stub functions for i/o and memory allocation
 *
 * Last changed in libpng 1.6.31 [July 27, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all error handling.  Users who
 * need special error handling are expected to write replacement functions
 * and use png_set_error_fn() to use those functions.  See the instructions
 * at each function.
 */
/* This is the default error handling function.  Note that replacements for
 * this function MUST NOT RETURN, or the program will likely crash.  This
 * function is used by default, or if the program supplies NULL for the
 * error function pointer in png_set_error_fn().
 */
unsafe extern "C" fn png_default_error(mut png_ptr: png_const_structrp,
                                       mut error_message: png_const_charp)
 -> ! {
    fprintf(stderr,
            b"libpng error: %s\x00" as *const u8 as *const std::os::raw::c_char,
            if !error_message.is_null() {
                error_message
            } else { b"undefined\x00" as *const u8 as *const std::os::raw::c_char });
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    png_longjmp(png_ptr, 1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn png_longjmp(mut png_ptr: png_const_structrp,
                                     mut val: std::os::raw::c_int) -> ! {
    /* If control reaches this point, png_longjmp() must not return. The only
    * choice is to terminate the whole process (or maybe the thread); to do
    * this the ANSI-C abort() function is used unless a different method is
    * implemented by overriding the default configuration setting for
    * PNG_ABORT().
    */
    abort();
}
/* This function is called when there is a warning, but the library thinks
 * it can continue anyway.  Replacement functions don't have to do anything
 * here if you don't want them to.  In the default configuration, png_ptr is
 * not used, but it is passed in case it may be useful.
 */
unsafe extern "C" fn png_default_warning(mut png_ptr: png_const_structrp,
                                         mut warning_message:
                                             png_const_charp) {
    fprintf(stderr,
            b"libpng warning: %s\x00" as *const u8 as *const std::os::raw::c_char,
            warning_message);
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    /* Make compiler happy */
}
/* WARNINGS */
/* This function is called when the application wants to use another method
 * of handling errors and warnings.  Note that the error function MUST NOT
 * return to the calling routine or serious problems will occur.  The return
 * method used in the default routine calls longjmp(png_ptr->jmp_buf_ptr, 1)
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_error_fn(mut png_ptr: png_structrp,
                                          mut error_ptr: png_voidp,
                                          mut error_fn: png_error_ptr,
                                          mut warning_fn: png_error_ptr) {
    if png_ptr.is_null() { return }
    (*png_ptr).error_ptr = error_ptr;
    (*png_ptr).error_fn = error_fn;
    (*png_ptr).warning_fn = warning_fn;
}
/* This function returns a pointer to the error_ptr associated with the user
 * functions.  The application should free any memory associated with this
 * pointer before png_write_destroy and png_read_destroy are called.
 */
#[no_mangle]
pub unsafe extern "C" fn png_get_error_ptr(mut png_ptr: png_const_structrp)
 -> png_voidp {
    if png_ptr.is_null() { return 0 as *mut std::os::raw::c_void }
    return (*png_ptr).error_ptr;
}
/* READ || WRITE */
/* SIMPLIFIED READ || SIMPLIFIED_WRITE */
