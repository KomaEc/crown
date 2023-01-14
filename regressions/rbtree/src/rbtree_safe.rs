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
#[repr(C)]
pub struct s_rbnode {
    pub key: t_key,
    pub value: t_value,
    pub color: t_rbcolor,
    pub left: Option<Box<s_rbnode>>,
    pub right: Option<Box<s_rbnode>>,
}
impl Default for s_rbnode {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: Default::default(),
            color: Default::default(),
            left: None,
            right: None,
        }
    }
}

pub type t_rbnode = s_rbnode;
#[no_mangle]
pub static mut root_rbtree: *mut t_rbnode = 0 as *const t_rbnode as *mut t_rbnode;
#[inline]
fn is_red(mut node: Option<&t_rbnode>) -> libc::c_int {
    return if !node.clone().is_none() {
        ((*node.as_deref().unwrap()).color as libc::c_uint == RED as libc::c_int as libc::c_uint)
            as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
fn my_compare(mut key1: libc::c_uint, mut key2: libc::c_uint) -> libc::c_int {
    return if key1 == key2 {
        0 as libc::c_int
    } else if key1 < key2 {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
}
fn flip_color(mut node: *mut t_rbnode) {
  unsafe {
    (*node).color = ((*node).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).left.as_deref_mut().unwrap()).color =
        ((*(*node).left.as_deref_mut().unwrap()).color as u64 == 0) as libc::c_int as t_rbcolor;
    (*(*node).right.as_deref_mut().unwrap()).color =
        ((*(*node).right.as_deref_mut().unwrap()).color as u64 == 0) as libc::c_int as t_rbcolor;
  }
}
fn rotate_left(mut left: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    let mut right = None;
    if left.as_deref().is_none() {
        return None;
    }
    right = (*left.as_deref_mut().unwrap()).right.take();
    // let ref mut fresh0 = (*left).right;
    // *fresh0 = (*right).left;
    (*left.as_deref_mut().unwrap()).right = (*right.as_deref_mut().unwrap()).left.take();
    // let ref mut fresh1 = (*right).left;
    // *fresh1 = left;
    (*right.as_deref_mut().unwrap()).color = (*left.as_deref().unwrap()).color;
    (*left.as_deref_mut().unwrap()).color = RED;
    (*right.as_deref_mut().unwrap()).left = left;
    return right;
}
fn rotate_right(mut right: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    let mut left = None;
    if right.as_deref().is_none() {
        return None;
    }
    left = (*right.as_deref_mut().unwrap()).left.take();
    // let ref mut fresh2 = (*right).left;
    // *fresh2 = (*left).right;
    (*right.as_deref_mut().unwrap()).left = (*left.as_deref_mut().unwrap()).right.take();
    // let ref mut fresh3 = (*left).right;
    // *fresh3 = right;
    (*left.as_deref_mut().unwrap()).color = (*right.as_deref().unwrap()).color;
    (*right.as_deref_mut().unwrap()).color = RED;
    (*left.as_deref_mut().unwrap()).right = right;
    return left;
}
fn create_node(mut key: t_key, mut value: t_value) -> Option<Box<t_rbnode>> {
    let mut new = None;
    new = Some(Box::new(<s_rbnode as Default>::default()));
    if new.as_deref().is_none() {
        return None;
    }
    (*new.as_deref_mut().unwrap()).key = key;
    (*new.as_deref_mut().unwrap()).value = value;
    (*new.as_deref_mut().unwrap()).color = RED;
    // let ref mut fresh4 = (*new).left;
    // *fresh4 = 0 as *mut t_rbnode;
    (*new.as_deref_mut().unwrap()).left = None;
    // let ref mut fresh5 = (*new).right;
    // *fresh5 = 0 as *mut t_rbnode;
    (*new.as_deref_mut().unwrap()).right = None;
    return new;
}
fn insert_this(
    mut node: Option<Box<t_rbnode>>,
    mut key: t_key,
    mut value: t_value,
) -> Option<Box<t_rbnode>> {
    let mut res: libc::c_int = 0;
    if node.as_deref().is_none() {
        return create_node(key, value);
    }
    res = my_compare(key, (*node.as_deref().unwrap()).key);
    if res == 0 as libc::c_int {
        (*node.as_deref_mut().unwrap()).value = value;
    } else if res < 0 as libc::c_int {
        // let ref mut fresh6 = (*node).left;
        // *fresh6 = insert_this((*node).left, key, value);
        (*node.as_deref_mut().unwrap()).left =
            insert_this((*node.as_deref_mut().unwrap()).left.take(), key, value);
    } else {
        // let ref mut fresh7 = (*node).right;
        // *fresh7 = insert_this((*node).right, key, value);
        (*node.as_deref_mut().unwrap()).right =
            insert_this((*node.as_deref_mut().unwrap()).right.take(), key, value);
    }
    if is_red((*node.as_deref().unwrap()).right.as_deref()) != 0
        && is_red((*node.as_deref().unwrap()).left.as_deref()) == 0
    {
        node = rotate_left(node);
    }
    if is_red((*node.as_deref().unwrap()).left.as_deref()) != 0
        && is_red(
            (*(*node.as_deref().unwrap()).left.as_deref().unwrap())
                .left
                .as_deref(),
        ) != 0
    {
        node = rotate_right(node);
    }
    if is_red((*node.as_deref().unwrap()).left.as_deref()) != 0
        && is_red((*node.as_deref().unwrap()).right.as_deref()) != 0
    {
        flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    }
    return node;
}
// #[no_mangle]
// pub unsafe extern "C" fn insert(mut key: t_key, mut value: t_value) {
//     root_rbtree = insert_this(Some(Box::from_raw(*todo_static_addr)), key, value);
//     if !(*todo_static_addr.clone().unwrap()).clone().is_none() {
//         (*(*todo_static_addr)).color= BLACK;
//     }else {  }
// }
fn get_key(mut node: Option<&t_rbnode>, mut key: t_key) -> t_value {
    let mut cmp: libc::c_int = 0;
    while !node.clone().is_none() {
        cmp = my_compare(key, (*node.clone().unwrap()).key);
        if cmp == 0 {
            return (*node.clone().unwrap()).value;
        }
        node = if cmp < 0 as libc::c_int {
            (*node.clone().unwrap()).left.as_deref()
        } else {
            (*node.clone().unwrap()).right.as_deref()
        };
    }
    return 0 as libc::c_int as t_value;
}
fn min(mut node: Option<&t_rbnode>) -> Option<&t_rbnode> {
    if node.clone().is_none() {
        return None;
    }
    while !(*node.clone().unwrap()).left.as_deref().is_none() {
        node = (*node.clone().unwrap()).left.as_deref();
    }
    return node.clone();
}
#[inline]
fn balance_me_that(mut node: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    if is_red((*node.as_deref().unwrap()).right.as_deref()) != 0 {
        node = rotate_left(node);
    }
    if is_red((*node.as_deref().unwrap()).left.as_deref()) != 0
        && is_red(
            (*(*node.as_deref().unwrap()).left.as_deref().unwrap())
                .left
                .as_deref(),
        ) != 0
    {
        node = rotate_right(node);
    }
    if is_red((*node.as_deref().unwrap()).left.as_deref()) != 0
        && is_red((*node.as_deref().unwrap()).right.as_deref()) != 0
    {
        flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    }
    return node;
}
fn move_red_to_left(mut node: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    if !node.as_deref().is_none()
        && !(*node.as_deref().unwrap()).right.as_deref().is_none()
        && is_red(
            (*(*node.as_deref().unwrap()).right.as_deref().unwrap())
                .left
                .as_deref(),
        ) != 0
    {
        // let ref mut fresh8 = (*node).right;
        // *fresh8 = rotate_right((*node).right);
        (*node.as_deref_mut().unwrap()).right =
            rotate_right((*node.as_deref_mut().unwrap()).right.take());
        node = rotate_left(node);
        flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    }
    return node;
}
fn move_red_to_right(mut node: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    if !node.as_deref().is_none()
        && !(*node.as_deref().unwrap()).left.as_deref().is_none()
        && is_red(
            (*(*node.as_deref().unwrap()).left.as_deref().unwrap())
                .left
                .as_deref(),
        ) != 0
    {
        node = rotate_right(node);
        flip_color(unsafe { core::mem::transmute(node.as_deref_mut()) });
    }
    return node;
}
fn remove_min(mut node: Option<Box<t_rbnode>>) -> Option<Box<t_rbnode>> {
    if node.as_deref().is_none() {
        return None;
    }
    if (*node.as_deref().unwrap()).left.as_deref().is_none() {
        // free(node as *mut libc::c_void);
        let _ = node;
        return None;
    }
    if is_red((*node.as_deref().unwrap()).left.as_deref()) == 0
        && is_red(
            (*(*node.as_deref().unwrap()).left.as_deref().unwrap())
                .left
                .as_deref(),
        ) == 0
    {
        node = move_red_to_left(node);
    }
    // let ref mut fresh9 = (*node).left;
    // *fresh9 = remove_min((*node).left);
    (*node.as_deref_mut().unwrap()).left = remove_min((*node.as_deref_mut().unwrap()).left.take());
    return balance_me_that(node);
}
fn remove_it(
    mut node: Option<Box<t_rbnode>>,
    mut key: t_key,
) -> Option<Box<t_rbnode>> {
    let mut tmp = None;
    if node.as_deref().is_none() {
        return None;
    }
    if my_compare(key, (*node.as_deref().unwrap()).key) == -(1 as libc::c_int) {
        if !(*node.as_deref().unwrap()).left.as_deref().is_none() {
            if is_red((*node.as_deref().unwrap()).left.as_deref()) == 0
                && is_red(
                    (*(*node.as_deref().unwrap()).left.as_deref().unwrap())
                        .left
                        .as_deref(),
                ) == 0
            {
                node = move_red_to_left(node);
            }
            // let ref mut fresh10 = (*node).left;
            // *fresh10 = remove_key((*node).left, key);
            (*node.as_deref_mut().unwrap()).left =
                remove_key((*node.as_deref_mut().unwrap()).left.take(), key);
        } else {
        }
    } else {
        if is_red((*node.as_deref().unwrap()).left.as_deref()) != 0 {
            node = rotate_right(node);
        }
        if my_compare(key, (*node.as_deref().unwrap()).key) == 0
            && (*node.as_deref().unwrap()).right.as_deref().is_none()
        {
            let _ = node;
            // free(node as *mut libc::c_void);
            return None;
        }
        if !(*node.as_deref().unwrap()).right.as_deref().is_none() {
            if is_red((*node.as_deref().unwrap()).right.as_deref()) == 0
                && is_red(
                    (*(*node.as_deref().unwrap()).right.as_deref().unwrap())
                        .left
                        .as_deref(),
                ) == 0
            {
                node = move_red_to_right(node);
            }
            if my_compare(key, (*node.as_deref().unwrap()).key) == 0 {
                tmp = min((*node.as_deref().unwrap()).right.as_deref());
                let key = (*tmp.clone().unwrap()).key;
                let value = (*tmp.clone().unwrap()).value;
                (*node.as_deref_mut().unwrap()).key = key;
                (*node.as_deref_mut().unwrap()).value = value;
                // let ref mut fresh11 = (*node).right;
                // *fresh11 = remove_min((*node).right);
                (*node.as_deref_mut().unwrap()).right =
                    remove_min((*node.as_deref_mut().unwrap()).right.take());
            } else {
                // let ref mut fresh12 = (*node).right;
                // *fresh12 = remove_key((*node).right, key);
                (*node.as_deref_mut().unwrap()).right =
                    remove_key((*node.as_deref_mut().unwrap()).right.take(), key);
            }
        } else {
        }
    }
    return balance_me_that(node);
}
pub fn remove_key(
    mut node: Option<Box<t_rbnode>>,
    mut key: t_key,
) -> Option<Box<t_rbnode>> {
    node = remove_it(node, key);
    if !node.as_deref().is_none() {
        (*node.as_deref_mut().unwrap()).color = BLACK;
    } else {
    }
    return node;
}
pub fn erase_tree(mut node: Option<Box<t_rbnode>>) -> Option<&'static t_rbnode> {
    if !node.as_deref().is_none() {
        if !(*node.as_deref().unwrap()).left.as_deref().is_none() {
            erase_tree((*node.as_deref_mut().unwrap()).left.take());
        } else {
        }
        if !(*node.as_deref().unwrap()).right.as_deref().is_none() {
            erase_tree((*node.as_deref_mut().unwrap()).right.take());
        } else {
        }
        // let ref mut fresh13 = (*node).left;
        // *fresh13 = 0 as *mut t_rbnode;
        (*node.as_deref_mut().unwrap()).left = None;
        // let ref mut fresh14 = (*node).right;
        // *fresh14 = 0 as *mut t_rbnode;
        (*node.as_deref_mut().unwrap()).right = None;
        let _ = node;
        // free(node as *mut libc::c_void);
    } else {
    }
    return None;
}
