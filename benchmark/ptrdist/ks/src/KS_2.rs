use ::libc;
extern "C" {
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static mut modules: [NetPtr; 1024];
    /* all modules -> nets */
    #[no_mangle]
    static mut numModules: libc::c_ulong;
    #[no_mangle]
    static mut nets: [ModulePtr; 1024];
    /* all nets -> modules */
    #[no_mangle]
    static mut numNets: libc::c_ulong;
    #[no_mangle]
    static mut groupA: ModuleList;
    #[no_mangle]
    static mut groupB: ModuleList;
    /* current A, B */
    #[no_mangle]
    static mut swapToA: ModuleList;
    #[no_mangle]
    static mut swapToB: ModuleList;
    /* swapped from A,B, ordered */
    #[no_mangle]
    static mut GP: [libc::c_float; 1024];
    #[no_mangle]
    static mut moduleToGroup: [Groups; 1024];
    /* current inverse mapping */
    #[no_mangle]
    static mut D: [libc::c_float; 1024];
    /* module costs */
    #[no_mangle]
    static mut cost: [libc::c_float; 1024];
    /* net costs */
    #[no_mangle]
    fn ReadNetList(fname: *mut libc::c_char);
    #[no_mangle]
    fn NetsToModules();
    #[no_mangle]
    fn ComputeNetCosts();
    #[no_mangle]
    fn InitLists();
    #[no_mangle]
    fn ComputeDs(group: ModuleListPtr, myGroup: Groups, mySwap: Groups);
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
pub struct _Net {
    pub next: *mut _Net,
    pub net: libc::c_ulong,
}
/*
 *      program:        Graph partition via Kernighan-Lin, modified
 *                      Kernighan-Lin, or Kernighan-Schweikert
 *
 *      author:         Todd M. Austin
 *                      ECE 756
 *
 *      date:           Thursday, February 25, 1993
 */
/*
 *      module configuration
 */
/* define WEIGHTED for 1/(1-n) weighted node cost matrix, otherwise 1 */
/* #define WEIGHTED */
/* define KS_MODE to execute with Kernighan-Schweikert algorithm, rather
than the Kernighan-Lin algorithm */
/* #undef KS_MODE */
/* maximum line length */
/* maximum group size */
/* simple exception handler */
/*
 *      the network, first the modules, then the nets
 */
/* modular view */
pub type Net = _Net;
pub type NetPtr = *mut Net;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Module {
    pub next: *mut _Module,
    pub module: libc::c_ulong,
}
/* net-ular view */
pub type Module = _Module;
pub type ModulePtr = *mut Module;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ModuleRec {
    pub next: *mut _ModuleRec,
    pub module: libc::c_ulong,
}
pub type ModuleRec = _ModuleRec;
pub type ModuleRecPtr = *mut ModuleRec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ModuleList {
    pub head: ModuleRecPtr,
    pub tail: ModuleRecPtr,
}
pub type ModuleList = _ModuleList;
pub type ModuleListPtr = *mut ModuleList;
pub type Groups = libc::c_uint;
pub const SwappedToB: Groups = 3;
pub const SwappedToA: Groups = 2;
pub const GroupB: Groups = 1;
pub const GroupA: Groups = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub total: libc::c_ulong,
    pub edgesCut: libc::c_ulong,
    pub netsCut: libc::c_ulong,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
/*
 *	program:	Graph partition via Kernighan-Lin, modified
 *			Kernighan-Lin, or Kernighan-Schweikert
 *
 *	author:		Todd M. Austin
 *			ECE 756
 *
 *	date:		Thursday, February 25, 1993
 */
/* handle special cases where both nodes are switched */
#[no_mangle]
pub unsafe extern "C" fn CAiBj(mut mrA: ModuleRecPtr, mut mrB: ModuleRecPtr) -> libc::c_float {
    let mut netNode: NetPtr = 0 as *mut Net;
    let mut modNode: ModulePtr = 0 as *mut Module;
    let mut gain: libc::c_float = 0.0f64 as libc::c_float;
    let mut netCost: libc::c_float = 0.;
    let mut module: libc::c_ulong = (*mrB).module;
    /* is mrA connected to mrB? */
    /* mrA and mrB are both un-Swapped */
    netNode = modules[(*mrA).module as usize];
    while !netNode.is_null() {
        netCost = cost[(*netNode).net as usize];
        modNode = nets[(*netNode).net as usize];
        while !modNode.is_null() {
            if (*modNode).module == module {
                gain = gain + netCost
            }
            modNode = (*modNode).next
        }
        netNode = (*netNode).next
    }
    return gain;
}
/* swap a node out of the current group, and into a target group */
#[no_mangle]
pub unsafe extern "C" fn SwapNode(
    mut maxPrev: ModuleRecPtr,
    mut max: ModuleRecPtr,
    mut group: ModuleListPtr,
    mut swapTo: ModuleListPtr,
) {
    if maxPrev.is_null() {
        /* found at head of list */
        if (*group).head == (*group).tail {
            /* only one in the list */
            (*group).head = NULL as ModuleRecPtr;
            (*group).tail = NULL as ModuleRecPtr;
            (*max).next = NULL as *mut _ModuleRec
        } else {
            (*group).head = (*max).next;
            (*max).next = NULL as *mut _ModuleRec
        }
    } else {
        /* middle or end of list */
        if (*group).tail == max {
            /* end of list */
            (*group).tail = maxPrev
        }
        (*maxPrev).next = (*max).next;
        (*max).next = NULL as *mut _ModuleRec
    }
    /* put max on the tail of swapTo */
    if (*swapTo).tail.is_null() {
        /* empty */
        (*swapTo).tail = max;
        (*swapTo).head = max
    } else {
        /* end of list */
        (*(*swapTo).tail).next = max;
        (*swapTo).tail = max
    }
    (*max).next = NULL as *mut _ModuleRec;
}
/* incrementally update the D values in Kernighan-Lin algorithm */
#[no_mangle]
pub unsafe extern "C" fn UpdateDs(mut max: ModuleRecPtr, mut group: Groups) {
    let mut net: NetPtr = 0 as *mut Net;
    let mut mod_0: ModulePtr = 0 as *mut Module;
    /* for all nets this is connected to */
    net = modules[(*max).module as usize];
    while !net.is_null() {
        /* for a modules this net is connected to */
        mod_0 = nets[(*net).net as usize];
        while !mod_0.is_null() {
            if (moduleToGroup[(*mod_0).module as usize] as libc::c_uint)
                < SwappedToA as libc::c_int as libc::c_uint
            {
                if moduleToGroup[(*mod_0).module as usize] as libc::c_uint == group as libc::c_uint
                {
                    D[(*mod_0).module as usize] =
                        D[(*mod_0).module as usize] + cost[(*net).net as usize]
                } else {
                    D[(*mod_0).module as usize] =
                        D[(*mod_0).module as usize] - cost[(*net).net as usize]
                }
            }
            mod_0 = (*mod_0).next
        }
        net = (*net).next
    }
}
/* find the best swap available and do it */
#[no_mangle]
pub unsafe extern "C" fn FindMaxGpAndSwap() -> libc::c_float {
    let mut mrA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrPrevA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrB: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrPrevB: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut maxA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut maxPrevA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut maxB: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut maxPrevB: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut gp: libc::c_float = 0.;
    let mut gpMax: libc::c_float = 0.;
    gpMax = -(9999999 as libc::c_int) as libc::c_float;
    maxPrevB = NULL as ModuleRecPtr;
    maxB = maxPrevB;
    maxPrevA = maxB;
    maxA = maxPrevA;
    mrA = groupA.head;
    mrPrevA = NULL as ModuleRecPtr;
    while !mrA.is_null() {
        mrB = groupB.head;
        mrPrevB = NULL as ModuleRecPtr;
        while !mrB.is_null() {
            /* !KS_MODE */
            gp = D[(*mrA).module as usize] + D[(*mrB).module as usize]
                - 2 as libc::c_int as libc::c_float * CAiBj(mrA, mrB);
            /* !KS_MODE */
            if gp > gpMax {
                gpMax = gp;
                maxA = mrA;
                maxPrevA = mrPrevA;
                maxB = mrB;
                maxPrevB = mrPrevB
            }
            mrPrevB = mrB;
            mrB = (*mrB).next
        }
        mrPrevA = mrA;
        mrA = (*mrA).next
    }
    /* swap the nodes out, into the swap lists */
    if !maxA.is_null() {
    } else {
        __assert_fail(
            b"maxA != NULL\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"float FindMaxGpAndSwap()\x00",
            ))
            .as_ptr(),
        );
    }
    SwapNode(maxPrevA, maxA, &mut groupA, &mut swapToB);
    if !maxB.is_null() {
    } else {
        __assert_fail(
            b"maxB != NULL\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"float FindMaxGpAndSwap()\x00",
            ))
            .as_ptr(),
        );
    }
    SwapNode(maxPrevB, maxB, &mut groupB, &mut swapToA);
    /* update the inverse mapping, these two node are now gone */
    if moduleToGroup[(*maxA).module as usize] as libc::c_uint
        == GroupA as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(
            b"moduleToGroup[(*maxA).module] == GroupA\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"float FindMaxGpAndSwap()\x00",
            ))
            .as_ptr(),
        );
    }
    moduleToGroup[(*maxA).module as usize] = SwappedToB;
    if moduleToGroup[(*maxB).module as usize] as libc::c_uint
        == GroupB as libc::c_int as libc::c_uint
    {
    } else {
        __assert_fail(
            b"moduleToGroup[(*maxB).module] == GroupB\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"float FindMaxGpAndSwap()\x00",
            ))
            .as_ptr(),
        );
    }
    moduleToGroup[(*maxB).module as usize] = SwappedToA;
    /* update the Ds */
    UpdateDs(maxA, GroupA);
    UpdateDs(maxB, GroupB);
    /* !KS_MODE */
    return gpMax;
}
/* find the best point, during the last numModules/2 swaps */
#[no_mangle]
pub unsafe extern "C" fn FindGMax(mut iMax: *mut libc::c_ulong) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut gMax: libc::c_float = 0.;
    gMax = -(9999999 as libc::c_int) as libc::c_float;
    *iMax = 0xffffffff as libc::c_uint as libc::c_ulong;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < numModules.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        if GP[i as usize] > gMax {
            gMax = GP[i as usize];
            *iMax = i as libc::c_ulong
        }
        i += 1
    }
    return gMax;
}
/* swap groupA and groupB from [0..iMax] */
#[no_mangle]
pub unsafe extern "C" fn SwapSubsetAndReset(mut iMax: libc::c_ulong) {
    let mut i: libc::c_ulong = 0;
    let mut mrPrevA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrA: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrPrevB: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut mrB: ModuleRecPtr = 0 as *mut ModuleRec;
    /* re-splice the lists @ iMax pointers into the lists */
    mrPrevA = NULL as ModuleRecPtr;
    mrA = swapToA.head;
    mrPrevB = NULL as ModuleRecPtr;
    mrB = swapToB.head;
    i = 0 as libc::c_int as libc::c_ulong;
    while i <= iMax {
        mrPrevA = mrA;
        mrA = (*mrA).next;
        mrPrevB = mrB;
        mrB = (*mrB).next;
        i = i.wrapping_add(1)
    }
    /* must at least select one to swap, case where gMax is first */
    if !mrPrevA.is_null() && !mrPrevB.is_null() {
    } else {
        __assert_fail(
            b"mrPrevA != NULL && mrPrevB != NULL\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                b"void SwapSubsetAndReset(unsigned long)\x00",
            ))
            .as_ptr(),
        );
    }
    if mrA.is_null() {
        /* swap entire list */
        groupA = swapToA;
        groupB = swapToB
    } else {
        /* splice the lists */
        (*mrPrevA).next = mrB;
        groupA.head = swapToA.head;
        groupA.tail = swapToB.tail;
        (*mrPrevB).next = mrA;
        groupB.head = swapToB.head;
        groupB.tail = swapToA.tail
    }
    /* reset the inverse mappings */
    mrA = groupA.head;
    while !mrA.is_null() {
        moduleToGroup[(*mrA).module as usize] = GroupA;
        mrA = (*mrA).next
    }
    mrB = groupB.head;
    while !mrB.is_null() {
        moduleToGroup[(*mrB).module as usize] = GroupB;
        mrB = (*mrB).next
    }
    /* clear the swap lists */
    swapToA.tail = NULL as ModuleRecPtr;
    swapToA.head = swapToA.tail;
    swapToB.tail = NULL as ModuleRecPtr;
    swapToB.head = swapToB.tail;
}
#[no_mangle]
pub static mut netStats: [C2RustUnnamed; 256] = [C2RustUnnamed {
    total: 0,
    edgesCut: 0,
    netsCut: 0,
}; 256];
#[no_mangle]
pub static mut maxStat: libc::c_long = 0;
/* print the current groups, and their edge and net cut counts */
#[no_mangle]
pub unsafe extern "C" fn PrintResults(mut verbose: libc::c_int) {
    let mut mr: ModuleRecPtr = 0 as *mut ModuleRec;
    let mut nn: NetPtr = 0 as *mut Net;
    let mut mn: ModulePtr = 0 as *mut Module;
    let mut cuts: libc::c_ulong = 0;
    let mut grp: Groups = GroupA;
    let mut i: libc::c_int = 0;
    let mut netSz: libc::c_int = 0;
    fprintf(
        stdout,
        b"----------------------------------------------\n\x00" as *const u8 as *const libc::c_char,
    );
    maxStat = -(1 as libc::c_int) as libc::c_long;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        netStats[i as usize].netsCut = 0 as libc::c_int as libc::c_ulong;
        netStats[i as usize].edgesCut = netStats[i as usize].netsCut;
        netStats[i as usize].total = netStats[i as usize].edgesCut;
        i += 1
    }
    /* partitions */
    if verbose != 0 {
        fprintf(
            stdout,
            b"Group A:  \n\x00" as *const u8 as *const libc::c_char,
        );
        mr = groupA.head;
        while !mr.is_null() {
            fprintf(
                stdout,
                b"%3lu \x00" as *const u8 as *const libc::c_char,
                (*mr).module.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            mr = (*mr).next
        }
        fprintf(stdout, b"\n\x00" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"Group B:  \n\x00" as *const u8 as *const libc::c_char,
        );
        mr = groupB.head;
        while !mr.is_null() {
            fprintf(
                stdout,
                b"%3lu \x00" as *const u8 as *const libc::c_char,
                (*mr).module.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            mr = (*mr).next
        }
        fprintf(stdout, b"\n\x00" as *const u8 as *const libc::c_char);
    }
    /* total edge cuts */
    cuts = 0 as libc::c_int as libc::c_ulong;
    mr = groupA.head;
    while !mr.is_null() {
        if moduleToGroup[(*mr).module as usize] as libc::c_uint
            == GroupA as libc::c_int as libc::c_uint
        {
        } else {
            __assert_fail(
                b"moduleToGroup[(*mr).module] == GroupA\x00" as *const u8 as *const libc::c_char,
                b"KS-2.c\x00" as *const u8 as *const libc::c_char,
                265 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"void PrintResults(int)\x00",
                ))
                .as_ptr(),
            );
        }
        /* for all nets on this module */
        nn = modules[(*mr).module as usize];
        while !nn.is_null() {
            netSz = 0 as libc::c_int;
            mn = nets[(*nn).net as usize];
            while !mn.is_null() {
                netSz += 1;
                mn = (*mn).next
            }
            if netSz >= 2 as libc::c_int {
            } else {
                __assert_fail(
                    b"netSz >= 2\x00" as *const u8 as *const libc::c_char,
                    b"KS-2.c\x00" as *const u8 as *const libc::c_char,
                    273 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"void PrintResults(int)\x00",
                    ))
                    .as_ptr(),
                );
            }
            /* for all modules on this net */
            mn = nets[(*nn).net as usize];
            while !mn.is_null() {
                /* only check nodes other than self, and not swapped */
                if moduleToGroup[(*mr).module as usize] as libc::c_uint
                    != moduleToGroup[(*mn).module as usize] as libc::c_uint
                {
                    if verbose != 0 {
                        fprintf(
                            stdout,
                            b"Conn %3lu - %3lu cut.\n\x00" as *const u8 as *const libc::c_char,
                            (*mr).module.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            (*mn).module.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        );
                    }
                    netStats[netSz as usize].edgesCut =
                        netStats[netSz as usize].edgesCut.wrapping_add(1);
                    cuts = cuts.wrapping_add(1)
                }
                mn = (*mn).next
            }
            nn = (*nn).next
        }
        mr = (*mr).next
    }
    fprintf(
        stdout,
        b"Total edge cuts = %lu\n\x00" as *const u8 as *const libc::c_char,
        cuts,
    );
    /* total net cuts */
    cuts = 0 as libc::c_int as libc::c_ulong;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < numNets {
        netSz = 0 as libc::c_int;
        mn = nets[i as usize];
        while !mn.is_null() {
            netSz += 1;
            mn = (*mn).next
        }
        if netSz >= 2 as libc::c_int {
        } else {
            __assert_fail(
                b"netSz >= 2\x00" as *const u8 as *const libc::c_char,
                b"KS-2.c\x00" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"void PrintResults(int)\x00",
                ))
                .as_ptr(),
            );
        }
        netStats[netSz as usize].total = netStats[netSz as usize].total.wrapping_add(1);
        if netSz as libc::c_long > maxStat {
            maxStat = netSz as libc::c_long
        }
        grp = moduleToGroup[(*nets[i as usize]).module as usize];
        mn = (*nets[i as usize]).next;
        while !mn.is_null() {
            /* only check nodes other than self, and not swapped */
            if grp as libc::c_uint != moduleToGroup[(*mn).module as usize] as libc::c_uint {
                if verbose != 0 {
                    fprintf(
                        stdout,
                        b"Net %3lu cut.\n\x00" as *const u8 as *const libc::c_char,
                        i + 1 as libc::c_int,
                    );
                }
                cuts = cuts.wrapping_add(1);
                netStats[netSz as usize].netsCut = netStats[netSz as usize].netsCut.wrapping_add(1);
                break;
            } else {
                mn = (*mn).next
            }
        }
        i += 1
    }
    fprintf(
        stdout,
        b"Total net cuts  = %lu\n\x00" as *const u8 as *const libc::c_char,
        cuts,
    );
    i = 2 as libc::c_int;
    while i as libc::c_long <= maxStat {
        fprintf(
            stdout,
            b"sz:%5lu     total:%5lu     edgesCut:%5lu     netsCuts:%5lu\n\x00" as *const u8
                as *const libc::c_char,
            i,
            netStats[i as usize].total,
            netStats[i as usize].edgesCut,
            netStats[i as usize].netsCut,
        );
        i += 1
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut p: libc::c_ulong = 0;
    let mut iMax: libc::c_ulong = 0;
    let mut gMax: libc::c_float = 0.;
    let mut lastGMax: libc::c_float = 0.;
    let mut mr: ModuleRecPtr = 0 as *mut ModuleRec;
    /* parse argument */
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: KL <input_file>\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    /* prepare the data structures */
    ReadNetList(*argv.offset(1 as libc::c_int as isize));
    NetsToModules();
    ComputeNetCosts();
    if numModules.wrapping_rem(2 as libc::c_int as libc::c_ulong)
        == 0 as libc::c_int as libc::c_ulong
    {
    } else {
        __assert_fail(
            b"(numModules % 2) == 0\x00" as *const u8 as *const libc::c_char,
            b"KS-2.c\x00" as *const u8 as *const libc::c_char,
            346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    /* initial partition */
    InitLists();
    lastGMax = 0 as libc::c_int as libc::c_float;
    loop
    /* do until we don't make any progress */
    {
        ComputeDs(&mut groupA, GroupA, SwappedToA); /* progress made? */
        ComputeDs(&mut groupB, GroupB, SwappedToB);
        /* compute the swap costs */
        /* !KS_MODE */
        /* for all pairs of nodes in A,B */
        p = 0 as libc::c_int as libc::c_ulong;
        while p < numModules.wrapping_div(2 as libc::c_int as libc::c_ulong) {
            /* KS_MODE */
            /* find the max swap opportunity, and swap */
            GP[p as usize] = FindMaxGpAndSwap();
            p = p.wrapping_add(1)
        }
        /* lists should both be empty now */
        if groupA.head.is_null() && groupA.tail.is_null() {
        } else {
            __assert_fail(
                b"groupA.head == NULL && groupA.tail == NULL\x00" as *const u8
                    as *const libc::c_char,
                b"KS-2.c\x00" as *const u8 as *const libc::c_char,
                375 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"void main(int, char **)\x00",
                ))
                .as_ptr(),
            );
        }
        if groupB.head.is_null() && groupB.tail.is_null() {
        } else {
            __assert_fail(
                b"groupB.head == NULL && groupB.tail == NULL\x00" as *const u8
                    as *const libc::c_char,
                b"KS-2.c\x00" as *const u8 as *const libc::c_char,
                376 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"void main(int, char **)\x00",
                ))
                .as_ptr(),
            );
        }
        gMax = FindGMax(&mut iMax);
        /* debug/statistics */
        if lastGMax == gMax {
            fprintf(
                stdout,
                b"No progress: gMax = %f\n\x00" as *const u8 as *const libc::c_char,
                gMax as libc::c_double,
            );
        }
        lastGMax = gMax;
        fprintf(
            stdout,
            b"gMax = %f, iMax = %lu\n\x00" as *const u8 as *const libc::c_char,
            gMax as libc::c_double,
            iMax,
        );
        if gMax as libc::c_double > 0.0f64 {
            SwapSubsetAndReset(iMax);
        }
        PrintResults(0 as libc::c_int);
        if !(gMax as libc::c_double > 0.0f64) {
            break;
        }
    }
    /* all swaps rejected */
    groupA = swapToB;
    mr = groupA.head;
    while !mr.is_null() {
        moduleToGroup[(*mr).module as usize] = GroupA;
        mr = (*mr).next
    }
    groupB = swapToA;
    mr = groupB.head;
    while !mr.is_null() {
        moduleToGroup[(*mr).module as usize] = GroupB;
        mr = (*mr).next
    }
    /* all done, show results */
    PrintResults(1 as libc::c_int);
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
