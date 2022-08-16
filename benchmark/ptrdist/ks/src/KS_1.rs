use ::libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
/* all modules -> nets */
/* net-ular view */
pub type Module = _Module;
pub type ModulePtr = *mut Module;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ModuleRec {
    pub next: *mut _ModuleRec,
    pub module: libc::c_ulong,
}
/* all nets -> modules */
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
pub const NULL_0: libc::c_int = 0 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        NULL as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
pub const __ASSERT_FUNCTION: [libc::c_char; 46] = unsafe {
    *::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
        b"void ComputeDs(ModuleListPtr, Groups, Groups)\x00",
    )
};
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
pub const BUF_LEN: libc::c_int = 1024 as libc::c_int;
/*
 *	program:	Graph partition via Kernighan-Lin, modified
 *			Kernighan-Lin, or Kernighan-Schweikert
 *
 *	author:		Todd M. Austin
 *			ECE 756
 *
 *	date:		Thursday, February 25, 1993
 */
#[no_mangle]
pub static mut modules: [NetPtr; 1024] = [0 as *const Net as *mut Net; 1024];
/* all modules -> nets */
#[no_mangle]
pub static mut numModules: libc::c_ulong = 0;
#[no_mangle]
pub static mut nets: [ModulePtr; 1024] = [0 as *const Module as *mut Module; 1024];
/* all nets -> modules */
#[no_mangle]
pub static mut numNets: libc::c_ulong = 0;
#[no_mangle]
pub static mut groupA: ModuleList = ModuleList {
    head: 0 as *const ModuleRec as *mut ModuleRec,
    tail: 0 as *const ModuleRec as *mut ModuleRec,
};
#[no_mangle]
pub static mut groupB: ModuleList = ModuleList {
    head: 0 as *const ModuleRec as *mut ModuleRec,
    tail: 0 as *const ModuleRec as *mut ModuleRec,
};
/* current A, B */
#[no_mangle]
pub static mut swapToA: ModuleList = ModuleList {
    head: 0 as *const ModuleRec as *mut ModuleRec,
    tail: 0 as *const ModuleRec as *mut ModuleRec,
};
#[no_mangle]
pub static mut swapToB: ModuleList = ModuleList {
    head: 0 as *const ModuleRec as *mut ModuleRec,
    tail: 0 as *const ModuleRec as *mut ModuleRec,
};
/* swapped from A,B, ordered */
#[no_mangle]
pub static mut GP: [libc::c_float; 1024] = [0.; 1024];
/* GPs, ordered */
#[no_mangle]
pub static mut moduleToGroup: [Groups; 1024] = [GroupA; 1024];
/* current inverse mapping */
#[no_mangle]
pub static mut D: [libc::c_float; 1024] = [0.; 1024];
/* module costs */
#[no_mangle]
pub static mut cost: [libc::c_float; 1024] = [0.; 1024];
/* current inverse mapping */
/* module costs */
/* net costs */
/* net costs */
/* read the netlist into the nets[] structure */
#[no_mangle]
pub unsafe extern "C" fn ReadNetList(mut fname: *mut libc::c_char) {
    let mut inFile: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut net: libc::c_ulong = 0;
    let mut dest: libc::c_ulong = 0;
    let mut node: ModulePtr = 0 as *mut Module;
    let mut prev: ModulePtr = 0 as *mut Module;
    let mut head: ModulePtr = 0 as *mut Module;
    inFile = fopen(fname, b"r\x00" as *const u8 as *const libc::c_char);
    if inFile.is_null() {
        fprintf(
            stderr,
            b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
            b"KS-1.c\x00" as *const u8 as *const libc::c_char,
            b"ReadData\x00" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
        );
        fprintf(
            stderr,
            b"unable to open input file [%s]\x00" as *const u8 as *const libc::c_char,
            inFile,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    fgets(line.as_mut_ptr(), 1024 as libc::c_int, inFile);
    if !(sscanf(
        line.as_mut_ptr(),
        b"%lu %lu\x00" as *const u8 as *const libc::c_char,
        &mut numNets as *mut libc::c_ulong,
        &mut numModules as *mut libc::c_ulong,
    ) == 2 as libc::c_int)
    {
        fprintf(
            stderr,
            b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
            b"KS-1.c\x00" as *const u8 as *const libc::c_char,
            b"ReadData\x00" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
        );
        fprintf(
            stderr,
            b"unable to parse header in file [%s]\x00" as *const u8 as *const libc::c_char,
            inFile,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    net = 0 as libc::c_int as libc::c_ulong;
    while net < numNets {
        fgets(line.as_mut_ptr(), BUF_LEN, inFile);
        /* net connections for "dest" */
        dest = (atol(strtok(
            line.as_mut_ptr(),
            b" \t\n\x00" as *const u8 as *const libc::c_char,
        )) - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
        /* parse out all the net module connections */
        prev = malloc(::std::mem::size_of::<Module>() as libc::c_ulong) as *mut Module;
        head = prev;
        if prev.is_null() {
            fprintf(
                stderr,
                b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
                b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                b"ReadData\x00" as *const u8 as *const libc::c_char,
                63 as libc::c_int,
            );
            fprintf(
                stderr,
                b"unable to allocate a module list node\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        (*prev).module = (atol(strtok(
            NULL_0 as *mut libc::c_char,
            b" \t\n\x00" as *const u8 as *const libc::c_char,
        )) - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
        (*prev).next = NULL_0 as *mut _Module;
        loop {
            tok = strtok(
                NULL_0 as *mut libc::c_char,
                b" \t\n\x00" as *const u8 as *const libc::c_char,
            );
            if tok.is_null() {
                break;
            }
            node = malloc(::std::mem::size_of::<Module>() as libc::c_ulong) as *mut Module;
            if node.is_null() {
                fprintf(
                    stderr,
                    b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
                    b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                    b"ReadData\x00" as *const u8 as *const libc::c_char,
                    70 as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"unable to allocate a module list node\x00" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            (*node).module = (atol(tok) - 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            (*node).next = NULL_0 as *mut _Module;
            (*prev).next = node;
            prev = node
        }
        nets[dest as usize] = head;
        net = net.wrapping_add(1)
    }
}
/* invert the previously read nets, into a module to net structure */
#[no_mangle]
pub unsafe extern "C" fn NetsToModules() {
    let mut net: libc::c_ulong = 0;
    let mut mod_0: libc::c_ulong = 0;
    let mut modNode: ModulePtr = 0 as *mut Module;
    let mut netNode: NetPtr = 0 as *mut Net;
    mod_0 = 0 as libc::c_int as libc::c_ulong;
    while mod_0 < numModules {
        modules[mod_0 as usize] = NULL_0 as NetPtr;
        mod_0 = mod_0.wrapping_add(1)
    }
    net = 0 as libc::c_int as libc::c_ulong;
    while net < numNets {
        modNode = nets[net as usize];
        while !modNode.is_null() {
            netNode = malloc(::std::mem::size_of::<Net>() as libc::c_ulong) as *mut Net;
            if netNode.is_null() {
                fprintf(
                    stderr,
                    b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
                    b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                    b"NetsToModules\x00" as *const u8 as *const libc::c_char,
                    96 as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"unable to allocate net list node\x00" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            (*netNode).net = net;
            (*netNode).next = modules[(*modNode).module as usize];
            modules[(*modNode).module as usize] = netNode;
            modNode = (*modNode).next
        }
        net = net.wrapping_add(1)
    }
}
/* compute the net edge costs, based on the weighting strategy */
#[no_mangle]
pub unsafe extern "C" fn ComputeNetCosts() {
    /* WEIGHTED */
    let mut i: libc::c_ulong = 0;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < numNets {
        cost[i as usize] = 1.0f64 as libc::c_float;
        i = i.wrapping_add(1)
        /* WEIGHTED */
    }
}
/* set up the initial groups, just split down the middle */
#[no_mangle]
pub unsafe extern "C" fn InitLists() {
    let mut p: libc::c_ulong = 0;
    let mut mr: ModuleRecPtr = 0 as *mut ModuleRec;
    groupA.tail = NULL_0 as ModuleRecPtr;
    groupA.head = groupA.tail;
    groupB.tail = NULL_0 as ModuleRecPtr;
    groupB.head = groupB.tail;
    /* for all modules */
    p = 0 as libc::c_int as libc::c_ulong;
    while p < numModules.wrapping_div(2 as libc::c_int as libc::c_ulong) {
        /* build the group A module list */
        mr = malloc(::std::mem::size_of::<ModuleRec>() as libc::c_ulong) as *mut ModuleRec;
        if mr.is_null() {
            fprintf(
                stderr,
                b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
                b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                b"main\x00" as *const u8 as *const libc::c_char,
                145 as libc::c_int,
            );
            fprintf(
                stderr,
                b"unable to allocate ModuleRec\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        (*mr).module = p;
        if groupA.head.is_null() {
            /* first item */
            groupA.tail = mr;
            groupA.head = groupA.tail;
            (*mr).next = NULL_0 as *mut _ModuleRec
        } else {
            /* add to tail */
            (*mr).next = NULL_0 as *mut _ModuleRec;
            (*groupA.tail).next = mr;
            groupA.tail = mr
        }
        moduleToGroup[p as usize] = GroupA;
        /* build the group B module list */
        mr = malloc(::std::mem::size_of::<ModuleRec>() as libc::c_ulong) as *mut ModuleRec;
        if mr.is_null() {
            fprintf(
                stderr,
                b"(%s:%s():%d): \x00" as *const u8 as *const libc::c_char,
                b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                b"main\x00" as *const u8 as *const libc::c_char,
                164 as libc::c_int,
            );
            fprintf(
                stderr,
                b"unable to allocate ModuleRec\x00" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            fprintf(stderr, b"\n\x00" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        (*mr).module = numModules
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(p);
        if groupB.head.is_null() {
            /* first item */
            groupB.tail = mr;
            groupB.head = groupB.tail;
            (*mr).next = NULL_0 as *mut _ModuleRec
        } else {
            /* add to tail */
            (*mr).next = NULL_0 as *mut _ModuleRec;
            (*groupB.tail).next = mr;
            groupB.tail = mr
        }
        moduleToGroup[numModules
            .wrapping_div(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(p) as usize] = GroupB;
        p = p.wrapping_add(1)
    }
    /* initially clear the swap chains */
    swapToA.tail = NULL_0 as ModuleRecPtr;
    swapToA.head = swapToA.tail;
    swapToB.tail = NULL_0 as ModuleRecPtr;
    swapToB.head = swapToB.tail;
}
/* compute the cost of switching every node in group to the other group */
#[no_mangle]
pub unsafe extern "C" fn ComputeDs(
    mut group: ModuleListPtr,
    mut myGroup: Groups,
    mut mySwap: Groups,
) {
    /* !KS_MODE */
    let mut I: libc::c_float = 0.;
    let mut E: libc::c_float = 0.;
    let mut netNode: NetPtr = 0 as *mut Net;
    let mut modNode: ModulePtr = 0 as *mut Module;
    let mut groupNode: ModuleRecPtr = 0 as *mut ModuleRec;
    /* for all modules in group */
    groupNode = (*group).head;
    while !groupNode.is_null() {
        if moduleToGroup[(*groupNode).module as usize] as libc::c_uint == myGroup as libc::c_uint {
        } else {
            __assert_fail(
                b"moduleToGroup[(*groupNode).module] == myGroup\x00" as *const u8
                    as *const libc::c_char,
                b"KS-1.c\x00" as *const u8 as *const libc::c_char,
                246 as libc::c_int as libc::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
        /* initial conditions */
        E = 0.0f64 as libc::c_float;
        I = E;
        /* for all nets on this module */
        netNode = modules[(*groupNode).module as usize];
        while !netNode.is_null() {
            /* for all modules on this net */
            modNode = nets[(*netNode).net as usize];
            while !modNode.is_null() {
                /* only check nodes other than self, and not swapped */
                if (*modNode).module != (*groupNode).module
                    && (moduleToGroup[(*modNode).module as usize] as libc::c_uint)
                        < SwappedToA as libc::c_int as libc::c_uint
                {
                    if moduleToGroup[(*modNode).module as usize] as libc::c_uint
                        == myGroup as libc::c_uint
                    {
                        I = I + cost[(*netNode).net as usize]
                    } else {
                        E = E + cost[(*netNode).net as usize]
                    } /* internal */
                    /* external */
                }
                modNode = (*modNode).next
            }
            netNode = (*netNode).next
        }
        D[(*groupNode).module as usize] = E - I;
        groupNode = (*groupNode).next
    }
    /* !KS_MODE */
}
