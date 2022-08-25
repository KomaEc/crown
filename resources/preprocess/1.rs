

unsafe fn f(p: *mut i32, q: *mut *mut i32) {
    if p.is_null() {

    }

    if !p.is_null() {

    }
    if !p.is_null() {

    } else {

    }

    if (*q).is_null() {

    }

    while cond() {
        if p.is_null() {

        } else {
            
        }
    }

    while !p.is_null() {

    }


}

#[inline(never)]
fn cond() -> bool {
    false
}
 