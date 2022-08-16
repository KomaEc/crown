use ::libc;
extern "C" {
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static mut tracksTopNotPref: *mut libc::c_ulong;
    #[no_mangle]
    static mut tracksBotNotPref: *mut libc::c_ulong;
    #[no_mangle]
    static mut tracksNotPref: *mut libc::c_ulong;
    #[no_mangle]
    static mut cardNotPref: libc::c_ulong;
    #[no_mangle]
    static mut cardTopNotPref: libc::c_ulong;
    #[no_mangle]
    static mut cardBotNotPref: libc::c_ulong;
    /*
     *
     * channel.h
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
    /* CHANNEL_CODE */
    #[no_mangle]
    static mut TOP: *mut libc::c_ulong;
    #[no_mangle]
    static mut BOT: *mut libc::c_ulong;
    #[no_mangle]
    static mut channelNets: libc::c_ulong;
    #[no_mangle]
    static mut channelColumns: libc::c_ulong;
    #[no_mangle]
    static mut channelTracks: libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _constraintVCGType {
    pub top: libc::c_ulong,
    pub bot: libc::c_ulong,
    pub col: libc::c_ulong,
    pub removed: libc::c_ulong,
}
/*
 *
 * Types.
 *
 */
pub type constraintVCGType = _constraintVCGType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _nodeVCGType {
    pub netsAboveHook: *mut constraintVCGType,
    pub netsAbove: libc::c_ulong,
    pub netsAboveLabel: libc::c_ulong,
    pub netsAboveReached: libc::c_ulong,
    pub netsBelowHook: *mut constraintVCGType,
    pub netsBelow: libc::c_ulong,
    pub netsBelowLabel: libc::c_ulong,
    pub netsBelowReached: libc::c_ulong,
}
pub type nodeVCGType = _nodeVCGType;
pub const NULL: libc::c_int = 0 as libc::c_int;
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
 * vcg.h
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
pub const FULL: libc::c_int = 6 as libc::c_int;
/*
 *
 * Globals.
 *
 */
#[no_mangle]
pub static mut VCG: *mut nodeVCGType = 0 as *const nodeVCGType as *mut nodeVCGType;
#[no_mangle]
pub static mut storageRootVCG: *mut constraintVCGType =
    0 as *const constraintVCGType as *mut constraintVCGType;
#[no_mangle]
pub static mut storageVCG: *mut constraintVCGType =
    0 as *const constraintVCGType as *mut constraintVCGType;
#[no_mangle]
pub static mut storageLimitVCG: libc::c_ulong = 0;
#[no_mangle]
pub static mut removeVCG: *mut *mut constraintVCGType =
    0 as *const *mut constraintVCGType as *mut *mut constraintVCGType;
#[no_mangle]
pub static mut removeTotalVCG: libc::c_ulong = 0;
#[no_mangle]
pub static mut SCC: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut totalSCC: libc::c_ulong = 0;
#[no_mangle]
pub static mut perSCC: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
/* VCG_CODE */
/* VCG_CODE */
/*
 *
 * Prototypes.
 *
 */
/*
 *
 * vcg.c
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
#[no_mangle]
pub unsafe extern "C" fn AllocVCG() {
    VCG = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<nodeVCGType>() as libc::c_ulong),
    ) as *mut nodeVCGType;
    storageRootVCG = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_mul(::std::mem::size_of::<constraintVCGType>() as libc::c_ulong),
    ) as *mut constraintVCGType;
    storageVCG = storageRootVCG;
    storageLimitVCG = channelNets
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong));
    SCC = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    perSCC = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    removeVCG = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_mul(::std::mem::size_of::<*mut constraintVCGType>() as libc::c_ulong),
    ) as *mut *mut constraintVCGType;
}
#[no_mangle]
pub unsafe extern "C" fn FreeVCG() {
    free(VCG as *mut libc::c_void);
    free(storageRootVCG as *mut libc::c_void);
    storageLimitVCG = 0 as libc::c_int as libc::c_ulong;
    free(SCC as *mut libc::c_void);
    free(perSCC as *mut libc::c_void);
    free(removeVCG as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn BuildVCG() {
    let mut col: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut constraint: libc::c_ulong = 0;
    let mut check: libc::c_ulong = 0;
    let mut add: libc::c_ulong = 0;
    /*
     * Allocate VCG storage.
     */
    AllocVCG();
    /*
     * Build VCG one net at a time.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        /*
         * Above constraints.
         */
        constraint = 0 as libc::c_int as libc::c_ulong;
        let ref mut fresh0 = (*VCG.offset(net as isize)).netsAboveHook;
        *fresh0 = storageVCG;
        col = 1 as libc::c_int as libc::c_ulong;
        while col <= channelColumns {
            if *TOP.offset(col as isize) == net
                && *BOT.offset(col as isize) != net
                && *BOT.offset(col as isize) != 0 as libc::c_int as libc::c_ulong
            {
                /*
                 * Check constraint already exist.
                 */
                add = TRUE as libc::c_ulong;
                check = 0 as libc::c_int as libc::c_ulong;
                while check < constraint {
                    if (*(*VCG.offset(net as isize))
                        .netsAboveHook
                        .offset(check as isize))
                    .bot == *BOT.offset(col as isize)
                    {
                        add = FALSE as libc::c_ulong;
                        break;
                    } else {
                        check = check.wrapping_add(1)
                    }
                }
                /*
                 * Add constraint.
                 */
                if add != 0 {
                    if storageLimitVCG > 0 as libc::c_int as libc::c_ulong {
                    } else {
                        __assert_fail(
                            b"storageLimitVCG > 0\x00" as *const u8 as *const libc::c_char,
                            b"vcg.c\x00" as *const u8 as *const libc::c_char,
                            96 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                                b"void BuildVCG(void)\x00",
                            ))
                            .as_ptr(),
                        );
                    }
                    (*(*VCG.offset(net as isize))
                        .netsAboveHook
                        .offset(constraint as isize))
                    .top = *TOP.offset(col as isize);
                    (*(*VCG.offset(net as isize))
                        .netsAboveHook
                        .offset(constraint as isize))
                    .bot = *BOT.offset(col as isize);
                    (*(*VCG.offset(net as isize))
                        .netsAboveHook
                        .offset(constraint as isize))
                    .col = col;
                    (*(*VCG.offset(net as isize))
                        .netsAboveHook
                        .offset(constraint as isize))
                    .removed = FALSE as libc::c_ulong;
                    storageVCG = storageVCG.offset(1);
                    storageLimitVCG = storageLimitVCG.wrapping_sub(1);
                    constraint = constraint.wrapping_add(1)
                }
            }
            col = col.wrapping_add(1)
        }
        (*VCG.offset(net as isize)).netsAbove = constraint;
        /*
         * Below constraints.
         */
        constraint = 0 as libc::c_int as libc::c_ulong;
        let ref mut fresh1 = (*VCG.offset(net as isize)).netsBelowHook;
        *fresh1 = storageVCG;
        col = 1 as libc::c_int as libc::c_ulong;
        while col <= channelColumns {
            if *BOT.offset(col as isize) == net
                && *TOP.offset(col as isize) != net
                && *TOP.offset(col as isize) != 0 as libc::c_int as libc::c_ulong
            {
                /*
                 * Check constraint already exist.
                 */
                add = TRUE as libc::c_ulong;
                check = 0 as libc::c_int as libc::c_ulong;
                while check < constraint {
                    if (*(*VCG.offset(net as isize))
                        .netsBelowHook
                        .offset(check as isize))
                    .top == *TOP.offset(col as isize)
                    {
                        add = FALSE as libc::c_ulong;
                        break;
                    } else {
                        check = check.wrapping_add(1)
                    }
                }
                /*
                 * Add constraint.
                 */
                if add != 0 {
                    if storageLimitVCG > 0 as libc::c_int as libc::c_ulong {
                    } else {
                        __assert_fail(
                            b"storageLimitVCG > 0\x00" as *const u8 as *const libc::c_char,
                            b"vcg.c\x00" as *const u8 as *const libc::c_char,
                            131 as libc::c_int as libc::c_uint,
                            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                                b"void BuildVCG(void)\x00",
                            ))
                            .as_ptr(),
                        );
                    }
                    (*(*VCG.offset(net as isize))
                        .netsBelowHook
                        .offset(constraint as isize))
                    .top = *TOP.offset(col as isize);
                    (*(*VCG.offset(net as isize))
                        .netsBelowHook
                        .offset(constraint as isize))
                    .bot = *BOT.offset(col as isize);
                    (*(*VCG.offset(net as isize))
                        .netsBelowHook
                        .offset(constraint as isize))
                    .col = col;
                    (*(*VCG.offset(net as isize))
                        .netsBelowHook
                        .offset(constraint as isize))
                    .removed = FALSE as libc::c_ulong;
                    storageVCG = storageVCG.offset(1);
                    storageLimitVCG = storageLimitVCG.wrapping_sub(1);
                    constraint = constraint.wrapping_add(1)
                }
            }
            col = col.wrapping_add(1)
        }
        (*VCG.offset(net as isize)).netsBelow = constraint;
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DFSClearVCG(mut VCG_0: *mut nodeVCGType) {
    let mut net: libc::c_ulong = 0;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        (*VCG_0.offset(net as isize)).netsAboveLabel = 0 as libc::c_int as libc::c_ulong;
        (*VCG_0.offset(net as isize)).netsAboveReached = FALSE as libc::c_ulong;
        (*VCG_0.offset(net as isize)).netsBelowLabel = 0 as libc::c_int as libc::c_ulong;
        (*VCG_0.offset(net as isize)).netsBelowReached = FALSE as libc::c_ulong;
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DumpVCG(mut VCG_0: *mut nodeVCGType) {
    let mut net: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        printf(b"[%d]\n\x00" as *const u8 as *const libc::c_char, net);
        printf(b"above: \x00" as *const u8 as *const libc::c_char);
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG_0.offset(net as isize)).netsAbove {
            if (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(which as isize))
            .removed
                == 0
            {
                if (*(*VCG_0.offset(net as isize))
                    .netsAboveHook
                    .offset(which as isize))
                .top == net
                {
                } else {
                    __assert_fail(
                        b"VCG[net].netsAboveHook[which].top == net\x00" as *const u8
                            as *const libc::c_char,
                        b"vcg.c\x00" as *const u8 as *const libc::c_char,
                        170 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            b"void DumpVCG(nodeVCGType *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                printf(
                    b"%d \x00" as *const u8 as *const libc::c_char,
                    (*(*VCG_0.offset(net as isize))
                        .netsAboveHook
                        .offset(which as isize))
                    .bot,
                );
            }
            which = which.wrapping_add(1)
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(b"below: \x00" as *const u8 as *const libc::c_char);
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG_0.offset(net as isize)).netsBelow {
            if (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(which as isize))
            .removed
                == 0
            {
                if (*(*VCG_0.offset(net as isize))
                    .netsBelowHook
                    .offset(which as isize))
                .bot == net
                {
                } else {
                    __assert_fail(
                        b"VCG[net].netsBelowHook[which].bot == net\x00" as *const u8
                            as *const libc::c_char,
                        b"vcg.c\x00" as *const u8 as *const libc::c_char,
                        179 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            b"void DumpVCG(nodeVCGType *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                printf(
                    b"%d \x00" as *const u8 as *const libc::c_char,
                    (*(*VCG_0.offset(net as isize))
                        .netsBelowHook
                        .offset(which as isize))
                    .top,
                );
            }
            which = which.wrapping_add(1)
        }
        printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DFSAboveVCG(mut VCG_0: *mut nodeVCGType, mut net: libc::c_ulong) {
    let mut s: libc::c_ulong = 0;
    let mut above: libc::c_ulong = 0;
    (*VCG_0.offset(net as isize)).netsAboveReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsAbove {
        if (*(*VCG_0.offset(net as isize))
            .netsAboveHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .top == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsAboveHook[s].top == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    197 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void DFSAboveVCG(nodeVCGType *, unsigned long)\x00",
                    ))
                    .as_ptr(),
                );
            }
            above = (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .bot;
            if (*VCG_0.offset(above as isize)).netsAboveReached == 0 {
                DFSAboveVCG(VCG_0, above);
            }
        }
        s = s.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DFSBelowVCG(mut VCG_0: *mut nodeVCGType, mut net: libc::c_ulong) {
    let mut s: libc::c_ulong = 0;
    let mut below: libc::c_ulong = 0;
    (*VCG_0.offset(net as isize)).netsBelowReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsBelow {
        if (*(*VCG_0.offset(net as isize))
            .netsBelowHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .bot == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsBelowHook[s].bot == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    216 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                        b"void DFSBelowVCG(nodeVCGType *, unsigned long)\x00",
                    ))
                    .as_ptr(),
                );
            }
            below = (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .top;
            if (*VCG_0.offset(below as isize)).netsBelowReached == 0 {
                DFSBelowVCG(VCG_0, below);
            }
        }
        s = s.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn SCCofVCG(
    mut VCG_0: *mut nodeVCGType,
    mut SCC_0: *mut libc::c_ulong,
    mut perSCC_0: *mut libc::c_ulong,
) {
    let mut net: libc::c_ulong = 0;
    let mut scc: libc::c_ulong = 0;
    let mut per: libc::c_ulong = 0;
    let mut label: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut choose: libc::c_ulong = 0;
    let mut large: libc::c_ulong = 0;
    let mut done: libc::c_ulong = 0;
    /*
     * DFS of above edges.
     */
    label = 0 as libc::c_int as libc::c_ulong;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        if (*VCG_0.offset(net as isize)).netsAboveReached == 0 {
            SCC_DFSAboveVCG(VCG_0, net, &mut label);
        }
        net = net.wrapping_add(1)
    }
    /*
     * DFS of below edges.
     */
    which = 0 as libc::c_int as libc::c_ulong;
    loop {
        done = TRUE as libc::c_ulong;
        /*
         * Choose not reached net with smallest label.
         */
        choose = 0 as libc::c_int as libc::c_ulong;
        large = 0 as libc::c_int as libc::c_ulong;
        net = 1 as libc::c_int as libc::c_ulong;
        while net <= channelNets {
            if (*VCG_0.offset(net as isize)).netsBelowReached == 0 {
                if (*VCG_0.offset(net as isize)).netsAboveLabel > 0 as libc::c_int as libc::c_ulong
                {
                } else {
                    __assert_fail(
                        b"VCG[net].netsAboveLabel > 0\x00" as *const u8 as *const libc::c_char,
                        b"vcg.c\x00" as *const u8 as *const libc::c_char,
                        264 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                            b"void SCCofVCG(nodeVCGType *, unsigned long *, unsigned long *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                if (*VCG_0.offset(net as isize)).netsAboveLabel > large {
                    choose = net;
                    large = (*VCG_0.offset(net as isize)).netsAboveLabel;
                    done = FALSE as libc::c_ulong
                }
            }
            net = net.wrapping_add(1)
        }
        /*
         * Find all nets in this SCC.
         */
        if done == 0 {
            which = which.wrapping_add(1);
            SCC_DFSBelowVCG(VCG_0, choose, which);
        }
        if !(done == 0) {
            break;
        }
    }
    /*
     * Identify all SCC.
     */
    totalSCC = 0 as libc::c_int as libc::c_ulong;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        *SCC_0.offset(net as isize) = (*VCG_0.offset(net as isize)).netsBelowLabel;
        if *SCC_0.offset(net as isize) > totalSCC {
            totalSCC = *SCC_0.offset(net as isize)
        }
        net = net.wrapping_add(1)
    }
    if totalSCC > 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"totalSCC > 0\x00" as *const u8 as *const libc::c_char,
            b"vcg.c\x00" as *const u8 as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 63], &[libc::c_char; 63]>(
                b"void SCCofVCG(nodeVCGType *, unsigned long *, unsigned long *)\x00",
            ))
            .as_ptr(),
        );
    }
    scc = 1 as libc::c_int as libc::c_ulong;
    while scc <= totalSCC {
        per = 0 as libc::c_int as libc::c_ulong;
        net = 1 as libc::c_int as libc::c_ulong;
        while net <= channelNets {
            if *SCC_0.offset(net as isize) == scc {
                per = per.wrapping_add(1)
            }
            net = net.wrapping_add(1)
        }
        *perSCC_0.offset(scc as isize) = per;
        scc = scc.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn SCC_DFSAboveVCG(
    mut VCG_0: *mut nodeVCGType,
    mut net: libc::c_ulong,
    mut label: *mut libc::c_ulong,
) {
    let mut s: libc::c_ulong = 0;
    let mut above: libc::c_ulong = 0;
    (*VCG_0.offset(net as isize)).netsAboveReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsAbove {
        if (*(*VCG_0.offset(net as isize))
            .netsAboveHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .top == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsAboveHook[s].top == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    316 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 68], &[libc::c_char; 68]>(
                        b"void SCC_DFSAboveVCG(nodeVCGType *, unsigned long, unsigned long *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            above = (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .bot;
            if (*VCG_0.offset(above as isize)).netsAboveReached == 0 {
                SCC_DFSAboveVCG(VCG_0, above, label);
            }
        }
        s = s.wrapping_add(1)
    }
    *label = (*label).wrapping_add(1);
    (*VCG_0.offset(net as isize)).netsAboveLabel = *label;
}
#[no_mangle]
pub unsafe extern "C" fn SCC_DFSBelowVCG(
    mut VCG_0: *mut nodeVCGType,
    mut net: libc::c_ulong,
    mut label: libc::c_ulong,
) {
    let mut s: libc::c_ulong = 0;
    let mut below: libc::c_ulong = 0;
    (*VCG_0.offset(net as isize)).netsBelowReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsBelow {
        if (*(*VCG_0.offset(net as isize))
            .netsBelowHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .bot == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsBelowHook[s].bot == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    338 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                        b"void SCC_DFSBelowVCG(nodeVCGType *, unsigned long, unsigned long)\x00",
                    ))
                    .as_ptr(),
                );
            }
            below = (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .top;
            if (*VCG_0.offset(below as isize)).netsBelowReached == 0 {
                SCC_DFSBelowVCG(VCG_0, below, label);
            }
        }
        s = s.wrapping_add(1)
    }
    (*VCG_0.offset(net as isize)).netsBelowLabel = label;
}
#[no_mangle]
pub unsafe extern "C" fn DumpSCC(mut SCC_0: *mut libc::c_ulong, mut perSCC_0: *mut libc::c_ulong) {
    let mut net: libc::c_ulong = 0;
    let mut scc: libc::c_ulong = 0;
    scc = 1 as libc::c_int as libc::c_ulong;
    while scc <= totalSCC {
        printf(b"[%d]\t\x00" as *const u8 as *const libc::c_char, scc);
        net = 1 as libc::c_int as libc::c_ulong;
        while net <= channelNets {
            if *SCC_0.offset(net as isize) == scc {
                printf(b"%d \x00" as *const u8 as *const libc::c_char, net);
            }
            net = net.wrapping_add(1)
        }
        printf(
            b"<%d>\x00" as *const u8 as *const libc::c_char,
            *perSCC_0.offset(scc as isize),
        );
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        scc = scc.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn AcyclicVCG() {
    let mut done: libc::c_ulong = 0;
    let mut scc: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut rep: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut total: libc::c_ulong = 0;
    let mut cycle: libc::c_ulong = 0;
    let mut acyclic: libc::c_ulong = 0;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG.offset(net as isize)).netsAbove {
            (*(*VCG.offset(net as isize))
                .netsAboveHook
                .offset(which as isize))
            .removed = FALSE as libc::c_ulong;
            which = which.wrapping_add(1)
        }
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG.offset(net as isize)).netsBelow {
            (*(*VCG.offset(net as isize))
                .netsBelowHook
                .offset(which as isize))
            .removed = FALSE as libc::c_ulong;
            which = which.wrapping_add(1)
        }
        net = net.wrapping_add(1)
    }
    acyclic = TRUE as libc::c_ulong;
    removeTotalVCG = 0 as libc::c_int as libc::c_ulong;
    loop {
        done = TRUE as libc::c_ulong;
        /*
         * Check acyclic (and more).
         */
        DFSClearVCG(VCG);
        SCCofVCG(VCG, SCC, perSCC);
        scc = 1 as libc::c_int as libc::c_ulong;
        while scc <= totalSCC {
            if *perSCC.offset(scc as isize) > 1 as libc::c_int as libc::c_ulong {
                acyclic = FALSE as libc::c_ulong;
                done = FALSE as libc::c_ulong;
                break;
            } else {
                scc = scc.wrapping_add(1)
            }
        }
        /*
         * Attempt to eliminate cycles by the
         * removal of a constraint from each SCC.
         */
        if done == 0 {
            RemoveConstraintVCG(VCG, SCC, perSCC, removeVCG);
        }
        if !(done == 0) {
            break;
        }
    }
    /*
     * Replace redundant constraints.
     * That is, those constraints which were removed
     * but can be added back without introducing a cycle.
     */
    total = removeTotalVCG;
    rep = 0 as libc::c_int as libc::c_ulong;
    while rep < removeTotalVCG {
        /*
         * Constraint to consider.
         */
        top = (**removeVCG.offset(rep as isize)).top;
        bot = (**removeVCG.offset(rep as isize)).bot;
        /*
         * Replace above.
         */
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG.offset(top as isize)).netsAbove {
            if (*(*VCG.offset(top as isize))
                .netsAboveHook
                .offset(which as isize))
            .bot == bot
            {
                (*(*VCG.offset(top as isize))
                    .netsAboveHook
                    .offset(which as isize))
                .removed = FALSE as libc::c_ulong;
                break;
            } else {
                which = which.wrapping_add(1)
            }
        }
        /*
         * Replace below.
         */
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*VCG.offset(bot as isize)).netsBelow {
            if (*(*VCG.offset(bot as isize))
                .netsBelowHook
                .offset(which as isize))
            .top == top
            {
                (*(*VCG.offset(bot as isize))
                    .netsBelowHook
                    .offset(which as isize))
                .removed = FALSE as libc::c_ulong;
                break;
            } else {
                which = which.wrapping_add(1)
            }
        }
        /*
         * Does replacement introduce a cycle?
         */
        cycle = FALSE as libc::c_ulong;
        DFSClearVCG(VCG);
        SCCofVCG(VCG, SCC, perSCC);
        scc = 1 as libc::c_int as libc::c_ulong;
        while scc <= totalSCC {
            if *perSCC.offset(scc as isize) > 1 as libc::c_int as libc::c_ulong {
                cycle = TRUE as libc::c_ulong;
                break;
            } else {
                scc = scc.wrapping_add(1)
            }
        }
        if cycle != 0 {
            /*
             * Introduces cycle.
             * Remove constraint (again).
             */
            which = 0 as libc::c_int as libc::c_ulong;
            while which < (*VCG.offset(top as isize)).netsAbove {
                if (*(*VCG.offset(top as isize))
                    .netsAboveHook
                    .offset(which as isize))
                .bot == bot
                {
                    (*(*VCG.offset(top as isize))
                        .netsAboveHook
                        .offset(which as isize))
                    .removed = TRUE as libc::c_ulong;
                    break;
                } else {
                    which = which.wrapping_add(1)
                }
            }
            which = 0 as libc::c_int as libc::c_ulong;
            while which < (*VCG.offset(bot as isize)).netsBelow {
                if (*(*VCG.offset(bot as isize))
                    .netsBelowHook
                    .offset(which as isize))
                .top == top
                {
                    (*(*VCG.offset(bot as isize))
                        .netsBelowHook
                        .offset(which as isize))
                    .removed = TRUE as libc::c_ulong;
                    break;
                } else {
                    which = which.wrapping_add(1)
                }
            }
        } else {
            /*
             * Does not introduce cycle.
             * Replace ok.
             */
            total = total.wrapping_sub(1)
        }
        rep = rep.wrapping_add(1)
    }
    if acyclic != 0 {
        printf(b"\n*** Input is acyclic! ***\n\x00" as *const u8 as *const libc::c_char);
    } else {
        printf(b"\n*** Input is cyclic! ***\n\x00" as *const u8 as *const libc::c_char);
        printf(
            b"*** VC\'s removed (%d) ***\n\x00" as *const u8 as *const libc::c_char,
            total,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn RemoveConstraintVCG(
    mut VCG_0: *mut nodeVCGType,
    mut SCC_0: *mut libc::c_ulong,
    mut perSCC_0: *mut libc::c_ulong,
    mut removeVCG_0: *mut *mut constraintVCGType,
) {
    let mut scc: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut best: libc::c_ulong = 0;
    let mut weight: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut remove: *mut constraintVCGType = 0 as *mut constraintVCGType;
    scc = 1 as libc::c_int as libc::c_ulong;
    while scc <= totalSCC {
        /*
         * For each SCC attempt to remove cycle.
         */
        if *perSCC_0.offset(scc as isize) > 1 as libc::c_int as libc::c_ulong {
            /*
             * SCC of more than one net in SCC, thus cycle.
             */
            remove = NULL as *mut constraintVCGType;
            best = (FULL + 1 as libc::c_int) as libc::c_ulong;
            net = 1 as libc::c_int as libc::c_ulong;
            while net <= channelNets {
                /*
                 * For each net in the SCC.
                 */
                if *SCC_0.offset(net as isize) == scc {
                    /*
                     * Choose constraint to remove.
                     * Consider only constraints within SCC.
                     */
                    which = 0 as libc::c_int as libc::c_ulong;
                    while which < (*VCG_0.offset(net as isize)).netsAbove {
                        bot = (*(*VCG_0.offset(net as isize))
                            .netsAboveHook
                            .offset(which as isize))
                        .bot;
                        if *SCC_0.offset(bot as isize) == scc
                            && (*(*VCG_0.offset(net as isize))
                                .netsAboveHook
                                .offset(which as isize))
                            .removed
                                == 0
                        {
                            /*
                             * Constraint within SCC.
                             * Weigh its removal.
                             */
                            /*
                             * Note: we consider the column from which the
                             * constraint was added only.  That is, since
                             * we do not add the same constraint twice, only
                             * first occurrence of the constraint has the
                             * column location registered.  Thus, when the
                             * columns are inspected to choose the easiest to
                             * route constraint to remove, the weight of the
                             * chosen constraint will only reflect that single
                             * column.  This may lead to a poor choice, but
                             * it is not likely to occur in practice, and it
                             * is not worth the effort to handle the problem.
                             */
                            col = (*(*VCG_0.offset(net as isize))
                                .netsAboveHook
                                .offset(which as isize))
                            .col; /* no left column */
                            weight = 0 as libc::c_int as libc::c_ulong; /* no right column */
                            if col == 1 as libc::c_int as libc::c_ulong {
                                weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong);
                                if *TOP
                                    .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    && *BOT
                                        .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                } else if *TOP
                                    .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    || *BOT
                                        .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                }
                            } else if col == channelColumns {
                                weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong);
                                if *TOP
                                    .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    && *BOT
                                        .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                } else if *TOP
                                    .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    || *BOT
                                        .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                }
                            } else {
                                if *TOP
                                    .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    && *BOT
                                        .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                } else if *TOP
                                    .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    || *BOT
                                        .offset(col.wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                }
                                if *TOP
                                    .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    && *BOT
                                        .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(3 as libc::c_int as libc::c_ulong)
                                } else if *TOP
                                    .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as isize)
                                    != 0
                                    || *BOT
                                        .offset(col.wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            as isize)
                                        != 0
                                {
                                    weight = weight.wrapping_add(2 as libc::c_int as libc::c_ulong)
                                }
                            }
                            /*
                             * Update best.
                             */
                            if weight < best {
                                best = weight;
                                remove = &mut *(*VCG_0.offset(net as isize))
                                    .netsAboveHook
                                    .offset(which as isize)
                                    as *mut constraintVCGType
                            }
                        }
                        which = which.wrapping_add(1)
                    }
                }
                net = net.wrapping_add(1)
            }
            /*
             * Remove.
             */
            if !remove.is_null() {
            } else {
                __assert_fail(b"remove != NULL\x00" as *const u8 as
                                  *const libc::c_char,
                              b"vcg.c\x00" as *const u8 as
                                  *const libc::c_char,
                              614 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 96],
                                                        &[libc::c_char; 96]>(b"void RemoveConstraintVCG(nodeVCGType *, unsigned long *, unsigned long *, constraintVCGType **)\x00")).as_ptr());
            }
            fflush(stdout);
            if removeTotalVCG
                < channelNets
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong))
            {
            } else {
                __assert_fail(b"removeTotalVCG < ((channelNets + 1) * (channelNets + 1))\x00"
                                  as *const u8 as *const libc::c_char,
                              b"vcg.c\x00" as *const u8 as
                                  *const libc::c_char,
                              616 as libc::c_int as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 96],
                                                        &[libc::c_char; 96]>(b"void RemoveConstraintVCG(nodeVCGType *, unsigned long *, unsigned long *, constraintVCGType **)\x00")).as_ptr());
            }
            let ref mut fresh2 = *removeVCG_0.offset(removeTotalVCG as isize);
            *fresh2 = remove;
            removeTotalVCG = removeTotalVCG.wrapping_add(1);
            top = (*remove).top;
            bot = (*remove).bot;
            /*
             * Remove above constraint.
             */
            (*remove).removed = TRUE as libc::c_ulong;
            /*
             * Remove below constraint.
             */
            which = 0 as libc::c_int as libc::c_ulong;
            while which < (*VCG_0.offset(bot as isize)).netsBelow {
                if (*(*VCG_0.offset(bot as isize))
                    .netsBelowHook
                    .offset(which as isize))
                .top == top
                {
                    (*(*VCG_0.offset(bot as isize))
                        .netsBelowHook
                        .offset(which as isize))
                    .removed = TRUE as libc::c_ulong;
                    break;
                } else {
                    which = which.wrapping_add(1)
                }
            }
        }
        scc = scc.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExistPathAboveVCG(
    mut VCG_0: *mut nodeVCGType,
    mut above: libc::c_ulong,
    mut below: libc::c_ulong,
) -> libc::c_ulong {
    DFSClearVCG(VCG_0);
    DFSAboveVCG(VCG_0, above);
    return (*VCG_0.offset(below as isize)).netsAboveReached;
}
#[no_mangle]
pub unsafe extern "C" fn LongestPathVCG(mut VCG_0: *mut nodeVCGType, mut net: libc::c_ulong) {
    let mut track: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut not: libc::c_ulong = 0;
    /*
     * How many nets this net is above (including this net)?
     * That is, longest path through nets which this net
     * is above.
     */
    DFSClearVCG(VCG_0);
    cardBotNotPref =
        DFSAboveLongestPathVCG(VCG_0, net).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    bot = cardBotNotPref;
    track = channelTracks;
    while track >= 1 as libc::c_int as libc::c_ulong {
        if bot > 0 as libc::c_int as libc::c_ulong {
            *tracksBotNotPref.offset(track as isize) = TRUE as libc::c_ulong;
            bot = bot.wrapping_sub(1)
        } else {
            *tracksBotNotPref.offset(track as isize) = FALSE as libc::c_ulong
        }
        track = track.wrapping_sub(1)
    }
    /*
     * How many nets this net is below (including this net)?
     * That is, longest path through nets which this net
     * is below.
     */
    DFSClearVCG(VCG_0);
    cardTopNotPref =
        DFSBelowLongestPathVCG(VCG_0, net).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    top = cardTopNotPref;
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        if top > 0 as libc::c_int as libc::c_ulong {
            *tracksTopNotPref.offset(track as isize) = TRUE as libc::c_ulong;
            top = top.wrapping_sub(1)
        } else {
            *tracksTopNotPref.offset(track as isize) = FALSE as libc::c_ulong
        }
        track = track.wrapping_add(1)
    }
    /*
     * How many tracks are guaranteed to make an HCV?
     * That is, what tracks contain nets which this
     * net must either be above or below.
     */
    not = 0 as libc::c_int as libc::c_ulong;
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        if *tracksTopNotPref.offset(track as isize) != 0
            || *tracksBotNotPref.offset(track as isize) != 0
        {
            *tracksNotPref.offset(track as isize) = TRUE as libc::c_ulong;
            not = not.wrapping_add(1)
        } else {
            *tracksNotPref.offset(track as isize) = FALSE as libc::c_ulong
        }
        track = track.wrapping_add(1)
    }
    cardNotPref = not;
}
#[no_mangle]
pub unsafe extern "C" fn DFSAboveLongestPathVCG(
    mut VCG_0: *mut nodeVCGType,
    mut net: libc::c_ulong,
) -> libc::c_ulong {
    let mut s: libc::c_ulong = 0;
    let mut above: libc::c_ulong = 0;
    let mut path: libc::c_ulong = 0;
    let mut longest: libc::c_ulong = 0;
    longest = 0 as libc::c_int as libc::c_ulong;
    (*VCG_0.offset(net as isize)).netsAboveReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsAbove {
        if (*(*VCG_0.offset(net as isize))
            .netsAboveHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .top == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsAboveHook[s].top == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    727 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                        b"unsigned long DFSAboveLongestPathVCG(nodeVCGType *, unsigned long)\x00",
                    ))
                    .as_ptr(),
                );
            }
            above = (*(*VCG_0.offset(net as isize))
                .netsAboveHook
                .offset(s as isize))
            .bot;
            if (*VCG_0.offset(above as isize)).netsAboveReached == 0 {
                path = DFSAboveLongestPathVCG(VCG_0, above);
                if path > longest {
                    longest = path
                }
            }
        }
        s = s.wrapping_add(1)
    }
    return longest.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn DFSBelowLongestPathVCG(
    mut VCG_0: *mut nodeVCGType,
    mut net: libc::c_ulong,
) -> libc::c_ulong {
    let mut s: libc::c_ulong = 0;
    let mut below: libc::c_ulong = 0;
    let mut path: libc::c_ulong = 0;
    let mut longest: libc::c_ulong = 0;
    longest = 0 as libc::c_int as libc::c_ulong;
    (*VCG_0.offset(net as isize)).netsBelowReached = TRUE as libc::c_ulong;
    s = 0 as libc::c_int as libc::c_ulong;
    while s < (*VCG_0.offset(net as isize)).netsBelow {
        if (*(*VCG_0.offset(net as isize))
            .netsBelowHook
            .offset(s as isize))
        .removed
            == 0
        {
            if (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .bot == net
            {
            } else {
                __assert_fail(
                    b"VCG[net].netsBelowHook[s].bot == net\x00" as *const u8 as *const libc::c_char,
                    b"vcg.c\x00" as *const u8 as *const libc::c_char,
                    753 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 67], &[libc::c_char; 67]>(
                        b"unsigned long DFSBelowLongestPathVCG(nodeVCGType *, unsigned long)\x00",
                    ))
                    .as_ptr(),
                );
            }
            below = (*(*VCG_0.offset(net as isize))
                .netsBelowHook
                .offset(s as isize))
            .top;
            if (*VCG_0.offset(below as isize)).netsBelowReached == 0 {
                path = DFSBelowLongestPathVCG(VCG_0, below);
                if path > longest {
                    longest = path
                }
            }
        }
        s = s.wrapping_add(1)
    }
    return longest.wrapping_add(1 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn VCV(
    mut VCG_0: *mut nodeVCGType,
    mut check: libc::c_ulong,
    mut track: libc::c_ulong,
    mut assign: *mut libc::c_ulong,
) -> libc::c_ulong {
    let mut net: libc::c_ulong = 0;
    let mut vcv: libc::c_ulong = 0;
    vcv = 0 as libc::c_int as libc::c_ulong;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        if *assign.offset(net as isize) != 0 {
            if *assign.offset(net as isize) < track {
                /*
                 * Above VCV?
                 */
                if ExistPathAboveVCG(VCG_0, net, check) != 0 {
                    vcv = vcv.wrapping_add(1)
                }
            } else if *assign.offset(net as isize) > track {
                /*
                 * Below VCV?
                 */
                if ExistPathAboveVCG(VCG_0, check, net) != 0 {
                    vcv = vcv.wrapping_add(1)
                }
            } else if ExistPathAboveVCG(VCG_0, net, check) != 0
                || ExistPathAboveVCG(VCG_0, check, net) != 0
            {
                vcv = vcv.wrapping_add(1)
            }
        }
        net = net.wrapping_add(1)
    }
    return vcv;
}
/*
 * Above or Below VCV?
 */
