#![feature(rustc_private)]
#![feature(min_specialization)]
#![feature(box_patterns)]

#[macro_use]
extern crate index;

extern crate rustc_middle;
extern crate rustc_serialize;

pub mod andersen;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
