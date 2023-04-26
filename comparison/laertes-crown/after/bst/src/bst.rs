
extern "C" {
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub key: std::os::raw::c_int,
    pub left: * mut crate::src::bst::node,
    pub right: * mut crate::src::bst::node,
}
impl node {
    pub const fn new() -> Self {
        node {
        key: 0,
        left: (0 as * mut crate::src::bst::node),
        right: (0 as * mut crate::src::bst::node)
        }
    }
}

impl std::default::Default for node {
    fn default() -> Self { node::new() }
}

#[no_mangle]
pub unsafe extern "C" fn newNode(mut item: std::os::raw::c_int) -> * mut crate::src::bst::node {
    let mut temp = malloc(::std::mem::size_of::<node>() as std::os::raw::c_ulong) as *mut node;
    (*temp).key = item;
    let ref mut fresh0 = (*temp).left;
    *fresh0 = 0 as *mut node;
    let ref mut fresh1 = (*temp).right;
    *fresh1 = 0 as *mut node;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn inorder(mut root: * mut crate::src::bst::node) {
    if !root.is_null() {
        inorder((*root).left);
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, (*root).key);
        inorder((*root).right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut node: * mut crate::src::bst::node, mut key: std::os::raw::c_int) -> * mut crate::src::bst::node {
    if node.is_null() {
        return newNode(key);
    }
    if key < (*node).key {
        let ref mut fresh2 = (*node).left;
        *fresh2 = insert((*node).left, key);
    } else {
        let ref mut fresh3 = (*node).right;
        *fresh3 = insert((*node).right, key);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: * mut crate::src::bst::node) -> * mut crate::src::bst::node {
    let mut current = node;
    while !current.is_null() && !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn deleteNode(
    mut root: * mut crate::src::bst::node,
    mut key: std::os::raw::c_int,
) -> * mut crate::src::bst::node {
    if root.is_null() {
        return root;
    }
    if key < (*root).key {
        let ref mut fresh4 = (*root).left;
        *fresh4 = deleteNode((*root).left, key);
    } else if key > (*root).key {
        let ref mut fresh5 = (*root).right;
        *fresh5 = deleteNode((*root).right, key);
    } else {
        if ((*root).left).is_null() {
            let mut temp = (*root).right;
            free(root as *mut std::os::raw::c_void);
            return temp;
        } else {
            if ((*root).right).is_null() {
                let mut temp_0 = (*root).left;
                free(root as *mut std::os::raw::c_void);
                return temp_0;
            }
        }
        let mut temp_1 = minValueNode((*root).right);
        (*root).key = (*temp_1).key;
        let ref mut fresh6 = (*root).right;
        *fresh6 = deleteNode((*root).right, (*temp_1).key);
    }
    return root;
}
use crate::laertes_rt::*;
