use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub key: libc::c_int,
    pub left: *mut Node,
    pub right: *mut Node,
    pub height: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn height(mut N: *mut Node) -> libc::c_int {
    if N.is_null() {
        return 0 as libc::c_int;
    }
    return (*N).height;
}
#[no_mangle]
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut key: libc::c_int) -> *mut Node {
    let mut node = malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node;
    (*node).key = key;
    let ref mut fresh0 = (*node).left;
    *fresh0 = 0 as *mut Node;
    let ref mut fresh1 = (*node).right;
    *fresh1 = 0 as *mut Node;
    (*node).height = 1 as libc::c_int;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    let mut x = (*y).left;
    let mut T2 = (*x).right;
    let ref mut fresh2 = (*y).left;
    *fresh2 = T2;
    (*y).height = max(height((*y).left), height((*y).right)) + 1 as libc::c_int;
    let ref mut fresh3 = (*x).right;
    *fresh3 = y;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as libc::c_int;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn leftRotate(mut x: *mut Node) -> *mut Node {
    let mut y = (*x).right;
    let mut T2 = (*y).left;
    let ref mut fresh4 = (*x).right;
    *fresh4 = T2;
    (*x).height = max(height((*x).left), height((*x).right)) + 1 as libc::c_int;
    let ref mut fresh5 = (*y).left;
    *fresh5 = x;
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
pub unsafe extern "C" fn insert(mut node: *mut Node, mut key: libc::c_int) -> *mut Node {
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
    (*node).height = 1 as libc::c_int + max(height((*node).left), height((*node).right));
    let mut balance = getBalance(node);
    if balance > 1 as libc::c_int && key < (*(*node).left).key {
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int) && key > (*(*node).right).key {
        return leftRotate(node);
    }
    if balance > 1 as libc::c_int && key > (*(*node).left).key {
        let ref mut fresh8 = (*node).left;
        *fresh8 = leftRotate((*node).left);
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int) && key < (*(*node).right).key {
        let ref mut fresh9 = (*node).right;
        *fresh9 = rightRotate((*node).right);
        return leftRotate(node);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: *mut Node) -> *mut Node {
    let mut current = node;
    while !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
// #[no_mangle]
// pub unsafe extern "C" fn deleteNode(
//     mut root: *mut Node,
//     mut key: libc::c_int,
// ) -> *mut Node {
//     if root.is_null() {
//         return root;
//     }
//     if key < (*root).key {
//         let ref mut fresh10 = (*root).left;
//         *fresh10 = deleteNode((*root).left, key);
//     } else if key > (*root).key {
//         let ref mut fresh11 = (*root).right;
//         *fresh11 = deleteNode((*root).right, key);
//     } else if ((*root).left).is_null() || ((*root).right).is_null() {
//         let mut temp = if !((*root).left).is_null() {
//             (*root).left
//         } else {
//             (*root).right
//         };
//         if temp.is_null() {
//             temp = root;
//             root = 0 as *mut Node;
//         } else {
//             *root = *temp;
//         }
//         free(temp as *mut libc::c_void);
//     } else {
//         let mut temp_0 = minValueNode((*root).right);
//         (*root).key = (*temp_0).key;
//         let ref mut fresh12 = (*root).right;
//         *fresh12 = deleteNode((*root).right, (*temp_0).key);
//     }
//     if root.is_null() {
//         return root;
//     }
//     (*root).height = 1 as libc::c_int + max(height((*root).left), height((*root).right));
//     let mut balance = getBalance(root);
//     if balance > 1 as libc::c_int && getBalance((*root).left) >= 0 as libc::c_int {
//         return rightRotate(root);
//     }
//     if balance > 1 as libc::c_int && getBalance((*root).left) < 0 as libc::c_int {
//         let ref mut fresh13 = (*root).left;
//         *fresh13 = leftRotate((*root).left);
//         return rightRotate(root);
//     }
//     if balance < -(1 as libc::c_int) && getBalance((*root).right) <= 0 as libc::c_int {
//         return leftRotate(root);
//     }
//     if balance < -(1 as libc::c_int) && getBalance((*root).right) > 0 as libc::c_int {
//         let ref mut fresh14 = (*root).right;
//         *fresh14 = rightRotate((*root).right);
//         return leftRotate(root);
//     }
//     return root;
// }
#[no_mangle]
pub unsafe extern "C" fn preOrder(mut root: *mut Node) {
    if !root.is_null() {
        printf(b"%d \0" as *const u8 as *const libc::c_char, (*root).key);
        preOrder((*root).left);
        preOrder((*root).right);
    }
}
// unsafe fn main_0() -> libc::c_int {
//     let mut root = 0 as *mut Node;
//     root = insert(root, 9 as libc::c_int);
//     root = insert(root, 5 as libc::c_int);
//     root = insert(root, 10 as libc::c_int);
//     root = insert(root, 0 as libc::c_int);
//     root = insert(root, 6 as libc::c_int);
//     root = insert(root, 11 as libc::c_int);
//     root = insert(root, -(1 as libc::c_int));
//     root = insert(root, 1 as libc::c_int);
//     root = insert(root, 2 as libc::c_int);
//     printf(
//         b"Preorder traversal of the constructed AVL tree is \n\0" as *const u8
//             as *const libc::c_char,
//     );
//     preOrder(root);
//     root = deleteNode(root, 10 as libc::c_int);
//     printf(
//         b"\nPreorder traversal after deletion of 10 \n\0" as *const u8
//             as *const libc::c_char,
//     );
//     preOrder(root);
//     return 0 as libc::c_int;
// }
// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
