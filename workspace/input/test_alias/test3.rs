fn f(mut p: *mut i32, q: *mut i32) {
    *p = *q;
    p = &mut *q;
}