use graph::implementation::forward_star;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::ty::TyCtxt;

pub struct CallGraph {
    /// Invariant: the set of functions are sorted by `DefId` to facilitate
    /// reverse lookup
    pub functions: IndexVec<Function, DefId>,
    pub call_graph: forward_star::Graph<Function, CallEdge>,
}

impl CallGraph {
    #[inline]
    pub fn lookup_function(&self, did: &DefId) -> Option<Function> {
        self.functions.binary_search(did).ok()
    }
}

rustc_index::newtype_index! {
    pub struct Function {
        DEBUG_FORMAT = "Function_({})"
    }
}

rustc_index::newtype_index! {
    pub struct CallEdge {
        DEBUG_FORMAT = "CallEdge_({})"
    }
}

pub fn construct_call_graph<'tcx>(_tcx: TyCtxt<'tcx>) -> CallGraph {
    todo!()
}
