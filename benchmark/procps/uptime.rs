use ::libc;
extern "C" {
    #[no_mangle]
    fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
    #[no_mangle]
    fn getutxent() -> *mut utmpx;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn printf(__format: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn getopt32(argv: *mut *mut libc::c_char,
                applet_opts: *const libc::c_char, _: ...) -> uint32_t;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: __int32_t,
    pub ut_tv: C2RustUnnamed,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tv_sec: __int32_t,
    pub tv_usec: __int32_t,
}
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [libc::c_char; 0],
}
#[no_mangle]
pub unsafe extern "C" fn uptime_main(mut argc: libc::c_int,
                                     mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut updays: libc::c_uint = 0;
    let mut uphours: libc::c_uint = 0;
    let mut upminutes: libc::c_uint = 0;
    let mut opts: libc::c_uint = 0;
    let mut info: sysinfo =
        sysinfo{uptime: 0,
                loads: [0; 3],
                totalram: 0,
                freeram: 0,
                sharedram: 0,
                bufferram: 0,
                totalswap: 0,
                freeswap: 0,
                procs: 0,
                pad: 0,
                totalhigh: 0,
                freehigh: 0,
                mem_unit: 0,
                _f: [0; 0],};
    let mut current_time: *mut tm = 0 as *mut tm;
    let mut current_secs: time_t = 0;
    opts = getopt32(argv, b"s\x00" as *const u8 as *const libc::c_char);
    time(&mut current_secs);
    sysinfo(&mut info);
    if opts != 0 { current_secs -= info.uptime }
    current_time = localtime(&mut current_secs);
    if opts != 0 {
        printf(b"%04u-%02u-%02u %02u:%02u:%02u\n\x00" as *const u8 as
                   *const libc::c_char,
               (*current_time).tm_year + 1900 as libc::c_int,
               (*current_time).tm_mon + 1 as libc::c_int,
               (*current_time).tm_mday, (*current_time).tm_hour,
               (*current_time).tm_min, (*current_time).tm_sec);
        return 0 as libc::c_int
    }
    printf(b" %02u:%02u:%02u up \x00" as *const u8 as *const libc::c_char,
           (*current_time).tm_hour, (*current_time).tm_min,
           (*current_time).tm_sec);
    updays =
        (info.uptime as
             libc::c_uint).wrapping_div((60 as libc::c_int * 60 as libc::c_int
                                             * 24 as libc::c_int) as
                                            libc::c_uint);
    if updays != 0 as libc::c_int as libc::c_uint {
        printf(b"%u day%s, \x00" as *const u8 as *const libc::c_char, updays,
               if updays != 1 as libc::c_int as libc::c_uint {
                   b"s\x00" as *const u8 as *const libc::c_char
               } else { b"\x00" as *const u8 as *const libc::c_char });
    }
    upminutes =
        (info.uptime as
             libc::c_uint).wrapping_div(60 as libc::c_int as libc::c_uint);
    uphours =
        upminutes.wrapping_div(60 as libc::c_int as
                                   libc::c_uint).wrapping_rem(24 as
                                                                  libc::c_int
                                                                  as
                                                                  libc::c_uint);
    upminutes = upminutes.wrapping_rem(60 as libc::c_int as libc::c_uint);
    if uphours != 0 as libc::c_int as libc::c_uint {
        printf(b"%2u:%02u\x00" as *const u8 as *const libc::c_char, uphours,
               upminutes);
    } else {
        printf(b"%u min\x00" as *const u8 as *const libc::c_char, upminutes);
    }
    let mut ut: *mut utmpx = 0 as *mut utmpx;
    let mut users: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop  {
        ut = getutxent();
        if ut.is_null() { break ; }
        if (*ut).ut_type as libc::c_int == 7 as libc::c_int &&
               (*ut).ut_user[0 as libc::c_int as usize] as libc::c_int !=
                   '\u{0}' as i32 {
            users = users.wrapping_add(1)
        }
    }
    printf(b",  %u users\x00" as *const u8 as *const libc::c_char, users);
    printf(b",  load average: %u.%02u, %u.%02u, %u.%02u\n\x00" as *const u8 as
               *const libc::c_char,
           (info.loads[0 as libc::c_int as usize] >> 16 as libc::c_int) as
               libc::c_uint,
           ((info.loads[0 as libc::c_int as usize] &
                 (((1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(100 as libc::c_int as
                                                     libc::c_ulong) >>
                16 as libc::c_int) as libc::c_uint,
           (info.loads[1 as libc::c_int as usize] >> 16 as libc::c_int) as
               libc::c_uint,
           ((info.loads[1 as libc::c_int as usize] &
                 (((1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(100 as libc::c_int as
                                                     libc::c_ulong) >>
                16 as libc::c_int) as libc::c_uint,
           (info.loads[2 as libc::c_int as usize] >> 16 as libc::c_int) as
               libc::c_uint,
           ((info.loads[2 as libc::c_int as usize] &
                 (((1 as libc::c_int) << 16 as libc::c_int) -
                      1 as libc::c_int) as
                     libc::c_ulong).wrapping_mul(100 as libc::c_int as
                                                     libc::c_ulong) >>
                16 as libc::c_int) as libc::c_uint);
    return 0 as libc::c_int;
}
