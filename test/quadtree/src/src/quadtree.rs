use ::libc;
extern "C" {
    
    
    
    
    
    
    
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]

struct ErasedByPreprocessor3 { dummy: () }
impl Default for ErasedByPreprocessor3 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_point_t = crate::src::src::bounds::quadtree_point;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor4 { dummy: () }
impl Default for ErasedByPreprocessor4 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_bounds_t = crate::src::src::bounds::quadtree_bounds;
#[derive(Copy, Clone)]

struct ErasedByPreprocessor5 { dummy: () }
impl Default for ErasedByPreprocessor5 {fn default() -> Self {Self {
dummy: Default::default(),
}}}

pub type quadtree_node_t = crate::src::src::node::quadtree_node;
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer2;
#[repr(C)]
pub struct quadtree {
    pub root: Option<Box<quadtree_node_t>>,
    pub key_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub length: libc::c_uint,
}
impl Default for quadtree {fn default() -> Self {Self {
root: None,
key_free: Default::default(),
length: Default::default(),
}}}
impl quadtree {pub fn take(&mut self) -> Self {core::mem::take(self)}}

pub type quadtree_t = quadtree;
unsafe extern "C" fn node_contains_(
    mut outer: *mut quadtree_node_t,
    mut it: *mut quadtree_point_t,
) -> libc::c_int {
    return (!(*outer).bounds.is_null() && (*(*(*outer).bounds).nw.as_deref().unwrap()).x < (*it).x
        && (*(*(*outer).bounds).nw.as_deref().unwrap()).y > (*it).y && (*(*(*outer).bounds).se.as_deref().unwrap()).x > (*it).x
        && (*(*(*outer).bounds).se.as_deref().unwrap()).y < (*it).y) as libc::c_int;
}
unsafe extern "C" fn elision_(mut key: *mut libc::c_void) {}
unsafe extern "C" fn reset_node_(
    mut tree: *mut quadtree_t,
    mut node: Option<&mut quadtree_node_t>,
) {
    if (*tree).key_free.is_some() {
        crate::src::src::node::quadtree_node_reset(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), (*tree).key_free);
    } else {
        crate::src::src::node::quadtree_node_reset(
            node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()),
            Some(elision_ as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    };
}
unsafe extern "C" fn get_quadrant_(
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
) -> *mut quadtree_node_t {
    if node_contains_((*root).nw, point) != 0 {
        return (*root).nw;
    }
    if node_contains_((*root).ne, point) != 0 {
        return (*root).ne;
    }
    if node_contains_((*root).sw, point) != 0 {
        return (*root).sw;
    }
    if node_contains_((*root).se, point) != 0 {
        return (*root).se;
    }
    return 0 as *mut quadtree_node_t;
}
unsafe extern "C" fn split_node_(
    mut tree: *mut quadtree_t,
    mut node: *mut quadtree_node_t,
) -> libc::c_int {
    let mut nw = 0 as *mut quadtree_node_t;
    let mut ne = 0 as *mut quadtree_node_t;
    let mut sw = 0 as *mut quadtree_node_t;
    let mut se = 0 as *mut quadtree_node_t;
    let mut x = (*(*(*node).bounds).nw.as_deref().unwrap()).x;
    let mut y = (*(*(*node).bounds).nw.as_deref().unwrap()).y;
    let mut hw = (*(*node).bounds).width / 2 as libc::c_int as libc::c_double;
    let mut hh = (*(*node).bounds).height / 2 as libc::c_int as libc::c_double;
    nw= crate::src::src::node::quadtree_node_with_bounds(x, y - hh, x + hw, y);
    if nw.is_null() {();
        return 0 as libc::c_int;
    }
    ne= crate::src::src::node::quadtree_node_with_bounds(
        x + hw,
        y - hh,
        x + hw * 2 as libc::c_int as libc::c_double,
        y,
    );
    if ne.is_null() {();
        return 0 as libc::c_int;
    }
    sw= crate::src::src::node::quadtree_node_with_bounds(
        x,
        y - hh * 2 as libc::c_int as libc::c_double,
        x + hw,
        y - hh,
    );
    if sw.is_null() {();
        return 0 as libc::c_int;
    }
    se= crate::src::src::node::quadtree_node_with_bounds(
        x + hw,
        y - hh * 2 as libc::c_int as libc::c_double,
        x + hw * 2 as libc::c_int as libc::c_double,
        y - hh,
    );
    if se.is_null() {();
        return 0 as libc::c_int;
    }
    (*node).nw= nw;
    (*node).ne= ne;
    (*node).sw= sw;
    (*node).se= se;
    let mut old = (*node).point;
    let mut key = (*node).key;
    (*node).point= 0 as *mut quadtree_point_t;
    (*node).key= 0 as *mut libc::c_void;
    return insert_(tree, node, old, key);
}
unsafe extern "C" fn find_(
    mut node: Option<&mut quadtree_node_t>,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut quadtree_point_t {
    if crate::src::src::node::quadtree_node_isleaf(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut())) != 0 {
        if (*(*node.as_deref().unwrap()).point).x == x && (*(*node.as_deref().unwrap()).point).y == y {
            return (*node.as_deref().unwrap()).point;
        }
    } else {
        let mut test = quadtree_point_t { x: 0., y: 0. };
        test.x= x;
        test.y= y;
        return find_(get_quadrant_(node.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), core::ptr::addr_of_mut!(test)).as_mut(), x, y);
    }
    return 0 as *mut quadtree_point_t;
}
unsafe extern "C" fn insert_(
    mut tree: *mut quadtree_t,
    mut root: *mut quadtree_node_t,
    mut point: *mut quadtree_point_t,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    if crate::src::src::node::quadtree_node_isempty(root) != 0 {
        (*root).point= point;
        (*root).key= key;
        return 1 as libc::c_int;
    } else {
        if crate::src::src::node::quadtree_node_isleaf(root) != 0 {
            if (*(*root).point).x == (*point).x && (*(*root).point).y == (*point).y {
                reset_node_(tree, root.as_mut());
                (*root).point= point;
                (*root).key= key;
                return 0 as libc::c_int;
            } else {
                if split_node_(tree, root) == 0 {
                    return 0 as libc::c_int;
                }
                return insert_(tree, root, point, key);
            }
        } else {
            if crate::src::src::node::quadtree_node_ispointer(root) != 0 {
                let mut quadrant = get_quadrant_(root, point);
                return if quadrant.is_null() {();
                    0 as libc::c_int
                } else {
                    insert_(tree, quadrant, point, key)
                };
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_new(
    mut minx: libc::c_double,
    mut miny: libc::c_double,
    mut maxx: libc::c_double,
    mut maxy: libc::c_double,
) -> Option<Box<quadtree_t>> {
    let mut tree = None;
    tree= Some(Box::new(<crate::src::src::quadtree::quadtree as Default>::default()));
    if tree.as_deref().is_none() {();
        return None;
    }
    (*tree.as_deref_mut().unwrap()).root= Some(Box::from_raw(crate::src::src::node::quadtree_node_with_bounds(minx, miny, maxx, maxy)));
    if (*tree.as_deref_mut().unwrap()).root.as_deref().is_none() {();
        ();
        return None;
    }
    (*tree.as_deref_mut().unwrap()).key_free= None;
    (*tree.as_deref_mut().unwrap()).length= 0 as libc::c_int as libc::c_uint;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_insert(
    mut tree: *mut quadtree_t,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut key: *mut libc::c_void,
) -> libc::c_int {
    let mut point = 0 as *mut quadtree_point_t;
    point= crate::src::src::point::quadtree_point_new(x, y).map(|b| Box::into_raw(b)).unwrap_or(std::ptr::null_mut());
    if point.is_null() {();
        return 0 as libc::c_int;
    }
    if node_contains_((*tree).root.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), point) == 0 {
        return 0 as libc::c_int;
    }
    if insert_(tree, (*tree).root.as_deref_mut().map(|r| r as *mut _).unwrap_or(std::ptr::null_mut()), point, key) == 0 {
        return 0 as libc::c_int;
    }
    (*tree).length= (*tree).length.wrapping_add(1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_search(
    mut tree: Option<&mut quadtree_t>,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> *mut quadtree_point_t {
    return find_((*tree.as_deref_mut().unwrap()).root.as_deref_mut(), x, y);
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_free(mut tree: Option<Box<quadtree_t>>) {
    if (*tree.as_deref().unwrap()).key_free.is_some() {
        crate::src::src::node::quadtree_node_free((*tree.as_deref_mut().unwrap()).root.take(), (*tree.as_deref().unwrap()).key_free);
    } else {
        crate::src::src::node::quadtree_node_free(
            (*tree.as_deref_mut().unwrap()).root.take(),
            Some(elision_ as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    ();
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_walk(
    mut root: *mut quadtree_node_t,
    mut descent: Option::<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
    mut ascent: Option::<unsafe extern "C" fn(*mut quadtree_node_t) -> ()>,
) {
    (Some(descent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
    if !(*root).nw.is_null() {
        quadtree_walk((*root).nw, descent, ascent);
    }else { (); }
    if !(*root).ne.is_null() {
        quadtree_walk((*root).ne, descent, ascent);
    }else { (); }
    if !(*root).sw.is_null() {
        quadtree_walk((*root).sw, descent, ascent);
    }else { (); }
    if !(*root).se.is_null() {
        quadtree_walk((*root).se, descent, ascent);
    }else { (); }
    (Some(ascent.expect("non-null function pointer")))
        .expect("non-null function pointer")(root);
}
