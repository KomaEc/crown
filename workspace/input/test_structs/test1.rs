pub struct X {
    f: Option<std::ptr::NonNull<i32>>,
    g: *mut i32
}

pub struct Y {
    h: Option<std::ptr::NonNull<Option<std::ptr::NonNull<i32>>>>,
}