use ::libc;
extern "C" {
    
    
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    
    
    
    
    
    
    
    
    
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type size_t = libc::c_ulong;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type brotli_alloc_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type brotli_free_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type BrotliDecoderState = crate::src::dec::decode::BrotliDecoderStateStruct;
pub type BrotliDecoderResult = libc::c_uint;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: BrotliDecoderResult = 3;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: BrotliDecoderResult = 2;
pub const BROTLI_DECODER_RESULT_SUCCESS: BrotliDecoderResult = 1;
pub const BROTLI_DECODER_RESULT_ERROR: BrotliDecoderResult = 0;
pub type BrotliDecoderParameter = libc::c_uint;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: BrotliDecoderParameter = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: BrotliDecoderParameter = 0;
pub type BrotliEncoderOperation = libc::c_uint;
pub const BROTLI_OPERATION_EMIT_METADATA: BrotliEncoderOperation = 3;
pub const BROTLI_OPERATION_FINISH: BrotliEncoderOperation = 2;
pub const BROTLI_OPERATION_FLUSH: BrotliEncoderOperation = 1;
pub const BROTLI_OPERATION_PROCESS: BrotliEncoderOperation = 0;
pub type BrotliEncoderParameter = libc::c_uint;
pub const BROTLI_PARAM_STREAM_OFFSET: BrotliEncoderParameter = 9;
pub const BROTLI_PARAM_NDIRECT: BrotliEncoderParameter = 8;
pub const BROTLI_PARAM_NPOSTFIX: BrotliEncoderParameter = 7;
pub const BROTLI_PARAM_LARGE_WINDOW: BrotliEncoderParameter = 6;
pub const BROTLI_PARAM_SIZE_HINT: BrotliEncoderParameter = 5;
pub const BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: BrotliEncoderParameter = 4;
pub const BROTLI_PARAM_LGBLOCK: BrotliEncoderParameter = 3;
pub const BROTLI_PARAM_LGWIN: BrotliEncoderParameter = 2;
pub const BROTLI_PARAM_QUALITY: BrotliEncoderParameter = 1;
pub const BROTLI_PARAM_MODE: BrotliEncoderParameter = 0;
pub type BrotliEncoderState = crate::src::enc::encode::BrotliEncoderStateStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
}
pub type Command = libc::c_uint;
pub const COMMAND_VERSION: Command = 6;
pub const COMMAND_NOOP: Command = 5;
pub const COMMAND_TEST_INTEGRITY: Command = 4;
pub const COMMAND_INVALID: Command = 3;
pub const COMMAND_HELP: Command = 2;
pub const COMMAND_DECOMPRESS: Command = 1;
pub const COMMAND_COMPRESS: Command = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub quality: libc::c_int,
    pub lgwin: libc::c_int,
    pub verbosity: libc::c_int,
    pub force_overwrite: libc::c_int,
    pub junk_source: libc::c_int,
    pub copy_stat: libc::c_int,
    pub write_to_stdout: libc::c_int,
    pub test_integrity: libc::c_int,
    pub decompress: libc::c_int,
    pub large_window: libc::c_int,
    pub output_path: *mut libc::c_char,
    pub suffix: *mut libc::c_char,
    pub not_input_indices: [libc::c_int; 20],
    pub longest_path_len: size_t,
    pub input_count: size_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub modified_path: *mut libc::c_char,
    pub iterator: libc::c_int,
    pub ignore: libc::c_int,
    pub iterator_error: libc::c_int,
    pub buffer: *mut uint8_t,
    pub input: *mut uint8_t,
    pub output: *mut uint8_t,
    pub current_input_path: *mut libc::c_char,
    pub current_output_path: *mut libc::c_char,
    pub input_file_length: int64_t,
    pub fin: *mut FILE,
    pub fout: *mut FILE,
    pub available_in: size_t,
    pub next_in: *const uint8_t,
    pub available_out: size_t,
    pub next_out: *mut uint8_t,
    pub total_in: size_t,
    pub total_out: size_t,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *mut libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn ParseInt(
    mut s: *const libc::c_char,
    mut low: libc::c_int,
    mut high: libc::c_int,
    mut result: Option<&mut libc::c_int>,
) -> libc::c_int {
    let mut value = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i= 0 as libc::c_int;
    while i < 5 as libc::c_int {
        let mut c = *s.offset(i as isize);
        if c as libc::c_int == 0 as libc::c_int {
            break;
        }
        if (*s.offset(i as isize) as libc::c_int) < '0' as i32
            || *s.offset(i as isize) as libc::c_int > '9' as i32
        {
            return 0 as libc::c_int;
        }
        value= 10 as libc::c_int * value + (c as libc::c_int - '0' as i32);
        i+= 1;
    }
    if i == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if i > 1 as libc::c_int
        && *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        return 0 as libc::c_int;
    }
    if *s.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if value < low || value > high {
        return 0 as libc::c_int;
    }
    *result.as_deref_mut().unwrap()= value;
    return 1 as libc::c_int;
}
unsafe extern "C" fn FileName(mut path: *mut libc::c_char) -> *mut libc::c_char {
    let mut separator_position: *const libc::c_char = strrchr(path, '/' as i32);
    if !separator_position.is_null() {
        path= separator_position.offset(1 as libc::c_int as isize);
    }else { (); }
    separator_position= strrchr(path, '\\' as i32);
    if !separator_position.is_null() {
        path= separator_position.offset(1 as libc::c_int as isize);
    }else { (); }
    return path;
}
unsafe extern "C" fn ParseAlias(mut name: *mut libc::c_char) -> Command {
    let mut unbrotli = b"unbrotli\0" as *const u8 as *const libc::c_char;
    let mut unbrotli_len = strlen(unbrotli);
    name= FileName(name);
    if strncmp(name, unbrotli, unbrotli_len) == 0 as libc::c_int {
        let mut terminator = *name.offset(unbrotli_len as isize);
        if terminator as libc::c_int == 0 as libc::c_int
            || terminator as libc::c_int == '.' as i32
        {
            return COMMAND_DECOMPRESS;
        }
    }
    return COMMAND_COMPRESS;
}
unsafe extern "C" fn ParseParams(mut params: Option<&mut Context>) -> Command {
    let mut argc = (*params.as_deref().unwrap()).argc;
    let mut argv = (*params.as_deref().unwrap()).argv;
    let mut i: libc::c_int = 0;
    let mut next_option_index = 0 as libc::c_int;
    let mut input_count = 0 as libc::c_int as size_t;
    let mut longest_path_len = 1 as libc::c_int as size_t;
    let mut command_set = 0 as libc::c_int;
    let mut quality_set = 0 as libc::c_int;
    let mut output_set = 0 as libc::c_int;
    let mut keep_set = 0 as libc::c_int;
    let mut lgwin_set = 0 as libc::c_int;
    let mut suffix_set = 0 as libc::c_int;
    let mut after_dash_dash = 0 as libc::c_int;
    let mut command = ParseAlias(*argv.offset(0 as libc::c_int as isize));
    i= 1 as libc::c_int;
    while i < argc {
        let mut arg: *const libc::c_char = *argv.offset(i as isize);
        let mut arg_len = if !arg.is_null() {
            strlen(arg)
        } else {();
            0 as libc::c_int as libc::c_ulong
        };
        if arg_len == 0 as libc::c_int as libc::c_ulong {
            let fresh0 = next_option_index;
            next_option_index= next_option_index + 1;
            (*params.as_deref_mut().unwrap()).not_input_indices[fresh0 as usize]= i;
        } else {
            if next_option_index > 20 as libc::c_int - 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"too many options passed\n\0" as *const u8 as *const libc::c_char,
                );
                return COMMAND_INVALID;
            }
            if after_dash_dash != 0
                || *arg.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                || arg_len == 1 as libc::c_int as libc::c_ulong
            {
                input_count= input_count.wrapping_add(1);
                if longest_path_len < arg_len {
                    longest_path_len= arg_len;
                }
            } else {
                let fresh1 = next_option_index;
                next_option_index= next_option_index + 1;
                (*params.as_deref_mut().unwrap()).not_input_indices[fresh1 as usize]= i;
                if arg_len == 2 as libc::c_int as libc::c_ulong
                    && *arg.offset(1 as libc::c_int as isize) as libc::c_int
                        == '-' as i32
                {
                    after_dash_dash= 1 as libc::c_int;
                } else if *arg.offset(1 as libc::c_int as isize) as libc::c_int
                    != '-' as i32
                {
                    let mut j: size_t = 0;
                    j= 1 as libc::c_int as size_t;
                    while j < arg_len {
                        let mut c = *arg.offset(j as isize);
                        if c as libc::c_int >= '0' as i32
                            && c as libc::c_int <= '9' as i32
                        {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).quality= c as libc::c_int - '0' as i32;
                        } else if c as libc::c_int == 'c' as i32 {
                            if output_set != 0 {
                                fprintf(
                                    stderr,
                                    b"write to standard output already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            output_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).write_to_stdout= 1 as libc::c_int;
                        } else if c as libc::c_int == 'd' as i32 {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing -d\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set= 1 as libc::c_int;
                            command= COMMAND_DECOMPRESS;
                        } else if c as libc::c_int == 'f' as i32 {
                            if (*params.as_deref().unwrap()).force_overwrite != 0 {
                                fprintf(
                                    stderr,
                                    b"force output overwrite already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params.as_deref_mut().unwrap()).force_overwrite= 1 as libc::c_int;
                        } else if c as libc::c_int == 'h' as i32 {
                            return COMMAND_HELP
                        } else if c as libc::c_int == 'j' as i32
                            || c as libc::c_int == 'k' as i32
                        {
                            if keep_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument --rm / -j or --keep / -k already set\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            keep_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).junk_source= if c as libc::c_int == 'j' as i32 {
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                        } else if c as libc::c_int == 'n' as i32 {
                            if (*params.as_deref().unwrap()).copy_stat == 0 {
                                fprintf(
                                    stderr,
                                    b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params.as_deref_mut().unwrap()).copy_stat= 0 as libc::c_int;
                        } else if c as libc::c_int == 't' as i32 {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing -t\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set= 1 as libc::c_int;
                            command= COMMAND_TEST_INTEGRITY;
                        } else if c as libc::c_int == 'v' as i32 {
                            if (*params.as_deref().unwrap()).verbosity > 0 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"argument --verbose / -v already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params.as_deref_mut().unwrap()).verbosity= 1 as libc::c_int;
                        } else if c as libc::c_int == 'V' as i32 {
                            return COMMAND_VERSION
                        } else if c as libc::c_int == 'Z' as i32 {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).quality= 11 as libc::c_int;
                        } else {
                            if c as libc::c_int != 'o' as i32
                                && c as libc::c_int != 'q' as i32
                                && c as libc::c_int != 'w' as i32
                                && c as libc::c_int != 'D' as i32
                                && c as libc::c_int != 'S' as i32
                            {
                                fprintf(
                                    stderr,
                                    b"invalid argument -%c\n\0" as *const u8
                                        as *const libc::c_char,
                                    c as libc::c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            if j.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                != arg_len
                            {
                                fprintf(
                                    stderr,
                                    b"expected parameter for argument -%c\n\0" as *const u8
                                        as *const libc::c_char,
                                    c as libc::c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            i+= 1;
                            if i == argc || (*argv.offset(i as isize)).is_null()
                                || *(*argv.offset(i as isize))
                                    .offset(0 as libc::c_int as isize) as libc::c_int
                                    == 0 as libc::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"expected parameter for argument -%c\n\0" as *const u8
                                        as *const libc::c_char,
                                    c as libc::c_int,
                                );
                                return COMMAND_INVALID;
                            }
                            let fresh2 = next_option_index;
                            next_option_index= next_option_index + 1;
                            (*params.as_deref_mut().unwrap()).not_input_indices[fresh2 as usize]= i;
                            if c as libc::c_int == 'o' as i32 {
                                if output_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"write to standard output already set (-o)\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                (*params.as_deref_mut().unwrap()).output_path= *argv.offset(i as isize);
                            } else if c as libc::c_int == 'q' as i32 {
                                if quality_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"quality already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                quality_set= ParseInt(
                                    *argv.offset(i as isize),
                                    0 as libc::c_int,
                                    11 as libc::c_int,
                                    Some(&mut (*params.as_deref_mut().unwrap()).quality),
                                );
                                if quality_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing quality value [%s]\n\0" as *const u8
                                            as *const libc::c_char,
                                        *argv.offset(i as isize),
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if c as libc::c_int == 'w' as i32 {
                                if lgwin_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                lgwin_set= ParseInt(
                                    *argv.offset(i as isize),
                                    0 as libc::c_int,
                                    24 as libc::c_int,
                                    Some(&mut (*params.as_deref_mut().unwrap()).lgwin),
                                );
                                if lgwin_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing lgwin value [%s]\n\0" as *const u8
                                            as *const libc::c_char,
                                        *argv.offset(i as isize),
                                    );
                                    return COMMAND_INVALID;
                                }
                                if (*params.as_deref().unwrap()).lgwin != 0 as libc::c_int
                                    && (*params.as_deref().unwrap()).lgwin < 10 as libc::c_int
                                {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*params).lgwin,
                                        10 as libc::c_int,
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if c as libc::c_int == 'S' as i32 {
                                if suffix_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"suffix already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                suffix_set= 1 as libc::c_int;
                                (*params.as_deref_mut().unwrap()).suffix= *argv.offset(i as isize);
                            }
                        }
                        j= j.wrapping_add(1);
                    }
                } else {
                    arg= &*arg.offset(2 as libc::c_int as isize) as *const libc::c_char;
                    if strcmp(b"best\0" as *const u8 as *const libc::c_char, arg)
                        == 0 as libc::c_int
                    {
                        if quality_set != 0 {
                            fprintf(
                                stderr,
                                b"quality already set\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        quality_set= 1 as libc::c_int;
                        (*params.as_deref_mut().unwrap()).quality= 11 as libc::c_int;
                    } else if strcmp(
                        b"decompress\0" as *const u8 as *const libc::c_char,
                        arg,
                    ) == 0 as libc::c_int
                    {
                        if command_set != 0 {
                            fprintf(
                                stderr,
                                b"command already set when parsing --decompress\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        command_set= 1 as libc::c_int;
                        command= COMMAND_DECOMPRESS;
                    } else if strcmp(b"force\0" as *const u8 as *const libc::c_char, arg)
                        == 0 as libc::c_int
                    {
                        if (*params.as_deref().unwrap()).force_overwrite != 0 {
                            fprintf(
                                stderr,
                                b"force output overwrite already set\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return COMMAND_INVALID;
                        }
                        (*params.as_deref_mut().unwrap()).force_overwrite= 1 as libc::c_int;
                    } else if strcmp(b"help\0" as *const u8 as *const libc::c_char, arg)
                        == 0 as libc::c_int
                    {
                        return COMMAND_HELP
                    } else {
                        if strcmp(b"keep\0" as *const u8 as *const libc::c_char, arg)
                            == 0 as libc::c_int
                        {
                            if keep_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument --rm / -j or --keep / -k already set\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            keep_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).junk_source= 0 as libc::c_int;
                        } else if strcmp(
                            b"no-copy-stat\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            if (*params.as_deref().unwrap()).copy_stat == 0 {
                                fprintf(
                                    stderr,
                                    b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params.as_deref_mut().unwrap()).copy_stat= 0 as libc::c_int;
                        } else if strcmp(
                            b"rm\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            if keep_set != 0 {
                                fprintf(
                                    stderr,
                                    b"argument --rm / -j or --keep / -k already set\n\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            keep_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).junk_source= 1 as libc::c_int;
                        } else if strcmp(
                            b"stdout\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            if output_set != 0 {
                                fprintf(
                                    stderr,
                                    b"write to standard output already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            output_set= 1 as libc::c_int;
                            (*params.as_deref_mut().unwrap()).write_to_stdout= 1 as libc::c_int;
                        } else if strcmp(
                            b"test\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing --test\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set= 1 as libc::c_int;
                            command= COMMAND_TEST_INTEGRITY;
                        } else if strcmp(
                            b"verbose\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            if (*params.as_deref().unwrap()).verbosity > 0 as libc::c_int {
                                fprintf(
                                    stderr,
                                    b"argument --verbose / -v already set\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return COMMAND_INVALID;
                            }
                            (*params.as_deref_mut().unwrap()).verbosity= 1 as libc::c_int;
                        } else if strcmp(
                            b"version\0" as *const u8 as *const libc::c_char,
                            arg,
                        ) == 0 as libc::c_int
                        {
                            return COMMAND_VERSION
                        } else {
                            let mut value: *const libc::c_char = strrchr(
                                arg,
                                '=' as i32,
                            );
                            let mut key_len: size_t = 0;
                            if value.is_null()
                                || *value.offset(1 as libc::c_int as isize) as libc::c_int
                                    == 0 as libc::c_int
                            {
                                fprintf(
                                    stderr,
                                    b"must pass the parameter as --%s=value\n\0" as *const u8
                                        as *const libc::c_char,
                                    arg,
                                );
                                return COMMAND_INVALID;
                            }
                            key_len= value.offset_from(arg) as libc::c_long as size_t;
                            value= value.offset(1);
                            if strncmp(
                                b"lgwin\0" as *const u8 as *const libc::c_char,
                                arg,
                                key_len,
                            ) == 0 as libc::c_int
                            {
                                if lgwin_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                lgwin_set= ParseInt(
                                    value,
                                    0 as libc::c_int,
                                    24 as libc::c_int,
                                    Some(&mut (*params.as_deref_mut().unwrap()).lgwin),
                                );
                                if lgwin_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing lgwin value [%s]\n\0" as *const u8
                                            as *const libc::c_char,
                                        value,
                                    );
                                    return COMMAND_INVALID;
                                }
                                if (*params.as_deref().unwrap()).lgwin != 0 as libc::c_int
                                    && (*params.as_deref().unwrap()).lgwin < 10 as libc::c_int
                                {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*params).lgwin,
                                        10 as libc::c_int,
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if strncmp(
                                b"large_window\0" as *const u8 as *const libc::c_char,
                                arg,
                                key_len,
                            ) == 0 as libc::c_int
                            {
                                if lgwin_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                lgwin_set= ParseInt(
                                    value,
                                    0 as libc::c_int,
                                    30 as libc::c_int,
                                    Some(&mut (*params.as_deref_mut().unwrap()).lgwin),
                                );
                                if lgwin_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing lgwin value [%s]\n\0" as *const u8
                                            as *const libc::c_char,
                                        value,
                                    );
                                    return COMMAND_INVALID;
                                }
                                if (*params.as_deref().unwrap()).lgwin != 0 as libc::c_int
                                    && (*params.as_deref().unwrap()).lgwin < 10 as libc::c_int
                                {
                                    fprintf(
                                        stderr,
                                        b"lgwin parameter (%d) smaller than the minimum (%d)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        (*params).lgwin,
                                        10 as libc::c_int,
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if strncmp(
                                b"output\0" as *const u8 as *const libc::c_char,
                                arg,
                                key_len,
                            ) == 0 as libc::c_int
                            {
                                if output_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"write to standard output already set (--output)\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                (*params.as_deref_mut().unwrap()).output_path= value;
                            } else if strncmp(
                                b"quality\0" as *const u8 as *const libc::c_char,
                                arg,
                                key_len,
                            ) == 0 as libc::c_int
                            {
                                if quality_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"quality already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                quality_set= ParseInt(
                                    value,
                                    0 as libc::c_int,
                                    11 as libc::c_int,
                                    Some(&mut (*params.as_deref_mut().unwrap()).quality),
                                );
                                if quality_set == 0 {
                                    fprintf(
                                        stderr,
                                        b"error parsing quality value [%s]\n\0" as *const u8
                                            as *const libc::c_char,
                                        value,
                                    );
                                    return COMMAND_INVALID;
                                }
                            } else if strncmp(
                                b"suffix\0" as *const u8 as *const libc::c_char,
                                arg,
                                key_len,
                            ) == 0 as libc::c_int
                            {
                                if suffix_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"suffix already set\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return COMMAND_INVALID;
                                }
                                suffix_set= 1 as libc::c_int;
                                (*params.as_deref_mut().unwrap()).suffix= value;
                            } else {
                                fprintf(
                                    stderr,
                                    b"invalid parameter: [%s]\n\0" as *const u8
                                        as *const libc::c_char,
                                    arg,
                                );
                                return COMMAND_INVALID;
                            }
                        }
                    }
                }
            }
        }
        i+= 1;
    }
    (*params.as_deref_mut().unwrap()).input_count= input_count;
    (*params.as_deref_mut().unwrap()).longest_path_len= longest_path_len;
    (*params.as_deref_mut().unwrap()).decompress= (command as libc::c_uint
        == COMMAND_DECOMPRESS as libc::c_int as libc::c_uint) as libc::c_int;
    (*params.as_deref_mut().unwrap()).test_integrity= (command as libc::c_uint
        == COMMAND_TEST_INTEGRITY as libc::c_int as libc::c_uint) as libc::c_int;
    if input_count > 1 as libc::c_int as libc::c_ulong && output_set != 0 {
        return COMMAND_INVALID;
    }
    if (*params.as_deref().unwrap()).test_integrity != 0 {
        if !(*params.as_deref().unwrap()).output_path.is_null() {
            return COMMAND_INVALID;
        }else { (); }
        if (*params.as_deref().unwrap()).write_to_stdout != 0 {
            return COMMAND_INVALID;
        }
    }
    if !(strchr((*params.as_deref().unwrap()).suffix, '/' as i32)).is_null()
        || !(strchr((*params.as_deref().unwrap()).suffix, '\\' as i32)).is_null()
    {
        return COMMAND_INVALID;
    }
    return command;
}
unsafe extern "C" fn PrintVersion() {
    let mut major = 0x1000009 as libc::c_int >> 24 as libc::c_int;
    let mut minor = 0x1000009 as libc::c_int >> 12 as libc::c_int & 0xfff as libc::c_int;
    let mut patch = 0x1000009 as libc::c_int & 0xfff as libc::c_int;
    fprintf(
        stdout,
        b"brotli %d.%d.%d\n\0" as *const u8 as *const libc::c_char,
        major,
        minor,
        patch,
    );
}
unsafe extern "C" fn PrintHelp(mut name: *const libc::c_char, mut error: libc::c_int) {
    let mut media = if error != 0 { stderr } else { stdout };
    fprintf(
        media,
        b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    fprintf(
        media,
        b"Options:\n  -#                          compression level (0-9)\n  -c, --stdout                write on standard output\n  -d, --decompress            decompress\n  -f, --force                 force output file overwrite\n  -h, --help                  display this help and exit\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        media,
        b"  -j, --rm                    remove source file(s)\n  -k, --keep                  keep source file(s) (default)\n  -n, --no-copy-stat          do not copy source file(s) attributes\n  -o FILE, --output=FILE      output file (only if 1 input file)\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        media,
        b"  -q NUM, --quality=NUM       compression level (%d-%d)\n\0" as *const u8
            as *const libc::c_char,
        0 as libc::c_int,
        11 as libc::c_int,
    );
    fprintf(
        media,
        b"  -t, --test                  test compressed file integrity\n  -v, --verbose               verbose mode\n\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        media,
        b"  -w NUM, --lgwin=NUM         set LZ77 window size (0, %d-%d)\n                              window size = 2**NUM - 16\n                              0 lets compressor choose the optimal value\n\0"
            as *const u8 as *const libc::c_char,
        10 as libc::c_int,
        24 as libc::c_int,
    );
    fprintf(
        media,
        b"  --large_window=NUM          use incompatible large-window brotli\n                              bitstream with window size (0, %d-%d)\n                              WARNING: this format is not compatible\n                              with brotli RFC 7932 and may not be\n                              decodable with regular brotli decoders\n\0"
            as *const u8 as *const libc::c_char,
        10 as libc::c_int,
        30 as libc::c_int,
    );
    fprintf(
        media,
        b"  -S SUF, --suffix=SUF        output file suffix (default:'%s')\n\0"
            as *const u8 as *const libc::c_char,
        b".br\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        media,
        b"  -V, --version               display version and exit\n  -Z, --best                  use best compression level (11) (default)\nSimple options could be coalesced, i.e. '-9kf' is equivalent to '-9 -k -f'.\nWith no FILE, or when FILE is -, read standard input.\nAll arguments after '--' are treated as files.\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn PrintablePath(
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    return if !path.is_null() {
        path
    } else {();
        b"con\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn OpenInputFile(
    mut input_path: *const libc::c_char,
    mut f: Option<&mut *mut FILE>,
) -> libc::c_int {
    *f.as_deref_mut().unwrap()= 0 as *mut FILE;
    if input_path.is_null() {();
        *f.as_deref_mut().unwrap()= fdopen(0 as libc::c_int, b"rb\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    *f.as_deref_mut().unwrap()= fopen(input_path, b"rb\0" as *const u8 as *const libc::c_char);
    if (*f.as_deref().unwrap()).is_null() {();
        fprintf(
            stderr,
            b"failed to open input file [%s]: %s\n\0" as *const u8
                as *const libc::c_char,
            PrintablePath(input_path),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn OpenOutputFile(
    mut output_path: *mut libc::c_char,
    mut f: Option<&mut *mut FILE>,
    mut force: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    *f.as_deref_mut().unwrap()= 0 as *mut FILE;
    if output_path.is_null() {();
        *f.as_deref_mut().unwrap()= fdopen(1 as libc::c_int, b"wb\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    fd= open(
        output_path,
        0o100 as libc::c_int
            | (if force != 0 { 0 as libc::c_int } else { 0o200 as libc::c_int })
            | 0o1 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        fprintf(
            stderr,
            b"failed to open output file [%s]: %s\n\0" as *const u8
                as *const libc::c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    *f.as_deref_mut().unwrap()= fdopen(fd, b"wb\0" as *const u8 as *const libc::c_char);
    if (*f.as_deref().unwrap()).is_null() {();
        fprintf(
            stderr,
            b"failed to open output file [%s]: %s\n\0" as *const u8
                as *const libc::c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn FileSize(mut path: *const libc::c_char) -> int64_t {
    let mut f = fopen(path, b"rb\0" as *const u8 as *const libc::c_char);
    let mut retval: int64_t = 0;
    if f.is_null() {();
        return -(1 as libc::c_int) as int64_t;
    }
    if fseek(f, 0 as libc::c_long, 2 as libc::c_int) != 0 as libc::c_int {
        fclose(f);
        return -(1 as libc::c_int) as int64_t;
    }
    retval= ftell(f);
    if fclose(f) != 0 as libc::c_int {
        return -(1 as libc::c_int) as int64_t;
    }
    return retval;
}
unsafe extern "C" fn CopyStat(
    mut input_path: *mut libc::c_char,
    mut output_path: *mut libc::c_char,
) {
    let mut statbuf = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut times = utimbuf { actime: 0, modtime: 0 };
    let mut res: libc::c_int = 0;
    if input_path.is_null() || output_path.is_null() {
        return;
    }
    if stat(input_path, core::ptr::addr_of_mut!(statbuf)) != 0 as libc::c_int {
        return;
    }
    times.actime= statbuf.st_atim.tv_sec;
    times.modtime= statbuf.st_mtim.tv_sec;
    utime(output_path, core::ptr::addr_of!(times));
    res= chmod(
        output_path,
        statbuf.st_mode
            & (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint,
    );
    if res != 0 as libc::c_int {
        fprintf(
            stderr,
            b"setting access bits failed for [%s]: %s\n\0" as *const u8
                as *const libc::c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
    res= chown(output_path, -(1 as libc::c_int) as uid_t, statbuf.st_gid);
    if res != 0 as libc::c_int {
        fprintf(
            stderr,
            b"setting group failed for [%s]: %s\n\0" as *const u8 as *const libc::c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
    res= chown(output_path, statbuf.st_uid, -(1 as libc::c_int) as gid_t);
    if res != 0 as libc::c_int {
        fprintf(
            stderr,
            b"setting user failed for [%s]: %s\n\0" as *const u8 as *const libc::c_char,
            PrintablePath(output_path),
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn NextFile(mut context: Option<&mut Context>) -> libc::c_int {
    let mut arg = 0 as *const libc::c_char;
    let mut arg_len: size_t = 0;
    (*context.as_deref_mut().unwrap()).iterator+= 1;
    (*context.as_deref_mut().unwrap()).input_file_length= -(1 as libc::c_int) as int64_t;
    if (*context.as_deref().unwrap()).input_count == 0 as libc::c_int as libc::c_ulong {
        if (*context.as_deref().unwrap()).iterator > 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        (*context.as_deref_mut().unwrap()).current_input_path= 0 as *const libc::c_char;
        (*context.as_deref_mut().unwrap()).current_output_path= (*context.as_deref().unwrap()).output_path;
        return 1 as libc::c_int;
    }
    while (*context.as_deref().unwrap()).iterator == (*context.as_deref().unwrap()).not_input_indices[(*context.as_deref().unwrap()).ignore as usize]
    {
        (*context.as_deref_mut().unwrap()).iterator+= 1;
        (*context.as_deref_mut().unwrap()).ignore+= 1;
    }
    if (*context.as_deref().unwrap()).iterator >= (*context.as_deref().unwrap()).argc {
        return 0 as libc::c_int;
    }
    arg= *(*context.as_deref().unwrap()).argv.offset((*context.as_deref().unwrap()).iterator as isize);
    arg_len= strlen(arg);
    if arg_len == 1 as libc::c_int as libc::c_ulong
        && *arg.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        (*context.as_deref_mut().unwrap()).current_input_path= 0 as *const libc::c_char;
        (*context.as_deref_mut().unwrap()).current_output_path= (*context.as_deref().unwrap()).output_path;
        return 1 as libc::c_int;
    }
    (*context.as_deref_mut().unwrap()).current_input_path= arg;
    (*context.as_deref_mut().unwrap()).input_file_length= FileSize(arg);
    (*context.as_deref_mut().unwrap()).current_output_path= (*context.as_deref().unwrap()).output_path;
    if !(*context.as_deref().unwrap()).output_path.is_null() {
        return 1 as libc::c_int;
    }else { (); }
    if (*context.as_deref().unwrap()).write_to_stdout != 0 {
        return 1 as libc::c_int;
    }
    strcpy((*context.as_deref().unwrap()).modified_path, arg);
    (*context.as_deref_mut().unwrap()).current_output_path= (*context.as_deref().unwrap()).modified_path;
    if (*context.as_deref().unwrap()).decompress != 0 {
        let mut suffix_len = strlen((*context.as_deref().unwrap()).suffix);
        let mut name = FileName((*context.as_deref().unwrap()).modified_path) as *mut libc::c_char;
        let mut name_suffix = 0 as *mut libc::c_char;
        let mut name_len = strlen(name);
        if name_len < suffix_len.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            fprintf(
                stderr,
                b"empty output file name for [%s] input file\n\0" as *const u8
                    as *const libc::c_char,
                PrintablePath(arg),
            );
            (*context.as_deref_mut().unwrap()).iterator_error= 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        name_suffix= name.offset(name_len as isize).offset(-(suffix_len as isize));
        if strcmp((*context.as_deref().unwrap()).suffix, name_suffix) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"input file [%s] suffix mismatch\n\0" as *const u8
                    as *const libc::c_char,
                PrintablePath(arg),
            );
            (*context.as_deref_mut().unwrap()).iterator_error= 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        *name_suffix
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return 1 as libc::c_int;
    } else {
        strcpy((*context.as_deref().unwrap()).modified_path.offset(arg_len as isize), (*context.as_deref().unwrap()).suffix);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn OpenFiles(mut context: Option<&mut Context>) -> libc::c_int {
    let mut is_ok = OpenInputFile((*context.as_deref().unwrap()).current_input_path, Some(&mut (*context.as_deref_mut().unwrap()).fin));
    if (*context.as_deref().unwrap()).test_integrity == 0 && is_ok != 0 {
        is_ok= OpenOutputFile(
            (*context.as_deref().unwrap()).current_output_path,
            Some(&mut (*context.as_deref_mut().unwrap()).fout),
            (*context.as_deref().unwrap()).force_overwrite,
        );
    }
    return is_ok;
}
unsafe extern "C" fn CloseFiles(
    mut context: Option<&mut Context>,
    mut success: libc::c_int,
) -> libc::c_int {
    let mut is_ok = 1 as libc::c_int;
    if (*context.as_deref().unwrap()).test_integrity == 0 && !(*context.as_deref().unwrap()).fout.is_null() {
        if success == 0 && !(*context.as_deref().unwrap()).current_output_path.is_null() {
            unlink((*context.as_deref().unwrap()).current_output_path);
        }
        if fclose((*context.as_deref().unwrap()).fout) != 0 as libc::c_int {
            if success != 0 {
                fprintf(
                    stderr,
                    b"fclose failed [%s]: %s\n\0" as *const u8 as *const libc::c_char,
                    PrintablePath((*context.as_deref().unwrap()).current_output_path),
                    strerror(*__errno_location()),
                );
            }
            is_ok= 0 as libc::c_int;
        }
        if success != 0 && is_ok != 0 && (*context.as_deref().unwrap()).copy_stat != 0 {
            CopyStat((*context.as_deref().unwrap()).current_input_path, (*context.as_deref().unwrap()).current_output_path);
        }
    }
    if !(*context.as_deref().unwrap()).fin.is_null() {
        if fclose((*context.as_deref().unwrap()).fin) != 0 as libc::c_int {
            if is_ok != 0 {
                fprintf(
                    stderr,
                    b"fclose failed [%s]: %s\n\0" as *const u8 as *const libc::c_char,
                    PrintablePath((*context.as_deref().unwrap()).current_input_path),
                    strerror(*__errno_location()),
                );
            }
            is_ok= 0 as libc::c_int;
        }
    }else { (); }
    if success != 0 && (*context.as_deref().unwrap()).junk_source != 0
        && !(*context.as_deref().unwrap()).current_input_path.is_null()
    {
        unlink((*context.as_deref().unwrap()).current_input_path);
    }
    (*context.as_deref_mut().unwrap()).fin= 0 as *mut FILE;
    (*context.as_deref_mut().unwrap()).fout= 0 as *mut FILE;
    return is_ok;
}
static mut kFileBufferSize: size_t = ((1 as libc::c_int) << 19 as libc::c_int) as size_t;
unsafe extern "C" fn InitializeBuffers(mut context: Option<&mut Context>) {
    (*context.as_deref_mut().unwrap()).available_in= 0 as libc::c_int as size_t;
    (*context.as_deref_mut().unwrap()).next_in= 0 as *const uint8_t;
    (*context.as_deref_mut().unwrap()).available_out= kFileBufferSize;
    (*context.as_deref_mut().unwrap()).next_out= (*context.as_deref().unwrap()).output;
    (*context.as_deref_mut().unwrap()).total_in= 0 as libc::c_int as size_t;
    (*context.as_deref_mut().unwrap()).total_out= 0 as libc::c_int as size_t;
}
unsafe extern "C" fn HasMoreInput(mut context: Option<&mut Context>) -> libc::c_int {
    return if feof((*context.as_deref().unwrap()).fin) != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn ProvideInput(mut context: Option<&mut Context>) -> libc::c_int {
    (*context.as_deref_mut().unwrap()).available_in= fread(
        (*context.as_deref().unwrap()).input as *mut libc::c_void,
        1 as libc::c_int as size_t,
        kFileBufferSize,
        (*context.as_deref().unwrap()).fin,
    );
    (*context.as_deref_mut().unwrap()).total_in= ((*context.as_deref().unwrap()).total_in as libc::c_ulong).wrapping_add((*context.as_deref().unwrap()).available_in)
        as size_t as size_t;
    (*context.as_deref_mut().unwrap()).next_in= (*context.as_deref().unwrap()).input;
    if ferror((*context.as_deref().unwrap()).fin) != 0 {
        fprintf(
            stderr,
            b"failed to read input [%s]: %s\n\0" as *const u8 as *const libc::c_char,
            PrintablePath((*context.as_deref().unwrap()).current_input_path),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn WriteOutput(mut context: Option<&mut Context>) -> libc::c_int {
    let mut out_size = (*context.as_deref().unwrap()).next_out.offset_from((*context.as_deref().unwrap()).output)
        as libc::c_long as size_t;
    (*context.as_deref_mut().unwrap()).total_out= ((*context.as_deref().unwrap()).total_out as libc::c_ulong).wrapping_add(out_size) as size_t as size_t;
    if out_size == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if (*context.as_deref().unwrap()).test_integrity != 0 {
        return 1 as libc::c_int;
    }
    fwrite(
        (*context.as_deref().unwrap()).output as *const libc::c_void,
        1 as libc::c_int as size_t,
        out_size,
        (*context.as_deref().unwrap()).fout,
    );
    if ferror((*context.as_deref().unwrap()).fout) != 0 {
        fprintf(
            stderr,
            b"failed to write output [%s]: %s\n\0" as *const u8 as *const libc::c_char,
            PrintablePath((*context.as_deref().unwrap()).current_output_path),
            strerror(*__errno_location()),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn ProvideOutput(mut context: Option<&mut Context>) -> libc::c_int {
    if WriteOutput(context.as_deref_mut()) == 0 {
        return 0 as libc::c_int;
    }
    (*context.as_deref_mut().unwrap()).available_out= kFileBufferSize;
    (*context.as_deref_mut().unwrap()).next_out= (*context.as_deref().unwrap()).output;
    return 1 as libc::c_int;
}
unsafe extern "C" fn FlushOutput(mut context: Option<&mut Context>) -> libc::c_int {
    if WriteOutput(context.as_deref_mut()) == 0 {
        return 0 as libc::c_int;
    }
    (*context.as_deref_mut().unwrap()).available_out= 0 as libc::c_int as size_t;
    return 1 as libc::c_int;
}
unsafe extern "C" fn PrintBytes(mut value: size_t) {
    if value < 1024 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"%d B\0" as *const u8 as *const libc::c_char,
            value as libc::c_int,
        );
    } else if value < 1048576 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"%0.3f KiB\0" as *const u8 as *const libc::c_char,
            value as libc::c_double / 1024.0f64,
        );
    } else if value < 1073741824 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"%0.3f MiB\0" as *const u8 as *const libc::c_char,
            value as libc::c_double / 1048576.0f64,
        );
    } else {
        fprintf(
            stderr,
            b"%0.3f GiB\0" as *const u8 as *const libc::c_char,
            value as libc::c_double / 1073741824.0f64,
        );
    };
}
unsafe extern "C" fn PrintFileProcessingProgress(mut context: *mut Context) {
    fprintf(
        stderr,
        b"[%s]: \0" as *const u8 as *const libc::c_char,
        PrintablePath((*context).current_input_path),
    );
    PrintBytes((*context).total_in);
    fprintf(stderr, b" -> \0" as *const u8 as *const libc::c_char);
    PrintBytes((*context).total_out);
}
unsafe extern "C" fn DecompressFile(
    mut context: Option<&mut Context>,
    mut s: *mut BrotliDecoderState,
) -> libc::c_int {
    let mut result = BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
    InitializeBuffers(context.as_deref_mut());
    loop {
        if result as libc::c_uint
            == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as libc::c_int as libc::c_uint
        {
            if HasMoreInput(context.as_deref_mut()) == 0 {
                fprintf(
                    stderr,
                    b"corrupt input [%s]\n\0" as *const u8 as *const libc::c_char,
                    PrintablePath((*context.as_deref().unwrap()).current_input_path),
                );
                return 0 as libc::c_int;
            }
            if ProvideInput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
        } else if result as libc::c_uint
            == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as libc::c_int as libc::c_uint
        {
            if ProvideOutput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
        } else if result as libc::c_uint
            == BROTLI_DECODER_RESULT_SUCCESS as libc::c_int as libc::c_uint
        {
            if FlushOutput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
            let mut has_more_input = ((*context.as_deref().unwrap()).available_in
                != 0 as libc::c_int as libc::c_ulong
                || fgetc((*context.as_deref().unwrap()).fin) != -(1 as libc::c_int)) as libc::c_int;
            if has_more_input != 0 {
                fprintf(
                    stderr,
                    b"corrupt input [%s]\n\0" as *const u8 as *const libc::c_char,
                    PrintablePath((*context.as_deref().unwrap()).current_input_path),
                );
                return 0 as libc::c_int;
            }
            if (*context.as_deref().unwrap()).verbosity > 0 as libc::c_int {
                fprintf(stderr, b"Decompressed \0" as *const u8 as *const libc::c_char);
                PrintFileProcessingProgress(context.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        } else {
            fprintf(
                stderr,
                b"corrupt input [%s]\n\0" as *const u8 as *const libc::c_char,
                PrintablePath((*context.as_deref().unwrap()).current_input_path),
            );
            return 0 as libc::c_int;
        }
        result= crate::src::dec::decode::BrotliDecoderDecompressStream(
            s,
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).available_in),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).next_in),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).available_out),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).next_out),
            0 as *mut size_t,
        );
    };
}
unsafe extern "C" fn DecompressFiles(mut context: *mut Context) -> libc::c_int {
    while NextFile(context.as_mut()) != 0 {
        let mut is_ok = 1 as libc::c_int;
        let mut s = crate::src::dec::decode::BrotliDecoderCreateInstance(None, None, 0 as *mut libc::c_void);
        if s.is_null() {();
            fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        crate::src::dec::decode::BrotliDecoderSetParameter(
            s,
            BROTLI_DECODER_PARAM_LARGE_WINDOW,
            1 as libc::c_uint,
        );
        is_ok= OpenFiles(context.as_mut());
        if is_ok != 0 && (*context).current_input_path.is_null()
            && (*context).force_overwrite == 0 && isatty(0 as libc::c_int) != 0
        {
            fprintf(
                stderr,
                b"Use -h help. Use -f to force input from a terminal.\n\0" as *const u8
                    as *const libc::c_char,
            );
            is_ok= 0 as libc::c_int;
        }
        if is_ok != 0 {
            is_ok= DecompressFile(context.as_mut(), s);
        }
        crate::src::dec::decode::BrotliDecoderDestroyInstance(s);
        if CloseFiles(context.as_mut(), is_ok) == 0 {
            is_ok= 0 as libc::c_int;
        }
        if is_ok == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CompressFile(
    mut context: Option<&mut Context>,
    mut s: Option<&mut BrotliEncoderState>,
) -> libc::c_int {
    let mut is_eof = 0 as libc::c_int;
    InitializeBuffers(context.as_deref_mut());
    loop {
        if (*context.as_deref().unwrap()).available_in == 0 as libc::c_int as libc::c_ulong && is_eof == 0 {
            if ProvideInput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
            is_eof= (HasMoreInput(context.as_deref_mut()) == 0) as libc::c_int;
        }
        if crate::src::enc::encode::BrotliEncoderCompressStream(
            s.as_deref_mut(),
            (if is_eof != 0 {
                BROTLI_OPERATION_FINISH as libc::c_int
            } else {
                BROTLI_OPERATION_PROCESS as libc::c_int
            }) as BrotliEncoderOperation,
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).available_in),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).next_in),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).available_out),
            core::ptr::addr_of_mut!((*context.as_deref().unwrap()).next_out),
            None,
        ) == 0
        {
            fprintf(
                stderr,
                b"failed to compress data [%s]\n\0" as *const u8 as *const libc::c_char,
                PrintablePath((*context.as_deref().unwrap()).current_input_path),
            );
            return 0 as libc::c_int;
        }
        if (*context.as_deref().unwrap()).available_out == 0 as libc::c_int as libc::c_ulong {
            if ProvideOutput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
        }
        if crate::src::enc::encode::BrotliEncoderIsFinished(s.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
            if FlushOutput(context.as_deref_mut()) == 0 {
                return 0 as libc::c_int;
            }
            if (*context.as_deref().unwrap()).verbosity > 0 as libc::c_int {
                fprintf(stderr, b"Compressed \0" as *const u8 as *const libc::c_char);
                PrintFileProcessingProgress(context.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            }
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn CompressFiles(mut context: *mut Context) -> libc::c_int {
    while NextFile(context.as_mut()) != 0 {
        let mut is_ok = 1 as libc::c_int;
        let mut s = crate::src::enc::encode::BrotliEncoderCreateInstance(None, None, 0 as *mut libc::c_void);
        if s.is_null() {();
            fprintf(stderr, b"out of memory\n\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        }
        crate::src::enc::encode::BrotliEncoderSetParameter(
            s.as_mut(),
            BROTLI_PARAM_QUALITY,
            (*context).quality as uint32_t,
        );
        if (*context).lgwin > 0 as libc::c_int {
            if (*context).lgwin > 24 as libc::c_int {
                crate::src::enc::encode::BrotliEncoderSetParameter(
                    s.as_mut(),
                    BROTLI_PARAM_LARGE_WINDOW,
                    1 as libc::c_uint,
                );
            }
            crate::src::enc::encode::BrotliEncoderSetParameter(
                s.as_mut(),
                BROTLI_PARAM_LGWIN,
                (*context).lgwin as uint32_t,
            );
        } else {
            let mut lgwin = 24 as libc::c_int as uint32_t;
            if (*context).input_file_length >= 0 as libc::c_int as libc::c_long {
                lgwin= 10 as libc::c_int as uint32_t;
                while ((1 as libc::c_int as size_t) << lgwin)
                    .wrapping_sub(16 as libc::c_int as libc::c_ulong)
                    < (*context).input_file_length as uint64_t
                {
                    lgwin= lgwin.wrapping_add(1);
                    if lgwin == 24 as libc::c_int as libc::c_uint {
                        break;
                    }
                }
            }
            crate::src::enc::encode::BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_LGWIN, lgwin);
        }
        if (*context).input_file_length > 0 as libc::c_int as libc::c_long {
            let mut size_hint = if (*context).input_file_length
                < ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_long
            {
                (*context).input_file_length as uint32_t
            } else {
                (1 as libc::c_uint) << 30 as libc::c_int
            };
            crate::src::enc::encode::BrotliEncoderSetParameter(s.as_mut(), BROTLI_PARAM_SIZE_HINT, size_hint);
        }
        is_ok= OpenFiles(context.as_mut());
        if is_ok != 0 && (*context).current_output_path.is_null()
            && (*context).force_overwrite == 0 && isatty(1 as libc::c_int) != 0
        {
            fprintf(
                stderr,
                b"Use -h help. Use -f to force output to a terminal.\n\0" as *const u8
                    as *const libc::c_char,
            );
            is_ok= 0 as libc::c_int;
        }
        if is_ok != 0 {
            is_ok= CompressFile(context.as_mut(), s.as_mut());
        }
        crate::src::enc::encode::BrotliEncoderDestroyInstance(s);
        if CloseFiles(context.as_mut(), is_ok) == 0 {
            is_ok= 0 as libc::c_int;
        }
        if is_ok == 0 {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut command = COMMAND_COMPRESS;
    let mut context = Context {
        quality: 0,
        lgwin: 0,
        verbosity: 0,
        force_overwrite: 0,
        junk_source: 0,
        copy_stat: 0,
        write_to_stdout: 0,
        test_integrity: 0,
        decompress: 0,
        large_window: 0,
        output_path: 0 as *const libc::c_char,
        suffix: 0 as *const libc::c_char,
        not_input_indices: [0; 20],
        longest_path_len: 0,
        input_count: 0,
        argc: 0,
        argv: 0 as *mut *mut libc::c_char,
        modified_path: 0 as *mut libc::c_char,
        iterator: 0,
        ignore: 0,
        iterator_error: 0,
        buffer: 0 as *mut uint8_t,
        input: 0 as *mut uint8_t,
        output: 0 as *mut uint8_t,
        current_input_path: 0 as *const libc::c_char,
        current_output_path: 0 as *const libc::c_char,
        input_file_length: 0,
        fin: 0 as *mut FILE,
        fout: 0 as *mut FILE,
        available_in: 0,
        next_in: 0 as *const uint8_t,
        available_out: 0,
        next_out: 0 as *mut uint8_t,
        total_in: 0,
        total_out: 0,
    };
    let mut is_ok = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    context.quality= 11 as libc::c_int;
    context.lgwin= -(1 as libc::c_int);
    context.verbosity= 0 as libc::c_int;
    context.force_overwrite= 0 as libc::c_int;
    context.junk_source= 0 as libc::c_int;
    context.copy_stat= 1 as libc::c_int;
    context.test_integrity= 0 as libc::c_int;
    context.write_to_stdout= 0 as libc::c_int;
    context.decompress= 0 as libc::c_int;
    context.large_window= 0 as libc::c_int;
    context.output_path= 0 as *const libc::c_char;
    context.suffix= b".br\0" as *const u8 as *const libc::c_char;
    i= 0 as libc::c_int;
    while i < 20 as libc::c_int {
        context.not_input_indices[i as usize]= 0 as libc::c_int;
        i+= 1;
    }
    context.longest_path_len= 1 as libc::c_int as size_t;
    context.input_count= 0 as libc::c_int as size_t;
    context.argc= argc;
    context.argv= argv;
    context.modified_path= 0 as *mut libc::c_char;
    context.iterator= 0 as libc::c_int;
    context.ignore= 0 as libc::c_int;
    context.iterator_error= 0 as libc::c_int;
    context.buffer= 0 as *mut uint8_t;
    context.current_input_path= 0 as *const libc::c_char;
    context.current_output_path= 0 as *const libc::c_char;
    context.fin= 0 as *mut FILE;
    context.fout= 0 as *mut FILE;
    command= ParseParams(Some(&mut context));
    if command as libc::c_uint == COMMAND_COMPRESS as libc::c_int as libc::c_uint
        || command as libc::c_uint == COMMAND_DECOMPRESS as libc::c_int as libc::c_uint
        || command as libc::c_uint
            == COMMAND_TEST_INTEGRITY as libc::c_int as libc::c_uint
    {
        if is_ok != 0 {
            let mut modified_path_len = context.longest_path_len
                .wrapping_add(strlen(context.suffix))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            context.modified_path= malloc(modified_path_len) as *mut libc::c_char;
            context.buffer= malloc(
                kFileBufferSize.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            ) as *mut uint8_t;
            if context.modified_path.is_null() || context.buffer.is_null() {
                fprintf(
                    stderr,
                    b"out of memory\n\0" as *const u8 as *const libc::c_char,
                );
                is_ok= 0 as libc::c_int;
            } else {
                context.input= context.buffer;
                context.output= context.buffer.offset(kFileBufferSize as isize);
            }
        }
    }
    if is_ok == 0 {
        command= COMMAND_NOOP;
    }
    match  command as libc::c_uint {
        5 => {}
        6 => {
            PrintVersion();
        }
        0 => {
            is_ok= CompressFiles(core::ptr::addr_of_mut!(context));
        }
        1 | 4 => {
            is_ok= DecompressFiles(core::ptr::addr_of_mut!(context));
        }
        2 | 3 | _ => {
            is_ok= (command as libc::c_uint
                == COMMAND_HELP as libc::c_int as libc::c_uint) as libc::c_int;
            PrintHelp(FileName(*argv.offset(0 as libc::c_int as isize)), is_ok);
        }
    }
    if context.iterator_error != 0 {
        is_ok= 0 as libc::c_int;
    }
    free(context.modified_path as *mut libc::c_void);
    free(context.buffer as *mut libc::c_void);
    if is_ok == 0 {
        exit(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
