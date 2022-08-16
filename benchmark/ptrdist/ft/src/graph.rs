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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn random() -> libc::c_long;
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
pub const NULL: libc::c_int = 0 as libc::c_int;
pub const NULL_EDGE: libc::c_int = 0 as libc::c_int;
pub const MAX_WEIGHT: libc::c_int = 100 as libc::c_int;
/*
 * For the ft algorithm.
 */
/*
 * Local variables.
 */
static mut generatedEdges: libc::c_int = 0;
/*
 * Local variables.
 */
static mut id: libc::c_int = 1 as libc::c_int;
/*
 * Generates random connected undirected graphs.  Unfortunately this current
 * implementation does not generate the graphs with a uniform distribution.
 *
 * Apparently a good reference is Tinhofer G., ,
 * C. Hanser, Verlag, M\"{u}nchen 1980.
 */
#[no_mangle]
pub unsafe extern "C" fn GenGraph(
    mut nVertex: libc::c_int,
    mut nEdge: libc::c_int,
) -> *mut Vertices {
    let mut graph: *mut Vertices = 0 as *mut Vertices;
    if nEdge + 1 as libc::c_int >= nVertex {
    } else {
        __assert_fail(
            b"nEdge + 1 >= nVertex\x00" as *const u8 as *const libc::c_char,
            b"graph.c\x00" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"Vertices *GenGraph(int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if nEdge <= nVertex * (nVertex - 1 as libc::c_int) / 2 as libc::c_int {
    } else {
        __assert_fail(
            b"nEdge <= nVertex * (nVertex - 1) / 2\x00" as *const u8 as *const libc::c_char,
            b"graph.c\x00" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"Vertices *GenGraph(int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    generatedEdges = 0 as libc::c_int;
    graph = GenTree(nVertex);
    graph = AddEdges(graph, nVertex, nEdge - nVertex + 1 as libc::c_int);
    return graph;
}
/*
 * Local functions.
 */
#[no_mangle]
pub unsafe extern "C" fn GenTree(mut nVertex: libc::c_int) -> *mut Vertices {
    let mut i: libc::c_int = 0;
    let mut weight: libc::c_int = 0;
    let mut vertex: *mut Vertices = 0 as *mut Vertices;
    let mut graph: *mut Vertices = 0 as *mut Vertices;
    let mut edge: *mut Edges = 0 as *mut Edges;
    graph = NewVertex();
    (*graph).next = graph;
    i = 1 as libc::c_int;
    while i < nVertex {
        vertex = NewVertex();
        edge = NewEdge();
        /*
         * The newly created vertex has one edge ...
         */
        (*vertex).edges = edge;
        /*
         * ... which is connected to the graph so far generated.  The connection
         * point in the graph is picked at random.
         */
        (*edge).vertex = PickVertex(graph, (random() % i as libc::c_long) as libc::c_int);
        weight = ((random() + 1 as libc::c_int as libc::c_long) % MAX_WEIGHT as libc::c_long)
            as libc::c_int;
        (*edge).weight = weight;
        (*edge).source = vertex;
        /*
         * Link the new vertex into the graph.
         */
        (*vertex).next = (*graph).next;
        (*graph).next = vertex;
        /*
         * Add an edge to the vertex randomly picked as the connection point.
         */
        edge = NewEdge();
        (*edge).weight = weight;
        (*edge).source = (*(*vertex).edges).vertex;
        (*edge).vertex = vertex;
        (*edge).next = (*(*(*vertex).edges).vertex).edges;
        (*(*(*vertex).edges).vertex).edges = edge;
        i += 1
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn AddEdges(
    mut graph: *mut Vertices,
    mut nVertex: libc::c_int,
    mut nEdge: libc::c_int,
) -> *mut Vertices {
    let mut i: libc::c_int = 0;
    let mut vertex1: *mut Vertices = 0 as *mut Vertices;
    let mut vertex2: *mut Vertices = 0 as *mut Vertices;
    if !graph.is_null() {
    } else {
        __assert_fail(
            b"graph != NULL_VERTEX\x00" as *const u8 as *const libc::c_char,
            b"graph.c\x00" as *const u8 as *const libc::c_char,
            143 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"Vertices *AddEdges(Vertices *, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    if nEdge >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"nEdge >= 0\x00" as *const u8 as *const libc::c_char,
            b"graph.c\x00" as *const u8 as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"Vertices *AddEdges(Vertices *, int, int)\x00",
            ))
            .as_ptr(),
        );
    }
    i = 0 as libc::c_int;
    while i < nEdge {
        loop {
            vertex1 = PickVertex(graph, (random() % nVertex as libc::c_long) as libc::c_int);
            vertex2 = PickVertex(
                (*vertex1).next,
                (random() % nVertex as libc::c_long - 1 as libc::c_int as libc::c_long)
                    as libc::c_int,
            );
            if vertex1 != vertex2 {
            } else {
                __assert_fail(
                    b"vertex1 != vertex2\x00" as *const u8 as *const libc::c_char,
                    b"graph.c\x00" as *const u8 as *const libc::c_char,
                    152 as libc::c_int as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                        b"Vertices *AddEdges(Vertices *, int, int)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if !(Duplicate(vertex1, vertex2) != 0) {
                break;
            }
        }
        Connect(vertex1, vertex2);
        i += 1
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn PickVertex(
    mut graph: *mut Vertices,
    mut whichVertex: libc::c_int,
) -> *mut Vertices {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < whichVertex {
        graph = (*graph).next;
        i += 1
    }
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn Connect(mut vertex1: *mut Vertices, mut vertex2: *mut Vertices) {
    let mut weight: libc::c_int = 0;
    let mut edge: *mut Edges = 0 as *mut Edges;
    weight =
        ((random() + 1 as libc::c_int as libc::c_long) % MAX_WEIGHT as libc::c_long) as libc::c_int;
    edge = NewEdge();
    (*edge).weight = weight;
    (*edge).source = vertex1;
    (*edge).vertex = vertex2;
    (*edge).next = (*vertex1).edges;
    (*vertex1).edges = edge;
    edge = NewEdge();
    (*edge).weight = weight;
    (*edge).source = vertex2;
    (*edge).vertex = vertex1;
    (*edge).next = (*vertex2).edges;
    (*vertex2).edges = edge;
}
#[no_mangle]
pub unsafe extern "C" fn Duplicate(
    mut vertex1: *mut Vertices,
    mut vertex2: *mut Vertices,
) -> libc::c_int {
    let mut edge: *mut Edges = 0 as *mut Edges;
    edge = (*vertex1).edges;
    while !edge.is_null() {
        if (*edge).vertex == vertex2 {
            return 1 as libc::c_int;
        }
        edge = (*edge).next
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn NewVertex() -> *mut Vertices {
    let mut vertex: *mut Vertices = 0 as *mut Vertices;
    vertex = malloc(::std::mem::size_of::<Vertices>() as libc::c_ulong) as *mut Vertices;
    if vertex.is_null() {
        fprintf(
            stderr,
            b"Could not malloc\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let fresh0 = id;
    id = id + 1;
    (*vertex).id = fresh0;
    (*vertex).edges = NULL as *mut Edges;
    (*vertex).next = NULL as *mut _Vertices;
    return vertex;
}
#[no_mangle]
pub unsafe extern "C" fn NewEdge() -> *mut Edges {
    let mut edge: *mut Edges = 0 as *mut Edges;
    edge = malloc(::std::mem::size_of::<Edges>() as libc::c_ulong) as *mut Edges;
    if edge.is_null() {
        fprintf(
            stderr,
            b"Could not malloc\n\x00" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*edge).weight = 0 as libc::c_int;
    (*edge).vertex = NULL as *mut _Vertices;
    (*edge).next = NULL as *mut _Edges;
    return edge;
}
#[no_mangle]
pub unsafe extern "C" fn PrintGraph(mut graph: *mut Vertices) {
    let mut vertex: *mut Vertices = 0 as *mut Vertices;
    if !graph.is_null() {
    } else {
        __assert_fail(
            b"graph != NULL\x00" as *const u8 as *const libc::c_char,
            b"graph.c\x00" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void PrintGraph(Vertices *)\x00",
            ))
            .as_ptr(),
        );
    }
    vertex = graph;
    loop {
        printf(
            b"Vertex %d is connected to:\x00" as *const u8 as *const libc::c_char,
            (*vertex).id,
        );
        PrintNeighbors(vertex);
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        vertex = (*vertex).next;
        if !(vertex != graph) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PrintNeighbors(mut vertex: *mut Vertices) {
    let mut edge: *mut Edges = 0 as *mut Edges;
    edge = (*vertex).edges;
    while !edge.is_null() {
        printf(
            b" %d(%d)[%d]\x00" as *const u8 as *const libc::c_char,
            (*(*edge).vertex).id,
            (*edge).weight,
            (*(*edge).source).id,
        );
        edge = (*edge).next
    }
}
