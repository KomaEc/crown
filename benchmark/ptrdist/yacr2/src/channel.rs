use ::libc;
extern "C" {
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
pub const EOF: libc::c_int = -(1 as libc::c_int);
pub const NULL: libc::c_int = 0 as libc::c_int;
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
#[no_mangle]
pub static mut TOP: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut BOT: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut FIRST: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut LAST: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut DENSITY: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut CROSSING: *mut libc::c_ulong = 0 as *const libc::c_ulong as *mut libc::c_ulong;
#[no_mangle]
pub static mut channelNets: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelColumns: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelTracks: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelTracksCopy: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelDensity: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelDensityColumn: libc::c_ulong = 0;
#[no_mangle]
pub static mut channelFile: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
/* CHANNEL_CODE */
/* CHANNEL_CODE */
/*
 *
 * Prototypes.
 *
 */
/*
 *
 * channel.c
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
pub unsafe extern "C" fn BuildChannel() {
    /*
     * What is the length dimension of the channel?
     * That is, how many columns in the channel?
     */
    DimensionChannel();
    /*
     * What are the particulargs of the channel?
     * That is, what are the nets and pin locations?
     */
    DescribeChannel();
    /*
     * What is the maximum density of the channel?
     * That is, what is the lower bound on the number of tracks
     * required to route the channel?
     */
    DensityChannel();
}
#[no_mangle]
pub unsafe extern "C" fn DimensionChannel() {
    let mut channelFP: *mut FILE = 0 as *mut FILE;
    let mut line: libc::c_ulong = 0;
    let mut dim: libc::c_ulong = 0;
    let mut net: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut stat: libc::c_long = 0;
    /*
     * Open channel description file.
     */
    channelFP = fopen(channelFile, b"r\x00" as *const u8 as *const libc::c_char);
    if channelFP.is_null() {
        /*
         * Error in channel file description.
         */
        printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel file cannot be opened.\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /*
     * Scan the file to find the last column
     * number.  The channel file description
     * contains non-negative integers in the
     * format...
     *
     * [column #] [bottom net #] [top net #]
     */
    line = 0 as libc::c_int as libc::c_ulong;
    dim = 0 as libc::c_int as libc::c_ulong;
    net = 0 as libc::c_int as libc::c_ulong;
    loop {
        line = line.wrapping_add(1);
        stat = fscanf(
            channelFP,
            b"%u%u%u\x00" as *const u8 as *const libc::c_char,
            &mut col as *mut libc::c_ulong,
            &mut bot as *mut libc::c_ulong,
            &mut top as *mut libc::c_ulong,
        ) as libc::c_long;
        if stat != EOF as libc::c_long {
            if stat == 3 as libc::c_int as libc::c_long {
                /*
                 * Update column #.
                 */
                if col > dim {
                    dim = col
                }
                /*
                 * Determine how many channel nets.
                 */
                if bot > net {
                    net = bot
                }
                if top > net {
                    net = top
                }
            } else {
                /*
                 * Error in channel file description.
                 */
                printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
                printf(
                    b"\tChannel file description invalid at line %d.\n\x00" as *const u8
                        as *const libc::c_char,
                    line,
                );
                printf(
                    b"\tIncorrect number of specifiers.\n\x00" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if !(stat != EOF as libc::c_long) {
            break;
        }
    }
    /*
     * Close channel description file.
     */
    if fclose(channelFP) == EOF {
        /*
         * Error in channel file description.
         */
        printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel file cannot be closed.\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /*
     * Check channel dimension.
     */
    if dim == 0 as libc::c_int as libc::c_ulong {
        /*
         * Error in channel file description.
         */
        printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel description invalid.\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel has null dimension.\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /*
     * Set global channel info.
     */
    channelColumns = dim;
    channelNets = net;
}
#[no_mangle]
pub unsafe extern "C" fn DescribeChannel() {
    let mut channelFP: *mut FILE = 0 as *mut FILE;
    let mut line: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut bot: libc::c_ulong = 0;
    let mut top: libc::c_ulong = 0;
    let mut stat: libc::c_long = 0;
    /*
     * Top terminals of channel.
     */
    TOP = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    /*
     * Bottom terminals of channel.
     */
    BOT = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    /*
     * Initialize terminals of channel.
     */
    col = 0 as libc::c_int as libc::c_ulong;
    while col <= channelColumns {
        *TOP.offset(col as isize) = 0 as libc::c_int as libc::c_ulong;
        *BOT.offset(col as isize) = 0 as libc::c_int as libc::c_ulong;
        col = col.wrapping_add(1)
    }
    /*
     * Open channel description file.
     */
    channelFP = fopen(channelFile, b"r\x00" as *const u8 as *const libc::c_char);
    if channelFP.is_null() {
        /*
         * Error in channel file description.
         */
        printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel file cannot be opened.\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    /*
     * Scan the file to find the last column
     * number.  The channel file description
     * contains non-negative integers in the
     * format...
     *
     * [column #] [bottom net #] [top net #]
     */
    line = 0 as libc::c_int as libc::c_ulong;
    loop {
        line = line.wrapping_add(1);
        stat = fscanf(
            channelFP,
            b"%u%u%u\x00" as *const u8 as *const libc::c_char,
            &mut col as *mut libc::c_ulong,
            &mut bot as *mut libc::c_ulong,
            &mut top as *mut libc::c_ulong,
        ) as libc::c_long;
        if stat != EOF as libc::c_long {
            if stat == 3 as libc::c_int as libc::c_long {
                /*
                 * Build column.
                 */
                if col > channelColumns {
                    /*
                     * Error in channel file description.
                     */
                    printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
                    printf(
                        b"\tChannel file description invalid at line %d.\n\x00" as *const u8
                            as *const libc::c_char,
                        line,
                    );
                    printf(
                        b"\tColumn number out of range.\n\x00" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                } else {
                    /*
                     * Doit.
                     */
                    *BOT.offset(col as isize) = bot;
                    *TOP.offset(col as isize) = top
                }
            } else {
                /*
                 * Error in channel file description.
                 */
                printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
                printf(
                    b"\tChannel file description invalid at line %d.\n\x00" as *const u8
                        as *const libc::c_char,
                    line,
                );
                printf(
                    b"\tIncorrect number of specifiers.\n\x00" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if !(stat != EOF as libc::c_long) {
            break;
        }
    }
    /*
     * Close channel description file.
     */
    if fclose(channelFP) == EOF {
        /*
         * Error in channel file description.
         */
        printf(b"Error:\n\x00" as *const u8 as *const libc::c_char);
        printf(b"\tChannel file cannot be closed.\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn DensityChannel() {
    let mut init: libc::c_ulong = 0;
    let mut which: libc::c_ulong = 0;
    let mut col: libc::c_ulong = 0;
    let mut bound: libc::c_ulong = 0;
    let mut boundColumn: libc::c_ulong = 0;
    /*
     * Allocate track dimension structures.
     */
    FIRST = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    LAST = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    DENSITY = malloc(
        channelColumns
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    CROSSING = malloc(
        channelNets
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong),
    ) as *mut libc::c_ulong;
    /*
     * Initialize track dimension structures.
     */
    init = 0 as libc::c_int as libc::c_ulong;
    while init <= channelNets {
        *FIRST.offset(init as isize) = 0 as libc::c_int as libc::c_ulong;
        *LAST.offset(init as isize) = 0 as libc::c_int as libc::c_ulong;
        *CROSSING.offset(init as isize) = 0 as libc::c_int as libc::c_ulong;
        init = init.wrapping_add(1)
    }
    init = 0 as libc::c_int as libc::c_ulong;
    while init <= channelColumns {
        *DENSITY.offset(init as isize) = 0 as libc::c_int as libc::c_ulong;
        init = init.wrapping_add(1)
    }
    /*
     * Compute the span for each net.
     */
    which = 1 as libc::c_int as libc::c_ulong;
    while which <= channelNets {
        /*
         * Compute first.  This is the location of
         * the first column for this net.
         */
        col = 1 as libc::c_int as libc::c_ulong;
        while col <= channelColumns {
            if *BOT.offset(col as isize) == which || *TOP.offset(col as isize) == which {
                *FIRST.offset(which as isize) = col;
                break;
            } else {
                col = col.wrapping_add(1)
            }
        }
        /*
         * Compute last.  This is the location of
         * the last column for this net.
         */
        col = channelColumns;
        while col >= 1 as libc::c_int as libc::c_ulong {
            if *BOT.offset(col as isize) == which || *TOP.offset(col as isize) == which {
                *LAST.offset(which as isize) = col;
                break;
            } else {
                col = col.wrapping_sub(1)
            }
        }
        /*
         * Increment the track for each column
         * of the channel which this net spans.
         */
        col = *FIRST.offset(which as isize);
        while col <= *LAST.offset(which as isize) {
            let ref mut fresh0 = *DENSITY.offset(col as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
            col = col.wrapping_add(1)
        }
        which = which.wrapping_add(1)
    }
    /*
     * Compute lower bound on channel tracks.
     */
    bound = 0 as libc::c_int as libc::c_ulong;
    col = channelColumns;
    while col >= 1 as libc::c_int as libc::c_ulong {
        if *DENSITY.offset(col as isize) > bound {
            bound = *DENSITY.offset(col as isize);
            boundColumn = col
        }
        col = col.wrapping_sub(1)
    }
    /*
     * Set global channel info.
     */
    channelTracks = bound; /* tracks available for routing */
    channelDensity = bound; /* max channel density */
    channelDensityColumn = boundColumn;
    /* column of max channel density */
}
