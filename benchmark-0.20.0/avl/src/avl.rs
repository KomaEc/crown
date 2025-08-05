use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: libc::c_int,
    pub left: *mut Node,
    pub right: *mut Node,
    pub height: libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn height(mut N: *mut Node) -> libc::c_int {
    if N.is_null() {
        return 0 as libc::c_int;
    }
    return (*N).height;
}
#[no_mangle]
pub unsafe extern "C" fn max(
    mut a: libc::c_int,
    mut b: libc::c_int
) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut key: libc::c_int) -> *mut Node {
    let mut node: *mut Node =
        malloc(::core::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    (*node).key = key;
    (*node).left = 0 as *mut Node;
    (*node).right = 0 as *mut Node;
    (*node).height = 1 as libc::c_int;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    let mut x: *mut Node = (*y).left;
    let mut T2: *mut Node = (*x).right;
    (*y).left = T2;
    (*y).height = max(height((*y).left), height((*y).right)) + 1 as libc::c_int;
    (*x).right = y;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as libc::c_int;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn leftRotate(mut x: *mut Node) -> *mut Node {
    let mut y: *mut Node = (*x).right;
    let mut T2: *mut Node = (*y).left;
    (*x).right = T2;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as libc::c_int;
    (*y).left = x;
    (*y).height = max(height((*y).left), height((*y).right)) + 1 as libc::c_int;
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn getBalance(mut N: *mut Node) -> libc::c_int {
    if N.is_null() {
        return 0 as libc::c_int;
    }
    return height((*N).left) - height((*N).right);
}
#[no_mangle]
pub unsafe extern "C" fn insert(
    mut node: *mut Node,
    mut key: libc::c_int
) -> *mut Node {
    if node.is_null() {
        return newNode(key);
    }
    if key < (*node).key {
        (*node).left = insert((*node).left, key);
    } else if key > (*node).key {
        (*node).right = insert((*node).right, key);
    } else {
        return node;
    }
    (*node).height =
        1 as libc::c_int + max(height((*node).left), height((*node).right));
    let mut balance: libc::c_int = getBalance(node);
    if balance > 1 as libc::c_int && key < (*(*node).left).key {
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int) && key > (*(*node).right).key {
        return leftRotate(node);
    }
    if balance > 1 as libc::c_int && key > (*(*node).left).key {
        (*node).left = leftRotate((*node).left);
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int) && key < (*(*node).right).key {
        (*node).right = rightRotate((*node).right);
        return leftRotate(node);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: *mut Node) -> *mut Node {
    let mut current: *mut Node = node;
    while !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn preOrder(mut root: *mut Node) {
    if !root.is_null() {
        printf(b"%d \0" as *const u8 as *const libc::c_char, (*root).key);
        preOrder((*root).left);
        preOrder((*root).right);
    }
}
