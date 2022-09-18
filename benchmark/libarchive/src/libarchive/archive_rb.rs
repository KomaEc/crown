use ::libc;
pub type uintptr_t = libc::c_ulong;
/*-
 * Copyright (c) 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Matt Thomas <matt@3am-software.com>.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Based on NetBSD: rb.h,v 1.13 2009/08/16 10:57:01 yamt Exp
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_node {
    pub rb_nodes: [*mut archive_rb_node; 2],
    pub rb_info: uintptr_t,
}
/*
 * archive_rbto_compare_nodes_fn:
 *	return a positive value if the first node < the second node.
 *	return a negative value if the first node > the second node.
 *	return 0 if they are considered same.
 *
 * archive_rbto_compare_key_fn:
 *	return a positive value if the node < the key.
 *	return a negative value if the node > the key.
 *	return 0 if they are considered same.
 */
pub type archive_rbto_compare_nodes_fn = Option<
    unsafe extern "C" fn(_: *const archive_rb_node, _: *const archive_rb_node) -> libc::c_int,
>;
pub type archive_rbto_compare_key_fn =
    Option<unsafe extern "C" fn(_: *const archive_rb_node, _: *const libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree_ops {
    pub rbto_compare_nodes: archive_rbto_compare_nodes_fn,
    pub rbto_compare_key: archive_rbto_compare_key_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct archive_rb_tree {
    pub rbt_root: *mut archive_rb_node,
    pub rbt_ops: *const archive_rb_tree_ops,
}
pub const NULL: libc::c_int = 0 as libc::c_int;
/*-
 * Copyright (c) 2001 The NetBSD Foundation, Inc.
 * All rights reserved.
 *
 * This code is derived from software contributed to The NetBSD Foundation
 * by Matt Thomas <matt@3am-software.com>.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE NETBSD FOUNDATION, INC. AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
 * TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Based on: NetBSD: rb.c,v 1.6 2010/04/30 13:58:09 joerg Exp
 */
/* Keep in sync with archive_rb.h */
pub const RB_DIR_LEFT: libc::c_int = 0 as libc::c_int;
pub const RB_DIR_RIGHT: libc::c_int = 1 as libc::c_int;
pub const RB_DIR_OTHER: libc::c_int = 1 as libc::c_int;
pub const RB_FLAG_POSITION: libc::c_int = 0x2 as libc::c_int;
pub const RB_FLAG_RED: libc::c_int = 0x1 as libc::c_int;
pub const RB_FLAG_MASK: libc::c_int = RB_FLAG_POSITION | RB_FLAG_RED;
pub const RB_SENTINEL_NODE: libc::c_int = 0 as libc::c_int;
pub const T: libc::c_int = 1 as libc::c_int;
pub const F: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_init(
    mut rbt: *mut archive_rb_tree,
    mut ops: *const archive_rb_tree_ops,
) {
    (*rbt).rbt_ops = ops;
    let ref mut fresh0 = *(&mut (*rbt).rbt_root as *mut *mut archive_rb_node);
    *fresh0 = RB_SENTINEL_NODE as *mut archive_rb_node;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_find_node(
    mut rbt: *mut archive_rb_tree,
    mut key: *const libc::c_void,
) -> *mut archive_rb_node {
    let compare_key: archive_rbto_compare_key_fn = (*(*rbt).rbt_ops).rbto_compare_key;
    let mut parent: *mut archive_rb_node = (*rbt).rbt_root;
    while !parent.is_null() {
        let diff: libc::c_int = Some(compare_key.expect("non-null function pointer"))
            .expect("non-null function pointer")(parent, key);
        if diff == 0 as libc::c_int {
            return parent;
        }
        parent = (*parent).rb_nodes[(diff > 0 as libc::c_int) as libc::c_int as usize]
    }
    return NULL as *mut archive_rb_node;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_find_node_geq(
    mut rbt: *mut archive_rb_tree,
    mut key: *const libc::c_void,
) -> *mut archive_rb_node {
    let compare_key: archive_rbto_compare_key_fn = (*(*rbt).rbt_ops).rbto_compare_key;
    let mut parent: *mut archive_rb_node = (*rbt).rbt_root;
    let mut last: *mut archive_rb_node = NULL as *mut archive_rb_node;
    while !parent.is_null() {
        let diff: libc::c_int = Some(compare_key.expect("non-null function pointer"))
            .expect("non-null function pointer")(parent, key);
        if diff == 0 as libc::c_int {
            return parent;
        }
        if diff < 0 as libc::c_int {
            last = parent
        }
        parent = (*parent).rb_nodes[(diff > 0 as libc::c_int) as libc::c_int as usize]
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_find_node_leq(
    mut rbt: *mut archive_rb_tree,
    mut key: *const libc::c_void,
) -> *mut archive_rb_node {
    let compare_key: archive_rbto_compare_key_fn = (*(*rbt).rbt_ops).rbto_compare_key;
    let mut parent: *mut archive_rb_node = (*rbt).rbt_root;
    let mut last: *mut archive_rb_node = NULL as *mut archive_rb_node;
    while !parent.is_null() {
        let diff: libc::c_int = Some(compare_key.expect("non-null function pointer"))
            .expect("non-null function pointer")(parent, key);
        if diff == 0 as libc::c_int {
            return parent;
        }
        if diff > 0 as libc::c_int {
            last = parent
        }
        parent = (*parent).rb_nodes[(diff > 0 as libc::c_int) as libc::c_int as usize]
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_insert_node(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
) -> libc::c_int {
    let compare_nodes: archive_rbto_compare_nodes_fn = (*(*rbt).rbt_ops).rbto_compare_nodes;
    let mut parent: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut tmp: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut position: libc::c_uint = 0;
    let mut rebalance: libc::c_int = 0;
    tmp = (*rbt).rbt_root;
    /*
     * This is a hack.  Because rbt->rbt_root is just a
     * struct archive_rb_node *, just like rb_node->rb_nodes[RB_DIR_LEFT],
     * we can use this fact to avoid a lot of tests for root and know
     * that even at root, updating
     * RB_FATHER(rb_node)->rb_nodes[RB_POSITION(rb_node)] will
     * update rbt->rbt_root.
     */
    parent = &mut (*rbt).rbt_root as *mut *mut archive_rb_node as *mut libc::c_void
        as *mut archive_rb_node;
    position = RB_DIR_LEFT as libc::c_uint;
    /*
     * Find out where to place this new leaf.
     */
    while !tmp.is_null() {
        let diff: libc::c_int = Some(compare_nodes.expect("non-null function pointer"))
            .expect("non-null function pointer")(tmp, self_0);
        if diff == 0 as libc::c_int {
            /*
             * Node already exists; don't insert.
             */
            return F;
        }
        parent = tmp;
        position = (diff > 0 as libc::c_int) as libc::c_int as libc::c_uint;
        tmp = (*parent).rb_nodes[position as usize]
    }
    /*
     * Initialize the node and insert as a leaf into the tree.
     */
    (*self_0).rb_info = parent as uintptr_t | (*self_0).rb_info & RB_FLAG_MASK as libc::c_ulong; /* root is always black */
    if position != 0 {
        (*self_0).rb_info |= RB_FLAG_POSITION as libc::c_ulong
    } else {
        (*self_0).rb_info &= !RB_FLAG_POSITION as libc::c_ulong
    };
    if parent
        == &mut (*rbt).rbt_root as *mut *mut archive_rb_node as *mut libc::c_void
            as *mut archive_rb_node
    {
        (*self_0).rb_info &= !RB_FLAG_RED as libc::c_ulong;
        rebalance = F
    } else {
        /*
         * All new nodes are colored red.  We only need to rebalance
         * if our parent is also red.
         */
        (*self_0).rb_info |= RB_FLAG_RED as libc::c_ulong;
        rebalance = (!parent.is_null()
            && (*parent).rb_info & RB_FLAG_RED as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong) as libc::c_int
    }
    (*self_0).rb_nodes[RB_DIR_LEFT as usize] = (*parent).rb_nodes[position as usize];
    (*self_0).rb_nodes[RB_DIR_RIGHT as usize] = (*parent).rb_nodes[position as usize];
    (*parent).rb_nodes[position as usize] = self_0;
    /*
     * Rebalance tree after insertion
     */
    if rebalance != 0 {
        __archive_rb_tree_insert_rebalance(rbt, self_0);
    }
    return T;
}
/*
 * Swap the location and colors of 'self' and its child @ which.  The child
 * can not be a sentinel node.  This is our rotation function.  However,
 * since it preserves coloring, it great simplifies both insertion and
 * removal since rotation almost always involves the exchanging of colors
 * as a separate step.
 */
/*ARGSUSED*/
unsafe extern "C" fn __archive_rb_tree_reparent_nodes(
    mut old_father: *mut archive_rb_node,
    which: libc::c_uint,
) {
    let other: libc::c_uint = which ^ RB_DIR_OTHER as libc::c_uint;
    let grandpa: *mut archive_rb_node =
        ((*old_father).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
    let old_child: *mut archive_rb_node = (*old_father).rb_nodes[which as usize];
    let new_father: *mut archive_rb_node = old_child;
    let new_child: *mut archive_rb_node = old_father;
    if new_father.is_null() {
        return;
    }
    /*
     * Exchange descendant linkages.
     */
    (*grandpa).rb_nodes[if (*old_father).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    } as usize] = new_father;
    (*new_child).rb_nodes[which as usize] = (*old_child).rb_nodes[other as usize];
    (*new_father).rb_nodes[other as usize] = new_child;
    /*
     * Update ancestor linkages
     */
    (*new_father).rb_info =
        grandpa as uintptr_t | (*new_father).rb_info & RB_FLAG_MASK as libc::c_ulong;
    (*new_child).rb_info =
        new_father as uintptr_t | (*new_child).rb_info & RB_FLAG_MASK as libc::c_ulong;
    /*
     * Exchange properties between new_father and new_child.  The only
     * change is that new_child's position is now on the other side.
     */
    let mut xorinfo: uintptr_t =
        ((*new_father).rb_info ^ (*new_child).rb_info) & RB_FLAG_MASK as libc::c_ulong;
    (*new_father).rb_info ^= xorinfo;
    (*new_child).rb_info ^= xorinfo;
    if other != 0 {
        (*new_child).rb_info |= RB_FLAG_POSITION as libc::c_ulong
    } else {
        (*new_child).rb_info &= !RB_FLAG_POSITION as libc::c_ulong
    };
    /*
     * Make sure to reparent the new child to ourself.
     */
    if !(*new_child).rb_nodes[which as usize].is_null() {
        (*(*new_child).rb_nodes[which as usize]).rb_info = new_child as uintptr_t
            | (*(*new_child).rb_nodes[which as usize]).rb_info & RB_FLAG_MASK as libc::c_ulong;
        if which != 0 {
            (*(*new_child).rb_nodes[which as usize]).rb_info |= RB_FLAG_POSITION as libc::c_ulong
        } else {
            (*(*new_child).rb_nodes[which as usize]).rb_info &= !RB_FLAG_POSITION as libc::c_ulong
        };
    };
}
/*CONSTCOND*/
unsafe extern "C" fn __archive_rb_tree_insert_rebalance(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
) {
    let mut father: *mut archive_rb_node =
        ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
    let mut grandpa: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut uncle: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut which: libc::c_uint = 0;
    let mut other: libc::c_uint = 0;
    loop {
        /*
         * We are red and our parent is red, therefore we must have a
         * grandfather and he must be black.
         */
        grandpa = ((*father).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
        which =
            (father == (*grandpa).rb_nodes[RB_DIR_RIGHT as usize]) as libc::c_int as libc::c_uint;
        other = which ^ RB_DIR_OTHER as libc::c_uint;
        uncle = (*grandpa).rb_nodes[other as usize];
        if uncle.is_null()
            || (*uncle).rb_info & RB_FLAG_RED as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
        {
            break;
        }
        /*
         * Case 1: our uncle is red
         *   Simply invert the colors of our parent and
         *   uncle and make our grandparent red.  And
         *   then solve the problem up at his level.
         */
        (*uncle).rb_info &= !RB_FLAG_RED as libc::c_ulong;
        (*father).rb_info &= !RB_FLAG_RED as libc::c_ulong;
        if (*rbt).rbt_root == grandpa {
            /*
             * If our grandpa is root, don't bother
             * setting him to red, just return.
             */
            return;
        }
        (*grandpa).rb_info |= RB_FLAG_RED as libc::c_ulong;
        self_0 = grandpa;
        father = ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
        if father.is_null()
            || (*father).rb_info & RB_FLAG_RED as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
        {
            /*
             * If our great-grandpa is black, we're done.
             */
            return;
        }
    }
    /*
     * Case 2&3: our uncle is black.
     */
    if self_0 == (*father).rb_nodes[other as usize] {
        /*
         * Case 2: we are on the same side as our uncle
         *   Swap ourselves with our parent so this case
         *   becomes case 3.  Basically our parent becomes our
         *   child.
         */
        __archive_rb_tree_reparent_nodes(father, other);
    }
    /*
     * Case 3: we are opposite a child of a black uncle.
     *   Swap our parent and grandparent.  Since our grandfather
     *   is black, our father will become black and our new sibling
     *   (former grandparent) will become red.
     */
    __archive_rb_tree_reparent_nodes(grandpa, which);
    /*
     * Final step: Set the root to black.
     */
    (*(*rbt).rbt_root).rb_info &= !RB_FLAG_RED as libc::c_ulong;
}
unsafe extern "C" fn __archive_rb_tree_prune_node(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
    mut rebalance: libc::c_int,
) {
    let which: libc::c_uint = if (*self_0).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    } as libc::c_uint;
    let mut father: *mut archive_rb_node =
        ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
    /*
     * Since we are childless, we know that self->rb_left is pointing
     * to the sentinel node.
     */
    (*father).rb_nodes[which as usize] = (*self_0).rb_nodes[RB_DIR_LEFT as usize];
    /*
     * Rebalance if requested.
     */
    if rebalance != 0 {
        __archive_rb_tree_removal_rebalance(rbt, father, which);
    };
}
/*
 * When deleting an interior node
 */
unsafe extern "C" fn __archive_rb_tree_swap_prune_and_rebalance(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
    mut standin: *mut archive_rb_node,
) {
    let standin_which: libc::c_uint = if (*standin).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0
    {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    } as libc::c_uint;
    let mut standin_other: libc::c_uint = standin_which ^ RB_DIR_OTHER as libc::c_uint;
    let mut standin_son: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut standin_father: *mut archive_rb_node =
        ((*standin).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
    let mut rebalance: libc::c_int = (standin.is_null()
        || (*standin).rb_info & RB_FLAG_RED as libc::c_ulong == 0 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    if standin_father == self_0 {
        /*
         * As a child of self, any children would be opposite of
         * our parent.
         */
        standin_son = (*standin).rb_nodes[standin_which as usize]
    } else {
        /*
         * Since we aren't a child of self, any children would be
         * on the same side as our parent.
         */
        standin_son = (*standin).rb_nodes[standin_other as usize]
    }
    if !standin_son.is_null()
        && (*standin_son).rb_info & RB_FLAG_RED as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        /*
         * We know we have a red child so if we flip it to black
         * we don't have to rebalance.
         */
        (*standin_son).rb_info &= !RB_FLAG_RED as libc::c_ulong;
        rebalance = F;
        if standin_father != self_0 {
            /*
             * Change the son's parentage to point to his grandpa.
             */
            (*standin_son).rb_info = standin_father as uintptr_t
                | (*standin_son).rb_info & RB_FLAG_MASK as libc::c_ulong;
            if standin_which != 0 {
                (*standin_son).rb_info |= RB_FLAG_POSITION as libc::c_ulong
            } else {
                (*standin_son).rb_info &= !RB_FLAG_POSITION as libc::c_ulong
            };
        }
    }
    if standin_father == self_0 {
        /*
         * If we are about to delete the standin's father, then when
         * we call rebalance, we need to use ourselves as our father.
         * Otherwise remember our original father.  Also, since we are
         * our standin's father we only need to reparent the standin's
         * brother.
         *
         * |    R      -->     S    |
         * |  Q   S    -->   Q   T  |
         * |        t  -->          |
         *
         * Have our son/standin adopt his brother as his new son.
         */
        standin_father = standin
    } else {
        /*
         * |    R          -->    S       .  |
         * |   / \  |   T  -->   / \  |  /   |
         * |  ..... | S    -->  ..... | T    |
         *
         * Sever standin's connection to his father.
         */
        (*standin_father).rb_nodes[standin_which as usize] = standin_son;
        /*
         * Adopt the far son.
         */
        (*standin).rb_nodes[standin_other as usize] = (*self_0).rb_nodes[standin_other as usize];
        (*(*standin).rb_nodes[standin_other as usize]).rb_info = standin as uintptr_t
            | (*(*standin).rb_nodes[standin_other as usize]).rb_info
                & RB_FLAG_MASK as libc::c_ulong;
        /*
         * Use standin_other because we need to preserve standin_which
         * for the removal_rebalance.
         */
        standin_other = standin_which
    }
    /*
     * Move the only remaining son to our standin.  If our standin is our
     * son, this will be the only son needed to be moved.
     */
    (*standin).rb_nodes[standin_other as usize] = (*self_0).rb_nodes[standin_other as usize];
    (*(*standin).rb_nodes[standin_other as usize]).rb_info = standin as uintptr_t
        | (*(*standin).rb_nodes[standin_other as usize]).rb_info & RB_FLAG_MASK as libc::c_ulong;
    /*
     * Now copy the result of self to standin and then replace
     * self with standin in the tree.
     */
    (*standin).rb_info ^= ((*standin).rb_info ^ (*self_0).rb_info) & RB_FLAG_MASK as libc::c_ulong;
    (*standin).rb_info = ((*self_0).rb_info
        & !(0x2 as libc::c_int | 0x1 as libc::c_int) as libc::c_ulong)
        as *mut archive_rb_node as uintptr_t
        | (*standin).rb_info & RB_FLAG_MASK as libc::c_ulong;
    let ref mut fresh1 = (*(((*standin).rb_info & !RB_FLAG_MASK as libc::c_ulong)
        as *mut archive_rb_node))
        .rb_nodes[if (*standin).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    } as usize];
    *fresh1 = standin;
    if rebalance != 0 {
        __archive_rb_tree_removal_rebalance(rbt, standin_father, standin_which);
    };
}
/*
 * We could do this by doing
 *	__archive_rb_tree_node_swap(rbt, self, which);
 *	__archive_rb_tree_prune_node(rbt, self, F);
 *
 * But it's more efficient to just evaluate and recolor the child.
 */
unsafe extern "C" fn __archive_rb_tree_prune_blackred_branch(
    mut self_0: *mut archive_rb_node,
    mut which: libc::c_uint,
) {
    let mut father: *mut archive_rb_node =
        ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
    let mut son: *mut archive_rb_node = (*self_0).rb_nodes[which as usize];
    /*
     * Remove ourselves from the tree and give our former child our
     * properties (position, color, root).
     */
    (*son).rb_info ^= ((*son).rb_info ^ (*self_0).rb_info) & RB_FLAG_MASK as libc::c_ulong;
    (*father).rb_nodes[if (*son).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    } as usize] = son;
    (*son).rb_info = father as uintptr_t | (*son).rb_info & RB_FLAG_MASK as libc::c_ulong;
}
/*
 *
 */
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_remove_node(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
) {
    let mut standin: *mut archive_rb_node = 0 as *mut archive_rb_node;
    let mut which: libc::c_uint = 0;
    /*
     * In the following diagrams, we (the node to be removed) are S.  Red
     * nodes are lowercase.  T could be either red or black.
     *
     * Remember the major axiom of the red-black tree: the number of
     * black nodes from the root to each leaf is constant across all
     * leaves, only the number of red nodes varies.
     *
     * Thus removing a red leaf doesn't require any other changes to a
     * red-black tree.  So if we must remove a node, attempt to rearrange
     * the tree so we can remove a red node.
     *
     * The simplest case is a childless red node or a childless root node:
     *
     * |    T  -->    T  |    or    |  R  -->  *  |
     * |  s    -->  *    |
     */
    if self_0.is_null()
        || (*self_0).rb_nodes[0 as libc::c_int as usize].is_null()
            && (*self_0).rb_nodes[1 as libc::c_int as usize].is_null()
    {
        let rebalance: libc::c_int = ((self_0.is_null()
            || (*self_0).rb_info & RB_FLAG_RED as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong)
            && !((*rbt).rbt_root == self_0)) as libc::c_int;
        __archive_rb_tree_prune_node(rbt, self_0, rebalance);
        return;
    }
    if !(!self_0.is_null()
        && !(*self_0).rb_nodes[0 as libc::c_int as usize].is_null()
        && !(*self_0).rb_nodes[1 as libc::c_int as usize].is_null())
    {
        /*
         * The next simplest case is the node we are deleting is
         * black and has one red child.
         *
         * |      T  -->      T  -->      T  |
         * |    S    -->  R      -->  R      |
         * |  r      -->    s    -->    *    |
         */
        which = if (*self_0).rb_nodes[0 as libc::c_int as usize].is_null() {
            RB_DIR_RIGHT
        } else {
            RB_DIR_LEFT
        } as libc::c_uint;
        __archive_rb_tree_prune_blackred_branch(self_0, which);
        return;
    }
    /*
     * We invert these because we prefer to remove from the inside of
     * the tree.
     */
    which = ((if (*self_0).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
        RB_DIR_RIGHT
    } else {
        RB_DIR_LEFT
    }) ^ RB_DIR_OTHER) as libc::c_uint;
    /*
     * Let's find the node closes to us opposite of our parent
     * Now swap it with ourself, "prune" it, and rebalance, if needed.
     */
    standin = __archive_rb_tree_iterate(rbt, self_0, which); /* The tree may be broken. */
    __archive_rb_tree_swap_prune_and_rebalance(rbt, self_0, standin);
}
unsafe extern "C" fn __archive_rb_tree_removal_rebalance(
    mut rbt: *mut archive_rb_tree,
    mut parent: *mut archive_rb_node,
    mut which: libc::c_uint,
) {
    while (*parent).rb_nodes[which as usize].is_null()
        || (*(*parent).rb_nodes[which as usize]).rb_info & RB_FLAG_RED as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong
    {
        let mut other: libc::c_uint = which ^ RB_DIR_OTHER as libc::c_uint;
        let mut brother: *mut archive_rb_node = (*parent).rb_nodes[other as usize];
        if brother.is_null() {
            return;
        }
        /*
         * For cases 1, 2a, and 2b, our brother's children must
         * be black and our father must be black
         */
        if (parent.is_null()
            || (*parent).rb_info & RB_FLAG_RED as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong)
            && ((*brother).rb_nodes[0 as libc::c_int as usize].is_null()
                || (*(*brother).rb_nodes[0 as libc::c_int as usize]).rb_info
                    & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
            && ((*brother).rb_nodes[1 as libc::c_int as usize].is_null()
                || (*(*brother).rb_nodes[1 as libc::c_int as usize]).rb_info
                    & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
        {
            if !brother.is_null()
                && (*brother).rb_info & RB_FLAG_RED as libc::c_ulong
                    != 0 as libc::c_int as libc::c_ulong
            {
                /*
                 * Case 1: Our brother is red, swap its
                 * position (and colors) with our parent.
                 * This should now be case 2b (unless C or E
                 * has a red child which is case 3; thus no
                 * explicit branch to case 2b).
                 *
                 *    B         ->        D
                 *  A     d     ->    b     E
                 *      C   E   ->  A   C
                 */
                __archive_rb_tree_reparent_nodes(parent, other);
                brother = (*parent).rb_nodes[other as usize];
                if brother.is_null() {
                    return;
                }
            /* The tree may be broken. */
            } else {
                /*
                 * Both our parent and brother are black.
                 * Change our brother to red, advance up rank
                 * and go through the loop again.
                 *
                 *    B         ->   *B
                 * *A     D     ->  A     d
                 *      C   E   ->      C   E
                 */
                (*brother).rb_info |= RB_FLAG_RED as libc::c_ulong; /* root == parent == black */
                if (*rbt).rbt_root == parent {
                    return;
                }
                which = if (*parent).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
                    RB_DIR_RIGHT
                } else {
                    RB_DIR_LEFT
                } as libc::c_uint;
                parent =
                    ((*parent).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
                continue;
            }
        }
        /*
         * Avoid an else here so that case 2a above can hit either
         * case 2b, 3, or 4.
         */
        if !parent.is_null()
            && (*parent).rb_info & RB_FLAG_RED as libc::c_ulong != 0 as libc::c_int as libc::c_ulong
            && (brother.is_null()
                || (*brother).rb_info & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
            && ((*brother).rb_nodes[0 as libc::c_int as usize].is_null()
                || (*(*brother).rb_nodes[0 as libc::c_int as usize]).rb_info
                    & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
            && ((*brother).rb_nodes[1 as libc::c_int as usize].is_null()
                || (*(*brother).rb_nodes[1 as libc::c_int as usize]).rb_info
                    & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong)
        {
            /*
             * We are black, our father is red, our brother and
             * both nephews are black.  Simply invert/exchange the
             * colors of our father and brother (to black and red
             * respectively).
             *
             *	|    f        -->    F        |
             *	|  *     B    -->  *     b    |
             *	|      N   N  -->      N   N  |
             */
            (*parent).rb_info &= !RB_FLAG_RED as libc::c_ulong;
            (*brother).rb_info |= RB_FLAG_RED as libc::c_ulong;
            break;
        /* We're done! */
        } else {
            /*
             * Our brother must be black and have at least one
             * red child (it may have two).
             */
            if (*brother).rb_nodes[other as usize].is_null()
                || (*(*brother).rb_nodes[other as usize]).rb_info & RB_FLAG_RED as libc::c_ulong
                    == 0 as libc::c_int as libc::c_ulong
            {
                /*
                 * Case 3: our brother is black, our near
                 * nephew is red, and our far nephew is black.
                 * Swap our brother with our near nephew.
                 * This result in a tree that matches case 4.
                 * (Our father could be red or black).
                 *
                 *	|    F      -->    F      |
                 *	|  x     B  -->  x   B    |
                 *	|      n    -->        n  |
                 */
                __archive_rb_tree_reparent_nodes(brother, which);
                brother = (*parent).rb_nodes[other as usize]
            }
            /* We're done! */
            /*
             * Case 4: our brother is black and our far nephew
             * is red.  Swap our father and brother locations and
             * change our far nephew to black.  (these can be
             * done in either order so we change the color first).
             * The result is a valid red-black tree and is a
             * terminal case.  (again we don't care about the
             * father's color)
             *
             * If the father is red, we will get a red-black-black
             * tree:
             *	|  f      ->  f      -->    b    |
             *	|    B    ->    B    -->  F   N  |
             *	|      n  ->      N  -->         |
             *
             * If the father is black, we will get an all black
             * tree:
             *	|  F      ->  F      -->    B    |
             *	|    B    ->    B    -->  F   N  |
             *	|      n  ->      N  -->         |
             *
             * If we had two red nephews, then after the swap,
             * our former father would have a red grandson.
             */
            if (*brother).rb_nodes[other as usize].is_null() {
                return;
            } /* The tree may be broken. */
            (*(*brother).rb_nodes[other as usize]).rb_info &= !RB_FLAG_RED as libc::c_ulong;
            __archive_rb_tree_reparent_nodes(parent, other);
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __archive_rb_tree_iterate(
    mut rbt: *mut archive_rb_tree,
    mut self_0: *mut archive_rb_node,
    direction: libc::c_uint,
) -> *mut archive_rb_node {
    let other: libc::c_uint = direction ^ RB_DIR_OTHER as libc::c_uint;
    if self_0.is_null() {
        self_0 = (*rbt).rbt_root;
        if self_0.is_null() {
            return NULL as *mut archive_rb_node;
        }
        while !(*self_0).rb_nodes[direction as usize].is_null() {
            self_0 = (*self_0).rb_nodes[direction as usize]
        }
        return self_0;
    }
    /*
     * We can't go any further in this direction.  We proceed up in the
     * opposite direction until our parent is in direction we want to go.
     */
    if (*self_0).rb_nodes[direction as usize].is_null() {
        while !((*rbt).rbt_root == self_0) {
            if other
                == (if (*self_0).rb_info & RB_FLAG_POSITION as libc::c_ulong != 0 {
                    RB_DIR_RIGHT
                } else {
                    RB_DIR_LEFT
                }) as libc::c_uint
            {
                return ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node;
            }
            self_0 = ((*self_0).rb_info & !RB_FLAG_MASK as libc::c_ulong) as *mut archive_rb_node
        }
        return NULL as *mut archive_rb_node;
    }
    /*
     * Advance down one in current direction and go down as far as possible
     * in the opposite direction.
     */
    self_0 = (*self_0).rb_nodes[direction as usize];
    while !(*self_0).rb_nodes[other as usize].is_null() {
        self_0 = (*self_0).rb_nodes[other as usize]
    }
    return self_0;
}
