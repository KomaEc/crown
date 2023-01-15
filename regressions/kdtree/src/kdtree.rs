use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *const __pthread_internal_list,
    pub __next: *const __pthread_internal_list,
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        Self {
            __prev: std::ptr::null_mut(),
            __next: std::ptr::null_mut(),
        }
    }
}

pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        Self {
            __lock: Default::default(),
            __count: Default::default(),
            __owner: Default::default(),
            __nusers: Default::default(),
            __kind: Default::default(),
            __spins: Default::default(),
            __elision: Default::default(),
            __list: Default::default(),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer0;
#[repr(C)]
pub struct kdtree {
    pub dim: libc::c_int,
    pub root: Option<Box<kdnode>>,
    pub rect: Option<Box<kdhyperrect>>,
    pub destr: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
impl Default for kdtree {
    fn default() -> Self {
        Self {
            dim: Default::default(),
            root: None,
            rect: None,
            destr: Default::default(),
        }
    }
}
impl kdtree {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer1;
#[repr(C)]
pub struct kdhyperrect {
    pub dim: libc::c_int,
    pub min: *mut libc::c_double,
    pub max: *mut libc::c_double,
}
impl Default for kdhyperrect {
    fn default() -> Self {
        Self {
            dim: Default::default(),
            min: std::ptr::null_mut(),
            max: std::ptr::null_mut(),
        }
    }
}
impl kdhyperrect {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer2;
#[repr(C)]
pub struct kdnode {
    pub pos: *mut libc::c_double,
    pub dir: libc::c_int,
    pub data: *mut libc::c_void,
    pub left: Option<Box<kdnode>>,
    pub right: Option<Box<kdnode>>,
}
impl Default for kdnode {
    fn default() -> Self {
        Self {
            pos: std::ptr::null_mut(),
            dir: Default::default(),
            data: std::ptr::null_mut(),
            left: None,
            right: None,
        }
    }
}
impl kdnode {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ErasedByRefactorer3;
#[repr(C)]
pub struct kdres {
    pub tree: *const kdtree,
    pub rlist: *mut res_node,
    pub riter: *const res_node,
    pub size: libc::c_int,
}
impl Default for kdres {
    fn default() -> Self {
        Self {
            tree: std::ptr::null_mut(),
            rlist: std::ptr::null_mut(),
            riter: std::ptr::null_mut(),
            size: Default::default(),
        }
    }
}
impl kdres {
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct res_node {
    pub item: *const kdnode,
    pub dist_sq: libc::c_double,
    pub next: *mut res_node,
}
impl Default for res_node {
    fn default() -> Self {
        Self {
            item: std::ptr::null_mut(),
            dist_sq: Default::default(),
            next: std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kd_create(mut k: libc::c_int) -> Option<Box<kdtree>> {
    let mut tree = None;
    tree = Some(Box::new(<crate::src::kdtree::kdtree as Default>::default()));
    if tree.as_deref().is_none() {
        ();
        return None;
    }
    (*tree.as_deref_mut().unwrap()).dim = k;
    (*tree.as_deref_mut().unwrap()).root = None;
    (*tree.as_deref_mut().unwrap()).destr = None;
    (*tree.as_deref_mut().unwrap()).rect = None;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn kd_free(mut tree: Option<Box<kdtree>>) {
    if !tree.as_deref().is_none() {
        kd_clear(tree.as_deref_mut());
        ();
    } else {
        ();
    }
}
unsafe extern "C" fn clear_rec(
    mut node: Option<Box<kdnode>>,
    mut destr: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if node.as_deref().is_none() {
        ();
        return;
    }
    clear_rec((*node.as_deref_mut().unwrap()).left.take(), destr);
    clear_rec((*node.as_deref_mut().unwrap()).right.take(), destr);
    if destr.is_some() {
        destr.expect("non-null function pointer")((*node).data);
    }
    free((*node.as_deref().unwrap()).pos as *mut libc::c_void);
    ();
}
#[no_mangle]
pub unsafe extern "C" fn kd_clear(mut tree: Option<&mut kdtree>) {
    clear_rec(
        (*tree.as_deref_mut().unwrap()).root.take(),
        (*tree.as_deref().unwrap()).destr,
    );
    (*tree.as_deref_mut().unwrap()).root = None;
    if !(*tree.as_deref().unwrap()).rect.as_deref().is_none() {
        hyperrect_free((*tree.as_deref_mut().unwrap()).rect.take());
        (*tree.as_deref_mut().unwrap()).rect = None;
    } else {
        ();
    }
}
#[no_mangle]
pub unsafe extern "C" fn kd_data_destructor(
    mut tree: Option<&mut kdtree>,
    mut destr: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    (*tree.as_deref_mut().unwrap()).destr = destr;
}
unsafe extern "C" fn insert_rec(
    mut nptr: *mut *mut kdnode,
    mut pos: *const libc::c_double,
    mut data: *mut libc::c_void,
    mut dir: libc::c_int,
    mut dim: libc::c_int,
) -> libc::c_int {
    let mut new_dir: libc::c_int = 0;
    let mut node = None;
    if (*nptr).is_null() {
        ();
        node = Some(Box::new(<crate::src::kdtree::kdnode as Default>::default()));
        if node.as_deref().is_none() {
            ();
            return -(1 as libc::c_int);
        }
        (*node.as_deref_mut().unwrap()).pos = malloc(
            (dim as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        ) as *mut libc::c_double;
        if (*node.as_deref().unwrap()).pos.is_null() {
            ();
            ();
            return -(1 as libc::c_int);
        }
        memcpy(
            (*node.as_deref().unwrap()).pos as *mut libc::c_void,
            pos as *const libc::c_void,
            (dim as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        (*node.as_deref_mut().unwrap()).data = data;
        (*node.as_deref_mut().unwrap()).dir = dir;
        (*node.as_deref_mut().unwrap()).right = None;
        (*node.as_deref_mut().unwrap()).left = (*node.as_deref_mut().unwrap()).right.take();
        *nptr = node
            .as_deref_mut()
            .map(|r| r as *mut _)
            .unwrap_or(std::ptr::null_mut());
        return 0 as libc::c_int;
    }
    node = Some(Box::from_raw((*nptr)));
    new_dir = ((*node).dir + 1 as libc::c_int) % dim;
    if *pos.offset((*node.as_deref().unwrap()).dir as isize)
        < *(*node.as_deref().unwrap())
            .pos
            .offset((*node.as_deref().unwrap()).dir as isize)
    {
        return insert_rec(&mut (**nptr).left, pos, data, new_dir, dim);
    }
    return insert_rec(&mut (**nptr).right, pos, data, new_dir, dim);
}
#[no_mangle]
pub unsafe extern "C" fn kd_insert(
    mut tree: *mut kdtree,
    mut pos: *const libc::c_double,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    if insert_rec(&mut (*tree).root, pos, data, 0 as libc::c_int, (*tree).dim) != 0 {
        return -(1 as libc::c_int);
    }
    if (*tree).rect.as_deref().is_none() {
        ();
        (*tree).rect = hyperrect_create((*tree).dim, pos, pos);
    } else {
        hyperrect_extend((*tree).rect.as_deref_mut(), pos);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kd_insertf(
    mut tree: *mut kdtree,
    mut pos: *const libc::c_float,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    static mut sbuf: [libc::c_double; 16] = [0.; 16];
    let mut bptr = 0 as *mut libc::c_double;
    let mut buf = 0 as *mut libc::c_double;
    let mut res: libc::c_int = 0;
    let mut dim = (*tree).dim;
    if dim > 16 as libc::c_int {
        if dim <= 256 as libc::c_int {
            // let mut fresh11 = ::std::vec::from_elem(
            //     0,
            //     (dim as libc::c_ulong)
            //         .wrapping_mul(
            //             ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            //         ) as usize,
            // );
            // buf = fresh11.as_mut_ptr() as *mut libc::c_double;
            // bptr = buf;
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return -(1 as libc::c_int);
            }
        } else {
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return -(1 as libc::c_int);
            }
        }
    } else {
        buf = sbuf.as_mut_ptr();
        bptr = buf;
    }
    loop {
        let fresh12 = dim;
        dim = dim - 1;
        if !(fresh12 > 0 as libc::c_int) {
            break;
        }
        let fresh13 = pos;
        pos = pos.offset(1);
        let fresh14 = bptr;
        bptr = bptr.offset(1);
        *fresh14 = (*fresh13) as libc::c_double;
    }
    res = kd_insert(tree, buf, data);
    if (*tree).dim > 256 as libc::c_int {
        free(buf as *mut libc::c_void);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn kd_insert3(
    mut tree: *mut kdtree,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: [libc::c_double; 3] = [0.; 3];
    buf[0 as libc::c_int as usize] = x;
    buf[1 as libc::c_int as usize] = y;
    buf[2 as libc::c_int as usize] = z;
    return kd_insert(tree, buf.as_mut_ptr(), data);
}
#[no_mangle]
pub unsafe extern "C" fn kd_insert3f(
    mut tree: *mut kdtree,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: [libc::c_double; 3] = [0.; 3];
    buf[0 as libc::c_int as usize] = x as libc::c_double;
    buf[1 as libc::c_int as usize] = y as libc::c_double;
    buf[2 as libc::c_int as usize] = z as libc::c_double;
    return kd_insert(tree, buf.as_mut_ptr(), data);
}
unsafe extern "C" fn find_nearest(
    mut node: *const kdnode,
    mut pos: *const libc::c_double,
    mut range: libc::c_double,
    mut list: Option<&mut res_node>,
    mut ordered: libc::c_int,
    mut dim: libc::c_int,
) -> libc::c_int {
    let mut dist_sq: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut added_res = 0 as libc::c_int;
    if node.is_null() {
        ();
        return 0 as libc::c_int;
    }
    dist_sq = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < dim {
        dist_sq += (*(*node).pos.offset(i as isize) - *pos.offset(i as isize))
            * (*(*node).pos.offset(i as isize) - *pos.offset(i as isize));
        i += 1;
    }
    if dist_sq <= range * range {
        if rlist_insert(
            list.as_deref_mut()
                .map(|r| r as *mut _)
                .unwrap_or(std::ptr::null_mut()),
            node,
            (if ordered != 0 { dist_sq } else { -1.0f64 }),
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        added_res = 1 as libc::c_int;
    }
    dx = *pos.offset((*node).dir as isize) - *(*node).pos.offset((*node).dir as isize);
    ret = find_nearest(
        if dx <= 0.0f64 {
            (*node)
                .left
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null())
        } else {
            (*node)
                .right
                .as_deref()
                .map(|r| r as *const _)
                .unwrap_or(std::ptr::null())
        },
        pos,
        range,
        list.as_deref_mut(),
        ordered,
        dim,
    );
    if ret >= 0 as libc::c_int && fabs(dx) < range {
        added_res += ret;
        ret = find_nearest(
            if dx <= 0.0f64 {
                (*node)
                    .right
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null())
            } else {
                (*node)
                    .left
                    .as_deref()
                    .map(|r| r as *const _)
                    .unwrap_or(std::ptr::null())
            },
            pos,
            range,
            list.as_deref_mut(),
            ordered,
            dim,
        );
    }
    if ret == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    added_res += ret;
    return added_res;
}
unsafe extern "C" fn kd_nearest_i(
    mut node: *const kdnode,
    mut pos: *const libc::c_double,
    mut result: Option<&mut *const kdnode>,
    mut result_dist_sq: Option<&mut libc::c_double>,
    mut rect: *mut kdhyperrect,
) {
    let mut dir = (*node).dir;
    let mut i: libc::c_int = 0;
    let mut dummy: libc::c_double = 0.;
    let mut dist_sq: libc::c_double = 0.;
    let mut nearer_subtree = 0 as *mut kdnode;
    let mut farther_subtree = 0 as *mut kdnode;
    let mut nearer_hyperrect_coord = 0 as *mut libc::c_double;
    let mut farther_hyperrect_coord = 0 as *mut libc::c_double;
    dummy = *pos.offset(dir as isize) - *(*node).pos.offset(dir as isize);
    if dummy <= 0 as libc::c_int as libc::c_double {
        nearer_subtree = (*node)
            .left
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null());
        farther_subtree = (*node)
            .right
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null());
        nearer_hyperrect_coord = (*rect).max.offset(dir as isize);
        farther_hyperrect_coord = (*rect).min.offset(dir as isize);
    } else {
        nearer_subtree = (*node)
            .right
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null());
        farther_subtree = (*node)
            .left
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null());
        nearer_hyperrect_coord = (*rect).min.offset(dir as isize);
        farther_hyperrect_coord = (*rect).max.offset(dir as isize);
    }
    if !nearer_subtree.is_null() {
        dummy = (*nearer_hyperrect_coord);
        *nearer_hyperrect_coord = *(*node).pos.offset(dir as isize);
        kd_nearest_i(
            nearer_subtree,
            pos,
            result.as_deref_mut(),
            result_dist_sq.as_deref_mut(),
            rect,
        );
        *nearer_hyperrect_coord = dummy;
    } else {
        ();
    }
    dist_sq = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*rect).dim {
        dist_sq += (*(*node).pos.offset(i as isize) - *pos.offset(i as isize))
            * (*(*node).pos.offset(i as isize) - *pos.offset(i as isize));
        i += 1;
    }
    if dist_sq < (*result_dist_sq.as_deref().unwrap()) {
        *result.as_deref_mut().unwrap() = node;
        *result_dist_sq.as_deref_mut().unwrap() = dist_sq;
    }
    if !farther_subtree.is_null() {
        dummy = (*farther_hyperrect_coord);
        *farther_hyperrect_coord = *(*node).pos.offset(dir as isize);
        if hyperrect_dist_sq(rect, pos) < (*result_dist_sq.as_deref().unwrap()) {
            kd_nearest_i(
                farther_subtree,
                pos,
                result.as_deref_mut(),
                result_dist_sq.as_deref_mut(),
                rect,
            );
        }
        *farther_hyperrect_coord = dummy;
    } else {
        ();
    }
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest(
    mut kd: *const kdtree,
    mut pos: *const libc::c_double,
) -> Option<Box<kdres>> {
    let mut rect = None;
    let mut result = 0 as *mut kdnode;
    let mut rset = None;
    let mut dist_sq: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if kd.is_null() {
        ();
        return None;
    }
    if (*kd).rect.as_deref().is_none() {
        ();
        return None;
    }
    rset = Some(Box::new(<crate::src::kdtree::kdres as Default>::default()));
    if rset.as_deref().is_none() {
        ();
        return None;
    }
    (*rset.as_deref_mut().unwrap()).rlist = alloc_resnode();
    if (*rset.as_deref().unwrap()).rlist.is_null() {
        ();
        ();
        return None;
    }
    (*(*rset.as_deref_mut().unwrap()).rlist).next = 0 as *mut res_node;
    (*rset.as_deref_mut().unwrap()).tree = kd;
    rect = hyperrect_duplicate(
        (*kd)
            .rect
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
    );
    if rect.as_deref().is_none() {
        ();
        kd_res_free(rset);
        return None;
    }
    result = (*kd)
        .root
        .as_deref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null());
    dist_sq = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*kd).dim {
        dist_sq += (*(*result).pos.offset(i as isize) - *pos.offset(i as isize))
            * (*(*result).pos.offset(i as isize) - *pos.offset(i as isize));
        i += 1;
    }
    kd_nearest_i(
        (*kd)
            .root
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
        pos,
        &mut result,
        &mut dist_sq,
        rect.as_deref_mut()
            .map(|r| r as *mut _)
            .unwrap_or(std::ptr::null_mut()),
    );
    hyperrect_free(rect);
    if !result.is_null() {
        if rlist_insert((*rset.as_deref().unwrap()).rlist, result, -1.0f64) == -(1 as libc::c_int) {
            kd_res_free(rset);
            return None;
        }
        (*rset.as_deref_mut().unwrap()).size = 1 as libc::c_int;
        kd_res_rewind(rset.as_deref_mut());
        return rset;
    } else {
        ();
        kd_res_free(rset);
        return None;
    };
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearestf(
    mut tree: *const kdtree,
    mut pos: *const libc::c_float,
) -> *const kdres {
    static mut sbuf: [libc::c_double; 16] = [0.; 16];
    let mut bptr = 0 as *mut libc::c_double;
    let mut buf = 0 as *mut libc::c_double;
    let mut dim = (*tree).dim;
    let mut res = 0 as *mut kdres;
    if dim > 16 as libc::c_int {
        if dim <= 256 as libc::c_int {
            // let mut fresh18 = ::std::vec::from_elem(
            //     0,
            //     (dim as libc::c_ulong)
            //         .wrapping_mul(
            //             ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            //         ) as usize,
            // );
            // buf = fresh18.as_mut_ptr() as *mut libc::c_double;
            // bptr = buf;
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return 0 as *mut kdres;
            }
        } else {
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return 0 as *mut kdres;
            }
        }
    } else {
        buf = sbuf.as_mut_ptr();
        bptr = buf;
    }
    loop {
        let fresh19 = dim;
        dim = dim - 1;
        if !(fresh19 > 0 as libc::c_int) {
            break;
        }
        let fresh20 = pos;
        pos = pos.offset(1);
        let fresh21 = bptr;
        bptr = bptr.offset(1);
        *fresh21 = (*fresh20) as libc::c_double;
    }
    res = kd_nearest(tree, buf);
    if (*tree).dim > 256 as libc::c_int {
        free(buf as *mut libc::c_void);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest3(
    mut tree: *const kdtree,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
) -> Option<Box<kdres>> {
    let mut pos: [libc::c_double; 3] = [0.; 3];
    pos[0 as libc::c_int as usize] = x;
    pos[1 as libc::c_int as usize] = y;
    pos[2 as libc::c_int as usize] = z;
    return kd_nearest(tree, pos.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest3f(
    mut tree: *const kdtree,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> Option<Box<kdres>> {
    let mut pos: [libc::c_double; 3] = [0.; 3];
    pos[0 as libc::c_int as usize] = x as libc::c_double;
    pos[1 as libc::c_int as usize] = y as libc::c_double;
    pos[2 as libc::c_int as usize] = z as libc::c_double;
    return kd_nearest(tree, pos.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest_range(
    mut kd: *const kdtree,
    mut pos: *const libc::c_double,
    mut range: libc::c_double,
) -> Option<Box<kdres>> {
    let mut ret: libc::c_int = 0;
    let mut rset = None;
    rset = Some(Box::new(<crate::src::kdtree::kdres as Default>::default()));
    if rset.as_deref().is_none() {
        ();
        return None;
    }
    (*rset.as_deref_mut().unwrap()).rlist = alloc_resnode();
    if (*rset.as_deref().unwrap()).rlist.is_null() {
        ();
        ();
        return None;
    }
    (*(*rset.as_deref_mut().unwrap()).rlist).next = 0 as *mut res_node;
    (*rset.as_deref_mut().unwrap()).tree = kd;
    ret = find_nearest(
        (*kd)
            .root
            .as_deref()
            .map(|r| r as *const _)
            .unwrap_or(std::ptr::null()),
        pos,
        range,
        (*rset.as_deref_mut().unwrap()).rlist.as_mut(),
        0 as libc::c_int,
        (*kd).dim,
    );
    if ret == -(1 as libc::c_int) {
        kd_res_free(rset);
        return None;
    }
    (*rset.as_deref_mut().unwrap()).size = ret;
    kd_res_rewind(rset.as_deref_mut());
    return rset;
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest_rangef(
    mut kd: *const kdtree,
    mut pos: *const libc::c_float,
    mut range: libc::c_float,
) -> *const kdres {
    static mut sbuf: [libc::c_double; 16] = [0.; 16];
    let mut bptr = 0 as *mut libc::c_double;
    let mut buf = 0 as *mut libc::c_double;
    let mut dim = (*kd).dim;
    let mut res = 0 as *mut kdres;
    if dim > 16 as libc::c_int {
        if dim <= 256 as libc::c_int {
            // let mut fresh25 = ::std::vec::from_elem(
            //     0,
            //     (dim as libc::c_ulong)
            //         .wrapping_mul(
            //             ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
            //         ) as usize,
            // );
            // buf = fresh25.as_mut_ptr() as *mut libc::c_double;
            // bptr = buf;
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return 0 as *mut kdres;
            }
        } else {
            buf = malloc(
                (dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            ) as *mut libc::c_double;
            bptr = buf;
            if bptr.is_null() {
                ();
                return 0 as *mut kdres;
            }
        }
    } else {
        buf = sbuf.as_mut_ptr();
        bptr = buf;
    }
    loop {
        let fresh26 = dim;
        dim = dim - 1;
        if !(fresh26 > 0 as libc::c_int) {
            break;
        }
        let fresh27 = pos;
        pos = pos.offset(1);
        let fresh28 = bptr;
        bptr = bptr.offset(1);
        *fresh28 = (*fresh27) as libc::c_double;
    }
    res = kd_nearest_range(kd, buf, range as libc::c_double);
    if (*kd).dim > 256 as libc::c_int {
        free(buf as *mut libc::c_void);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest_range3(
    mut tree: *const kdtree,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut z: libc::c_double,
    mut range: libc::c_double,
) -> Option<Box<kdres>> {
    let mut buf: [libc::c_double; 3] = [0.; 3];
    buf[0 as libc::c_int as usize] = x;
    buf[1 as libc::c_int as usize] = y;
    buf[2 as libc::c_int as usize] = z;
    return kd_nearest_range(tree, buf.as_mut_ptr(), range);
}
#[no_mangle]
pub unsafe extern "C" fn kd_nearest_range3f(
    mut tree: *const kdtree,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut range: libc::c_float,
) -> Option<Box<kdres>> {
    let mut buf: [libc::c_double; 3] = [0.; 3];
    buf[0 as libc::c_int as usize] = x as libc::c_double;
    buf[1 as libc::c_int as usize] = y as libc::c_double;
    buf[2 as libc::c_int as usize] = z as libc::c_double;
    return kd_nearest_range(tree, buf.as_mut_ptr(), range as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_free(mut rset: Option<Box<kdres>>) {
    clear_results(rset.as_deref_mut());
    free_resnode((*rset.as_deref().unwrap()).rlist);
    ();
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_size(mut set: *const kdres) -> libc::c_int {
    return (*set).size;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_rewind(mut rset: Option<&mut kdres>) {
    (*rset.as_deref_mut().unwrap()).riter = (*(*rset.as_deref().unwrap()).rlist).next;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_end(mut rset: *const kdres) -> libc::c_int {
    return ((*rset).riter == 0 as *mut res_node) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_next(mut rset: Option<&mut kdres>) -> libc::c_int {
    (*rset.as_deref_mut().unwrap()).riter = (*(*rset.as_deref().unwrap()).riter).next;
    return ((*rset.as_deref().unwrap()).riter != 0 as *mut res_node) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_item(
    mut rset: *const kdres,
    mut pos: *mut libc::c_double,
) -> *const libc::c_void {
    if !(*rset).riter.is_null() {
        if !pos.is_null() {
            memcpy(
                pos as *mut libc::c_void,
                (*(*(*rset).riter).item).pos as *const f64 as *const libc::c_void,
                ((*(*rset).tree).dim as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
            );
        } else {
            ();
        }
        return (*(*(*rset).riter).item).data;
    } else {
        ();
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_itemf(
    mut rset: *const kdres,
    mut pos: *mut libc::c_float,
) -> *const libc::c_void {
    if !(*rset).riter.is_null() {
        if !pos.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*(*rset).tree).dim {
                *pos.offset(i as isize) =
                    *(*(*(*rset).riter).item).pos.offset(i as isize) as libc::c_float;
                i += 1;
            }
        } else {
            ();
        }
        return (*(*(*rset).riter).item).data;
    } else {
        ();
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_item3(
    mut rset: *const kdres,
    mut x: Option<&mut libc::c_double>,
    mut y: Option<&mut libc::c_double>,
    mut z: Option<&mut libc::c_double>,
) -> *const libc::c_void {
    if !(*rset).riter.is_null() {
        if !x.as_deref().is_none() {
            *x.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(0 as libc::c_int as isize);
        } else {
            ();
        }
        if !y.as_deref().is_none() {
            *y.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(1 as libc::c_int as isize);
        } else {
            ();
        }
        if !z.as_deref().is_none() {
            *z.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(2 as libc::c_int as isize);
        } else {
            ();
        }
        return (*(*(*rset).riter).item).data;
    } else {
        ();
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_item3f(
    mut rset: *const kdres,
    mut x: Option<&mut libc::c_float>,
    mut y: Option<&mut libc::c_float>,
    mut z: Option<&mut libc::c_float>,
) -> *const libc::c_void {
    if !(*rset).riter.is_null() {
        if !x.as_deref().is_none() {
            *x.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(0 as libc::c_int as isize)
                as libc::c_float;
        } else {
            ();
        }
        if !y.as_deref().is_none() {
            *y.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(1 as libc::c_int as isize)
                as libc::c_float;
        } else {
            ();
        }
        if !z.as_deref().is_none() {
            *z.as_deref_mut().unwrap() = *(*(*(*rset).riter).item)
                .pos
                .offset(2 as libc::c_int as isize)
                as libc::c_float;
        } else {
            ();
        }
        return (*(*(*rset).riter).item).data;
    } else {
        ();
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn kd_res_item_data(mut set: *const kdres) -> *const libc::c_void {
    return kd_res_item(set, 0 as *mut libc::c_double);
}
unsafe extern "C" fn hyperrect_create(
    mut dim: libc::c_int,
    mut min: *const libc::c_double,
    mut max: *const libc::c_double,
) -> Option<Box<kdhyperrect>> {
    let mut size = (dim as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong);
    let mut rect = None;
    rect = Some(Box::new(
        <crate::src::kdtree::kdhyperrect as Default>::default(),
    ));
    if rect.as_deref().is_none() {
        ();
        return None;
    }
    (*rect.as_deref_mut().unwrap()).dim = dim;
    (*rect.as_deref_mut().unwrap()).min = malloc(size) as *mut libc::c_double;
    if (*rect.as_deref().unwrap()).min.is_null() {
        ();
        ();
        return None;
    }
    (*rect.as_deref_mut().unwrap()).max = malloc(size) as *mut libc::c_double;
    if (*rect.as_deref().unwrap()).max.is_null() {
        ();
        free((*rect.as_deref().unwrap()).min as *mut libc::c_void);
        ();
        return None;
    }
    memcpy(
        (*rect.as_deref().unwrap()).min as *mut libc::c_void,
        min as *const libc::c_void,
        size,
    );
    memcpy(
        (*rect.as_deref().unwrap()).max as *mut libc::c_void,
        max as *const libc::c_void,
        size,
    );
    return rect;
}
unsafe extern "C" fn hyperrect_free(mut rect: Option<Box<kdhyperrect>>) {
    free((*rect.as_deref().unwrap()).min as *mut libc::c_void);
    free((*rect.as_deref().unwrap()).max as *mut libc::c_void);
    ();
}
unsafe extern "C" fn hyperrect_duplicate(mut rect: *const kdhyperrect) -> Option<Box<kdhyperrect>> {
    return hyperrect_create(
        (*rect).dim,
        (*rect).min as *const f64,
        (*rect).max as *const f64,
    );
}
unsafe extern "C" fn hyperrect_extend(
    mut rect: Option<&mut kdhyperrect>,
    mut pos: *const libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*rect.as_deref().unwrap()).dim {
        if *pos.offset(i as isize) < *(*rect.as_deref().unwrap()).min.offset(i as isize) {
            *(*rect.as_deref().unwrap()).min.offset(i as isize) = *pos.offset(i as isize);
        }
        if *pos.offset(i as isize) > *(*rect.as_deref().unwrap()).max.offset(i as isize) {
            *(*rect.as_deref().unwrap()).max.offset(i as isize) = *pos.offset(i as isize);
        }
        i += 1;
    }
}
unsafe extern "C" fn hyperrect_dist_sq(
    mut rect: *const kdhyperrect,
    mut pos: *const libc::c_double,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut result = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*rect).dim {
        if *pos.offset(i as isize) < *(*rect).min.offset(i as isize) {
            result += (*(*rect).min.offset(i as isize) - *pos.offset(i as isize))
                * (*(*rect).min.offset(i as isize) - *pos.offset(i as isize));
        } else if *pos.offset(i as isize) > *(*rect).max.offset(i as isize) {
            result += (*(*rect).max.offset(i as isize) - *pos.offset(i as isize))
                * (*(*rect).max.offset(i as isize) - *pos.offset(i as isize));
        }
        i += 1;
    }
    return result;
}
static mut free_nodes: *mut res_node = 0 as *const res_node as *mut res_node;
static mut alloc_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn alloc_resnode() -> Option<Box<res_node>> {
    let mut node = None;
    pthread_mutex_lock(&mut alloc_mutex);
    if free_nodes.is_null() {
        ();
        node = Some(Box::new(
            <crate::src::kdtree::res_node as Default>::default(),
        ));
    } else {
        node = free_nodes;
        free_nodes = (*free_nodes).next;
        (*node.as_deref_mut().unwrap()).next = 0 as *mut res_node;
    }
    pthread_mutex_unlock(&mut alloc_mutex);
    return node;
}
unsafe extern "C" fn free_resnode(mut node: *mut res_node) {
    pthread_mutex_lock(&mut alloc_mutex);
    (*node).next = free_nodes;
    free_nodes = node;
    pthread_mutex_unlock(&mut alloc_mutex);
}
unsafe extern "C" fn rlist_insert(
    mut list: *mut res_node,
    mut item: *const kdnode,
    mut dist_sq: libc::c_double,
) -> libc::c_int {
    let mut rnode = None;
    rnode = alloc_resnode();
    if rnode.as_deref().is_none() {
        ();
        return -(1 as libc::c_int);
    }
    (*rnode.as_deref_mut().unwrap()).item = item;
    (*rnode.as_deref_mut().unwrap()).dist_sq = dist_sq;
    if dist_sq >= 0.0f64 {
        while !(*list).next.is_null() && (*(*list).next).dist_sq < dist_sq {
            list = (*list).next;
        }
    }
    (*rnode.as_deref_mut().unwrap()).next = (*list).next;
    (*list).next = rnode
        .as_deref_mut()
        .map(|r| r as *mut _)
        .unwrap_or(std::ptr::null_mut());
    return 0 as libc::c_int;
}
unsafe extern "C" fn clear_results(mut rset: Option<&mut kdres>) {
    let mut tmp = 0 as *mut res_node;
    let mut node = (*(*rset.as_deref().unwrap()).rlist).next;
    while !node.is_null() {
        tmp = node;
        node = (*node).next;
        free_resnode(tmp);
    }
    ();
    (*(*rset.as_deref_mut().unwrap()).rlist).next = 0 as *mut res_node;
}
