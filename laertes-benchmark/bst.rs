// Ignore the compiler pragmas below
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

use std::ptr::{null, null_mut};
use std::os::raw::c_int;

extern "C" {
fn malloc(size: usize) -> * mut std::os::raw::c_void;
}

// BST node
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub left: *mut node_t,
    pub right: *mut node_t,
    pub value: *mut c_int,
}

#[no_mangle]
pub unsafe extern "C" fn insert(mut value: c_int,
                                mut node: *mut node_t) {
    if value < *(*node).value {
        if (*node).left.is_null() {
            // the current misconfiguration skipped the call to malloc here
            (*node).left = malloc(::std::mem::size_of::<node_t>()) as * mut node_t;
            (*(*node).left).left = 0 as *mut node_t;
            (*(*node).left).right = 0 as *mut node_t;
            (*(*node).left).value = malloc(::std::mem::size_of::<c_int>()) as * mut c_int;
            *(*(*node).right).value = value;
        } else { insert(value, (*node).left); }
    } else if value > *(*node).value {
        if (*node).right.is_null() {
            // the current misconfiguration skipped the call to malloc here
            (*node).right = malloc(::std::mem::size_of::<node_t>()) as * mut node_t;
            (*(*node).right).right = 0 as *mut node_t;
            (*(*node).right).right = 0 as *mut node_t;
            (*(*node).right).value = malloc(::std::mem::size_of::<c_int>()) as * mut c_int;
            *(*(*node).right).value = value;
        } else { insert(value, (*node).right); }
    };
}

// this is problematic with lack of mutability overloads, we are going to ignore them initially. C also does not have mutability overloads anyway
#[no_mangle]
pub unsafe extern "C" fn find(mut value: c_int, mut node: *mut node_t)
 -> *mut node_t {
    if value < *(*node).value && !(*node).left.is_null() {
        return find(value, (*node).left)
    } else {
        if value > *(*node).value && !(*node).right.is_null() {
            return find(value, (*node).right)
        } else { if value == *(*node).value { return node } }
    }
    return 0 as *mut node_t;
}

pub unsafe fn main_0() {
    // Using Box to avoid malloc clutter
    let mut tree = malloc(::std::mem::size_of::<node_t>()) as * mut node_t;
    *(*tree).value = 3;
    // insert 2 nodes
    insert(1, tree);
    insert(2, tree);
    // change the value of node containing 3 to 4
    *(*find(3, tree)).value = 4;
}

// use the inner-unsafe-main pattern from c2rust
pub fn main() {
    unsafe { main_0() }
}
