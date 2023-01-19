use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct node {
    pub key: libc::c_int,
    pub left: Option<Box<node>>,
    pub right: Option<Box<node>>,
}
impl Default for node {fn default() -> Self {Self {
key: Default::default(),
left: None,
right: None,
}}}
impl node {pub fn take(&mut self) -> Self {core::mem::take(self)}}

#[no_mangle]
pub unsafe extern "C" fn newNode(mut item: libc::c_int) -> Option<Box<node>> {
    let mut temp = Some(Box::new(<crate::src::bst::node as Default>::default()));
    (*temp.as_deref_mut().unwrap()).key= item;
    (*temp.as_deref_mut().unwrap()).left= None;
    (*temp.as_deref_mut().unwrap()).right= None;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn inorder(mut root: *mut node) {
    if !root.is_null() {
        inorder((*root).left.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        printf(b"%d \0" as *const u8 as *const libc::c_char, (*root).key);
        inorder((*root).right.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
    }else { (); }
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut node: Option<Box<node>>, mut key: libc::c_int) -> Option<Box<node>> {
    if node.as_deref().is_none() {();
        return newNode(key);
    }
    if key < (*node.as_deref().unwrap()).key {
        (*node.as_deref_mut().unwrap()).left= insert((*node.as_deref_mut().unwrap()).left.take(), key);
    } else {
        (*node.as_deref_mut().unwrap()).right= insert((*node.as_deref_mut().unwrap()).right.take(), key);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: *mut node) -> *mut node {
    let mut current = node;
    while !current.is_null() && !(*current).left.as_deref().is_none() {
        current= (*current).left.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut());
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn deleteNode(
    mut root: Option<Box<node>>,
    mut key: libc::c_int,
) -> Option<Box<node>> {
    if root.as_deref().is_none() {();
        return root;
    }
    if key < (*root.as_deref().unwrap()).key {
        (*root.as_deref_mut().unwrap()).left= deleteNode((*root.as_deref_mut().unwrap()).left.take(), key);
    } else if key > (*root.as_deref().unwrap()).key {
        (*root.as_deref_mut().unwrap()).right= deleteNode((*root.as_deref_mut().unwrap()).right.take(), key);
    } else {
        if (*root.as_deref_mut().unwrap()).left.as_deref().is_none() {();
            let mut temp = (*root.as_deref_mut().unwrap()).right.take();
            ();
            return temp;
        } else {
            if (*root.as_deref_mut().unwrap()).right.as_deref().is_none() {();
                let mut temp_0 = (*root.as_deref_mut().unwrap()).left.take();
                ();
                return temp_0;
            }
        }
        let mut temp_1 = minValueNode((*root.as_deref_mut().unwrap()).right.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()));
        (*root.as_deref_mut().unwrap()).key= (*temp_1).key;
        (*root.as_deref_mut().unwrap()).right= deleteNode((*root.as_deref_mut().unwrap()).right.take(), (*temp_1).key);
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
