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
    pub right: *mut node,
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut item: libc::c_int) -> *mut node {
    let mut temp = malloc(::std::mem::size_of::<node>() as libc::c_ulong) as *mut node;
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
    }else { assert!((root).is_null()); }
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut node: *mut node, mut key: libc::c_int) -> *mut node {
    if node.is_null() {assert!((node).is_null());
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
    let mut current = node;
    while !current.is_null() && !((*current).left).is_null() {
        current = (*current).left;
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn deleteNode(
    mut root: *mut node,
    mut key: libc::c_int,
) -> *mut node {
    if root.is_null() {assert!((root).is_null());
        return root;
    }
    if key < (*root).key {
        (*root).left = deleteNode((*root).left, key);
    } else if key > (*root).key {
        (*root).right = deleteNode((*root).right, key);
    } else {
        if ((*root).left).is_null() {assert!(((*root).left).is_null());
            let mut temp = (*root).right;
            free(root as *mut libc::c_void);
            return temp;
        } else {
            if ((*root).right).is_null() {assert!(((*root).right).is_null());
                let mut temp_0 = (*root).left;
                free(root as *mut libc::c_void);
                return temp_0;
            }
        }
        let mut temp_1 = minValueNode((*root).right);
        (*root).key = (*temp_1).key;
        (*root).right = deleteNode((*root).right, (*temp_1).key);
    }
    return root;
}
// unsafe fn main_0() -> libc::c_int {
//     let mut root = 0 as *mut node;
//     root = insert(root, 50 as libc::c_int);
//     root = insert(root, 30 as libc::c_int);
//     root = insert(root, 20 as libc::c_int);
//     root = insert(root, 40 as libc::c_int);
//     root = insert(root, 70 as libc::c_int);
//     root = insert(root, 60 as libc::c_int);
//     root = insert(root, 80 as libc::c_int);
//     printf(
//         b"Inorder traversal of the given tree \n\0" as *const u8 as *const libc::c_char,
//     );
//     inorder(root);
//     printf(b"\nDelete 20\n\0" as *const u8 as *const libc::c_char);
//     root = deleteNode(root, 20 as libc::c_int);
//     printf(
//         b"Inorder traversal of the modified tree \n\0" as *const u8
//             as *const libc::c_char,
//     );
//     inorder(root);
//     printf(b"\nDelete 30\n\0" as *const u8 as *const libc::c_char);
//     root = deleteNode(root, 30 as libc::c_int);
//     printf(
//         b"Inorder traversal of the modified tree \n\0" as *const u8
//             as *const libc::c_char,
//     );
//     inorder(root);
//     printf(b"\nDelete 50\n\0" as *const u8 as *const libc::c_char);
//     root = deleteNode(root, 50 as libc::c_int);
//     printf(
//         b"Inorder traversal of the modified tree \n\0" as *const u8
//             as *const libc::c_char,
//     );
//     inorder(root);
//     return 0 as libc::c_int;
// }
// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
