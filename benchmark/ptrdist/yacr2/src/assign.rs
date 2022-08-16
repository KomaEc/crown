use ::libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    /*
     *
     * Globals.
     *
     */
    /* VCG_CODE */
    #[no_mangle]
    static mut VCG: *mut nodeVCGType;
    #[no_mangle]
    fn LongestPathVCG(_: *mut nodeVCGType, _: libc::c_ulong);
    #[no_mangle]
    fn VCV(
        _: *mut nodeVCGType,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut libc::c_ulong,
    ) -> libc::c_ulong;
    /*
     *
     * Globals.
     *
     */
    /* HCG_CODE */
    #[no_mangle]
    static mut HCG: *mut nodeHCGType;
    #[no_mangle]
    fn NoHCV(_: *mut nodeHCGType, _: libc::c_ulong, _: *mut libc::c_ulong, _: *mut libc::c_ulong);
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
    static mut FIRST: *mut libc::c_ulong;
    #[no_mangle]
    static mut LAST: *mut libc::c_ulong;
    #[no_mangle]
    static mut CROSSING: *mut libc::c_ulong;
    #[no_mangle]
    static mut channelNets: libc::c_ulong;
    #[no_mangle]
    static mut channelColumns: libc::c_ulong;
    #[no_mangle]
    static mut channelTracks: libc::c_ulong;
    #[no_mangle]
    static mut channelDensityColumn: libc::c_ulong;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _nodeHCGType {
    pub netsHook: *mut libc::c_ulong,
    pub nets: libc::c_ulong,
    pub netsReached: libc::c_ulong,
}
pub type nodeHCGType = _nodeHCGType;
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
 * assign.h
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
#[no_mangle]
pub static mut tracksAssign: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut costMatrix: *mut *mut libc::c_long =
    0 as *const *mut libc::c_long as *mut *mut libc::c_long;
#[no_mangle]
pub static mut tracksNoHCV: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut tracksNotPref: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut tracksTopNotPref: *mut libc::c_ulong =
    0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut tracksBotNotPref: *mut libc::c_ulong =
    0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut cardNotPref: libc::c_ulong = 0;
#[no_mangle]
pub static mut cardTopNotPref: libc::c_ulong = 0;
#[no_mangle]
pub static mut cardBotNotPref: libc::c_ulong = 0;
pub const MEDIUM: libc::c_int = 100 as libc::c_int;
pub const HIGH: libc::c_int = 10000 as libc::c_int;
pub const LOW: libc::c_int = 1 as libc::c_int;
pub const INFINITY: libc::c_int = 1000000 as libc::c_int;
#[no_mangle]
pub static mut netsAssign: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut netsAssignCopy: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
/* ASSIGN_CODE */
/* ASSIGN_CODE */
/*
 *
 * Prototypes.
 *
 */
/*
 *
 * assign.c
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
pub unsafe extern "C" fn AllocAssign() {
    let mut net: libc::c_ulong = 0;
    /*
     * Allocate cost matrix.
     */
    costMatrix = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_long>() as libc::c_ulong),
    ) as *mut *mut libc::c_long;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        let ref mut fresh0 = *costMatrix.offset(net as isize);
        *fresh0 = malloc(
            channelTracks
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_long>() as libc::c_ulong),
        ) as *mut libc::c_long;
        net = net.wrapping_add(1)
    }
    /*
     * Allocate structures associated with cost matrix.
     */
    tracksNotPref = malloc(
        channelTracks
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    tracksTopNotPref = malloc(
        channelTracks
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    tracksBotNotPref = malloc(
        channelTracks
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    tracksNoHCV = malloc(
        channelTracks
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    tracksAssign = malloc(
        channelTracks
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    netsAssign = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    netsAssignCopy = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn FreeAssign() {
    let mut net: libc::c_ulong = 0;
    /*
     * Free cost matrix.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        free(*costMatrix.offset(net as isize) as *mut libc::c_void);
        net = net.wrapping_add(1)
    }
    free(costMatrix as *mut libc::c_void);
    /*
     * Free structures associated with cost matrix.
     */
    free(tracksNotPref as *mut libc::c_void);
    free(tracksTopNotPref as *mut libc::c_void);
    free(tracksBotNotPref as *mut libc::c_void);
    free(tracksNoHCV as *mut libc::c_void);
    free(tracksAssign as *mut libc::c_void);
    free(netsAssign as *mut libc::c_void);
    free(netsAssignCopy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn NetsAssign() {
    let mut net: libc::c_ulong = 0;
    /*
     * Init.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        *netsAssign.offset(net as isize) = 0 as libc::c_int as libc::c_ulong;
        net = net.wrapping_add(1)
    }
    /*
     * Assign nets in max density column.
     */
    MaxNetsAssign();
    /*
     * Assign net in columns left of max density column.
     */
    LeftNetsAssign();
    /*
     * Assign net in columns right of max density column.
     */
    RightNetsAssign();
}
#[no_mangle]
pub unsafe extern "C" fn MaxNetsAssign() {
    let mut net: libc::c_ulong = 0;
    let mut netSelect: libc::c_ulong = 0;
    let mut netsCrossing: libc::c_ulong = 0;
    /*
     * Determine nets crossing column of max channel density.
     */
    netsCrossing = 0 as libc::c_int as libc::c_ulong;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        if *FIRST.offset(net as isize) <= channelDensityColumn
            && *LAST.offset(net as isize) >= channelDensityColumn
        {
            *CROSSING.offset(net as isize) = TRUE as libc::c_ulong;
            netsCrossing = netsCrossing.wrapping_add(1)
        } else {
            *CROSSING.offset(net as isize) = FALSE as libc::c_ulong
        }
        net = net.wrapping_add(1)
    }
    while netsCrossing > 0 as libc::c_int as libc::c_ulong {
        /*
         * Choose net from CROSSING to assign.
         */
        Select(VCG, HCG, netsAssign, &mut netSelect, CROSSING);
        /*
         * Assign net to a track.
         */
        Assign(VCG, netsAssign, netSelect);
        /*
         * Done net.
         */
        *CROSSING.offset(netSelect as isize) = FALSE as libc::c_ulong;
        netsCrossing = netsCrossing.wrapping_sub(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn RightNetsAssign() {
    let mut net: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut netsCrossing: libc::c_ulong = 0;
    let mut netSelect: libc::c_ulong = 0;
    /*
     * Init.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        *CROSSING.offset(net as isize) = FALSE as libc::c_ulong;
        net = net.wrapping_add(1)
    }
    netsCrossing = 0 as libc::c_int as libc::c_ulong;
    /*
     * Assign nets within channel.
     */
    col = channelDensityColumn.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while col <= channelColumns {
        /*
         * Collection.
         */
        top = *TOP.offset(col as isize);
        bot = *BOT.offset(col as isize);
        if top != bot {
            if top != 0 && *FIRST.offset(top as isize) == col {
                *CROSSING.offset(top as isize) = TRUE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_add(1)
            }
            if bot != 0 && *FIRST.offset(bot as isize) == col {
                *CROSSING.offset(bot as isize) = TRUE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_add(1)
            }
        } else if top != 0 && *FIRST.offset(top as isize) == col {
            *CROSSING.offset(top as isize) = TRUE as libc::c_ulong;
            netsCrossing = netsCrossing.wrapping_add(1)
        }
        /*
         * Assignment.
         */
        if *LAST.offset(top as isize) == col || *LAST.offset(bot as isize) == col {
            while netsCrossing > 0 as libc::c_int as libc::c_ulong {
                /*
                 * Choose net from CROSSING to assign.
                 */
                Select(VCG, HCG, netsAssign, &mut netSelect, CROSSING);
                /*
                 * Assign net to a track.
                 */
                Assign(VCG, netsAssign, netSelect);
                /*
                 * Done net.
                 */
                *CROSSING.offset(netSelect as isize) = FALSE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_sub(1)
            }
        }
        col = col.wrapping_add(1)
    }
    if netsCrossing == 0 {
    } else {
        __assert_fail(
            b"! netsCrossing\x00" as *const u8 as *const libc::c_char,
            b"assign.c\x00" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void RightNetsAssign(void)\x00",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn LeftNetsAssign() {
    let mut net: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut netsCrossing: libc::c_ulong = 0;
    let mut netSelect: libc::c_ulong = 0;
    /*
     * Init.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        *CROSSING.offset(net as isize) = FALSE as libc::c_ulong;
        net = net.wrapping_add(1)
    }
    netsCrossing = 0 as libc::c_int as libc::c_ulong;
    /*
     * Assign nets within channel.
     */
    col = channelDensityColumn.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while col >= 1 as libc::c_int as libc::c_ulong {
        /*
         * Collection.
         */
        top = *TOP.offset(col as isize);
        bot = *BOT.offset(col as isize);
        if top != bot {
            if top != 0 && *LAST.offset(top as isize) == col {
                *CROSSING.offset(top as isize) = TRUE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_add(1)
            }
            if bot != 0 && *LAST.offset(bot as isize) == col {
                *CROSSING.offset(bot as isize) = TRUE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_add(1)
            }
        } else if top != 0 && *LAST.offset(top as isize) == col {
            *CROSSING.offset(top as isize) = TRUE as libc::c_ulong;
            netsCrossing = netsCrossing.wrapping_add(1)
        }
        /*
         * Assignment.
         */
        if *FIRST.offset(top as isize) == col || *FIRST.offset(bot as isize) == col {
            while netsCrossing > 0 as libc::c_int as libc::c_ulong {
                /*
                 * Choose net from CROSSING to assign.
                 */
                Select(VCG, HCG, netsAssign, &mut netSelect, CROSSING);
                /*
                 * Assign net to a track.
                 */
                Assign(VCG, netsAssign, netSelect);
                /*
                 * Done net.
                 */
                *CROSSING.offset(netSelect as isize) = FALSE as libc::c_ulong;
                netsCrossing = netsCrossing.wrapping_sub(1)
            }
        }
        col = col.wrapping_sub(1)
    }
    if netsCrossing == 0 {
    } else {
        __assert_fail(
            b"! netsCrossing\x00" as *const u8 as *const libc::c_char,
            b"assign.c\x00" as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void LeftNetsAssign(void)\x00",
            ))
            .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Assign(
    mut VCG_0: *mut nodeVCGType,
    mut assign: *mut libc::c_ulong,
    mut select: libc::c_ulong,
) {
    let mut dist: libc::c_long = 0;
    let mut ideal: libc::c_ulong = 0;
    let mut track: libc::c_ulong = 0;
    let mut tracks: libc::c_ulong = 0;
    let mut trackAssign: libc::c_ulong = 0;
    let mut vcv: libc::c_ulong = 0;
    let mut vcvDist: libc::c_long = 0;
    let mut vcvAssign: libc::c_ulong = 0;
    let mut costNet: *mut libc::c_long = 0 as *mut libc::c_long;
    /*
     * Need information for the selected net.
     * (Must recompute...not current for the selected net)
     *
     * tracksNoHCV
     * tracksNotPref
     * tracksTopNotPref
     * tracksBotNotPref
     * cardNotPref
     * cardTopNotPref
     * cardBotNotPref
     */
    LongestPathVCG(VCG_0, select);
    NoHCV(HCG, select, assign, tracksNoHCV);
    IdealTrack(channelTracks, cardTopNotPref, cardBotNotPref, &mut ideal);
    /*
     * What tracks to consider for assign.
     */
    costNet = *costMatrix.offset(select as isize);
    if select >= 1 as libc::c_int as libc::c_ulong && select <= channelNets {
    } else {
        __assert_fail(
            b"(select >= 1) && (select <= channelNets)\x00" as *const u8 as *const libc::c_char,
            b"assign.c\x00" as *const u8 as *const libc::c_char,
            367 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void Assign(nodeVCGType *, unsigned long *, unsigned long)\x00",
            ))
            .as_ptr(),
        );
    }
    tracks = 0 as libc::c_int as libc::c_ulong;
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        *tracksAssign.offset(track as isize) = FALSE as libc::c_ulong;
        track = track.wrapping_add(1)
    }
    /*
     * Choice 1.
     */
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        if *tracksNoHCV.offset(track as isize) != 0 && *tracksNotPref.offset(track as isize) == 0 {
            *tracksAssign.offset(track as isize) = TRUE as libc::c_ulong;
            tracks = tracks.wrapping_add(1)
        }
        track = track.wrapping_add(1)
    }
    /*
     * Choice 2.
     */
    if tracks == 0 as libc::c_int as libc::c_ulong {
        track = 1 as libc::c_int as libc::c_ulong;
        while track <= channelTracks {
            if *tracksNoHCV.offset(track as isize) != 0
                && *tracksTopNotPref.offset(track as isize) != 0
                && *tracksBotNotPref.offset(track as isize) != 0
            {
                *tracksAssign.offset(track as isize) = TRUE as libc::c_ulong;
                tracks = tracks.wrapping_add(1)
            }
            track = track.wrapping_add(1)
        }
    }
    /*
     * Choice 3.
     */
    if tracks == 0 as libc::c_int as libc::c_ulong {
        track = 2 as libc::c_int as libc::c_ulong;
        while track < channelTracks {
            if *tracksNoHCV.offset(track as isize) != 0
                && *tracksNotPref.offset(track as isize) != 0
            {
                *tracksAssign.offset(track as isize) = TRUE as libc::c_ulong;
                tracks = tracks.wrapping_add(1)
            }
            track = track.wrapping_add(1)
        }
    }
    /*
     * Choice 4.
     */
    if tracks == 0 as libc::c_int as libc::c_ulong {
        if *tracksNoHCV.offset(1 as libc::c_int as isize) != 0 {
            *tracksAssign.offset(1 as libc::c_int as isize) = TRUE as libc::c_ulong;
            tracks = tracks.wrapping_add(1)
        }
        if *tracksNoHCV.offset(channelTracks as isize) != 0 {
            *tracksAssign.offset(channelTracks as isize) = TRUE as libc::c_ulong;
            tracks = tracks.wrapping_add(1)
        }
    }
    if tracks != 0 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"tracks != 0\x00" as *const u8 as *const libc::c_char,
            b"assign.c\x00" as *const u8 as *const libc::c_char,
            435 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void Assign(nodeVCGType *, unsigned long *, unsigned long)\x00",
            ))
            .as_ptr(),
        );
    }
    /*
        costNet = costMatrix[select];
        assert((select >= 1) && (select <= channelNets));
        tracks = 0;
        for (track = 1; track <= channelTracks; track++) {
        tracksAssign[track] = FALSE;
        }
        for (track = 1; track <= channelTracks; track++) {
        if ((costNet[track] < HIGH) {
            tracksAssign[track] = TRUE;
            tracks++;
        }
        }
        if (tracks == 0) {
        for (track = 2; track < channelTracks; track++) {
            if (costNet[track] < INFINITY) {
            tracksAssign[track] = TRUE;
            tracks++;
            }
        }
        }
        if (tracks == 0) {
        if (costNet[1] < INFINITY) {
            tracksAssign[1] = TRUE;
            tracks++;
        }
        if (costNet[channelTracks] < INFINITY) {
            tracksAssign[channelTracks] = TRUE;
            tracks++;
        }
        }
        assert(tracks != 0);
    */
    /*
     * Assign to one of the tracks under consideration.
     */
    trackAssign = 0 as libc::c_int as libc::c_ulong;
    vcvAssign = INFINITY as libc::c_ulong;
    track = 1 as libc::c_int as libc::c_ulong;
    while track <= channelTracks {
        if *tracksAssign.offset(track as isize) != 0 {
            vcv = VCV(VCG_0, select, track, netsAssign);
            if vcv < vcvAssign {
                vcvAssign = vcv;
                trackAssign = track;
                vcvDist = track.wrapping_sub(ideal) as libc::c_long;
                if vcvDist < 0 as libc::c_int as libc::c_long {
                    vcvDist *= -(1 as libc::c_int) as libc::c_long
                }
            } else if vcv == vcvAssign {
                if trackAssign != 0 {
                } else {
                    __assert_fail(
                        b"trackAssign\x00" as *const u8 as *const libc::c_char,
                        b"assign.c\x00" as *const u8 as *const libc::c_char,
                        488 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                            b"void Assign(nodeVCGType *, unsigned long *, unsigned long)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                dist = track.wrapping_sub(ideal) as libc::c_long;
                if dist < 0 as libc::c_int as libc::c_long {
                    dist *= -(1 as libc::c_int) as libc::c_long
                }
                if dist < vcvDist {
                    vcvDist = dist;
                    vcvAssign = vcv;
                    trackAssign = track
                }
            }
        }
        track = track.wrapping_add(1)
    }
    if trackAssign != 0 {
    } else {
        __assert_fail(
            b"trackAssign\x00" as *const u8 as *const libc::c_char,
            b"assign.c\x00" as *const u8 as *const libc::c_char,
            501 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 59], &[libc::c_char; 59]>(
                b"void Assign(nodeVCGType *, unsigned long *, unsigned long)\x00",
            ))
            .as_ptr(),
        );
    }
    *assign.offset(select as isize) = trackAssign;
}
#[no_mangle]
pub unsafe extern "C" fn Select(
    mut VCG_0: *mut nodeVCGType,
    mut HCG_0: *mut nodeHCGType,
    mut netsAssign_0: *mut libc::c_ulong,
    mut netSelect: *mut libc::c_ulong,
    mut CROSSING_0: *mut libc::c_ulong,
) {
    let mut net: libc::c_ulong = 0;
    let mut track: libc::c_ulong = 0;
    let mut select: libc::c_ulong = 0;
    let mut costNet: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut cost: libc::c_long = 0;
    let mut largest: libc::c_long = 0;
    /*
     * Build cost matrix.
     */
    BuildCostMatrix(VCG_0, HCG_0, netsAssign_0, CROSSING_0);
    /*
     * Select most restricted net.
     * That is, net with greatest column sum in cost matrix.
     */
    largest = -(1 as libc::c_int) as libc::c_long;
    select = 0 as libc::c_int as libc::c_ulong;
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        if *CROSSING_0.offset(net as isize) != 0 {
            cost = 0 as libc::c_int as libc::c_long;
            costNet = *costMatrix.offset(net as isize);
            track = 1 as libc::c_int as libc::c_ulong;
            while track <= channelTracks {
                cost += *costNet.offset(track as isize);
                track = track.wrapping_add(1)
            }
            if cost > largest {
                largest = cost;
                select = net
            }
        }
        net = net.wrapping_add(1)
    }
    if select != 0 {
    } else {
        __assert_fail(b"select\x00" as *const u8 as *const libc::c_char,
                      b"assign.c\x00" as *const u8 as *const libc::c_char,
                      547 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 93],
                                                &[libc::c_char; 93]>(b"void Select(nodeVCGType *, nodeHCGType *, unsigned long *, unsigned long *, unsigned long *)\x00")).as_ptr());
    }
    /*
     * Net selected.
     */
    *netSelect = select;
}
#[no_mangle]
pub unsafe extern "C" fn BuildCostMatrix(
    mut VCG_0: *mut nodeVCGType,
    mut HCG_0: *mut nodeHCGType,
    mut netsAssign_0: *mut libc::c_ulong,
    mut CROSSING_0: *mut libc::c_ulong,
) {
    let mut net: libc::c_ulong = 0;
    let mut track: libc::c_ulong = 0;
    let mut ideal: libc::c_ulong = 0;
    let mut dist: libc::c_long = 0;
    let mut mult: libc::c_long = 0;
    let mut costNet: *mut libc::c_long = 0 as *mut libc::c_long;
    /*
     * Initialize cost matrix.
     */
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        costNet = *costMatrix.offset(net as isize);
        track = 1 as libc::c_int as libc::c_ulong;
        while track <= channelTracks {
            *costNet.offset(track as isize) = 0 as libc::c_int as libc::c_long;
            track = track.wrapping_add(1)
        }
        net = net.wrapping_add(1)
    }
    net = 1 as libc::c_int as libc::c_ulong;
    while net <= channelNets {
        if *CROSSING_0.offset(net as isize) != 0 {
            /*
             * Compute one column in cost matrix.
             * That is, the cost associated with each track for some net.
             */
            costNet = *costMatrix.offset(net as isize);
            /*
             * Compute measures related to cost.
             */
            LongestPathVCG(VCG_0, net);
            NoHCV(HCG_0, net, netsAssign_0, tracksNoHCV);
            IdealTrack(channelTracks, cardTopNotPref, cardBotNotPref, &mut ideal);
            /*
             * Compute cost associated with each track.
             */
            track = 1 as libc::c_int as libc::c_ulong;
            while track <= channelTracks {
                if *tracksNoHCV.offset(track as isize) != 0 {
                    if cardNotPref != channelTracks {
                        if *tracksNotPref.offset(track as isize) != 0 {
                            *costNet.offset(track as isize) = HIGH as libc::c_long
                        } else {
                            *costNet.offset(track as isize) =
                                (MEDIUM as libc::c_ulong).wrapping_mul(cardNotPref) as libc::c_long
                        }
                    } else if track > channelTracks.wrapping_sub(cardBotNotPref)
                        && track <= cardTopNotPref
                    {
                        mult = cardNotPref.wrapping_sub(
                            cardTopNotPref
                                .wrapping_add(cardBotNotPref)
                                .wrapping_sub(channelTracks),
                        ) as libc::c_long;
                        if mult >= 0 as libc::c_int as libc::c_long {
                        } else {
                            __assert_fail(b"mult >= 0\x00" as *const u8 as
                                              *const libc::c_char,
                                          b"assign.c\x00" as *const u8 as
                                              *const libc::c_char,
                                          610 as libc::c_int as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 85],
                                                                    &[libc::c_char; 85]>(b"void BuildCostMatrix(nodeVCGType *, nodeHCGType *, unsigned long *, unsigned long *)\x00")).as_ptr());
                        }
                        *costNet.offset(track as isize) = MEDIUM as libc::c_long * mult
                    } else {
                        *costNet.offset(track as isize) = HIGH as libc::c_long
                    }
                    if *costNet.offset(track as isize) < INFINITY as libc::c_long {
                        dist = ideal.wrapping_sub(track) as libc::c_long;
                        if dist < 0 as libc::c_int as libc::c_long {
                            dist *= -(1 as libc::c_int) as libc::c_long
                        }
                        *costNet.offset(track as isize) += LOW as libc::c_long * dist
                    }
                } else {
                    *costNet.offset(track as isize) = INFINITY as libc::c_long
                }
                track = track.wrapping_add(1)
            }
        }
        net = net.wrapping_add(1)
    }
}
#[no_mangle]
pub unsafe extern "C" fn IdealTrack(
    mut tracks: libc::c_ulong,
    mut top: libc::c_ulong,
    mut bot: libc::c_ulong,
    mut ideal: *mut libc::c_ulong,
) {
    let mut num: libc::c_ulong = 0;
    let mut den: libc::c_ulong = 0;
    num = top
        .wrapping_mul(tracks.wrapping_sub(bot))
        .wrapping_add(bot.wrapping_mul(top.wrapping_add(1 as libc::c_int as libc::c_ulong)));
    den = top.wrapping_add(bot);
    if den != 0 as libc::c_int as libc::c_ulong {
        *ideal = num.wrapping_div(den)
    } else {
        *ideal = 1 as libc::c_int as libc::c_ulong
    };
}
