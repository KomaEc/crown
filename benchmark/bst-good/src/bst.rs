use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
// C program to demonstrate
// delete operation in binary
// search tree

#[repr(C)]#[derive(Clone)]
pub struct node {
    pub key: i32,
    pub left: *mut node,
    pub right: *mut node,
}
// A utility function to create a new BST node
pub unsafe extern "C" fn newNode(mut item: i32) -> *mut node {
    let mut temp =
        malloc(::std::mem::size_of::<node>() as u64) as *mut node;
    (*temp).key = item;
    (*temp).right = 0 as *mut node;
    (*temp).left = 0 as *mut node;//(*temp).right;
    return temp;
}
// A utility function to do inorder traversal of BST
pub unsafe extern "C" fn inorder(mut root: *mut node) {
    if !root.is_null() {
        inorder((*root).left);
        printf(b"%d \x00" as *const u8 as *const libc::c_char, (*root).key);
        inorder((*root).right);
    } else {
        root = 0 as *mut node;
    }
}

pub unsafe extern "C" fn insert(mut node: *mut node, mut key: i32)
 -> *mut node {
    /* If the tree is empty, return a new node */
    if node.is_null() { 
        node = 0 as *mut node;
    //     // free(node as *mut libc::c_void);
        return newNode(key) 
    }
    /* Otherwise, recur down the tree */
    if key < (*node).key {
        (*node).left = insert((*node).left, key)
    } else { (*node).right = insert((*node).right, key) }
    /* return the (unchanged) node pointer */
    return node;
}

/* Given a non-empty binary search
tree, return the node
with minimum key value found in
that tree. Note that the
entire tree does not need to be searched. */
pub unsafe extern "C" fn minValueNode(mut node: *mut node) -> *mut node {
    let mut current = node;
    /* loop down to find the leftmost leaf */
    while !current.is_null() && !(*current).left.is_null() {
        current = (*current).left
    }
    return current;
}


/* Given a binary search tree
and a key, this function
deletes the key and
returns the new root */
pub unsafe extern "C" fn deleteNode(mut root: *mut node, mut key: i32)
 -> *mut node {
    // base case
    if root.is_null() { return root }
    // If the key to be deleted
	// is smaller than the root's
	// key, then it lies in left subtree
    if key < (*root).key {
        (*root).left = deleteNode((*root).left, key)
    } else if key > (*root).key {
        (*root).right = deleteNode((*root).right, key)
    } else {
        // If the key to be deleted
	// is greater than the root's
	// key, then it lies in right subtree
        // if key is same as root's key,
	// then This is the node
	// to be deleted
        // node with only one child or no child
        if (*root).left.is_null() {
            let mut temp = (*root).right;
            free(root as *mut libc::c_void);
            return temp
        } else {
            if (*root).right.is_null() {
                let mut temp_0 = (*root).left;
                free(root as *mut libc::c_void);
                return temp_0
            }
        }
        let mut temp_1 = minValueNode((*root).right);
        (*root).key = (*temp_1).key;
        (*root).right = deleteNode((*root).right, (*root).key)
    }
    return root;
}

pub unsafe fn deleteFirst(mut head_ref: *mut *mut node)
{
    if!(*head_ref).is_null()
    {
       // store the old value of pointer to head pointer
       let mut temp = *head_ref;
 
       // Change head pointer to point to next node
       *head_ref = (**head_ref).left;
 
       // delete memory allocated for the previous head node
       free(temp as *mut libc::c_void);
    }
}

unsafe fn f(mut node: *mut node) {
    let mut temp = node;
    free(temp as *mut libc::c_void);
}


/* A utility function to
insert a new node with given key in
* BST */

/* 
pub unsafe extern "C" fn insert(mut node: *mut node, mut new: *mut node)
 -> *mut node {
    /* If the tree is empty, return a new node */
    if node.is_null() { 
        node = 0 as *mut node;
        // free(node as *mut libc::c_void);
        return new// newNode(key) 
    }
    /* Otherwise, recur down the tree */
    if (*new).key < (*node).key {
        (*node).left = insert((*node).left, new)
    } else { (*node).right = insert((*node).right, new) }
    /* return the (unchanged) node pointer */
    return node;
}
*/

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