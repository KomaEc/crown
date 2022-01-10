//! Fixpoint algorithm:
//! 1. CallGraph
//! 2. In reverse-post-order, process each body
//! 3. `p = q` => update dim(p) with max(dim(p), dim(q))
