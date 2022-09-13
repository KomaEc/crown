use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
// C program to delete a node from AVL Tree
// An AVL tree node

#[repr(C)]#[derive(Copy, Clone)]
pub struct Node {
    pub key: i32,
    pub left: *mut Node,
    pub right: *mut Node,
    pub height: i32,
}
// A utility function to get height of the tree
pub unsafe extern "C" fn height(mut N: *mut Node) -> i32 {
    if N.is_null() { return 0 as i32 }
    return (*N).height;
}
// A utility function to get maximum of two integers
// A utility function to get maximum of two integers
pub unsafe extern "C" fn max(mut a: i32, mut b: i32)
 -> i32 {
    return if a > b { a } else { b };
}
/* Helper function that allocates a new node with the given key and
    NULL left and right pointers. */
pub unsafe extern "C" fn newNode(mut key: i32) -> *mut Node {
    let mut node: *mut Node =
        malloc(::std::mem::size_of::<Node>() as u64) as
            *mut Node; // new node is initially added at leaf
    (*node).key = key;
    (*node).left = 0 as *mut Node;
    (*node).right = 0 as *mut Node;
    (*node).height = 1 as i32;
    return node;
}


// A utility function to right rotate subtree rooted with y
// See the diagram given above.
pub unsafe extern "C" fn rightRotate(mut y: *mut Node) -> *mut Node {
    let mut x: *mut Node = (*y).left;
    let mut T2: *mut Node = (*x).right;
    // Perform rotation
    (*x).right = y;
    (*y).left = T2;
    // Update heights
    (*y).height =
        max(height((*y).left), height((*y).right)) + 1 as i32;
    (*x).height =
        max(height((*x).left), height((*x).right)) + 1 as i32;
    // Return new root
    return x;


    /*
    let x = (*y).left;
    let T2 = (*x).right;
    (*x).right = y;
    // the semantics of this line is that
    // the ownership of y is moved to (*x).right
    // after this line, y should really be an alias
    // do a substitution [y |-> (*x).right]
    (*y).left = T2;
    (*y).height = use(&*y);
    (*x).height = use(&*x);
     */


     /*
     p = q ---> p is an alias of q
                => p = q.as_ref()
                => &*q     (search space prune by lifetime constraints)
                => &mut *q
           ---> p takes over q
                => p = q.take()
                => substituion[q |-> p]
      */
}
// A utility function to left rotate subtree rooted with x
// See the diagram given above.
pub unsafe extern "C" fn leftRotate(mut x: *mut Node) -> *mut Node {
    let mut y: *mut Node = (*x).right;
    let mut T2: *mut Node = (*y).left;
    // Perform rotation
    (*y).left = x;
    (*x).right = T2;
    //  Update heights
    (*x).height =
        max(height((*x).left), height((*x).right)) + 1 as i32;
    (*y).height =
        max(height((*y).left), height((*y).right)) + 1 as i32;
    // Return new root
    return y;
}
// Get Balance factor of node N
pub unsafe extern "C" fn getBalance(mut N: *mut Node) -> i32 {
    if N.is_null() { return 0 as i32 }
    return height((*N).left) - height((*N).right);
}


pub unsafe extern "C" fn insert(mut node: *mut Node, mut key: i32)
 -> *mut Node {
    /* 1.  Perform the normal BST rotation */
    if node.is_null() { return newNode(key) }
    if key < (*node).key {
        (*node).left = insert((*node).left, key)
    } else if key > (*node).key {
        (*node).right = insert((*node).right, key)
    } else {
        // Equal keys not allowed
        return node
    }
    /* 2. Update height of this ancestor node */
    (*node).height =
        1 as i32 + max(height((*node).left), height((*node).right));
    /* 3. Get the balance factor of this ancestor
          node to check whether this node became
          unbalanced */
    let mut balance: i32 = getBalance(node);
    // If this node becomes unbalanced, then there are 4 cases
    // Left Left Case
    if balance > 1 as i32 && key < (*(*node).left).key {
        return rightRotate(node)
    }
    // Right Right Case
    if balance < -(1 as i32) && key > (*(*node).right).key {
        return leftRotate(node)
    }
    // Left Right Case
    if balance > 1 as i32 && key > (*(*node).left).key {
        (*node).left = leftRotate((*node).left);
        return rightRotate(node)
    }
    // Right Left Case
    if balance < -(1 as i32) && key < (*(*node).right).key {
        (*node).right = rightRotate((*node).right);
        return leftRotate(node)
    }
    /* return the (unchanged) node pointer */
    return node;
}
/* Given a non-empty binary search tree, return the
   node with minimum key value found in that tree.
   Note that the entire tree does not need to be
   searched. */
pub unsafe extern "C" fn minValueNode(mut node: *mut Node) -> *mut Node {
    let mut current: *mut Node = node;
    /* loop down to find the leftmost leaf */
    while !(*current).left.is_null() { current = (*current).left }
    return current;
}
// Recursive function to delete a node with given key
// from subtree with given root. It returns root of
// the modified subtree.
pub unsafe extern "C" fn deleteNode(mut root: *mut Node, mut key: i32)
 -> *mut Node {
    // STEP 1: PERFORM STANDARD BST DELETE
    if root.is_null() { return root }
    // If the key to be deleted is smaller than the
    // root's key, then it lies in left subtree
    if key < (*root).key {
        (*root).left = deleteNode((*root).left, key)
    } else if key > (*root).key {
        (*root).right = deleteNode((*root).right, key)
    } else if (*root).left.is_null() || (*root).right.is_null() {
        let mut temp: *mut Node =
            if !(*root).left.is_null() {
                (*root).left
            } else { (*root).right };
        // If the key to be deleted is greater than the
    // root's key, then it lies in right subtree
        // if key is same as root's key, then This is
    // the node to be deleted
        // node with only one child or no child
        // No child case
        if temp.is_null() {
            temp =
                root; // Copy the contents of
                            // the non-empty child
            root = 0 as *mut Node
        } else {
            // One child case
            *root = *temp
        }
        free(temp as *mut libc::c_void);
    } else {
        // node with two children: Get the inorder
            // successor (smallest in the right subtree)
        let mut temp_0: *mut Node = minValueNode((*root).right);
        // Copy the inorder successor's data to this node
        (*root).key = (*temp_0).key;
        // Delete the inorder successor
        (*root).right = deleteNode((*root).right, (*temp_0).key)
    }
    // If the tree had only one node then return
    if root.is_null() { return root }
    // STEP 2: UPDATE HEIGHT OF THE CURRENT NODE
    (*root).height =
        1 as i32 + max(height((*root).left), height((*root).right));
    // STEP 3: GET THE BALANCE FACTOR OF THIS NODE (to
    // check whether this node became unbalanced)
    let mut balance: i32 = getBalance(root);
    // If this node becomes unbalanced, then there are 4 cases
    // Left Left Case
    if balance > 1 as i32 &&
           getBalance((*root).left) >= 0 as i32 {
        return rightRotate(root)
    }
    // Left Right Case
    if balance > 1 as i32 &&
           getBalance((*root).left) < 0 as i32 {
        (*root).left = leftRotate((*root).left);
        return rightRotate(root)
    }
    // Right Right Case
    if balance < -(1 as i32) &&
           getBalance((*root).right) <= 0 as i32 {
        return leftRotate(root)
    }
    // Right Left Case
    if balance < -(1 as i32) &&
           getBalance((*root).right) > 0 as i32 {
        (*root).right = rightRotate((*root).right);
        return leftRotate(root)
    }
    return root;
}
// A utility function to print preorder traversal of
// the tree.
// The function also prints height of every node
pub unsafe extern "C" fn preOrder(mut root: *mut Node) {
    if !root.is_null() {
        printf(b"%d \x00" as *const u8 as *const libc::c_char, (*root).key);
        preOrder((*root).left);
        preOrder((*root).right);
    };
}

/* 
/* Driver program to test above function*/
unsafe fn main_0() -> i32  {
    let mut root: *mut Node = 0 as *mut Node;
    /* Constructing tree given in the above figure */
    root = insert(root, 9 as i32);
    root = insert(root, 5 as i32);
    root = insert(root, 10 as i32);
    root = insert(root, 0 as i32);
    root = insert(root, 6 as i32);
    root = insert(root, 11 as i32);
    root = insert(root, -(1 as i32));
    root = insert(root, 1 as i32);
    root = insert(root, 2 as i32);
    /* The constructed AVL Tree would be
            9
           /  \
          1    10
        /  \     \
       0    5     11
      /    /  \
     -1   2    6
    */
    printf(b"Preorder traversal of the constructed AVL tree is \n\x00" as
               *const u8 as *const libc::c_char);
    preOrder(root);
    root = deleteNode(root, 10 as i32);
    /* The AVL Tree after deletion of 10
            1
           /  \
          0    9
        /     /  \
       -1    5     11
           /  \
          2    6
    */
    printf(b"\nPreorder traversal after deletion of 10 \n\x00" as *const u8 as
               *const libc::c_char);
    preOrder(root);
    return 0 as i32;
}
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
*/