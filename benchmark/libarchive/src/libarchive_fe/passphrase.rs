use ::libc;
extern "C" {
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn getpid() -> __pid_t;
    #[no_mangle]
    fn lafe_errc(eval: libc::c_int, code: libc::c_int, fmt: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    #[no_mangle]
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    #[no_mangle]
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    #[no_mangle]
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction:
        Option<unsafe extern "C" fn(_: libc::c_int, _: *mut siginfo_t, _: *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_9,
    pub _timer: C2RustUnnamed_8,
    pub _rt: C2RustUnnamed_7,
    pub _sigchld: C2RustUnnamed_6,
    pub _sigfault: C2RustUnnamed_3,
    pub _sigpoll: C2RustUnnamed_2,
    pub _sigsys: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _addr_bnd: C2RustUnnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
pub type speed_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type tcflag_t = libc::c_uint;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
pub const EINTR: libc::c_int = 4 as libc::c_int;
pub const EINVAL: libc::c_int = 22 as libc::c_int;
pub const ENOTTY: libc::c_int = 25 as libc::c_int;
pub const errno: libc::c_int = *__errno_location();
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const STDIN_FILENO: libc::c_int = 0 as libc::c_int;
pub const STDERR_FILENO: libc::c_int = 2 as libc::c_int;
pub const RPP_ECHO_OFF: libc::c_int = 0 as libc::c_int;
/* Turn off echo (default). */
pub const RPP_ECHO_ON: libc::c_int = 0x1 as libc::c_int;
/* Leave echo on. */
pub const RPP_REQUIRE_TTY: libc::c_int = 0x2 as libc::c_int;
/* Fail if there is no tty. */
pub const RPP_FORCELOWER: libc::c_int = 0x4 as libc::c_int;
/* Force input to lower case. */
pub const RPP_FORCEUPPER: libc::c_int = 0x8 as libc::c_int;
/* Force input to upper case. */
pub const RPP_SEVENBIT: libc::c_int = 0x10 as libc::c_int;
/* Strip the high bit from input. */
pub const RPP_STDIN: libc::c_int = 0x20 as libc::c_int;
pub const __ASSERT_FUNCTION: [libc::c_char; 18] =
    unsafe { *::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"void handler(int)\x00") };
pub const O_RDWR: libc::c_int = 0o2 as libc::c_int;
pub const _PATH_TTY: [libc::c_char; 9] =
    unsafe { *::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"/dev/tty\x00") };
pub const SIGINT: libc::c_int = 2 as libc::c_int;
pub const SIGTERM: libc::c_int = 15 as libc::c_int;
pub const SIGHUP: libc::c_int = 1 as libc::c_int;
pub const SIGQUIT: libc::c_int = 3 as libc::c_int;
pub const SIGPIPE: libc::c_int = 13 as libc::c_int;
pub const SIGALRM: libc::c_int = 14 as libc::c_int;
pub const SIGTSTP: libc::c_int = 20 as libc::c_int;
pub const SIGTTIN: libc::c_int = 21 as libc::c_int;
pub const SIGTTOU: libc::c_int = 22 as libc::c_int;
pub const ECHONL: libc::c_int = 0o100 as libc::c_int;
pub const ECHO: libc::c_int = 0o10 as libc::c_int;
/* Read from stdin, not /dev/tty */
/* _WIN32 && !__CYGWIN__ */
pub const _T_FLUSH: libc::c_int = 2 as libc::c_int;
static mut signo: [sig_atomic_t; 23] = [0; 23];
unsafe extern "C" fn handler(mut s: libc::c_int) {
    if s <= (if (if (if 14 as libc::c_int > 1 as libc::c_int {
        14 as libc::c_int
    } else {
        1 as libc::c_int
    }) > (if 2 as libc::c_int > 13 as libc::c_int {
        2 as libc::c_int
    } else {
        13 as libc::c_int
    }) {
        (if 14 as libc::c_int > 1 as libc::c_int {
            14 as libc::c_int
        } else {
            1 as libc::c_int
        })
    } else {
        (if 2 as libc::c_int > 13 as libc::c_int {
            2 as libc::c_int
        } else {
            13 as libc::c_int
        })
    }) > (if (if 3 as libc::c_int > 15 as libc::c_int {
        3 as libc::c_int
    } else {
        15 as libc::c_int
    }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
        20 as libc::c_int
    } else {
        21 as libc::c_int
    }) > 22 as libc::c_int
    {
        (if 20 as libc::c_int > 21 as libc::c_int {
            20 as libc::c_int
        } else {
            21 as libc::c_int
        })
    } else {
        22 as libc::c_int
    }) {
        (if 3 as libc::c_int > 15 as libc::c_int {
            3 as libc::c_int
        } else {
            15 as libc::c_int
        })
    } else {
        (if (if 20 as libc::c_int > 21 as libc::c_int {
            20 as libc::c_int
        } else {
            21 as libc::c_int
        }) > 22 as libc::c_int
        {
            (if 20 as libc::c_int > 21 as libc::c_int {
                20 as libc::c_int
            } else {
                21 as libc::c_int
            })
        } else {
            22 as libc::c_int
        })
    }) {
        (if (if 14 as libc::c_int > 1 as libc::c_int {
            14 as libc::c_int
        } else {
            1 as libc::c_int
        }) > (if 2 as libc::c_int > 13 as libc::c_int {
            2 as libc::c_int
        } else {
            13 as libc::c_int
        }) {
            (if 14 as libc::c_int > 1 as libc::c_int {
                14 as libc::c_int
            } else {
                1 as libc::c_int
            })
        } else {
            (if 2 as libc::c_int > 13 as libc::c_int {
                2 as libc::c_int
            } else {
                13 as libc::c_int
            })
        })
    } else {
        (if (if 3 as libc::c_int > 15 as libc::c_int {
            3 as libc::c_int
        } else {
            15 as libc::c_int
        }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
            20 as libc::c_int
        } else {
            21 as libc::c_int
        }) > 22 as libc::c_int
        {
            (if 20 as libc::c_int > 21 as libc::c_int {
                20 as libc::c_int
            } else {
                21 as libc::c_int
            })
        } else {
            22 as libc::c_int
        }) {
            (if 3 as libc::c_int > 15 as libc::c_int {
                3 as libc::c_int
            } else {
                15 as libc::c_int
            })
        } else {
            (if (if 20 as libc::c_int > 21 as libc::c_int {
                20 as libc::c_int
            } else {
                21 as libc::c_int
            }) > 22 as libc::c_int
            {
                (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                })
            } else {
                22 as libc::c_int
            })
        })
    }) {
    } else {
        __assert_fail(
            b"s <= MAX_SIGNO\x00" as *const u8 as *const libc::c_char,
            b"libarchive_fe/passphrase.c\x00" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    }
    ::std::ptr::write_volatile(
        &mut signo[s as usize] as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn readpassphrase(
    mut prompt: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut nr: ssize_t = 0;
    let mut input: libc::c_int = 0;
    let mut output: libc::c_int = 0;
    let mut save_errno: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut need_restart: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut term: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut oterm: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savealrm: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saveint: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savehup: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savequit: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut saveterm: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savetstp: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savettin: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savettou: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut savepipe: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    /* I suppose we could alloc on demand in this case (XXX). */
    if bufsiz == 0 as libc::c_int as libc::c_ulong {
        errno = EINVAL;
        return 0 as *mut libc::c_char;
    }
    loop {
        i = 0 as libc::c_int;
        while i
            <= (if (if (if 14 as libc::c_int > 1 as libc::c_int {
                14 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (if 2 as libc::c_int > 13 as libc::c_int {
                2 as libc::c_int
            } else {
                13 as libc::c_int
            }) {
                (if 14 as libc::c_int > 1 as libc::c_int {
                    14 as libc::c_int
                } else {
                    1 as libc::c_int
                })
            } else {
                (if 2 as libc::c_int > 13 as libc::c_int {
                    2 as libc::c_int
                } else {
                    13 as libc::c_int
                })
            }) > (if (if 3 as libc::c_int > 15 as libc::c_int {
                3 as libc::c_int
            } else {
                15 as libc::c_int
            }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
                20 as libc::c_int
            } else {
                21 as libc::c_int
            }) > 22 as libc::c_int
            {
                (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                })
            } else {
                22 as libc::c_int
            }) {
                (if 3 as libc::c_int > 15 as libc::c_int {
                    3 as libc::c_int
                } else {
                    15 as libc::c_int
                })
            } else {
                (if (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                }) > 22 as libc::c_int
                {
                    (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    })
                } else {
                    22 as libc::c_int
                })
            }) {
                (if (if 14 as libc::c_int > 1 as libc::c_int {
                    14 as libc::c_int
                } else {
                    1 as libc::c_int
                }) > (if 2 as libc::c_int > 13 as libc::c_int {
                    2 as libc::c_int
                } else {
                    13 as libc::c_int
                }) {
                    (if 14 as libc::c_int > 1 as libc::c_int {
                        14 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })
                } else {
                    (if 2 as libc::c_int > 13 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        13 as libc::c_int
                    })
                })
            } else {
                (if (if 3 as libc::c_int > 15 as libc::c_int {
                    3 as libc::c_int
                } else {
                    15 as libc::c_int
                }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                }) > 22 as libc::c_int
                {
                    (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    })
                } else {
                    22 as libc::c_int
                }) {
                    (if 3 as libc::c_int > 15 as libc::c_int {
                        3 as libc::c_int
                    } else {
                        15 as libc::c_int
                    })
                } else {
                    (if (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    }) > 22 as libc::c_int
                    {
                        (if 20 as libc::c_int > 21 as libc::c_int {
                            20 as libc::c_int
                        } else {
                            21 as libc::c_int
                        })
                    } else {
                        22 as libc::c_int
                    })
                })
            })
        {
            ::std::ptr::write_volatile(
                &mut signo[i as usize] as *mut sig_atomic_t,
                0 as libc::c_int,
            );
            i += 1
        }
        nr = -(1 as libc::c_int) as ssize_t;
        save_errno = 0 as libc::c_int;
        need_restart = 0 as libc::c_int;
        /*
         * Read and write to /dev/tty if available.  If not, read from
         * stdin and write to stderr unless a tty is required.
         */
        if flags & RPP_STDIN != 0 || {
            output = open(_PATH_TTY.as_ptr(), O_RDWR);
            input = output;
            (input) == -(1 as libc::c_int)
        } {
            if flags & RPP_REQUIRE_TTY != 0 {
                errno = ENOTTY;
                return 0 as *mut libc::c_char;
            }
            input = STDIN_FILENO;
            output = STDERR_FILENO
        }
        /*
         * Turn off echo if possible.
         * If we are using a tty but are not the foreground pgrp this will
         * generate SIGTTOU, so do it *before* installing the signal handlers.
         */
        if input != STDIN_FILENO && tcgetattr(input, &mut oterm) == 0 as libc::c_int {
            memcpy(
                &mut term as *mut termios as *mut libc::c_void,
                &mut oterm as *mut termios as *const libc::c_void,
                ::std::mem::size_of::<termios>() as libc::c_ulong,
            );
            if flags & RPP_ECHO_ON == 0 {
                term.c_lflag &= !(ECHO | ECHONL) as libc::c_uint
            }
            tcsetattr(input, _T_FLUSH, &mut term);
        } else {
            memset(
                &mut term as *mut termios as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<termios>() as libc::c_ulong,
            );
            term.c_lflag |= ECHO as libc::c_uint;
            memset(
                &mut oterm as *mut termios as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<termios>() as libc::c_ulong,
            );
            oterm.c_lflag |= ECHO as libc::c_uint
        }
        /*
         * Catch signals that would otherwise cause the user to end
         * up with echo turned off in the shell.  Don't worry about
         * things like SIGXCPU and SIGVTALRM for now.
         */
        sigemptyset(&mut sa.sa_mask); /* don't restart system calls */
        sa.sa_flags = 0 as libc::c_int;
        sa.__sigaction_handler.sa_handler =
            Some(handler as unsafe extern "C" fn(_: libc::c_int) -> ());
        /* Keep this list in sync with MAX_SIGNO! */
        sigaction(SIGALRM, &mut sa, &mut savealrm);
        sigaction(SIGHUP, &mut sa, &mut savehup);
        sigaction(SIGINT, &mut sa, &mut saveint);
        sigaction(SIGPIPE, &mut sa, &mut savepipe);
        sigaction(SIGQUIT, &mut sa, &mut savequit);
        sigaction(SIGTERM, &mut sa, &mut saveterm);
        sigaction(SIGTSTP, &mut sa, &mut savetstp);
        sigaction(SIGTTIN, &mut sa, &mut savettin);
        sigaction(SIGTTOU, &mut sa, &mut savettou);
        if flags & RPP_STDIN == 0 {
            let mut r: libc::c_int =
                write(output, prompt as *const libc::c_void, strlen(prompt)) as libc::c_int;
        }
        end = buf
            .offset(bufsiz as isize)
            .offset(-(1 as libc::c_int as isize));
        p = buf;
        loop {
            nr = read(
                input,
                &mut ch as *mut libc::c_char as *mut libc::c_void,
                1 as libc::c_int as size_t,
            );
            if !(nr == 1 as libc::c_int as libc::c_long
                && ch as libc::c_int != '\n' as i32
                && ch as libc::c_int != '\r' as i32)
            {
                break;
            }
            if p < end {
                if flags & RPP_SEVENBIT != 0 {
                    ch = (ch as libc::c_int & 0x7f as libc::c_int) as libc::c_char
                }
                if *(*__ctype_b_loc()).offset(ch as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    if flags & RPP_FORCELOWER != 0 {
                        ch = ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                                    __res =
                                        if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        }
                                } else {
                                    __res = tolower(ch as libc::c_uchar as libc::c_int)
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(ch as libc::c_uchar as libc::c_int as isize)
                            }
                            __res
                        }) as libc::c_char
                    }
                    if flags & RPP_FORCEUPPER != 0 {
                        ch = ({
                            let mut __res: libc::c_int = 0;
                            if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                                    __res =
                                        if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        }
                                } else {
                                    __res = toupper(ch as libc::c_uchar as libc::c_int)
                                }
                            } else {
                                __res = *(*__ctype_toupper_loc())
                                    .offset(ch as libc::c_uchar as libc::c_int as isize)
                            }
                            __res
                        }) as libc::c_char
                    }
                }
                let fresh0 = p;
                p = p.offset(1);
                *fresh0 = ch
            }
        }
        *p = '\u{0}' as i32 as libc::c_char;
        save_errno = errno;
        if term.c_lflag & ECHO as libc::c_uint == 0 {
            let mut r_0: libc::c_int = write(
                output,
                b"\n\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            ) as libc::c_int;
        }
        /* Restore old terminal settings and signals. */
        if memcmp(
            &mut term as *mut termios as *const libc::c_void,
            &mut oterm as *mut termios as *const libc::c_void,
            ::std::mem::size_of::<termios>() as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            let sigttou: libc::c_int = signo[SIGTTOU as usize];
            /* Ignore SIGTTOU generated when we are not the fg pgrp. */
            while tcsetattr(input, _T_FLUSH, &mut oterm) == -(1 as libc::c_int)
                && errno == EINTR
                && signo[SIGTTOU as usize] == 0
            {}
            ::std::ptr::write_volatile(&mut signo[SIGTTOU as usize] as *mut sig_atomic_t, sigttou)
        }
        sigaction(SIGALRM, &mut savealrm, NULL as *mut sigaction);
        sigaction(SIGHUP, &mut savehup, NULL as *mut sigaction);
        sigaction(SIGINT, &mut saveint, NULL as *mut sigaction);
        sigaction(SIGQUIT, &mut savequit, NULL as *mut sigaction);
        sigaction(SIGPIPE, &mut savepipe, NULL as *mut sigaction);
        sigaction(SIGTERM, &mut saveterm, NULL as *mut sigaction);
        sigaction(SIGTSTP, &mut savetstp, NULL as *mut sigaction);
        sigaction(SIGTTIN, &mut savettin, NULL as *mut sigaction);
        sigaction(SIGTTOU, &mut savettou, NULL as *mut sigaction);
        if input != STDIN_FILENO {
            close(input);
        }
        /*
         * If we were interrupted by a signal, resend it to ourselves
         * now that we have restored the signal handlers.
         */
        i = 0 as libc::c_int;
        while i
            <= (if (if (if 14 as libc::c_int > 1 as libc::c_int {
                14 as libc::c_int
            } else {
                1 as libc::c_int
            }) > (if 2 as libc::c_int > 13 as libc::c_int {
                2 as libc::c_int
            } else {
                13 as libc::c_int
            }) {
                (if 14 as libc::c_int > 1 as libc::c_int {
                    14 as libc::c_int
                } else {
                    1 as libc::c_int
                })
            } else {
                (if 2 as libc::c_int > 13 as libc::c_int {
                    2 as libc::c_int
                } else {
                    13 as libc::c_int
                })
            }) > (if (if 3 as libc::c_int > 15 as libc::c_int {
                3 as libc::c_int
            } else {
                15 as libc::c_int
            }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
                20 as libc::c_int
            } else {
                21 as libc::c_int
            }) > 22 as libc::c_int
            {
                (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                })
            } else {
                22 as libc::c_int
            }) {
                (if 3 as libc::c_int > 15 as libc::c_int {
                    3 as libc::c_int
                } else {
                    15 as libc::c_int
                })
            } else {
                (if (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                }) > 22 as libc::c_int
                {
                    (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    })
                } else {
                    22 as libc::c_int
                })
            }) {
                (if (if 14 as libc::c_int > 1 as libc::c_int {
                    14 as libc::c_int
                } else {
                    1 as libc::c_int
                }) > (if 2 as libc::c_int > 13 as libc::c_int {
                    2 as libc::c_int
                } else {
                    13 as libc::c_int
                }) {
                    (if 14 as libc::c_int > 1 as libc::c_int {
                        14 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })
                } else {
                    (if 2 as libc::c_int > 13 as libc::c_int {
                        2 as libc::c_int
                    } else {
                        13 as libc::c_int
                    })
                })
            } else {
                (if (if 3 as libc::c_int > 15 as libc::c_int {
                    3 as libc::c_int
                } else {
                    15 as libc::c_int
                }) > (if (if 20 as libc::c_int > 21 as libc::c_int {
                    20 as libc::c_int
                } else {
                    21 as libc::c_int
                }) > 22 as libc::c_int
                {
                    (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    })
                } else {
                    22 as libc::c_int
                }) {
                    (if 3 as libc::c_int > 15 as libc::c_int {
                        3 as libc::c_int
                    } else {
                        15 as libc::c_int
                    })
                } else {
                    (if (if 20 as libc::c_int > 21 as libc::c_int {
                        20 as libc::c_int
                    } else {
                        21 as libc::c_int
                    }) > 22 as libc::c_int
                    {
                        (if 20 as libc::c_int > 21 as libc::c_int {
                            20 as libc::c_int
                        } else {
                            21 as libc::c_int
                        })
                    } else {
                        22 as libc::c_int
                    })
                })
            })
        {
            if signo[i as usize] != 0 {
                kill(getpid(), i);
                match i {
                    SIGTSTP | SIGTTIN | SIGTTOU => need_restart = 1 as libc::c_int,
                    _ => {}
                }
            }
            i += 1
        }
        if !(need_restart != 0) {
            break;
        }
    }
    if save_errno != 0 {
        errno = save_errno
    }
    return if nr == -(1 as libc::c_int) as libc::c_long {
        NULL as *mut libc::c_char
    } else {
        buf
    };
}
/*-
 * Copyright (c) 2014 Michihiro NAKAJIMA
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR(S) ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* _WIN32 && !__CYGWIN__ */
/* HAVE_READPASSPHRASE */
#[no_mangle]
pub unsafe extern "C" fn lafe_readpassphrase(
    mut prompt: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsiz: size_t,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = readpassphrase(prompt, buf, bufsiz, RPP_ECHO_OFF);
    if p.is_null() {
        match errno {
            EINTR => {}
            _ => {
                lafe_errc(
                    1 as libc::c_int,
                    errno,
                    b"Couldn\'t read passphrase\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    return p;
}
