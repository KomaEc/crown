use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
// C program to demonstrate
// delete operation in binary
// search tree

#[repr(C)]#[derive(Copy, Clone)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct node {
    pub key: i32,
    pub left: Option<Box<node>>,
    pub right: Option<Box<node>>,
}
impl Default for node {fn default() -> Self {Self {
key: Default::default(),
left: None,
right: None,
}}}
impl node {pub fn take(&mut self) -> Self {core::mem::take(self)}}

// A utility function to create a new BST node
pub unsafe extern "C" fn newNode(mut item: i32) -> Option<Box<node>> {
    let mut temp =
        Some(Box::new(<crate::src::bst::node as Default>::default()));
    (*temp.as_deref_mut().unwrap()).key= item;
    (*temp.as_deref_mut().unwrap()).right= None;
    (*temp.as_deref_mut().unwrap()).left= None;
    // (*temp).left = (*temp).right;
    return temp;
}
// A utility function to do inorder traversal of BST
pub unsafe extern "C" fn inorder(mut root: *const node) {
    if !root.is_null() {
        inorder((*root).left.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
        printf(b"%d \x00" as *const u8 as *const libc::c_char, (*root).key);
        inorder((*root).right.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
    }else { (); };
}
/* A utility function to
insert a new node with given key in
* BST */
pub unsafe extern "C" fn insert(mut node: Option<Box<node>>, mut key: i32)
 -> Option<Box<node>> {
    /* If the tree is empty, return a new node */
    if node.as_deref().is_none() {(); return newNode(key) }
    /* Otherwise, recur down the tree */
    if key < (*node.as_deref().unwrap()).key {
        (*node.as_deref_mut().unwrap()).left= insert((*node.as_deref_mut().unwrap()).left.take(), key)
    } else { (*node.as_deref_mut().unwrap()).right= insert((*node.as_deref_mut().unwrap()).right.take(), key) }
    /* return the (unchanged) node pointer */
    return node;
}
/* Given a non-empty binary search
tree, return the node
with minimum key value found in
that tree. Note that the
entire tree does not need to be searched. */
pub unsafe extern "C" fn minValueNode(mut node: *const node) -> *const node {
    let mut current = node;
    /* loop down to find the leftmost leaf */
    while !current.is_null() && !(*current).left.as_deref().is_none() {
        current= (*current).left.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null())
    }
    return current;
}
/* Given a binary search tree
and a key, this function
deletes the key and
returns the new root */
pub unsafe extern "C" fn deleteNode(mut root: Option<Box<node>>, mut key: i32)
 -> Option<Box<node>> {
    // base case
    if root.as_deref().is_none() {(); return root }
    // If the key to be deleted
	// is smaller than the root's
	// key, then it lies in left subtree
    if key < (*root.as_deref().unwrap()).key {
        (*root.as_deref_mut().unwrap()).left= deleteNode((*root.as_deref_mut().unwrap()).left.take(), key)
    } else if key > (*root.as_deref().unwrap()).key {
        (*root.as_deref_mut().unwrap()).right= deleteNode((*root.as_deref_mut().unwrap()).right.take(), key)
    } else {
        // If the key to be deleted
	// is greater than the root's
	// key, then it lies in right subtree
        // if key is same as root's key,
	// then This is the node
	// to be deleted
        // node with only one child or no child
        if (*root.as_deref().unwrap()).left.as_deref().is_none() {();
            let mut temp = (*root.as_deref_mut().unwrap()).right.take();
            ();
            return temp
        } else {
            if (*root.as_deref().unwrap()).right.as_deref().is_none() {();
                let mut temp_0 = (*root.as_deref_mut().unwrap()).left.take();
                ();
                return temp_0
            }
        }
        let mut temp_1 = minValueNode((*root.as_deref().unwrap()).right.as_deref().map(|r| r as *const _).unwrap_or(std::ptr::null()));
        (*root.as_deref_mut().unwrap()).key= (*temp_1).key;
        (*root.as_deref_mut().unwrap()).right= deleteNode((*root.as_deref_mut().unwrap()).right.take(), (*temp_1).key)
    }
    return root;
}


/* 
// node with two children:

// Get the inorder successor

// (smallest in the right subtree)

// Copy the inorder

// successor's content to this node

// Delete the inorder successor

// Driver Code
unsafe fn main() {
    /* Let us create following BST
			50
		/	 \
		30	 70
		/ \ / \
	20 40 60 80 */
    let mut root: *mut node = 0 as *mut node;
    root = insert(root, 50 as i32);
    root = insert(root, 30 as i32);
    root = insert(root, 20 as i32);
    root = insert(root, 40 as i32);
    root = insert(root, 70 as i32);
    root = insert(root, 60 as i32);
    root = insert(root, 80 as i32);
    printf(b"Inorder traversal of the given tree \n\x00" as *const u8 as
               *const libc::c_char);
    inorder(root);
    printf(b"\nDelete 20\n\x00" as *const u8 as *const libc::c_char);
    root = deleteNode(root, 20 as i32);
    printf(b"Inorder traversal of the modified tree \n\x00" as *const u8 as
               *const libc::c_char);
    inorder(root);
    printf(b"\nDelete 30\n\x00" as *const u8 as *const libc::c_char);
    root = deleteNode(root, 30 as i32);
    printf(b"Inorder traversal of the modified tree \n\x00" as *const u8 as
               *const libc::c_char);
    inorder(root);
    printf(b"\nDelete 50\n\x00" as *const u8 as *const libc::c_char);
    root = deleteNode(root, 50 as i32);
    printf(b"Inorder traversal of the modified tree \n\x00" as *const u8 as
               *const libc::c_char);
    inorder(root);
    return 0 as i32;
}
*/