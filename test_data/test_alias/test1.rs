
fn main() {
    unsafe {
        let mut a = 0;
        let mut b = 1;
        let mut c = 2;
        let mut p: *mut _ = &mut a;
        p = &mut b;
        let mut m: *mut _ = &mut p;
        let r = *m;
        let mut q: *mut _ = &mut c;
        m = &mut q;
    }
}