struct X {
    f: *mut X,
    g: *mut X,
    h: usize,
}

fn f(x: *mut X, y: *mut X) -> *mut X {
    unsafe {
        let res = (*(*x).f).g;
        (*(*x).f).g = y;
        // let curr_tmp_1 = *x;
        // let curr_tmp_2 = curr_tmp_1.f;
        // let curr_tmp_3 = *curr_tmp_2;
        // let curr_tmp_4 = curr_tmp_3.g;
        // curr_tmp_4 = y;
        return res;
    }
}
