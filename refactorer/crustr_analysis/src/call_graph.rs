use graph::*;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;

pub struct CallGraph {
    /// Invariant: the set of functions are sorted by `DefId`
    pub functions: IndexVec<Function, DefId>,
    
}

rustc_index::newtype_index!{
    pub struct Function {
        DEBUG_FORMAT = "Function_({})"
    }
}