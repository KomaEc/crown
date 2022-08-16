use ::libc;
extern "C" {
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
    static mut FIRST: *mut libc::c_ulong;
    #[no_mangle]
    static mut LAST: *mut libc::c_ulong;
    #[no_mangle]
    static mut channelNets: libc::c_ulong;
    #[no_mangle]
    static mut channelTracks: libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _nodeHCGType {
    pub netsHook: *mut libc::c_ulong,
    pub nets: libc::c_ulong,
    pub netsReached: libc::c_ulong,
}
/*
 *
 * hcg.h
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
pub type nodeHCGType = _nodeHCGType;
pub const __ASSERT_FUNCTION: [libc::c_char; 20] =
    unsafe { *::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"void BuildHCG(void)\x00") };
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
 * Globals.
 *
 */
#[no_mangle]
pub static mut HCG: *mut nodeHCGType = 0 as *const nodeHCGType as *mut nodeHCGType;
#[no_mangle]
pub static mut storageRootHCG: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut storageHCG: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut storageLimitHCG: libc::c_ulong = 0;
/* HCG_CODE */
/* HCG_CODE */
/*
 *
 * Prototypes.
 *
 */
/*
 *
 * hcg.c
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
pub unsafe extern "C" fn AllocHCG() {
    HCG = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<nodeHCGType>() as libc::c_ulong),
    ) as *mut nodeHCGType;
    storageRootHCG = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong))
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    storageHCG = storageRootHCG;
    storageLimitHCG = channelNets
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_mul(channelNets.wrapping_add(1 as libc::c_int as libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn FreeHCG() {
    free(HCG as *mut libc::c_void);
    free(storageRootHCG as *mut libc::c_void);
    storageLimitHCG = 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn BuildHCG() {
    let mut net: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut constraint: libc::c_ulong = 0;
    let mut first: libc::c_ulong = 0;
    let mut last: libc::c_ulong = 0;
    let mut check: libc::c_ulong = 0;
    let mut add: libc::c_ulong = 0;
    /*
     * Allocate HCG storage.
     */
    AllocHCG();
    /*
     * Build HCG one net at a time.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        first = *FIRST.offset(net as isize);
        last = *LAST.offset(net as isize);
        constraint = 0 as libc::c_int as libc::c_ulong;
        let ref mut fresh0 = (*HCG.offset(net as isize)).netsHook;
        *fresh0 = storageHCG;
        which = 1 as libc::c_int as libc::c_ulong;
        while which <= channelNets {
            if !(*FIRST.offset(which as isize) < first && *LAST.offset(which as isize) < first
                || *FIRST.offset(which as isize) > last && *LAST.offset(which as isize) > last)
            {
                /*
                 * No constraint should ever already exist.
                 * Because there is only one first and last
                 * for each net, the same constraint could
                 * never be added twice.
                 */
                add = TRUE as libc::c_ulong;
                check = 0 as libc::c_int as libc::c_ulong;
                while check < constraint {
                    if *(*HCG.offset(net as isize)).netsHook.offset(check as isize) == which {
                        add = FALSE as libc::c_ulong;
                        break;
                    } else {
                        check = check.wrapping_add(1)
                    }
                }
                if add != 0 {
                } else {
                    __assert_fail(
                        b"add\x00" as *const u8 as *const libc::c_char,
                        b"hcg.c\x00" as *const u8 as *const libc::c_char,
                        92 as libc::c_int as libc::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                }
                /*
                 * Add constraint.
                 */
                if storageLimitHCG > 0 as libc::c_int as libc::c_ulong {
                } else {
                    __assert_fail(
                        b"storageLimitHCG > 0\x00" as *const u8 as *const libc::c_char,
                        b"hcg.c\x00" as *const u8 as *const libc::c_char,
                        97 as libc::c_int as libc::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                }
                *(*HCG.offset(net as isize))
                    .netsHook
                    .offset(constraint as isize) = which;
                storageHCG = storageHCG.offset(1);
                storageLimitHCG = storageLimitHCG.wrapping_sub(1);
                constraint = constraint.wrapping_add(1)
            }
            which = which.wrapping_add(1)
        }
        (*HCG.offset(net as isize)).nets = constraint;
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DFSClearHCG(mut HCG_0: *mut nodeHCGType) {
    let mut net: libc::c_ulong = 0;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        (*HCG_0.offset(net as isize)).netsReached = FALSE as libc::c_ulong;
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn DumpHCG(mut HCG_0: *mut nodeHCGType) {
    let mut net: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        printf(b"[%d]\n\x00" as *const u8 as *const libc::c_char, net);
        which = 0 as libc::c_int as libc::c_ulong;
        while which < (*HCG_0.offset(net as isize)).nets {
            printf(
                b"%d \x00" as *const u8 as *const libc::c_char,
                *(*HCG_0.offset(net as isize))
                    .netsHook
                    .offset(which as isize),
            );
            which = which.wrapping_add(1)
        }
        printf(b"\n\n\x00" as *const u8 as *const libc::c_char);
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn NoHCV(
    mut HCG_0: *mut nodeHCGType,
    mut select: libc::c_ulong,
    mut netsAssign: *mut libc::c_ulong,
    mut tracksNoHCV: *mut libc::c_ulong,
) {
    let mut track: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut ok: libc::c_ulong = 0;
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        /*
         * For each track, check to see if any nets assigned
         * to it would be an HCV for the selected net.
         */
        ok = TRUE as libc::c_ulong;
        net = 1 as libc::c_int as libc::c_ulong;
        while net <= channelNets {
            if *netsAssign.offset(net as isize) == track {
                /*
                 * Net assigned to track.
                 */
                which = 0 as libc::c_int as libc::c_ulong;
                while which < (*HCG_0.offset(select as isize)).nets {
                    if *(*HCG_0.offset(select as isize))
                        .netsHook
                        .offset(which as isize)
                        == net
                    {
                        /*
                         * HCV.
                         */
                        ok = FALSE as libc::c_ulong;
                        break;
                    } else {
                        which = which.wrapping_add(1)
                    }
                }
                /*
                 * Is net an HCV?
                 */
                if ok == 0 {
                    break;
                }
            }
            net = net.wrapping_add(1)
        }
        /*
         * Is this track ok (no HCV's)?
         */
        *tracksNoHCV.offset(track as isize) = ok;
        track = track.wrapping_add(1)
    }
}
