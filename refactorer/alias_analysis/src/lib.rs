#![feature(rustc_private)]
#![feature(min_specialization)]

#[macro_use]
extern crate rustc_index;

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
