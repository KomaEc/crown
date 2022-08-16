use ::libc;
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    /*
     *
     * option.h
     *
     */
    /*
     *
     * Includes.
     *
     */
    /*
     *
     * Defines.
     *
     */
    /*
     *
     * Types.
     *
     */
    /*
     *
     * Globals.
     *
     */
    /* OPTION_CODE */
    /* OPTION_CODE */
    /*
     *
     * Prototypes.
     *
     */
    /* OPTION_CODE */
    #[no_mangle]
    fn Option(_: libc::c_int, _: *mut *mut libc::c_char);
    #[no_mangle]
    static mut channelNets: libc::c_ulong;
    #[no_mangle]
    static mut channelTracks: libc::c_ulong;
    #[no_mangle]
    static mut channelTracksCopy: libc::c_ulong;
    /* CHANNEL_CODE */
    /*
     *
     * Prototypes.
     *
     */
    /* CHANNEL_CODE */
    #[no_mangle]
    fn BuildChannel();
    #[no_mangle]
    fn BuildVCG();
    #[no_mangle]
    fn AcyclicVCG();
    #[no_mangle]
    fn BuildHCG();
    #[no_mangle]
    static mut netsAssign: *mut libc::c_ulong;
    #[no_mangle]
    static mut netsAssignCopy: *mut libc::c_ulong;
    /* ASSIGN_CODE */
    /*
     *
     * Prototypes.
     *
     */
    /* ASSIGN_CODE */
    #[no_mangle]
    fn AllocAssign();
    #[no_mangle]
    fn FreeAssign();
    #[no_mangle]
    fn NetsAssign();
    #[no_mangle]
    fn InitAllocMaps();
    #[no_mangle]
    fn FreeAllocMaps();
    #[no_mangle]
    fn PrintChannel();
    #[no_mangle]
    fn DrawNets() -> libc::c_int;
    #[no_mangle]
    fn Maze1() -> libc::c_int;
    #[no_mangle]
    fn Maze2() -> libc::c_int;
    #[no_mangle]
    fn Maze3() -> libc::c_int;
}
pub const __ASSERT_FUNCTION: [libc::c_char; 24] = unsafe {
    *::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(b"void main(int, char **)\x00")
};
/*
 *
 * types.h
 *
 */
/*
 * Built-in type synonyms.
 */
/*
 * Useful constants.
 */
pub const TRUE: libc::c_int = 1 as libc::c_int;
/* TRUE */
pub const FALSE: libc::c_int = 0 as libc::c_int;
/*
 *
 * main.c
 *
 */
/*
 *
 * Includes.
 *
 */
/*
 *
 * Code.
 *
 */
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut done: libc::c_ulong = 0;
    let mut fail: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut insert: libc::c_ulong = 0;
    let mut netsLeft: libc::c_int = 0;
    Option(argc, argv);
    BuildChannel();
    BuildVCG();
    AcyclicVCG();
    BuildHCG();
    loop {
        /*
         * Setup.
         */
        AllocAssign();
        NetsAssign();
        InitAllocMaps();
        /*
         * Copy the nets assign.
         */
        channelTracksCopy = channelTracks;
        net = 1 as libc::c_int as libc::c_ulong;
        while net <= channelNets {
            *netsAssignCopy.offset(net as isize) = *netsAssign.offset(net as isize);
            net = net.wrapping_add(1)
        }
        /*
         * Route, adding a row if necessary.
         */
        fail = 0 as libc::c_int as libc::c_ulong;
        loop {
            done = TRUE as libc::c_ulong;
            netsLeft = DrawNets();
            if netsLeft != 0 as libc::c_int {
                printf(
                    b"Assignment could not route %d columns, trying maze1...\n\x00" as *const u8
                        as *const libc::c_char,
                    netsLeft,
                );
                netsLeft = Maze1();
                if netsLeft != 0 as libc::c_int {
                    printf(
                        b"Maze1 could not route %d columns, trying maze2...\n\x00" as *const u8
                            as *const libc::c_char,
                        netsLeft,
                    );
                    netsLeft = Maze2();
                    if netsLeft != 0 as libc::c_int {
                        printf(
                            b"Maze2 could not route %d columns, trying maze3...\n\x00" as *const u8
                                as *const libc::c_char,
                            netsLeft,
                        );
                        netsLeft = Maze3();
                        if netsLeft != 0 as libc::c_int {
                            printf(
                                b"Maze3 could not route %d columns, adding a track...\n\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                netsLeft,
                            );
                            /* PrintChannel(); */
                            if fail == 0 {
                                channelTracks = channelTracks.wrapping_add(1)
                            }
                            fail = fail.wrapping_add(1);
                            /*
                             * Restore the nets assign.
                             */
                            net = 1 as libc::c_int as libc::c_ulong;
                            while net <= channelNets {
                                *netsAssign.offset(net as isize) =
                                    *netsAssignCopy.offset(net as isize);
                                net = net.wrapping_add(1)
                            }
                            /*
                             * Damn!
                             */
                            done = FALSE as libc::c_ulong
                        }
                    }
                }
            }
            /*
             * Add a track at track # fail, thereby shifting
             * all tracks at that point down one track.
             */
            if done == 0 && fail != 0 {
                insert = 1 as libc::c_int as libc::c_ulong;
                while insert <= channelNets {
                    if *netsAssign.offset(insert as isize) >= fail {
                        let ref mut fresh0 = *netsAssign.offset(insert as isize);
                        *fresh0 = (*fresh0).wrapping_add(1)
                    }
                    insert = insert.wrapping_add(1)
                }
            }
            if !(done == 0
                && fail <= channelTracksCopy.wrapping_add(1 as libc::c_int as libc::c_ulong))
            {
                break;
            }
        }
        /*
         * Did adding a row within existing assignment work?
         * If not, just start over.
         */
        if done == 0 {
            FreeAllocMaps();
            FreeAssign();
            if channelTracks == channelTracksCopy.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            } else {
                __assert_fail(
                    b"channelTracks == channelTracksCopy + 1\x00" as *const u8
                        as *const libc::c_char,
                    b"main.c\x00" as *const u8 as *const libc::c_char,
                    131 as libc::c_int as libc::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        }
        if !(done == 0) {
            break;
        }
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    PrintChannel();
    /* PLUS_STATS */
    exit(0 as libc::c_int);
}
#[main]
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        )
    }
    ::std::process::exit(0i32);
}
