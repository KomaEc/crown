
extern "C" {
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    fn free(_: * mut core::ffi::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: std::os::raw::c_int,
    pub left: * mut crate::src::avl::Node,
    pub right: * mut crate::src::avl::Node,
    pub height: std::os::raw::c_int,
}
impl Node {
    pub const fn new() -> Self {
        Node {
        key: 0,
        left: (0 as * mut crate::src::avl::Node),
        right: (0 as * mut crate::src::avl::Node),
        height: 0
        }
    }
}

impl std::default::Default for Node {
    fn default() -> Self { Node::new() }
}

#[no_mangle]
pub unsafe extern "C" fn height(mut N: * mut crate::src::avl::Node) -> std::os::raw::c_int {
    if N.is_null() {
        return 0 as std::os::raw::c_int;
    }
    return (*N).height;
}
#[no_mangle]
pub extern "C" fn max(mut a: std::os::raw::c_int, mut b: std::os::raw::c_int) -> std::os::raw::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut key: std::os::raw::c_int) -> * mut crate::src::avl::Node {
    let mut node = malloc(::std::mem::size_of::<Node>() as std::os::raw::c_ulong) as *mut Node;
    (*node).key = key;
    let ref mut fresh0 = (*node).left;
    *fresh0 = (0 as * mut crate::src::avl::Node);
    let ref mut fresh1 = (*node).right;
    *fresh1 = (0 as * mut crate::src::avl::Node);
    (*node).height = 1 as std::os::raw::c_int;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn rightRotate(mut y: * mut crate::src::avl::Node) -> * mut crate::src::avl::Node {
    let mut x = (*y).left;
    let mut T2 = (*x).right;
    let ref mut fresh2 = (*y).left;
    *fresh2 = T2;
    (*y).height = max(height((*y).left), height((*y).right)) + 1 as std::os::raw::c_int;
    let ref mut fresh3 = (*x).right;
    *fresh3 = y;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as std::os::raw::c_int;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn leftRotate(mut x: * mut crate::src::avl::Node) -> * mut crate::src::avl::Node {
    let mut y = (*x).right;
    let mut T2 = (*y).left;
    let ref mut fresh4 = (*x).right;
    *fresh4 = T2;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as std::os::raw::c_int;
    let ref mut fresh5 = (*y).left;
    *fresh5 = x;
    (*y).height = max(height((*y).left), height((*y).right)) + 1 as std::os::raw::c_int;
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn getBalance(mut N: * mut crate::src::avl::Node) -> std::os::raw::c_int {
    if N.is_null() {
        return 0 as std::os::raw::c_int;
    }
    return height((*N).left) - height((*N).right);
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut node: * mut crate::src::avl::Node, mut key: std::os::raw::c_int) -> * mut crate::src::avl::Node {
    if node.is_null() {
        return newNode(key);
    }
    if key < (*node).key {
        let ref mut fresh6 = (*node).left;
        *fresh6 = insert((*node).left, key);
    } else if key > (*node).key {
        let ref mut fresh7 = (*node).right;
        *fresh7 = insert((*node).right, key);
    } else {
        return node
    }
    (*node).height = 1 as std::os::raw::c_int + max(height((*node).left), height((*node).right));
    let mut balance = getBalance(node);
    if balance > 1 as std::os::raw::c_int && key < (*(*node).left).key {
        return rightRotate(node);
    }
    if balance < -(1 as std::os::raw::c_int) && key > (*(*node).right).key {
        return leftRotate(node);
    }
    if balance > 1 as std::os::raw::c_int && key > (*(*node).left).key {
        let ref mut fresh8 = (*node).left;
        *fresh8 = leftRotate((*node).left);
        return rightRotate(node);
    }
    if balance < -(1 as std::os::raw::c_int) && key < (*(*node).right).key {
        let ref mut fresh9 = (*node).right;
        *fresh9 = rightRotate((*node).right);
        return leftRotate(node);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: * mut crate::src::avl::Node) -> * mut crate::src::avl::Node {
    let mut current = node;
    while !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn preOrder(mut root: * mut crate::src::avl::Node) {
    if !root.is_null() {
        printf(b"%d \0" as *const u8 as *const std::os::raw::c_char, (*root).key);
        preOrder((*root).left);
        preOrder((*root).right);
    }
}
use crate::laertes_rt::*;
