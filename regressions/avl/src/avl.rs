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
pub struct Node {
    pub key: libc::c_int,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub height: libc::c_int,
}
impl Default for Node {
    fn default() -> Self {
        Self {
            key: Default::default(),
            left: None,
            right: None,
            height: Default::default(),
        }
    }
}
impl Node {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[no_mangle]
pub unsafe extern "C" fn height(mut N: *const Node) -> libc::c_int {
    if N.is_null() {
        ();
        return 0 as libc::c_int;
    }
    return (*N).height;
}
#[no_mangle]
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn newNode(mut key: libc::c_int) -> Option<Box<Node>> {
    let mut node = Some(Box::new(<crate::src::avl::Node as Default>::default()));
    (*node.as_deref_mut().unwrap()).key = key;
    (*node.as_deref_mut().unwrap()).left = None;
    (*node.as_deref_mut().unwrap()).right = None;
    (*node.as_deref_mut().unwrap()).height = 1 as libc::c_int;
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn rightRotate(mut y: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut x = (*y.as_deref_mut().unwrap()).left.take();
    let mut T2 = (*x.as_deref_mut().unwrap()).right.take();
    (*y.as_deref_mut().unwrap()).left = T2;
    (*y.as_deref_mut().unwrap()).height = max(
        height(
            (*y.as_deref().unwrap())
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
        height(
            (*y.as_deref().unwrap())
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
    ) + 1 as libc::c_int;
    (*x.as_deref_mut().unwrap()).right = y;
    (*x.as_deref_mut().unwrap()).height = max(
        height(
            (*x.as_deref().unwrap())
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
        height(
            (*x.as_deref().unwrap())
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
    ) + 1 as libc::c_int;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn leftRotate(mut x: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut y = (*x.as_deref_mut().unwrap()).right.take();
    let mut T2 = (*y.as_deref_mut().unwrap()).left.take();
    (*x.as_deref_mut().unwrap()).right = T2;
    (*x.as_deref_mut().unwrap()).height = max(
        height(
            (*x.as_deref().unwrap())
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
        height(
            (*x.as_deref().unwrap())
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
    ) + 1 as libc::c_int;
    (*y.as_deref_mut().unwrap()).left = x;
    (*y.as_deref_mut().unwrap()).height = max(
        height(
            (*y.as_deref().unwrap())
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
        height(
            (*y.as_deref().unwrap())
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ),
    ) + 1 as libc::c_int;
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn getBalance(mut N: *const Node) -> libc::c_int {
    if N.is_null() {
        ();
        return 0 as libc::c_int;
    }
    return height(
        (*N).left
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    ) - height(
        (*N).right
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn insert(
    mut node: Option<Box<Node>>,
    mut key: libc::c_int,
) -> Option<Box<Node>> {
    if node.as_deref().is_none() {
        ();
        return newNode(key);
    }
    if key < (*node.as_deref().unwrap()).key {
        (*node.as_deref_mut().unwrap()).left =
            insert((*node.as_deref_mut().unwrap()).left.take(), key);
    } else if key > (*node.as_deref().unwrap()).key {
        (*node.as_deref_mut().unwrap()).right =
            insert((*node.as_deref_mut().unwrap()).right.take(), key);
    } else {
        return node;
    }
    (*node.as_deref_mut().unwrap()).height = 1 as libc::c_int
        + max(
            height(
                (*node.as_deref().unwrap())
                    .left
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null()),
            ),
            height(
                (*node.as_deref().unwrap())
                    .right
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null()),
            ),
        );
    let mut balance = getBalance(
        node.as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    );
    if balance > 1 as libc::c_int
        && key < (*(*node.as_deref().unwrap()).left.as_deref().unwrap()).key
    {
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int)
        && key > (*(*node.as_deref().unwrap()).right.as_deref().unwrap()).key
    {
        return leftRotate(node);
    }
    if balance > 1 as libc::c_int
        && key > (*(*node.as_deref().unwrap()).left.as_deref().unwrap()).key
    {
        (*node.as_deref_mut().unwrap()).left =
            leftRotate((*node.as_deref_mut().unwrap()).left.take());
        return rightRotate(node);
    }
    if balance < -(1 as libc::c_int)
        && key < (*(*node.as_deref().unwrap()).right.as_deref().unwrap()).key
    {
        (*node.as_deref_mut().unwrap()).right =
            rightRotate((*node.as_deref_mut().unwrap()).right.take());
        return leftRotate(node);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn minValueNode(mut node: *const Node) -> *const Node {
    let mut current = node;
    while !(*current).left.as_deref().is_none() {
        current = (*current)
            .left
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null());
    }
    ();
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn deleteNode(mut root: *mut Node, mut key: libc::c_int) -> *mut Node {
    if root.is_null() {
        ();
        return root;
    }
    if key < (*root).key {
        (*root).left = deleteNode(
            (*root)
                .left
                .as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut()),
            key,
        );
    } else if key > (*root).key {
        (*root).right = deleteNode(
            (*root)
                .right
                .as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut()),
            key,
        );
    } else if (*root).left.as_deref().is_none() || (*root).right.as_deref().is_none() {
        let mut temp = if !(*root).left.as_deref().is_none() {
            (*root)
                .left
                .as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut())
        } else {
            ();
            (*root)
                .right
                .as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut())
        };
        if temp.is_null() {
            ();
            temp = root;
            root = 0 as *mut Node;
        } else {
            *root = (*temp).take();
        }
        free(temp as *mut libc::c_void);
    } else {
        let mut temp_0 = minValueNode(
            (*root)
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        );
        (*root).key = (*temp_0).key;
        (*root).right = deleteNode(
            (*root)
                .right
                .as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut()),
            (*temp_0).key,
        );
    }
    if root.is_null() {
        ();
        return root;
    }
    (*root).height = 1 as libc::c_int
        + max(
            height(
                (*root)
                    .left
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null()),
            ),
            height(
                (*root)
                    .right
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null()),
            ),
        );
    let mut balance = getBalance(root);
    if balance > 1 as libc::c_int
        && getBalance(
            (*root)
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ) >= 0 as libc::c_int
    {
        return rightRotate(Some(Box::from_raw(root)));
    }
    if balance > 1 as libc::c_int
        && getBalance(
            (*root)
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ) < 0 as libc::c_int
    {
        (*root).left = leftRotate((*root).left.take());
        return rightRotate(Some(Box::from_raw(root)));
    }
    if balance < -(1 as libc::c_int)
        && getBalance(
            (*root)
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ) <= 0 as libc::c_int
    {
        return leftRotate(Some(Box::from_raw(root)));
    }
    if balance < -(1 as libc::c_int)
        && getBalance(
            (*root)
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        ) > 0 as libc::c_int
    {
        (*root).right = rightRotate((*root).right.take());
        return leftRotate(Some(Box::from_raw(root)));
    }
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn preOrder(mut root: *const Node) {
    if !root.is_null() {
        printf(b"%d \0" as *const u8 as *const libc::c_char, (*root).key);
        preOrder(
            (*root)
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        );
        preOrder(
            (*root)
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null()),
        );
    } else {
        ();
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut root = None;
    root = insert(None, 9 as libc::c_int);
    root = insert(root, 5 as libc::c_int);
    root = insert(root, 10 as libc::c_int);
    root = insert(root, 0 as libc::c_int);
    root = insert(root, 6 as libc::c_int);
    root = insert(root, 11 as libc::c_int);
    root = insert(root, -(1 as libc::c_int));
    root = insert(root, 1 as libc::c_int);
    root = insert(root, 2 as libc::c_int);
    printf(
        b"Preorder traversal of the constructed AVL tree is \n\0" as *const u8
            as *const libc::c_char,
    );
    preOrder(
        root.as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    );
    root = deleteNode(
        root.as_deref_mut()
            .map(|r| r as *mut _)
            .unwrap_or(std::ptr::null_mut()),
        10 as libc::c_int,
    );
    printf(b"\nPreorder traversal after deletion of 10 \n\0" as *const u8 as *const libc::c_char);
    preOrder(
        root.as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    );
    return 0 as libc::c_int;
}
// pub fn main() {
//     unsafe { ::std::process::exit(main_0() as i32) }
// }
