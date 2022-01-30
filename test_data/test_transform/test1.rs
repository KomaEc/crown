use std::cmp::max;

struct X {
    f: *mut X,
    g: *mut X,
    h: usize,
}

unsafe fn height(mut n: *mut X) -> usize {
    if n.is_null() {
        return 0;
    }
    return (*n).h;
}

fn rotate(y: *mut X) -> *mut X {
    unsafe {
        let mut x: *mut X = (*y).f;
        let mut t2: *mut X = (*x).g;
        (*x).g = y;
        (*y).f = t2;
        (*y).h = max(height((*y).f), height((*y).g)) + 1;
        (*x).h = max(height((*x).f), height((*x).g)) + 1;
        return x
    }
}
