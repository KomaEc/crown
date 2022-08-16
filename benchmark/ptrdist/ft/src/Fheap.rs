use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
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
    #[no_mangle]
    fn Equal(_: *mut Item, _: *mut Item) -> libc::c_int;
    #[no_mangle]
    fn Subtract(_: *mut Item, _: libc::c_int) -> *mut Item;
    #[no_mangle]
    fn LessThan(_: *mut Item, _: *mut Item) -> libc::c_int;
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
/*
 * graph.h
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
 * The graph etc.
 *
 * ------------------------------------------------------------------------
 *
 * $Id: graph.h,v 1.2 1993/03/15 04:25:10 alain Exp alain $
 *
 */
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
 * For the ft algorithm.
 */
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const NULL_HEAP: libc::c_int = 0 as libc::c_int;
pub const FALSE: libc::c_int = 0 as libc::c_int;
pub const MAX_RANK: libc::c_int = 10000 as libc::c_int;
/*
 * Fheap.c
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
 * $Id: Fheap.c,v 1.4 1993/03/01 17:12:04 alain Exp alain $
 *
 */
static mut hTable: [*mut HeapP; 10000] = [0 as *const HeapP as *mut HeapP; 10000];
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
pub unsafe extern "C" fn InitFHeap() {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < MAX_RANK {
        hTable[j as usize] = NULL as *mut HeapP;
        j += 1
    }
}
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
pub unsafe extern "C" fn MakeHeap() -> *mut HeapP {
    return 0 as *mut HeapP;
}
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
pub unsafe extern "C" fn FindMin(mut h: *mut HeapP) -> *mut Item {
    if h.is_null() {
        return 0 as *mut Item;
    } else {
        return (*h).item;
    };
}
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
pub unsafe extern "C" fn Insert(mut h: *mut *mut HeapP, mut i: *mut Item) -> *mut HeapP {
    let mut h1: *mut HeapP = 0 as *mut HeapP;
    h1 = NewHeap(i);
    *h = Meld(*h, h1);
    return h1;
}
/*
 * Meld to heaps.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h1, h2	the heaps to meld, possibly NULL_HEAP
 *
 * Return values:
 *   a bigger heap, possibly NULL_HEAP
 */
#[no_mangle]
pub unsafe extern "C" fn Meld(mut h1: *mut HeapP, mut h2: *mut HeapP) -> *mut HeapP {
    if h2.is_null() {
        return h1;
    } /* TBD note that update to PARENT is not necessary!! */
    if h1.is_null() {
        return h2;
    }
    CombineLists(h1, h2);
    if LessThan((*h1).item, (*h2).item) != 0 {
        return h1;
    } else {
        return h2;
    };
}
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
/*
 * This function needs some aesthetic changes.
 */
#[no_mangle]
pub unsafe extern "C" fn DeleteMin(mut h: *mut HeapP) -> *mut HeapP {
    let mut r: libc::c_int = 0;
    let mut rMax: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut h1: *mut HeapP = 0 as *mut HeapP;
    let mut h2: *mut HeapP = 0 as *mut HeapP;
    let mut h3: *mut HeapP = 0 as *mut HeapP;
    let mut min: *mut HeapP = 0 as *mut HeapP;
    rMax = 0 as libc::c_int;
    if h.is_null() {
        return 0 as *mut HeapP;
    }
    h1 = RemoveEntry(h);
    if h1.is_null() {
        free(h as *mut libc::c_void);
        return 0 as *mut HeapP;
    }
    /*
     * hack.
     */
    if h1 == (*h).child {
        (*h).child = NULL as *mut _Heap
    }
    /*
     * Put the tree entries in the table.
     */
    h2 = h1; /* have to do this, b/c of above hack. */
    loop {
        h3 = (*h2).forward;
        (*h2).forward = h2;
        (*h2).backward = h2;
        (*h2).parent = NULL as *mut _Heap;
        r = (*h2).rank;
        if r < 10000 as libc::c_int {
        } else {
            __assert_fail(
                b"r < MAX_RANK\x00" as *const u8 as *const libc::c_char,
                b"Fheap.c\x00" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"HeapP *DeleteMin(HeapP *)\x00",
                ))
                .as_ptr(),
            );
        }
        while !hTable[r as usize].is_null() {
            if LessThan((*hTable[r as usize]).item, (*h2).item) != 0 {
                AddEntry(hTable[r as usize], h2);
                h2 = hTable[r as usize]
            } else {
                AddEntry(h2, hTable[r as usize]);
            }
            hTable[r as usize] = NULL as *mut HeapP;
            r = (*h2).rank;
            if r < 10000 as libc::c_int {
            } else {
                __assert_fail(
                    b"r < MAX_RANK\x00" as *const u8 as *const libc::c_char,
                    b"Fheap.c\x00" as *const u8 as *const libc::c_char,
                    184 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"HeapP *DeleteMin(HeapP *)\x00",
                    ))
                    .as_ptr(),
                );
            }
        }
        hTable[r as usize] = h2;
        if r > rMax {
            rMax = r
        }
        h2 = h3;
        if !(h2 != h1) {
            break;
        }
    }
    /*
     * Put the children of h in the table.
     */
    if !(*h).child.is_null() {
        h2 = (*h).child;
        loop {
            h3 = (*h2).forward;
            (*h2).forward = h2;
            (*h2).backward = h2;
            (*h2).parent = NULL as *mut _Heap;
            r = (*h2).rank;
            if r < 10000 as libc::c_int {
            } else {
                __assert_fail(
                    b"r < MAX_RANK\x00" as *const u8 as *const libc::c_char,
                    b"Fheap.c\x00" as *const u8 as *const libc::c_char,
                    211 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                        b"HeapP *DeleteMin(HeapP *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            while !hTable[r as usize].is_null() {
                if LessThan((*hTable[r as usize]).item, (*h2).item) != 0 {
                    AddEntry(hTable[r as usize], h2);
                    h2 = hTable[r as usize]
                } else {
                    AddEntry(h2, hTable[r as usize]);
                }
                hTable[r as usize] = NULL as *mut HeapP;
                r = (*h2).rank;
                if r < 10000 as libc::c_int {
                } else {
                    __assert_fail(
                        b"r < MAX_RANK\x00" as *const u8 as *const libc::c_char,
                        b"Fheap.c\x00" as *const u8 as *const libc::c_char,
                        225 as libc::c_int as libc::c_uint,
                        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                            b"HeapP *DeleteMin(HeapP *)\x00",
                        ))
                        .as_ptr(),
                    );
                }
            }
            hTable[r as usize] = h2;
            if r > rMax {
                rMax = r
            }
            h2 = h3;
            if !(h2 != (*h).child) {
                break;
            }
        }
    }
    /*
     * Empty table, find min.
     * Inefficient as is.
     */
    j = 0 as libc::c_int; /* TBD note that update to PARENT not necessary!! */
    while j <= rMax {
        if !hTable[j as usize].is_null() {
            break;
        }
        j += 1
    }
    h1 = hTable[j as usize];
    min = h1;
    hTable[j as usize] = NULL as *mut HeapP;
    j += 1;
    while j <= rMax {
        if !hTable[j as usize].is_null() {
            CombineLists(h1, hTable[j as usize]);
            if LessThan((*hTable[j as usize]).item, (*min).item) != 0 {
                min = hTable[j as usize]
            }
            hTable[j as usize] = NULL as *mut HeapP
        }
        j += 1
    }
    free(h as *mut libc::c_void);
    return min;
}
/*
 * Decrease the key of an item in a heap.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h	the structure to access, must be different than
 *			NULL_HEAP
 *   INPUT:	i	the item to change, must different than NULL_HEAP
 *   INPUT:	delta	a "positive" value which will be subtracted from
 *                      the key in i.
 *
 * Return values:
 *   a heap, possibly NULL_HEAP
 */
#[no_mangle]
pub unsafe extern "C" fn DecreaseKey(
    mut h: *mut HeapP,
    mut i: *mut HeapP,
    mut delta: libc::c_int,
) -> *mut HeapP {
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"HeapP *DecreaseKey(HeapP *, HeapP *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !i.is_null() {
    } else {
        __assert_fail(
            b"i != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            275 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 42], &[libc::c_char; 42]>(
                b"HeapP *DecreaseKey(HeapP *, HeapP *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if !(*i).parent.is_null() {
        RemoveChild(i);
        CombineLists(h, i);
        /* TBD note that update to PARENT not necessary!! */
    }
    (*i).item = Subtract((*i).item, delta);
    if LessThan((*i).item, (*h).item) != 0 {
        return i;
    } else {
        return h;
    };
}
/*
 * Note: i must have a parent (;-).
 */
#[no_mangle]
pub unsafe extern "C" fn RemoveChild(mut i: *mut HeapP) {
    let mut parent: *mut HeapP = 0 as *mut HeapP; /* works in all cases! */
    if !i.is_null() {
    } else {
        __assert_fail(
            b"i != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void RemoveChild(HeapP *)\x00",
            ))
            .as_ptr(),
        ); /* TBD note that update to PARENT not necessary!! */
    }
    parent = (*i).parent;
    if !parent.is_null() {
    } else {
        __assert_fail(
            b"parent != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            305 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"void RemoveChild(HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*parent).child == i {
        if i == (*i).forward {
            (*parent).child = NULL as *mut _Heap
        } else {
            (*parent).child = (*i).forward
        }
    }
    RemoveEntry(i);
    FixRank(parent, (*i).rank + 1 as libc::c_int);
    (*i).forward = i;
    (*i).backward = i;
    (*i).parent = NULL as *mut _Heap;
}
#[no_mangle]
pub unsafe extern "C" fn FixRank(mut h: *mut HeapP, mut delta: libc::c_int) {
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void FixRank(HeapP *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if delta > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"delta > 0\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            330 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"void FixRank(HeapP *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    loop {
        (*h).rank = (*h).rank - delta;
        h = (*h).parent;
        if h.is_null() {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Delete(mut h: *mut HeapP, mut i: *mut HeapP) -> *mut HeapP {
    let mut h1: *mut HeapP = 0 as *mut HeapP;
    let mut h2: *mut HeapP = 0 as *mut HeapP;
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            346 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"HeapP *Delete(HeapP *, HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    if !i.is_null() {
    } else {
        __assert_fail(
            b"i != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            347 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"HeapP *Delete(HeapP *, HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    if h == i {
        return DeleteMin(h);
    }
    if (*i).parent.is_null() {
        RemoveEntry(i);
    } else {
        RemoveChild(i);
    }
    h1 = (*i).child;
    if !h1.is_null() {
        loop {
            h2 = (*h1).forward;
            (*h1).forward = h1;
            (*h1).backward = h1;
            (*h1).parent = NULL as *mut _Heap;
            CombineLists(h, h1);
            /*
             * Fix minimum.
             */
            if LessThan((*h1).item, (*h).item) != 0 {
                h = h1
            }
            h1 = h2;
            if !(h1 != (*i).child) {
                break;
            }
        }
    }
    free(i as *mut libc::c_void);
    return h;
}
/*
 * Combine two doubly linked lists.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h1, h2	the structure to access, must be different than
 *			NULL
 *
 * Return values:
 *   none
 */
#[no_mangle]
pub unsafe extern "C" fn CombineLists(mut h1: *mut HeapP, mut h2: *mut HeapP) {
    let mut h: *mut HeapP = 0 as *mut HeapP;
    if !h1.is_null() && !h2.is_null() {
    } else {
        __assert_fail(
            b"(h1 != NULL) && (h2 != NULL)\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            410 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"void CombineLists(HeapP *, HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    h = h1;
    (*(*h1).forward).backward = h2;
    (*(*h2).forward).backward = h1;
    h = (*h1).forward;
    (*h1).forward = (*h2).forward;
    (*h2).forward = h;
}
/*
 * Add an entry as a child of the root of a heap.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h1, h2	the structure to access, must be different than
 *			NULL
 *
 * Return values:
 *   h1 with h2 as new child of h1.
 */
#[no_mangle]
pub unsafe extern "C" fn AddEntry(mut h1: *mut HeapP, mut h2: *mut HeapP) {
    if !h1.is_null() && !h2.is_null() {
    } else {
        __assert_fail(
            b"(h1 != NULL) && (h2 != NULL)\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            437 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void AddEntry(HeapP *, HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*h1).child.is_null() {
        (*h1).child = h2
    } else {
        CombineLists((*h1).child, h2);
    }
    (*h2).parent = h1;
    (*h1).rank = (*h1).rank + (*h2).rank + 1 as libc::c_int;
}
/*
 * Remove an entry from a heap.  Note that PARENT(h) is not updated.
 * TBD: should this be an invariant?
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h	the structure to access, must be different than
 *			NULL
 *
 * Return values:
 *   a smaller heap, possibly NULL
 */
#[no_mangle]
pub unsafe extern "C" fn RemoveEntry(mut h: *mut HeapP) -> *mut HeapP {
    if !h.is_null() {
    } else {
        __assert_fail(
            b"h != NULL\x00" as *const u8 as *const libc::c_char,
            b"Fheap.c\x00" as *const u8 as *const libc::c_char,
            468 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"HeapP *RemoveEntry(HeapP *)\x00",
            ))
            .as_ptr(),
        );
    }
    if h == (*h).forward {
        return (*h).child;
    }
    (*(*h).forward).backward = (*h).backward;
    (*(*h).backward).forward = (*h).forward;
    return (*h).forward;
}
/*
 * Create a single unmarked entry heap, with parent and child pointers zeroed,
 * forward and backward pointing to self.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	i	item to insert in h, must be different than NULL
 *
 * Return values:
 *   a single entry heap
 */
#[no_mangle]
pub unsafe extern "C" fn NewHeap(mut i: *mut Item) -> *mut HeapP {
    let mut h: *mut HeapP = 0 as *mut HeapP;
    h = malloc(::std::mem::size_of::<HeapP>() as libc::c_ulong) as *mut HeapP;
    if h.is_null() {
        fprintf(
            stderr,
            b"Oops, could not malloc\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*h).item = i;
    (*h).parent = NULL as *mut _Heap;
    (*h).child = NULL as *mut _Heap;
    (*h).forward = h;
    (*h).backward = h;
    (*h).rank = 0 as libc::c_int;
    (*h).marked = FALSE as libc::c_short;
    return h;
}
/*
 * Converts a item handle into an item pointer.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h	the item handle, must be different than NULL_HEAP
 *
 * Return values:
 *   an pointer to the item
 */
#[no_mangle]
pub unsafe extern "C" fn ItemOf(mut h: *mut HeapP) -> *mut Item {
    return (*h).item;
}
/*
 * Delete an entry in a heap.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h	the structure to access, must be different than
 *			NULL_HEAP
 *   INPUT:	i	the item to delete, must be different than NULL_HEAP
 *
 * Return values:
 *   a smaller heap, possibly NULL_HEAP
 */
/*
 * Search for an item with a particular key in a heap.
 * Beware the fibonacci heaps are not efficient at searching.
 *
 * Special external object accessed:
 *   none
 *
 * Arguments:
 *   INPUT:	h	the structure to access, possibly NULL_HEAP
 *   INPUT:	item	the item to search for in h (only the key in item is
 *                      accessed through Equal() and LessThan())
 *
 * Return values:
 *   an handle to the item, possibly NULL_HEAP
 */
#[no_mangle]
pub unsafe extern "C" fn Find(mut h: *mut HeapP, mut item: *mut Item) -> *mut HeapP {
    let mut h1: *mut HeapP = 0 as *mut HeapP;
    let mut h2: *mut HeapP = 0 as *mut HeapP;
    if h.is_null() {
        return 0 as *mut HeapP;
    }
    h1 = h;
    loop {
        if Equal((*h1).item, item) != 0 {
            return h1;
        } else {
            if LessThan((*h1).item, item) != 0 {
                h2 = Find((*h1).child, item);
                if !h2.is_null() {
                    return h2;
                }
            }
        }
        h1 = (*h1).forward;
        if !(h1 != h) {
            break;
        }
    }
    return 0 as *mut HeapP;
}
