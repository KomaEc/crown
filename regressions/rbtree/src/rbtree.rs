use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type rbcolor = libc::c_uint;
pub const RED: rbcolor = 1;
pub const BLACK: rbcolor = 0;
pub type t_rbcolor = rbcolor;
pub type t_key = libc::c_uint;
pub type t_value = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct s_rbnode {
    pub key: t_key,
    pub value: t_value,
    pub color: t_rbcolor,
    pub left: *mut /* owning */ s_rbnode,
    pub right: *mut s_rbnode,
}
impl Default for s_rbnode {fn default() -> Self {Self {
key: Default::default(),
value: Default::default(),
color: Default::default(),
left: std::ptr::null_mut(),
right: std::ptr::null_mut(),
}}}
impl s_rbnode {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type t_rbnode = s_rbnode;
#[no_mangle]
pub static mut root_rbtree: *mut t_rbnode = 0 as *const t_rbnode as *mut t_rbnode;
#[inline]
unsafe extern "C" fn is_red(mut node: *const t_rbnode) -> libc::c_int {
    return if !node.is_null() {
        ((*node).color as libc::c_uint == RED as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        ();
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn my_compare(mut key1: libc::c_uint, mut key2: libc::c_uint) -> libc::c_int {
    return if key1 == key2 {
        0 as libc::c_int
    } else if key1 < key2 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn flip_color(mut node: *mut t_rbnode) {
    (*node).color= ((*node).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).left).color= ((*(*node).left).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).right).color= ((*(*node).right).color as u64 == 0) as libc::c_int as t_rbcolor;
}
unsafe extern "C" fn rotate_left(mut left: *mut t_rbnode) -> *mut t_rbnode {
    let mut right = 0 as *mut t_rbnode;
    if left.is_null() {
        ();
        return 0 as *mut t_rbnode;
    }
    right= (*left).right;
    // let ref mut fresh0 = (*left).right;
    // *fresh0 = (*right).left;
    (*left).right= (*right).left;
    // let ref mut fresh1 = (*right).left;
    // *fresh1 = left;
    (*right).left= left;
    (*right).color= (*left).color;
    (*left).color= RED;
    return right;
}
unsafe extern "C" fn rotate_right(mut right: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    let mut left = None;
    if right.as_deref().is_none() {
        ();
        return None;
    }
    left= Some(Box::from_raw((*right.as_deref_mut().unwrap()).left));
    // let ref mut fresh2 = (*right).left;
    // *fresh2 = (*left).right;
    (*right.as_deref_mut().unwrap()).left= (*left.as_deref().unwrap()).right;
    // let ref mut fresh3 = (*left).right;
    // *fresh3 = right;
    (*left.as_deref_mut().unwrap()).right= core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(right.as_deref_mut());
    (*left.as_deref_mut().unwrap()).color= (*right.as_deref().unwrap()).color;
    (*right.as_deref_mut().unwrap()).color= RED;
    return left;
}
#[no_mangle]
pub unsafe extern "C" fn create_node(mut key: t_key, mut value: t_value) -> Option<Box<t_rbnode>> {
    let mut new = None;
    new= Some(Box::new(<crate::src::rbtree::s_rbnode as Default>::default()));
    if new.as_deref().is_none() {
        ();
        return None;
    }
    (*new.as_deref_mut().unwrap()).key= key;
    (*new.as_deref_mut().unwrap()).value= value;
    (*new.as_deref_mut().unwrap()).color= RED;
    // let ref mut fresh4 = (*new).left;
    // *fresh4 = 0 as *mut t_rbnode;
    (*new.as_deref_mut().unwrap()).left= 0 as *mut t_rbnode;
    // let ref mut fresh5 = (*new).right;
    // *fresh5 = 0 as *mut t_rbnode;
    (*new.as_deref_mut().unwrap()).right= 0 as *mut t_rbnode;
    return new;
}
unsafe extern "C" fn insert_this(
    mut node: *mut t_rbnode,
    mut key: t_key,
    mut value: t_value,
) -> *mut t_rbnode {
    let mut res: libc::c_int = 0;
    if node.is_null() {
        ();
        return create_node(key, value);
    }
    res= my_compare(key, (*node).key);
    if res == 0 as libc::c_int {
        (*node).value= value;
    } else if res < 0 as libc::c_int {
        // let ref mut fresh6 = (*node).left;
        // *fresh6 = insert_this((*node).left, key, value);
        (*node).left= insert_this((*node).left, key, value);
    } else {
        // let ref mut fresh7 = (*node).right;
        // *fresh7 = insert_this((*node).right, key, value);
        (*node).right= insert_this((*node).right, key, value);
    }
    if is_red((*node).right) != 0 && is_red((*node).left as *const crate::src::rbtree::s_rbnode) == 0 {
        node= rotate_left(node);
    }
    if is_red((*node).left as *const crate::src::rbtree::s_rbnode) != 0 && is_red((*(*node).left).left as *const crate::src::rbtree::s_rbnode) != 0 {
        node= rotate_right(Some(Box::from_raw(node)));
    }
    if is_red((*node).left as *const crate::src::rbtree::s_rbnode) != 0 && is_red((*node).right) != 0 {
        flip_color(node);
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn insert(mut key: t_key, mut value: t_value) {
    root_rbtree = insert_this(root_rbtree, key, value);
    if !root_rbtree.is_null() {
        (*root_rbtree).color = BLACK;
    } else {
        ();
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_key(mut node: *const t_rbnode, mut key: t_key) -> t_value {
    let mut cmp: libc::c_int = 0;
    while !node.is_null() {
        cmp= my_compare(key, (*node).key);
        if cmp == 0 {
            return (*node).value;
        }
        node= if cmp < 0 as libc::c_int {
            (*node).left as *const crate::src::rbtree::s_rbnode
        } else {
            (*node).right
        };
    }
    ();
    return 0 as libc::c_int as t_value;
}
unsafe extern "C" fn min(mut node: *const t_rbnode) -> *const t_rbnode {
    if node.is_null() {
        ();
        return 0 as *mut t_rbnode;
    }
    while !(*node).left.is_null() {
        node= (*node).left as *const crate::src::rbtree::s_rbnode;
    }
    ();
    return node;
}
#[inline]
unsafe extern "C" fn balance_me_that(mut node: *mut t_rbnode) -> *mut t_rbnode {
    if is_red((*node).right) != 0 {
        node= rotate_left(node);
    }
    if is_red((*node).left as *const crate::src::rbtree::s_rbnode) != 0 && is_red((*(*node).left).left as *const crate::src::rbtree::s_rbnode) != 0 {
        node= rotate_right(Some(Box::from_raw(node)));
    }
    if is_red((*node).left as *const crate::src::rbtree::s_rbnode) != 0 && is_red((*node).right) != 0 {
        flip_color(node);
    }
    return node;
}
unsafe extern "C" fn move_red_to_left(mut node: *mut t_rbnode) -> *mut t_rbnode {
    flip_color(node);
    if !node.is_null() && !(*node).right.is_null() && is_red((*(*node).right).left as *const crate::src::rbtree::s_rbnode) != 0 {
        // let ref mut fresh8 = (*node).right;
        // *fresh8 = rotate_right((*node).right);
        (*node).right= rotate_right(Some(Box::from_raw((*node).right)));
        node= rotate_left(node);
        flip_color(node);
    }
    return node;
}
unsafe extern "C" fn move_red_to_right(mut node: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    flip_color(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
    if !node.as_deref().is_none() && !(*node.as_deref().unwrap()).left.is_null() && is_red((*(*node.as_deref().unwrap()).left).left as *const crate::src::rbtree::s_rbnode) != 0 {
        node= rotate_right(node);
        flip_color(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
    }
    return node;
}
unsafe extern "C" fn remove_min(mut node: Option<Box<t_rbnode>>) -> *mut t_rbnode {
    if node.as_deref().is_none() {
        ();
        return 0 as *mut t_rbnode;
    }
    if (*node.as_deref().unwrap()).left.is_null() {
        ();
        // std::intrinsics::assume((*node).right as usize == 0);
        ();
        return 0 as *mut t_rbnode;
    }
    if is_red((*node.as_deref().unwrap()).left as *const crate::src::rbtree::s_rbnode) == 0 && is_red((*(*node.as_deref().unwrap()).left).left as *const crate::src::rbtree::s_rbnode) == 0 {
        node= move_red_to_left(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
    }
    // let ref mut fresh9 = (*node).left;
    // *fresh9 = remove_min((*node).left);
    (*node.as_deref_mut().unwrap()).left= remove_min(Some(Box::from_raw((*node.as_deref_mut().unwrap()).left)));
    return balance_me_that(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
}
unsafe extern "C" fn remove_it(mut node: Option<Box<t_rbnode>>, mut key: t_key) -> *mut t_rbnode {
    let mut tmp = 0 as *mut t_rbnode;
    if node.as_deref().is_none() {
        ();
        return 0 as *mut t_rbnode;
    }
    if my_compare(key, (*node.as_deref().unwrap()).key) == -(1 as libc::c_int) {
        if !(*node.as_deref().unwrap()).left.is_null() {
            if is_red((*node.as_deref().unwrap()).left as *const crate::src::rbtree::s_rbnode) == 0 && is_red((*(*node.as_deref().unwrap()).left).left as *const crate::src::rbtree::s_rbnode) == 0 {
                node= move_red_to_left(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
            }
            // let ref mut fresh10 = (*node).left;
            // *fresh10 = remove_key((*node).left, key);
            (*node.as_deref_mut().unwrap()).left= remove_key((*node.as_deref().unwrap()).left, key);
        } else {
            ();
        }
    } else {
        if is_red((*node.as_deref().unwrap()).left as *const crate::src::rbtree::s_rbnode) != 0 {
            node= rotate_right(node);
        }
        if my_compare(key, (*node.as_deref().unwrap()).key) == 0 && (*node.as_deref().unwrap()).right.is_null() {
            // std::intrinsics::assume((*node).left as usize == 0);
            // std::intrinsics::assume((*node).right as usize == 0);
            ();
            return 0 as *mut t_rbnode;
        }
        if !(*node.as_deref().unwrap()).right.is_null() {
            if is_red((*node.as_deref().unwrap()).right) == 0 && is_red((*(*node.as_deref().unwrap()).right).left as *const crate::src::rbtree::s_rbnode) == 0 {
                node= move_red_to_right(node);
            }
            if my_compare(key, (*node.as_deref().unwrap()).key) == 0 {
                tmp= min((*node.as_deref().unwrap()).right);
                (*node.as_deref_mut().unwrap()).key= (*tmp).key;
                (*node.as_deref_mut().unwrap()).value= (*tmp).value;
                // let ref mut fresh11 = (*node).right;
                // *fresh11 = remove_min((*node).right);
                (*node.as_deref_mut().unwrap()).right= remove_min(Some(Box::from_raw((*node.as_deref().unwrap()).right)));
            } else {
                // let ref mut fresh12 = (*node).right;
                // *fresh12 = remove_key((*node).right, key);
                (*node.as_deref_mut().unwrap()).right= remove_key((*node.as_deref().unwrap()).right, key);
            }
        } else {
            ();
        }
    }
    return balance_me_that(core::mem::transmute::<_, *mut crate::src::rbtree::s_rbnode>(node.as_deref_mut()));
}
#[no_mangle]
pub unsafe extern "C" fn remove_key(mut node: *mut t_rbnode, mut key: t_key) -> *mut t_rbnode {
    node= remove_it(Some(Box::from_raw(node)), key);
    if !node.is_null() {
        (*node).color= BLACK;
    } else {
        ();
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn erase_tree(mut node: Option<Box<t_rbnode>>) -> *const t_rbnode {
    if !node.as_deref().is_none() {
        if !(*node.as_deref().unwrap()).left.is_null() {
            erase_tree(Some(Box::from_raw((*node.as_deref_mut().unwrap()).left)));
        } else {
            ();
        }
        if !(*node.as_deref().unwrap()).right.is_null() {
            erase_tree(Some(Box::from_raw((*node.as_deref().unwrap()).right)));
        } else {
            ();
        }
        // let ref mut fresh13 = (*node).left;
        // *fresh13 = 0 as *mut t_rbnode;
        (*node.as_deref_mut().unwrap()).left= 0 as *mut t_rbnode;
        // let ref mut fresh14 = (*node).right;
        // *fresh14 = 0 as *mut t_rbnode;
        (*node.as_deref_mut().unwrap()).right= 0 as *mut t_rbnode;
        ();
    } else {
        ();
    }
    return 0 as *mut t_rbnode;
}
