use ::libc;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Vertices {
    pub id: libc::c_int,
    pub edges: *mut Edges,
    pub next: *mut _Vertices,
    pub key: libc::c_int,
    pub chosenEdge: *mut Edges,
}
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
 * For the ft algorithm.
 */
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
pub const __ASSERT_FUNCTION: [libc::c_char; 28] = unsafe {
    *::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(b"Item *Subtract(Item *, int)\x00")
};
/*
 * item.c
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
 * $Id: item.c,v 1.2 1993/03/15 04:25:10 alain Exp alain $
 *
 */
#[no_mangle]
pub unsafe extern "C" fn LessThan(mut item1: *mut Item, mut item2: *mut Item) -> libc::c_int {
    return ((*item1).key < (*item2).key) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Equal(mut item1: *mut Item, mut item2: *mut Item) -> libc::c_int {
    return ((*item1).key == (*item2).key) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Subtract(mut item: *mut Item, mut delta: libc::c_int) -> *mut Item {
    if delta > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"delta > 0\x00" as *const u8 as *const libc::c_char,
            b"item.c\x00" as *const u8 as *const libc::c_char,
            49 as libc::c_int as libc::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    }
    (*item).key = (*item).key - delta;
    return item;
}
