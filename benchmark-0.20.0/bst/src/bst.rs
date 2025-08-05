use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub key: libc::c_int,
    pub left: *mut node,
    pub right: *mut node
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut item: libc::c_int) -> *mut node {
    let mut temp: *mut node =
        malloc(::core::mem::size_of::<node>() as libc::c_ulong) as *mut node;
    (*temp).key = item;
    (*temp).left = 0 as *mut node;
    (*temp).right = 0 as *mut node;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn inorder(mut root: *mut node) {
    if !root.is_null() {
        inorder((*root).left);
        printf(b"%d \0" as *const u8 as *const libc::c_char, (*root).key);
        inorder((*root).right);
    }
}
#[no_mangle]
pub unsafe extern "C" fn insert(
    mut node: *mut node,
    mut key: libc::c_int
) -> *mut node {
    if node.is_null() {
        return newNode(key);
    }
    if key < (*node).key {
        (*node).left = insert((*node).left, key);
    } else {
        (*node).right = insert((*node).right, key);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: *mut node) -> *mut node {
    let mut current: *mut node = node;
    while !current.is_null() && !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn deleteNode(
    mut root: *mut node,
    mut key: libc::c_int
) -> *mut node {
    if root.is_null() {
        return root;
    }
    if key < (*root).key {
        (*root).left = deleteNode((*root).left, key);
    } else if key > (*root).key {
        (*root).right = deleteNode((*root).right, key);
    } else {
        if ((*root).left).is_null() {
            let mut temp: *mut node = (*root).right;
            free(root as *mut libc::c_void);
            return temp;
        } else if ((*root).right).is_null() {
            let mut temp_0: *mut node = (*root).left;
            free(root as *mut libc::c_void);
            return temp_0;
        }
        let mut temp_1: *mut node = minValueNode((*root).right);
        (*root).key = (*temp_1).key;
        (*root).right = deleteNode((*root).right, (*temp_1).key);
    }
    return root;
}
