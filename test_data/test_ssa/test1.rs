fn f() -> u32 {
    let i = 1;
    let mut j = 1;
    let mut k = 0;
    while k < 100 {
        if j < 20 {
            j = i;
            k += 1;
        } else {
            j = k;
            k += 2;
        }
    }
    return j
}