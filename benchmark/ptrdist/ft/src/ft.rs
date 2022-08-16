use ::libc;
extern "C" {
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn srandom(__seed: libc::c_uint);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    /*
     * Initialize the package.  A single call is sufficient even if multiple
     * heaps are alive in the course of a computation.
     *
     * Special external object accessed:
     *   none
     *
     * Arguments:
     *   none
     *
     * Return values:
     *   none
     */
    #[no_mangle]
    fn InitFHeap();
    /*
     * Create a heap structure.
     *
     * Special external object accessed:
     *   none
     *
     * Arguments:
     *   none
     *
     * Return values:
     *   a heap, to be precise an empty, i.e. NULL_HEAP
     */
    #[no_mangle]
    fn MakeHeap() -> *mut HeapP;
    /*
     * Find the item with lowest key.
     *
     * Special external object accessed:
     *   none
     *
     * Arguments:
     *   INPUT:	h	the structure to access, possibly NULL_HEAP
     *
     * Return values:
     *   an item if the heap is not empty
     *   NULL_ITEM otherwise
     */
    #[no_mangle]
    fn FindMin(h: *mut HeapP) -> *mut Item;
    /*
     * Insert an item in a heap.
     *
     * Special external object accessed:
     *   none
     *
     * Arguments:
     *   IN/OUT:	h	the structure to access, possibly NULL_HEAP
     *   INPUT:	i	the item to insert, must be different than NULL_ITEM
     *
     * Return values:
     *   a handle to the inserted item, useful in connection with Delete()
     *   and DecreaseKey().
     */
    #[no_mangle]
    fn Insert(h: *mut *mut HeapP, i: *mut Item) -> *mut HeapP;
    /*
     * Remove the smallest item in a heap
     *
     * Special external object accessed:
     *   none
     *
     * Arguments:
     *   INPUT:	h	the structure to access, possibly NULL_HEAP
     *
     * Return values:
     *   a smaller heap, possibly NULL_HEAP
     */
    #[no_mangle]
    fn DeleteMin(h: *mut HeapP) -> *mut HeapP;
    #[no_mangle]
    fn GenGraph(nVertex: libc::c_int, nEdge: libc::c_int) -> *mut Vertices;
    #[no_mangle]
    fn PrintGraph(graph: *mut Vertices);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __rusage_who = libc::c_int;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Vertices {
    pub id: libc::c_int,
    pub edges: *mut Edges,
    pub next: *mut _Vertices,
    pub key: libc::c_int,
    pub chosenEdge: *mut Edges,
}
pub type Edges = _Edges;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Edges {
    pub weight: libc::c_int,
    pub source: *mut _Vertices,
    pub vertex: *mut _Vertices,
    pub next: *mut _Edges,
}
pub type Vertices = _Vertices;
/*
 * item.h
 *
 * The author of this software is Alain K\"{a}gi.
 *
 * Copyright (c) 1993 by Alain K\"{a}gi and the University of Wisconsin
 * Board of Trustees.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose without fee is hereby granted, provided that this entire notice
 * is included in all copies of any software which is or includes a copy
 * or modification of this software and in all copies of the supporting
 * documentation for such software.
 *
 * THIS SOFTWARE IS BEING PROVIDED "AS IS", WITHOUT ANY EXPRESS OR IMPLIED
 * WARRANTY.  IN PARTICULAR, NEITHER THE AUTHOR NOR THE UNIVERSITY OF
 * WISCONSIN MAKE ANY REPRESENTATION OR WARRANTY OF ANY KIND CONCERNING
 * THE MERCHANTABILITY OF THIS SOFTWARE OR ITS FITNESS FOR ANY PARTICULAR
 * PURPOSE.
 *
 * ------------------------------------------------------------------------
 *
 * Interface between graph and Fheap.
 *
 * ------------------------------------------------------------------------
 *
 * $Id: item.h,v 1.2 1993/03/15 04:25:10 alain Exp alain $
 *
 */
pub type Item = Vertices;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Heap {
    pub item: *mut Item,
    pub parent: *mut _Heap,
    pub child: *mut _Heap,
    pub forward: *mut _Heap,
    pub backward: *mut _Heap,
    pub rank: libc::c_int,
    pub marked: libc::c_short,
}
/*
 * Fheap.h
 *
 * The author of this software is Alain K\"{a}gi.
 *
 * Copyright (c) 1993 by Alain K\"{a}gi and the University of Wisconsin
 * Board of Trustees.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose without fee is hereby granted, provided that this entire notice
 * is included in all copies of any software which is or includes a copy
 * or modification of this software and in all copies of the supporting
 * documentation for such software.
 *
 * THIS SOFTWARE IS BEING PROVIDED "AS IS", WITHOUT ANY EXPRESS OR IMPLIED
 * WARRANTY.  IN PARTICULAR, NEITHER THE AUTHOR NOR THE UNIVERSITY OF
 * WISCONSIN MAKE ANY REPRESENTATION OR WARRANTY OF ANY KIND CONCERNING
 * THE MERCHANTABILITY OF THIS SOFTWARE OR ITS FITNESS FOR ANY PARTICULAR
 * PURPOSE.
 *
 * ------------------------------------------------------------------------
 *
 * This is an implementation of an algorithm described in the paper:
 *
 *   , by Michael L. Fredman and Robert Endre Tarjan, in
 *   Journal of Association for Computing Machinery, Vol. 34, No. 3,
 *   July 1987, Pages 596-615.
 *
 * The algorithm is theirs.  Any discrepancy between the algorithm
 * description which appears in the paper and this implementation is
 * a consequence of my misunderstanding of their intent.
 *
 * ------------------------------------------------------------------------
 *
 * $Id: Fheap.h,v 1.3 1993/03/01 17:12:04 alain Exp alain $
 *
 */
/*
 * The following "item.h" must define
 *   o a structure "Item",
 *   o the definition of NULL_ITEM,
 *   o the definition of "int LessThan(Item *, Item *);",
 *   o the definition of "int Equal(Item *, Item *);",
 *   o the definition of "Item *Subtract(Item *, void *);"
 *     this function must returm an item with its key reduced by a positive
 *     value passed as void *.
 */
pub type HeapP = _Heap;
pub const __INT_MAX__: libc::c_int = 2147483647 as libc::c_int;
pub const INT_MAX: libc::c_int = __INT_MAX__;
pub const INT_MIN: libc::c_int = -__INT_MAX__ - 1 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        NULL as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub const RUSAGE_SELF_0: libc::c_int = RUSAGE_SELF as libc::c_int;
pub const NULL_VERTEX: libc::c_int = 0 as libc::c_int;
pub const NULL_EDGE: libc::c_int = 0 as libc::c_int;
/*
 * ft.c
 *
 * The author of this software is Alain K\"{a}gi.
 *
 * Copyright (c) 1993 by Alain K\"{a}gi and the University of Wisconsin
 * Board of Trustees.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose without fee is hereby granted, provided that this entire notice
 * is included in all copies of any software which is or includes a copy
 * or modification of this software and in all copies of the supporting
 * documentation for such software.
 *
 * THIS SOFTWARE IS BEING PROVIDED "AS IS", WITHOUT ANY EXPRESS OR IMPLIED
 * WARRANTY.  IN PARTICULAR, NEITHER THE AUTHOR NOR THE UNIVERSITY OF
 * WISCONSIN MAKE ANY REPRESENTATION OR WARRANTY OF ANY KIND CONCERNING
 * THE MERCHANTABILITY OF THIS SOFTWARE OR ITS FITNESS FOR ANY PARTICULAR
 * PURPOSE.
 *
 * ------------------------------------------------------------------------
 *
 * This is the first algorithm presented in
 *
 *   , by Michael L. Fredman and Robert Endre Tarjan, in
 *   Journal of Association for Computing Machinery, Vol. 34, No. 3,
 *   July 1987, Pages 596-615,
 *
 * for find the minimum spanning tree.
 *
 * ------------------------------------------------------------------------
 *
 * $Id: ft.c,v 1.3 1993/03/15 04:30:47 alain Exp alain $
 *
 */
pub const MINUS_INFINITY: libc::c_int = INT_MIN;
pub const PLUS_INFINITY: libc::c_int = INT_MAX;
pub const DEFAULT_N_VERTEX: libc::c_int = 10 as libc::c_int;
pub const DEFAULT_N_EDGE: libc::c_int = 9 as libc::c_int;
/*
 * Local variables.
 */
#[no_mangle]
pub static mut debug: libc::c_int = 1 as libc::c_int;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) {
    let mut nVertex: libc::c_int = 0;
    let mut nEdge: libc::c_int = 0;
    let mut graph: *mut Vertices = 0 as *mut Vertices;
    let mut rUBuf1: *mut rusage = 0 as *mut rusage;
    let mut rUBuf2: *mut rusage = 0 as *mut rusage;
    nVertex = DEFAULT_N_VERTEX;
    nEdge = DEFAULT_N_EDGE;
    if argc > 1 as libc::c_int {
        nVertex = atoi(*argv.offset(1 as libc::c_int as isize));
        if argc > 2 as libc::c_int {
            nEdge = atoi(*argv.offset(2 as libc::c_int as isize));
            if argc > 3 as libc::c_int {
                srandom(atoi(*argv.offset(3 as libc::c_int as isize)) as libc::c_uint);
            }
        }
    }
    rUBuf1 = malloc(::std::mem::size_of::<rusage>() as libc::c_ulong) as *mut rusage;
    if !rUBuf1.is_null() {
    } else {
        __assert_fail(
            b"rUBuf1 != NULL\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    rUBuf2 = malloc(::std::mem::size_of::<rusage>() as libc::c_ulong) as *mut rusage;
    if !rUBuf1.is_null() {
    } else {
        __assert_fail(
            b"rUBuf1 != NULL\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"void main(int, char **)\x00",
            ))
            .as_ptr(),
        );
    }
    if debug != 0 {
        printf(b"Generating a connected graph ... \x00" as *const u8 as *const libc::c_char);
    }
    graph = GenGraph(nVertex, nEdge);
    if debug != 0 {
        printf(
            b"done\nFinding the mininmum spanning tree ... \x00" as *const u8
                as *const libc::c_char,
        );
    }
    getrusage(RUSAGE_SELF_0, rUBuf1);
    graph = MST(graph);
    getrusage(RUSAGE_SELF_0, rUBuf2);
    if debug != 0 {
        printf(b"done\nThe graph:\n\x00" as *const u8 as *const libc::c_char);
        PrintGraph(graph);
        printf(b"The minimum spanning tree:\n\x00" as *const u8 as *const libc::c_char);
        PrintMST(graph);
    }
    if debug != 0 {
        printf(
            b"Time spent in finding the mininum spanning tree:\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    PrintRUsage(rUBuf1, rUBuf2);
    /* PLUS_STATS */
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MST(mut graph: *mut Vertices) -> *mut Vertices {
    let mut heap: *mut HeapP = 0 as *mut HeapP;
    let mut vertex: *mut Vertices = 0 as *mut Vertices;
    let mut edge: *mut Edges = 0 as *mut Edges;
    InitFHeap();
    /*
     * key(s) = 0;
     * key(v) = infty for v != s;
     * init heap;
     * make a heap;
     * put s in heap;
     */
    vertex = graph;
    (*vertex).key = 0 as libc::c_int;
    heap = MakeHeap();
    Insert(&mut heap, vertex as *mut Item);
    vertex = (*vertex).next;
    while vertex != graph {
        (*vertex).key = PLUS_INFINITY;
        vertex = (*vertex).next
    }
    while vertex != graph {}
    vertex = FindMin(heap);
    while !vertex.is_null() {
        heap = DeleteMin(heap);
        (*vertex).key = MINUS_INFINITY;
        edge = (*vertex).edges;
        while !edge.is_null() {
            if (*edge).weight < (*(*edge).vertex).key {
                (*(*edge).vertex).key = (*edge).weight;
                (*(*edge).vertex).chosenEdge = edge;
                Insert(&mut heap, (*edge).vertex);
            }
            edge = (*edge).next
        }
        vertex = FindMin(heap)
    }
    return graph;
}
/*
 * Local functions.
 */
#[no_mangle]
pub unsafe extern "C" fn PrintMST(mut graph: *mut Vertices) {
    let mut vertex: *mut Vertices = 0 as *mut Vertices;
    if !graph.is_null() {
    } else {
        __assert_fail(
            b"graph != NULL_VERTEX\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void PrintMST(Vertices *)\x00",
            ))
            .as_ptr(),
        );
    }
    vertex = (*graph).next;
    while vertex != graph {
        printf(
            b"vertex %d to %d\n\x00" as *const u8 as *const libc::c_char,
            (*vertex).id,
            (*(*(*vertex).chosenEdge).source).id,
        );
        vertex = (*vertex).next
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrintRUsage(mut rUBuf1: *mut rusage, mut rUBuf2: *mut rusage) {
    let mut sec: libc::c_long = 0;
    let mut usec: libc::c_long = 0;
    sec = (*rUBuf2).ru_utime.tv_sec - (*rUBuf1).ru_utime.tv_sec;
    usec = (*rUBuf2).ru_utime.tv_usec - (*rUBuf1).ru_utime.tv_usec;
    if usec < 0 as libc::c_int as libc::c_long {
        sec -= 1;
        usec += 1000000 as libc::c_int as libc::c_long
    }
    if sec >= 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(
            b"sec >= 0\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void PrintRUsage(struct rusage *, struct rusage *)\x00",
            ))
            .as_ptr(),
        );
    }
    if usec >= 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(
            b"usec >= 0\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void PrintRUsage(struct rusage *, struct rusage *)\x00",
            ))
            .as_ptr(),
        );
    }
    fprintf(
        stderr,
        b"User time: %d%06d usec\n\x00" as *const u8 as *const libc::c_char,
        sec,
        usec,
    );
    sec = (*rUBuf2).ru_stime.tv_sec - (*rUBuf1).ru_stime.tv_sec;
    usec = (*rUBuf2).ru_stime.tv_usec - (*rUBuf1).ru_stime.tv_usec;
    if usec < 0 as libc::c_int as libc::c_long {
        sec -= 1;
        usec += 1000000 as libc::c_int as libc::c_long
    }
    if sec >= 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(
            b"sec >= 0\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            229 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void PrintRUsage(struct rusage *, struct rusage *)\x00",
            ))
            .as_ptr(),
        );
    }
    if usec >= 0 as libc::c_int as libc::c_long {
    } else {
        __assert_fail(
            b"usec >= 0\x00" as *const u8 as *const libc::c_char,
            b"ft.c\x00" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void PrintRUsage(struct rusage *, struct rusage *)\x00",
            ))
            .as_ptr(),
        );
    }
    fprintf(
        stderr,
        b"Sys time:  %d%06d usec\n\x00" as *const u8 as *const libc::c_char,
        sec,
        usec,
    );
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
