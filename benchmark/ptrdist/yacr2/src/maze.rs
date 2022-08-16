use ::libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abort() -> !;
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
    static mut channelNets: libc::c_ulong;
    #[no_mangle]
    static mut channelColumns: libc::c_ulong;
    #[no_mangle]
    static mut channelTracks: libc::c_ulong;
    #[no_mangle]
    static mut netsAssign: *mut libc::c_ulong;
    #[no_mangle]
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
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
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 *	plane allocation structures and routines
 */
pub const FROM_LEFT: libc::c_int = 0x1 as libc::c_int;
pub const FROM_RIGHT: libc::c_int = 0x2 as libc::c_int;
pub const FROM_TOP: libc::c_int = 0x4 as libc::c_int;
pub const FROM_BOT: libc::c_int = 0x8 as libc::c_int;
/* generic r/lvalue allocation map access macro */
static mut horzPlane: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* horizontal plane allocation map */
/* r/lvalue for accessing horizontal plane allocation map */
static mut vertPlane: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* vertical plane allocation map */
/* r/lvalue for accessing vertical plane allocation map */
static mut viaPlane: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* via plane allocation map */
/* r/lvalue for accessing via plane allocation map */
static mut mazeRoute: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* true if the col needs to be maze routed */
/*
 *	set up the plane allocation maps, note: the channel
 *	dimensions must be set correctly when this routine
 *	is called
 */
#[no_mangle]
pub unsafe extern "C" fn InitAllocMaps() {
    /* allocate maps */
    horzPlane = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(3 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    vertPlane = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(3 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    viaPlane = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(3 as libc::c_int as libc::c_ulong)),
    ) as *mut libc::c_char;
    mazeRoute =
        malloc(channelColumns.wrapping_add(1 as libc::c_int as libc::c_ulong)) as *mut libc::c_char;
    /* if (!horzPlane || !vertPlane || !viaPlane || !mazeRoute) { */
    if horzPlane.is_null() || vertPlane.is_null() || viaPlane.is_null() || mazeRoute.is_null() {
        fprintf(
            stderr,
            b"unable to allocate plane allocation maps\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreeAllocMaps() {
    free(horzPlane as *mut libc::c_void);
    free(vertPlane as *mut libc::c_void);
    free(viaPlane as *mut libc::c_void);
    free(mazeRoute as *mut libc::c_void);
}
/*
 *	draw a segment, horizontal or vertical, from (x1,y1) to
 * 	(x2,y2) in the passed channel plane allocation map,
 *	note: the map entries are overwritten irreguardless of
 *	previous allocation, so check that the space is empty first,
 *	note: no spatial order is required on the two points, as
 *	they are sorted as needed by the line drawer
 */
#[no_mangle]
pub unsafe extern "C" fn DrawSegment(
    mut plane: *mut libc::c_char,
    mut x1: libc::c_ulong,
    mut y1: libc::c_ulong,
    mut x2: libc::c_ulong,
    mut y2: libc::c_ulong,
) {
    let mut x: libc::c_ulong = 0;
    let mut y: libc::c_ulong = 0;
    /* only horz or vert segments allowed */
    if x1 == x2 || y1 == y2 {
    } else {
        __assert_fail(b"(x1 == x2) || (y1 == y2)\x00" as *const u8 as
                          *const libc::c_char,
                      b"maze.c\x00" as *const u8 as *const libc::c_char,
                      93 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"void DrawSegment(char *, unsigned long, unsigned long, unsigned long, unsigned long)\x00")).as_ptr());
    }
    /* must be a line */
    if x1 != x2 || y1 != y2 {
    } else {
        __assert_fail(b"(x1 != x2) || (y1 != y2)\x00" as *const u8 as
                          *const libc::c_char,
                      b"maze.c\x00" as *const u8 as *const libc::c_char,
                      96 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"void DrawSegment(char *, unsigned long, unsigned long, unsigned long, unsigned long)\x00")).as_ptr());
    }
    if x1 == x2 {
        /* vertical */
        /* FROM_BOT at top end */
        /* assert(ACCESS_MAP(plane, x1, min(y1, y2)) == 0); */
        let ref mut fresh0 = *plane.offset(
            (if y1 < y2 { y1 } else { y2 })
                .wrapping_mul(channelColumns)
                .wrapping_add(x1) as isize,
        );
        *fresh0 = (*fresh0 as libc::c_int | FROM_BOT) as libc::c_char;
        /* FROM_TOP|FROM_BOT in the middle */
        y = (if y1 < y2 { y1 } else { y2 }).wrapping_add(1 as libc::c_int as libc::c_ulong);
        while y < (if y1 < y2 { y2 } else { y1 }) {
            /* assert((ACCESS_MAP(plane, x1, y)&(FROM_TOP|FROM_BOT)) == 0); */
            let ref mut fresh1 =
                *plane.offset(y.wrapping_mul(channelColumns).wrapping_add(x1) as isize);
            *fresh1 = (*fresh1 as libc::c_int | (FROM_TOP | FROM_BOT)) as libc::c_char;
            y = y.wrapping_add(1)
        }
        /* FROM_TOP at the bottom end */
        /* assert((ACCESS_MAP(plane, x1, max(y1, y2))&FROM_TOP) == 0); */
        let ref mut fresh2 = *plane.offset(
            (if y1 < y2 { y2 } else { y1 })
                .wrapping_mul(channelColumns)
                .wrapping_add(x1) as isize,
        );
        *fresh2 = (*fresh2 as libc::c_int | FROM_TOP) as libc::c_char
    } else {
        /* (y1 == y2), horizontal */
        /* FROM_RIGHT at left end */
        /* assert((ACCESS_MAP(plane, min(x1, x2), y1)&FROM_RIGHT) == 0); */
        let ref mut fresh3 = *plane.offset(
            y1.wrapping_mul(channelColumns)
                .wrapping_add((if x1 < x2 { x1 } else { x2 })) as isize,
        );
        *fresh3 = (*fresh3 as libc::c_int | FROM_RIGHT) as libc::c_char;
        /* FROM_LEFT|FROM_RIGHT in the middle */
        x = (if x1 < x2 { x1 } else { x2 }).wrapping_add(1 as libc::c_int as libc::c_ulong);
        while x < (if x1 < x2 { x2 } else { x1 }) {
            /* assert((ACCESS_MAP(plane,x,y1)&(FROM_LEFT|FROM_RIGHT)) == 0); */
            *plane.offset(y1.wrapping_mul(channelColumns).wrapping_add(x) as isize) =
                (FROM_LEFT | FROM_RIGHT) as libc::c_char;
            x = x.wrapping_add(1)
        }
        /* FROM_LEFT at the right end */
        /* assert((ACCESS_MAP(plane, max(x1, x2), y1)&FROM_LEFT) == 0);	*/
        let ref mut fresh4 = *plane.offset(
            y1.wrapping_mul(channelColumns)
                .wrapping_add((if x1 < x2 { x2 } else { x1 })) as isize,
        );
        *fresh4 = (*fresh4 as libc::c_int | FROM_LEFT) as libc::c_char
    };
}
/*
 *	draw a via at (x, y)
 */
#[no_mangle]
pub unsafe extern "C" fn DrawVia(mut x: libc::c_ulong, mut y: libc::c_ulong) {
    /* assert(ACCESS_MAP(viaPlane, x, y) == 0); */
    *viaPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize) =
        1 as libc::c_int as libc::c_char;
}
/*
 *	is there a via at (x, y) ?
 */
#[no_mangle]
pub unsafe extern "C" fn HasVia(mut x: libc::c_ulong, mut y: libc::c_ulong) -> libc::c_int {
    return *viaPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
        as libc::c_int;
}
/*
 *	return non-zero if the segment from (x1,y1) to (x2,y2)
 * 	in the plane allocation map, plane, note: no spatial order
 *	is required on the two points, as they are sorted as
 *	they are sorted as needed by the line drawer
 */
#[no_mangle]
pub unsafe extern "C" fn SegmentFree(
    mut plane: *mut libc::c_char,
    mut x1: libc::c_ulong,
    mut y1: libc::c_ulong,
    mut x2: libc::c_ulong,
    mut y2: libc::c_ulong,
) -> libc::c_int {
    let mut x: libc::c_ulong = 0;
    let mut y: libc::c_ulong = 0;
    let mut index: libc::c_ulong = 0;
    /* only horz or vert segments allowed */
    if x1 == x2 || y1 == y2 {
    } else {
        __assert_fail(b"(x1 == x2) || (y1 == y2)\x00" as *const u8 as
                          *const libc::c_char,
                      b"maze.c\x00" as *const u8 as *const libc::c_char,
                      168 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 84],
                                                &[libc::c_char; 84]>(b"int SegmentFree(char *, unsigned long, unsigned long, unsigned long, unsigned long)\x00")).as_ptr());
    }
    if x1 == x2 {
        /* vertical */
        index = (if y1 < y2 { y1 } else { y2 })
            .wrapping_mul(channelColumns)
            .wrapping_add(x1);
        y = if y1 < y2 { y1 } else { y2 };
        while y <= (if y1 < y2 { y2 } else { y1 }) {
            if *plane.offset(index as isize) != 0 {
                return 0 as libc::c_int;
            }
            y = y.wrapping_add(1);
            index = index.wrapping_add(channelColumns)
        }
    } else {
        /* (y1 == y2), horizontal */
        index = y1
            .wrapping_mul(channelColumns)
            .wrapping_add((if x1 < x2 { x1 } else { x2 }));
        x = if x1 < x2 { x1 } else { x2 };
        while x <= (if x1 < x2 { x2 } else { x1 }) {
            if *plane.offset(index as isize) != 0 {
                return 0 as libc::c_int;
            }
            x = x.wrapping_add(1);
            index = index.wrapping_add(1)
        }
    }
    return 1 as libc::c_int;
}
/*
 *	print the channel, note: via crossovers are denoted with
 * 	a "X" and non-via crossovers with a "+"
 */
#[no_mangle]
pub unsafe extern "C" fn PrintChannel() {
    let mut x: libc::c_ulong = 0;
    let mut y: libc::c_ulong = 0;
    /* ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*TOP.offset(x as isize)).wrapping_div(100 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    /* next ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*TOP.offset(x as isize))
                .wrapping_sub(
                    (*TOP.offset(x as isize))
                        .wrapping_div(100 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(100 as libc::c_int as libc::c_ulong),
                )
                .wrapping_div(10 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    /* next ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*TOP.offset(x as isize)).wrapping_rem(10 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b"%%%c%%\x00" as *const u8 as *const libc::c_char,
            if *vertPlane.offset(
                (0 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(channelColumns)
                    .wrapping_add(x) as isize,
            ) as libc::c_int
                != 0
            {
                '|' as i32
            } else {
                ' ' as i32
            },
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    y = 1 as libc::c_int as libc::c_ulong;
    while y <= channelTracks {
        printf(b"           \x00" as *const u8 as *const libc::c_char);
        x = 1 as libc::c_int as libc::c_ulong;
        while x <= channelColumns {
            if *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_TOP
                != 0
            {
                printf(b" | \x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b"   \x00" as *const u8 as *const libc::c_char);
            }
            x = x.wrapping_add(1)
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(b"Track %3d: \x00" as *const u8 as *const libc::c_char, y);
        x = 1 as libc::c_int as libc::c_ulong;
        while x <= channelColumns {
            if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_LEFT
                != 0
                && *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                    as libc::c_int
                    & FROM_LEFT
                    != 0
            {
                printf(b"=\x00" as *const u8 as *const libc::c_char);
            } else if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_LEFT
                != 0
            {
                printf(b"-\x00" as *const u8 as *const libc::c_char);
            } else if *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_LEFT
                != 0
            {
                printf(b"^\x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if *viaPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize) != 0 {
                printf(b"X\x00" as *const u8 as *const libc::c_char);
            } else if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                != 0
                && *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                    as libc::c_int
                    != 0
            {
                printf(b"+\x00" as *const u8 as *const libc::c_char);
            } else if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                != 0
            {
                printf(b"-\x00" as *const u8 as *const libc::c_char);
            } else if *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                != 0
            {
                printf(b"|\x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_RIGHT
                != 0
                && *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                    as libc::c_int
                    & FROM_RIGHT
                    != 0
            {
                printf(b"=\x00" as *const u8 as *const libc::c_char);
            } else if *horzPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_RIGHT
                != 0
            {
                printf(b"-\x00" as *const u8 as *const libc::c_char);
            } else if *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_RIGHT
                != 0
            {
                printf(b"^\x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            x = x.wrapping_add(1)
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        printf(b"           \x00" as *const u8 as *const libc::c_char);
        x = 1 as libc::c_int as libc::c_ulong;
        while x <= channelColumns {
            if *vertPlane.offset(y.wrapping_mul(channelColumns).wrapping_add(x) as isize)
                as libc::c_int
                & FROM_BOT
                != 0
            {
                printf(b" | \x00" as *const u8 as *const libc::c_char);
            } else {
                printf(b"   \x00" as *const u8 as *const libc::c_char);
            }
            x = x.wrapping_add(1)
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        y = y.wrapping_add(1)
    }
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b"%%%c%%\x00" as *const u8 as *const libc::c_char,
            if *vertPlane.offset(
                channelTracks
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(channelColumns)
                    .wrapping_add(x) as isize,
            ) as libc::c_int
                != 0
            {
                '|' as i32
            } else {
                ' ' as i32
            },
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    /* ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*BOT.offset(x as isize)).wrapping_div(100 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    /* next ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*BOT.offset(x as isize))
                .wrapping_sub(
                    (*BOT.offset(x as isize))
                        .wrapping_div(100 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(100 as libc::c_int as libc::c_ulong),
                )
                .wrapping_div(10 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    /* next ms digit */
    printf(b"           \x00" as *const u8 as *const libc::c_char);
    x = 1 as libc::c_int as libc::c_ulong;
    while x <= channelColumns {
        printf(
            b" %d \x00" as *const u8 as *const libc::c_char,
            (*BOT.offset(x as isize)).wrapping_rem(10 as libc::c_int as libc::c_ulong),
        );
        x = x.wrapping_add(1)
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
}
/*
 *	draw the assigned nets, except for any segments that need to be
 *	maze routed, this should be called just before maze routing
 *	starts, DrawSegment() will catch any overlapp errors, vias
 * 	are also drawn in this routine, and the mazeRoute flags
 *	are set, returns the total columns that need to be maze routed
 */
#[no_mangle]
pub unsafe extern "C" fn DrawNets() -> libc::c_int {
    let mut i: libc::c_ulong = 0;
    let mut numLeft: libc::c_int = 0 as libc::c_int;
    /* initialize maps to empty */
    bzero(
        horzPlane as *mut libc::c_void,
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as libc::c_int as libc::c_ulong,
    );
    bzero(
        vertPlane as *mut libc::c_void,
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as libc::c_int as libc::c_ulong,
    );
    bzero(
        viaPlane as *mut libc::c_void,
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(channelTracks.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as libc::c_int as libc::c_ulong,
    );
    bzero(
        mazeRoute as *mut libc::c_void,
        channelColumns.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int
            as libc::c_ulong,
    );
    /* draw all horizontal segments */
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= channelNets {
        if *FIRST.offset(i as isize) != *LAST.offset(i as isize) {
            DrawSegment(
                horzPlane,
                *FIRST.offset(i as isize),
                *netsAssign.offset(i as isize),
                *LAST.offset(i as isize),
                *netsAssign.offset(i as isize),
            );
        }
        i = i.wrapping_add(1)
    }
    /* draw all vertical segments that do not require maze routing */
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= channelColumns {
        if !(*BOT.offset(i as isize) == 0 as libc::c_int as libc::c_ulong
            && *TOP.offset(i as isize) == 0 as libc::c_int as libc::c_ulong)
        {
            if *BOT.offset(i as isize) == 0 as libc::c_int as libc::c_ulong
                && *TOP.offset(i as isize) != 0 as libc::c_int as libc::c_ulong
            {
                /* only one segment, therefore no vertical constraint violation */
                DrawSegment(
                    vertPlane,
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    i,
                    *netsAssign.offset(*TOP.offset(i as isize) as isize),
                );
                DrawVia(i, *netsAssign.offset(*TOP.offset(i as isize) as isize));
            } else if *TOP.offset(i as isize) == 0 as libc::c_int as libc::c_ulong
                && *BOT.offset(i as isize) != 0 as libc::c_int as libc::c_ulong
            {
                /* only one segment, therefore no vertical constraint violation */
                DrawSegment(
                    vertPlane,
                    i,
                    *netsAssign.offset(*BOT.offset(i as isize) as isize),
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                DrawVia(i, *netsAssign.offset(*BOT.offset(i as isize) as isize));
            } else if *TOP.offset(i as isize) == *BOT.offset(i as isize)
                && *FIRST.offset(*TOP.offset(i as isize) as isize)
                    == *LAST.offset(*TOP.offset(i as isize) as isize)
            {
                /* two segments to route */
                /* same net, no track needed to route */
                if *FIRST.offset(*TOP.offset(i as isize) as isize) == i
                    && *LAST.offset(*TOP.offset(i as isize) as isize) == i
                {
                } else {
                    __assert_fail(
                        b"(FIRST[TOP[i]] == i) && (LAST[TOP[i]] == i)\x00" as *const u8
                            as *const libc::c_char,
                        b"maze.c\x00" as *const u8 as *const libc::c_char,
                        358 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                            b"int DrawNets(void)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                DrawSegment(
                    vertPlane,
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            } else if *TOP.offset(i as isize) == *BOT.offset(i as isize) {
                /* connecting to same track, therefore no vcv */
                DrawSegment(
                    vertPlane,
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                DrawVia(i, *netsAssign.offset(*BOT.offset(i as isize) as isize));
            } else if *netsAssign.offset(*TOP.offset(i as isize) as isize)
                < *netsAssign.offset(*BOT.offset(i as isize) as isize)
            {
                /* two segments to route, going to different tracks */
                /* no vertical constraint violation */
                DrawSegment(
                    vertPlane,
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    i,
                    *netsAssign.offset(*TOP.offset(i as isize) as isize),
                );
                DrawVia(i, *netsAssign.offset(*TOP.offset(i as isize) as isize));
                DrawSegment(
                    vertPlane,
                    i,
                    *netsAssign.offset(*BOT.offset(i as isize) as isize),
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                DrawVia(i, *netsAssign.offset(*BOT.offset(i as isize) as isize));
            } else {
                /* otherwise, maze routing is required */
                if *netsAssign.offset(*TOP.offset(i as isize) as isize)
                    > *netsAssign.offset(*BOT.offset(i as isize) as isize)
                {
                } else {
                    __assert_fail(
                        b"netsAssign[TOP[i]] > netsAssign[BOT[i]]\x00" as *const u8
                            as *const libc::c_char,
                        b"maze.c\x00" as *const u8 as *const libc::c_char,
                        384 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                            b"int DrawNets(void)\x00",
                        ))
                        .as_ptr(),
                    );
                }
                *mazeRoute.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                numLeft += 1
            }
        }
        i = i.wrapping_add(1)
    }
    return numLeft;
}
/*
 * clean net "net", that is, check if it is complete, and if so
 * trim the ends of any un-needed horizontal net segments, this does
 * not need to be too efficient as it is only called when a colunm
 * is mazed
 */
unsafe extern "C" fn CleanNet(mut net: libc::c_ulong) {
    let mut i: libc::c_ulong = 0;
    let mut firstVia: libc::c_ulong = 0;
    let mut lastVia: libc::c_ulong = 0;
    let mut effFIRST: libc::c_ulong = 0;
    let mut effLAST: libc::c_ulong = 0;
    let mut track: libc::c_ulong = 0;
    /* is this net finished */
    i = *FIRST.offset(net as isize);
    while i <= *LAST.offset(net as isize) {
        if (*TOP.offset(i as isize) == net || *BOT.offset(i as isize) == net)
            && *mazeRoute.offset(i as isize) as libc::c_int != 0
        {
            return;
        }
        i = i.wrapping_add(1)
        /* not done, leave it alone */
    }
    track = *netsAssign.offset(net as isize);
    /* find effective FIRST and LAST */
    effFIRST = *FIRST.offset(net as isize);
    while *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(effFIRST) as isize)
        as libc::c_int
        & FROM_LEFT
        != 0
    {
        effFIRST = effFIRST.wrapping_sub(1)
    }
    effLAST = *LAST.offset(net as isize);
    while *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(effLAST) as isize)
        as libc::c_int
        & FROM_RIGHT
        != 0
    {
        effLAST = effLAST.wrapping_add(1)
    }
    /* net is finished */
    firstVia = 9999999 as libc::c_int as libc::c_ulong;
    lastVia = 0 as libc::c_int as libc::c_ulong;
    i = effFIRST;
    while i <= effLAST {
        if HasVia(i, track) != 0 {
            if i < firstVia {
                firstVia = i
            }
            if i > lastVia {
                lastVia = i
            }
        }
        i = i.wrapping_add(1)
    }
    if effFIRST < firstVia {
        /* clean up the segment */
        i = effFIRST;
        while i < firstVia {
            *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(i) as isize) =
                0 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1)
        }
        /* and the left edge at the via */
        let ref mut fresh5 =
            *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(firstVia) as isize);
        *fresh5 = (*fresh5 as libc::c_int & !FROM_LEFT) as libc::c_char
    }
    if lastVia < effLAST {
        /* get the right edge */
        let ref mut fresh6 =
            *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(lastVia) as isize);
        *fresh6 = (*fresh6 as libc::c_int & !FROM_RIGHT) as libc::c_char;
        /* clean up the segment */
        i = lastVia.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while i <= effLAST {
            *horzPlane.offset(track.wrapping_mul(channelColumns).wrapping_add(i) as isize) =
                0 as libc::c_int as libc::c_char;
            i = i.wrapping_add(1)
        }
    };
}
/*
 * return non-zero if there is a vertical constraint violation in col "i"
 */
unsafe extern "C" fn HasVCV(mut i: libc::c_ulong) -> libc::c_int {
    return (*TOP.offset(i as isize) != 0 as libc::c_int as libc::c_ulong
        && *BOT.offset(i as isize) != 0 as libc::c_int as libc::c_ulong
        && *TOP.offset(i as isize) != *BOT.offset(i as isize)
        && *netsAssign.offset(*TOP.offset(i as isize) as isize)
            > *netsAssign.offset(*BOT.offset(i as isize) as isize)) as libc::c_int;
}
/*
 *	maze1 route the channel, return the number of channels,
 *	with vertical constraint violations, that could not be maze1
 *	routed
 */
unsafe extern "C" fn Maze1Mech(
    mut i: libc::c_ulong,
    mut s1: libc::c_ulong,
    mut s2: libc::c_ulong,
    mut b1: libc::c_ulong,
    mut b2: libc::c_ulong,
    mut bXdelta: libc::c_int,
    mut bYdelta: libc::c_int,
) -> libc::c_int
/* bend X, Y delta from s */ {
    if SegmentFree(vertPlane, i, s1, i, s2) != 0
        && SegmentFree(
            vertPlane,
            i,
            b1,
            i,
            s2.wrapping_add(bYdelta as libc::c_ulong),
        ) != 0
        && SegmentFree(
            vertPlane,
            i,
            s2.wrapping_add(bYdelta as libc::c_ulong),
            i.wrapping_add(bXdelta as libc::c_ulong),
            s2.wrapping_add(bYdelta as libc::c_ulong),
        ) != 0
        && SegmentFree(
            vertPlane,
            i.wrapping_add(bXdelta as libc::c_ulong),
            s2.wrapping_add(bYdelta as libc::c_ulong),
            i.wrapping_add(bXdelta as libc::c_ulong),
            b2,
        ) != 0
        && HasVCV(i.wrapping_add(bXdelta as libc::c_ulong)) == 0
    {
        DrawSegment(vertPlane, i, s1, i, s2); /* via down to horz plane */
        DrawVia(i, s2); /* via down to horz plane */
        DrawSegment(
            vertPlane,
            i,
            b1,
            i,
            s2.wrapping_add(bYdelta as libc::c_ulong),
        );
        DrawSegment(
            vertPlane,
            i,
            s2.wrapping_add(bYdelta as libc::c_ulong),
            i.wrapping_add(bXdelta as libc::c_ulong),
            s2.wrapping_add(bYdelta as libc::c_ulong),
        );
        DrawSegment(
            vertPlane,
            i.wrapping_add(bXdelta as libc::c_ulong),
            s2.wrapping_add(bYdelta as libc::c_ulong),
            i.wrapping_add(bXdelta as libc::c_ulong),
            b2,
        );
        DrawVia(i.wrapping_add(bXdelta as libc::c_ulong), b2);
        DrawSegment(
            horzPlane,
            i.wrapping_add(bXdelta as libc::c_ulong),
            b2,
            i,
            b2,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Maze1() -> libc::c_int {
    let mut numLeft: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_ulong = 0;
    let mut s: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= channelColumns {
        if *mazeRoute.offset(i as isize) != 0 {
            s = *netsAssign.offset(*TOP.offset(i as isize) as isize);
            p = *netsAssign.offset(*BOT.offset(i as isize) as isize);
            if s > p {
            } else {
                __assert_fail(
                    b"s > p\x00" as *const u8 as *const libc::c_char,
                    b"maze.c\x00" as *const u8 as *const libc::c_char,
                    537 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"int Maze1(void)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if i > 1 as libc::c_int as libc::c_ulong
                && p > 1 as libc::c_int as libc::c_ulong
                && Maze1Mech(
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    -(1 as libc::c_int),
                    -(1 as libc::c_int),
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i < channelColumns
                && p > 1 as libc::c_int as libc::c_ulong
                && Maze1Mech(
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    1 as libc::c_int,
                    -(1 as libc::c_int),
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i > 1 as libc::c_int as libc::c_ulong
                && s < channelTracks
                && Maze1Mech(
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i < channelColumns
                && s < channelTracks
                && Maze1Mech(
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    1 as libc::c_int,
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else {
                /* could not maze1 route this column */
                numLeft += 1
            }
        }
        i = i.wrapping_add(1)
    }
    return numLeft;
}
/*
 *	maze2 route the channel, return the number of channels,
 *	with vertical constraint violations, that could not be maze1
 *	routed
 */
/*
 * can this track be extended to the range specified, return result
 */
#[no_mangle]
pub unsafe extern "C" fn ExtendOK(
    mut net: libc::c_ulong,
    mut plane: *mut libc::c_char,
    mut _x1: libc::c_ulong,
    mut _y1: libc::c_ulong,
    mut _x2: libc::c_ulong,
    mut _y2: libc::c_ulong,
) -> libc::c_int
/* end seg */ {
    let mut x1: libc::c_ulong = 0;
    let mut y1: libc::c_ulong = 0;
    let mut x2: libc::c_ulong = 0;
    let mut y2: libc::c_ulong = 0;
    /* sort, (x1,y1) => (x2,y2) */
    x1 = if _x1 < _x2 { _x1 } else { _x2 }; /* inside the net */
    y1 = if _y1 < _y2 { _y1 } else { _y2 };
    x2 = if _x1 < _x2 { _x2 } else { _x1 };
    y2 = if _y1 < _y2 { _y2 } else { _y1 };
    if y1 == y2 && *netsAssign.offset(net as isize) == y1 {
    } else {
        __assert_fail(b"(y1 == y2) && (netsAssign[net] == y1)\x00" as
                          *const u8 as *const libc::c_char,
                      b"maze.c\x00" as *const u8 as *const libc::c_char,
                      659 as libc::c_int as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 96],
                                                &[libc::c_char; 96]>(b"int ExtendOK(unsigned long, char *, unsigned long, unsigned long, unsigned long, unsigned long)\x00")).as_ptr());
    }
    if x1 >= *FIRST.offset(net as isize) && x2 <= *LAST.offset(net as isize) {
        return 1 as libc::c_int;
    }
    if x1 < *FIRST.offset(net as isize) && x2 > *LAST.offset(net as isize) {
        /* subsumes */
        return (SegmentFree(
            plane,
            x1,
            y1,
            (*FIRST.offset(net as isize)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            y1,
        ) != 0
            && SegmentFree(
                plane,
                (*LAST.offset(net as isize)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                y1,
                x2,
                y1,
            ) != 0) as libc::c_int;
    } else {
        if x1 < *FIRST.offset(net as isize) {
            /* to the left possibly overlapping */
            return SegmentFree(
                plane,
                x1,
                y1,
                (*FIRST.offset(net as isize)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
                y1,
            );
        } else {
            if x2 > *LAST.offset(net as isize) {
                /* to the right possibly overlapping */
                return SegmentFree(
                    plane,
                    (*LAST.offset(net as isize)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                    y1,
                    x2,
                    y1,
                );
            }
        }
    }
    /* should not get here */
    abort();
}
unsafe extern "C" fn Maze2Mech(
    mut bentNet: libc::c_ulong,
    mut i: libc::c_ulong,
    mut s1: libc::c_ulong,
    mut s2: libc::c_ulong,
    mut b1: libc::c_ulong,
    mut b2: libc::c_ulong,
    mut xStart: libc::c_ulong,
    mut xEnd: libc::c_ulong,
    mut bXdelta: libc::c_int,
    mut yStart: libc::c_ulong,
    mut yEnd: libc::c_ulong,
    mut bYdelta: libc::c_int,
) -> libc::c_int
/* direction of bend vert seg */ {
    let mut row: libc::c_ulong = 0; /* so I can use != */
    let mut col: libc::c_ulong = 0;
    let mut colFree: libc::c_int = 0;
    xEnd = xEnd.wrapping_add(bXdelta as libc::c_ulong);
    yEnd = yEnd.wrapping_add(bYdelta as libc::c_ulong);
    row = yStart;
    while row != yEnd {
        /* search for row */
        colFree = 1 as libc::c_int;
        col = xStart;
        while colFree != 0 && col != xEnd {
            /* search for col */
            colFree = SegmentFree(horzPlane, i, row, col, row);
            if colFree != 0
                && SegmentFree(vertPlane, i, s1, i, s2) != 0
                && SegmentFree(vertPlane, i, b1, i, row) != 0
                && SegmentFree(
                    vertPlane,
                    col,
                    row,
                    col,
                    b2.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) != 0
                && HasVCV(col) == 0
                && ExtendOK(bentNet, horzPlane, col, b2, i, b2) != 0
            {
                /* draw it! */
                DrawSegment(vertPlane, i, s1, i, s2); /* conn to horz */
                DrawVia(i, s2); /* conn to horz plane */
                DrawSegment(vertPlane, i, b1, i, row); /* conn to vert plane */
                DrawVia(i, row); /* back to horz plane */
                DrawSegment(horzPlane, i, row, col, row);
                DrawVia(col, row);
                DrawSegment(vertPlane, col, row, col, b2);
                DrawVia(col, b2);
                DrawSegment(horzPlane, col, b2, i, b2);
                return 1 as libc::c_int;
            }
            col = col.wrapping_add(bXdelta as libc::c_ulong)
        }
        row = row.wrapping_add(bYdelta as libc::c_ulong)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Maze2() -> libc::c_int {
    let mut numLeft: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_ulong = 0;
    let mut s: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= channelColumns {
        if *mazeRoute.offset(i as isize) != 0 {
            s = *netsAssign.offset(*TOP.offset(i as isize) as isize);
            p = *netsAssign.offset(*BOT.offset(i as isize) as isize);
            if s > p {
            } else {
                __assert_fail(
                    b"s > p\x00" as *const u8 as *const libc::c_char,
                    b"maze.c\x00" as *const u8 as *const libc::c_char,
                    772 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"int Maze2(void)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if i > 1 as libc::c_int as libc::c_ulong
                && p > 1 as libc::c_int as libc::c_ulong
                && Maze2Mech(
                    *TOP.offset(i as isize),
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as libc::c_ulong,
                    -(1 as libc::c_int),
                    1 as libc::c_int as libc::c_ulong,
                    p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i < channelColumns
                && p > 1 as libc::c_int as libc::c_ulong
                && Maze2Mech(
                    *TOP.offset(i as isize),
                    i,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    channelColumns,
                    1 as libc::c_int,
                    1 as libc::c_int as libc::c_ulong,
                    p.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i > 1 as libc::c_int as libc::c_ulong
                && s < channelTracks
                && Maze2Mech(
                    *BOT.offset(i as isize),
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    1 as libc::c_int as libc::c_ulong,
                    -(1 as libc::c_int),
                    s.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    channelTracks,
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else if i < channelColumns
                && s < channelTracks
                && Maze2Mech(
                    *BOT.offset(i as isize),
                    i,
                    0 as libc::c_int as libc::c_ulong,
                    s,
                    channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    p,
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    channelColumns,
                    1 as libc::c_int,
                    s.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    channelTracks,
                    1 as libc::c_int,
                ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else {
                /* could not maze2 route this column */
                numLeft += 1
            }
        }
        i = i.wrapping_add(1)
    }
    return numLeft;
}
#[no_mangle]
pub unsafe extern "C" fn FindFreeHorzSeg(
    mut startCol: libc::c_ulong,
    mut row: libc::c_ulong,
    mut rowStart: *mut libc::c_ulong,
    mut rowEnd: *mut libc::c_ulong,
) {
    let mut i: libc::c_ulong = 0;
    i = startCol;
    while i >= 1 as libc::c_int as libc::c_ulong {
        if *horzPlane.offset(row.wrapping_mul(channelColumns).wrapping_add(i) as isize) != 0 {
            break;
        }
        i = i.wrapping_sub(1)
    }
    *rowStart = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    i = startCol;
    while i <= channelColumns {
        if *horzPlane.offset(row.wrapping_mul(channelColumns).wrapping_add(i) as isize) != 0 {
            break;
        }
        i = i.wrapping_add(1)
    }
    *rowEnd = i.wrapping_sub(1 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn Maze3Mech(
    mut topNet: libc::c_ulong,
    mut botNet: libc::c_ulong,
    mut i: libc::c_ulong,
    mut s1: libc::c_ulong,
    mut s2: libc::c_ulong,
    mut b1: libc::c_ulong,
    mut b2: libc::c_ulong,
) -> libc::c_int
/* s1, b1 are at the terminals */ {
    let mut topRow: libc::c_ulong = 0;
    let mut topCol: libc::c_ulong = 0;
    let mut botRow: libc::c_ulong = 0;
    let mut botCol: libc::c_ulong = 0;
    let mut topStart: libc::c_ulong = 0;
    let mut topEnd: libc::c_ulong = 0;
    let mut botStart: libc::c_ulong = 0;
    let mut botEnd: libc::c_ulong = 0;
    topRow = b2.wrapping_add(1 as libc::c_int as libc::c_ulong);
    while topRow < s2.wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        FindFreeHorzSeg(i, topRow, &mut topStart, &mut topEnd);
        if !(topEnd <= topStart) {
            botRow = topRow.wrapping_add(1 as libc::c_int as libc::c_ulong);
            while botRow < s2 {
                FindFreeHorzSeg(i, botRow, &mut botStart, &mut botEnd);
                if !(botEnd <= botStart) {
                    topCol = topStart;
                    while topCol <= topEnd {
                        botCol = botStart;
                        while botCol <= botEnd {
                            if topCol != i
                                && botCol != i
                                && topRow != botRow
                                && topCol != botCol
                                && SegmentFree(vertPlane, i, s1, i, topRow) != 0
                                && SegmentFree(horzPlane, i, topRow, topCol, topRow) != 0
                                && SegmentFree(
                                    vertPlane,
                                    topCol,
                                    topRow,
                                    topCol,
                                    s2.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                ) != 0
                                && HasVCV(topCol) == 0
                                && ExtendOK(topNet, horzPlane, topCol, s2, i, s2) != 0
                                && SegmentFree(vertPlane, i, b1, i, botRow) != 0
                                && SegmentFree(horzPlane, i, botRow, botCol, botRow) != 0
                                && SegmentFree(
                                    vertPlane,
                                    botCol,
                                    botRow,
                                    botCol,
                                    b2.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) != 0
                                && HasVCV(botCol) == 0
                                && ExtendOK(botNet, horzPlane, botCol, b2, i, b2) != 0
                            {
                                /* draw it! */
                                DrawSegment(vertPlane, i, s1, i, topRow); /* via to horz */
                                DrawVia(i, topRow); /* up to vert */
                                DrawSegment(horzPlane, i, topRow, topCol, topRow); /* via to net */
                                DrawVia(topCol, topRow); /* via to horz */
                                DrawSegment(vertPlane, topCol, topRow, topCol, s2); /* via to vert */
                                DrawVia(topCol, s2); /* via to net */
                                DrawSegment(horzPlane, topCol, s2, i, s2);
                                DrawSegment(vertPlane, i, b1, i, botRow);
                                DrawVia(i, botRow);
                                DrawSegment(horzPlane, i, botRow, botCol, botRow);
                                DrawVia(botCol, botRow);
                                DrawSegment(vertPlane, botCol, botRow, botCol, b2);
                                DrawVia(botCol, b2);
                                DrawSegment(horzPlane, botCol, b2, i, b2);
                                return 1 as libc::c_int;
                            }
                            botCol = botCol.wrapping_add(1)
                        }
                        topCol = topCol.wrapping_add(1)
                    }
                }
                botRow = botRow.wrapping_add(1)
            }
        }
        topRow = topRow.wrapping_add(1)
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Maze3() -> libc::c_int {
    let mut numLeft: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_ulong = 0;
    let mut s: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    i = 1 as libc::c_int as libc::c_ulong;
    while i <= channelColumns {
        if *mazeRoute.offset(i as isize) != 0 {
            s = *netsAssign.offset(*TOP.offset(i as isize) as isize);
            p = *netsAssign.offset(*BOT.offset(i as isize) as isize);
            if s > p {
            } else {
                __assert_fail(
                    b"s > p\x00" as *const u8 as *const libc::c_char,
                    b"maze.c\x00" as *const u8 as *const libc::c_char,
                    1004 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"int Maze3(void)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if Maze3Mech(
                *TOP.offset(i as isize),
                *BOT.offset(i as isize),
                i,
                0 as libc::c_int as libc::c_ulong,
                s,
                channelTracks.wrapping_add(1 as libc::c_int as libc::c_ulong),
                p,
            ) != 0
            {
                *mazeRoute.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                CleanNet(*TOP.offset(i as isize));
                CleanNet(*BOT.offset(i as isize));
            } else {
                /* could not maze2 route this column */
                numLeft += 1
            }
        }
        i = i.wrapping_add(1)
    }
    return numLeft;
}
