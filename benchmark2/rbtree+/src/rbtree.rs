use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type rbcolor = libc::c_uint;
pub const RED: rbcolor = 1;
pub const BLACK: rbcolor = 0;
pub type t_rbcolor = rbcolor;
pub type t_key = libc::c_uint;
pub type t_value = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_rbnode {
    pub key: t_key,
    pub value: t_value,
    pub color: t_rbcolor,
    pub left: *mut s_rbnode,
    pub right: *mut s_rbnode,
}
pub type t_rbnode = s_rbnode;
// #[no_mangle]
// pub static mut root_rbtree: *mut t_rbnode = 0 as *const t_rbnode as *mut t_rbnode;
#[inline]
unsafe extern "C" fn is_red(mut node: *mut t_rbnode) -> libc::c_int {
    return if !node.is_null() {
        ((*node).color as libc::c_uint == RED as libc::c_int as libc::c_uint)
            as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn my_compare(
    mut key1: libc::c_uint,
    mut key2: libc::c_uint,
) -> libc::c_int {
    return if key1 == key2 {
        0 as libc::c_int
    } else if key1 < key2 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn flip_color(mut node: *mut t_rbnode) {
    (*node).color = ((*node).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).left)
        .color = ((*(*node).left).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).right)
        .color = ((*(*node).right).color as u64 == 0) as libc::c_int as t_rbcolor;
}
unsafe extern "C" fn rotate_left(mut left: *mut t_rbnode) -> *mut t_rbnode {
    let mut right = 0 as *mut t_rbnode;
    if left.is_null() {
        return 0 as *mut t_rbnode;
    }
    right = (*left).right;
    let ref mut fresh0 = (*left).right;
    *fresh0 = (*right).left;
    (*right).color = (*left).color;
    (*left).color = RED;
    let ref mut fresh1 = (*right).left;
    *fresh1 = left;
    return right;
}
unsafe extern "C" fn rotate_right(mut right: *mut t_rbnode) -> *mut t_rbnode {
    let mut left = 0 as *mut t_rbnode;
    if right.is_null() {
        return 0 as *mut t_rbnode;
    }
    left = (*right).left;
    let ref mut fresh2 = (*right).left;
    *fresh2 = (*left).right;
    (*left).color = (*right).color;
    (*right).color = RED;
    let ref mut fresh3 = (*left).right;
    *fresh3 = right;
    return left;
}
#[no_mangle]
pub unsafe extern "C" fn create_node(
    mut key: t_key,
    mut value: t_value,
) -> *mut t_rbnode {
    let mut new = 0 as *mut t_rbnode;
    new = malloc(::std::mem::size_of::<t_rbnode>() as libc::c_ulong) as *mut t_rbnode;
    if new.is_null() {
        return 0 as *mut t_rbnode;
    }
    (*new).key = key;
    (*new).value = value;
    (*new).color = RED;
    let ref mut fresh4 = (*new).left;
    *fresh4 = 0 as *mut t_rbnode;
    let ref mut fresh5 = (*new).right;
    *fresh5 = 0 as *mut t_rbnode;
    return new;
}
unsafe extern "C" fn insert_this(
    mut node: *mut t_rbnode,
    mut key: t_key,
    mut value: t_value,
) -> *mut t_rbnode {
    let mut res: libc::c_int = 0;
    if node.is_null() {
        return create_node(key, value);
    }
    res = my_compare(key, (*node).key);
    if res == 0 as libc::c_int {
        (*node).value = value;
    } else if res < 0 as libc::c_int {
        let ref mut fresh6 = (*node).left;
        *fresh6 = insert_this((*node).left, key, value);
    } else {
        let ref mut fresh7 = (*node).right;
        *fresh7 = insert_this((*node).right, key, value);
    }
    if is_red((*node).right) != 0 && is_red((*node).left) == 0 {
        node = rotate_left(node);
    }
    if is_red((*node).left) != 0 && is_red((*(*node).left).left) != 0 {
        node = rotate_right(node);
    }
    if is_red((*node).left) != 0 && is_red((*node).right) != 0 {
        flip_color(node);
    }
    return node;
}
// #[no_mangle]
// pub unsafe extern "C" fn insert(mut key: t_key, mut value: t_value) {
//     root_rbtree = insert_this(root_rbtree, key, value);
//     if !root_rbtree.is_null() {
//         (*root_rbtree).color = BLACK;
//     }
// }
#[no_mangle]
pub unsafe extern "C" fn get_key(mut node: *mut t_rbnode, mut key: t_key) -> t_value {
    let mut cmp: libc::c_int = 0;
    while !node.is_null() {
        cmp = my_compare(key, (*node).key);
        if cmp == 0 {
            return (*node).value;
        }
        node = if cmp < 0 as libc::c_int { (*node).left } else { (*node).right };
    }
    return 0 as libc::c_int as t_value;
}
unsafe extern "C" fn min(mut node: *mut t_rbnode) -> *mut t_rbnode {
    if node.is_null() {
        return 0 as *mut t_rbnode;
    }
    while !((*node).left).is_null() {
        node = (*node).left;
    }
    return node;
}
#[inline]
unsafe extern "C" fn balance_me_that(mut node: *mut t_rbnode) -> *mut t_rbnode {
    if is_red((*node).right) != 0 {
        node = rotate_left(node);
    }
    if is_red((*node).left) != 0 && is_red((*(*node).left).left) != 0 {
        node = rotate_right(node);
    }
    if is_red((*node).left) != 0 && is_red((*node).right) != 0 {
        flip_color(node);
    }
    return node;
}
unsafe extern "C" fn move_red_to_left(mut node: *mut t_rbnode) -> *mut t_rbnode {
    flip_color(node);
    if !node.is_null() && !((*node).right).is_null()
        && is_red((*(*node).right).left) != 0
    {
        let ref mut fresh8 = (*node).right;
        *fresh8 = rotate_right((*node).right);
        node = rotate_left(node);
        flip_color(node);
    }
    return node;
}
unsafe extern "C" fn move_red_to_right(mut node: *mut t_rbnode) -> *mut t_rbnode {
    flip_color(node);
    if !node.is_null() && !((*node).left).is_null() && is_red((*(*node).left).left) != 0
    {
        node = rotate_right(node);
        flip_color(node);
    }
    return node;
}
unsafe extern "C" fn remove_min(mut node: *mut t_rbnode) -> *mut t_rbnode {
    if node.is_null() {
        return 0 as *mut t_rbnode;
    }
    if ((*node).left).is_null() {
        std::intrinsics::assume((*node).right.addr() == 0);
        free(node as *mut libc::c_void);
        return 0 as *mut t_rbnode;
    }
    if is_red((*node).left) == 0 && is_red((*(*node).left).left) == 0 {
        node = move_red_to_left(node);
    }
    let ref mut fresh9 = (*node).left;
    *fresh9 = remove_min((*node).left);
    return balance_me_that(node);
}
unsafe extern "C" fn remove_it(
    mut node: *mut t_rbnode,
    mut key: t_key,
) -> *mut t_rbnode {
    let mut tmp = 0 as *mut t_rbnode;
    if node.is_null() {
        return 0 as *mut t_rbnode;
    }
    if my_compare(key, (*node).key) == -(1 as libc::c_int) {
        if !((*node).left).is_null() {
            if is_red((*node).left) == 0 && is_red((*(*node).left).left) == 0 {
                node = move_red_to_left(node);
            }
            let ref mut fresh10 = (*node).left;
            *fresh10 = remove_key((*node).left, key);
        }
    } else {
        if is_red((*node).left) != 0 {
            node = rotate_right(node);
        }
        if my_compare(key, (*node).key) == 0 && ((*node).right).is_null() {
            std::intrinsics::assume((*node).left.addr() == 0);
            std::intrinsics::assume((*node).right.addr() == 0);
            free(node as *mut libc::c_void);
            return 0 as *mut t_rbnode;
        }
        if !((*node).right).is_null() {
            if is_red((*node).right) == 0 && is_red((*(*node).right).left) == 0 {
                node = move_red_to_right(node);
            }
            if my_compare(key, (*node).key) == 0 {
                tmp = min((*node).right);
                (*node).key = (*tmp).key;
                (*node).value = (*tmp).value;
                let ref mut fresh11 = (*node).right;
                *fresh11 = remove_min((*node).right);
            } else {
                let ref mut fresh12 = (*node).right;
                *fresh12 = remove_key((*node).right, key);
            }
        }
    }
    return balance_me_that(node);
}
#[no_mangle]
pub unsafe extern "C" fn remove_key(
    mut node: *mut t_rbnode,
    mut key: t_key,
) -> *mut t_rbnode {
    node = remove_it(node, key);
    if !node.is_null() {
        (*node).color = BLACK;
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn erase_tree(mut node: *mut t_rbnode) -> *mut t_rbnode {
    if !node.is_null() {
        if !((*node).left).is_null() {
            erase_tree((*node).left);
        }
        if !((*node).right).is_null() {
            erase_tree((*node).right);
        }
        let ref mut fresh13 = (*node).left;
        *fresh13 = 0 as *mut t_rbnode;
        let ref mut fresh14 = (*node).right;
        *fresh14 = 0 as *mut t_rbnode;
        free(node as *mut libc::c_void);
    }
    return 0 as *mut t_rbnode;
}
