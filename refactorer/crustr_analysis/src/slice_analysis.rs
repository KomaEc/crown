//! Fixpoint algorithm:
//! 1. CallGraph
//! 2. In reverse-post-order, process each body
//! 3. `p = q` => update dim(p) with max(dim(p), dim(q))
//! 4. Differences between nested slice and multidimensional array?
//!    int ***p ~> &[&[&[i32]]]
//!             ~> [[[i32; 3]; 4]; 5] (???? possible or not)
