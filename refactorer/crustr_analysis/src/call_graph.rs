use graph::*;
use rustc_data_structures::binary_search_util::binary_search_slice;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub struct CallGraph {
    /// Invariant: the set of functions are sorted by `DefId`
    pub functions: IndexVec<Function, DefId>,
}

rustc_index::newtype_index! {
    pub struct Function {
        DEBUG_FORMAT = "Function_({})"
    }
}
