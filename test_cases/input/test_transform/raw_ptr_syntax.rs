#![feature(raw_ref_op)]

fn main() {
    let mut local = 0;
    let r = &raw mut local;
    unsafe {
        *r = 1;
    }
    println!("{}", &local);
}