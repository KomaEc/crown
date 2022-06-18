use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::{Local, Location};

use crate::call_graph::Func;

pub trait AnalysisResults {
    fn local_result(&self, func: Func, local: Local, ptr_depth: usize) -> Option<bool>;
    fn local_result_at(
        &self,
        func: Func,
        local: Local,
        loc: Location,
        ptr_depth: usize,
    ) -> Option<bool>;
    fn field_result(&self, def_id: LocalDefId, field: usize, ptr_depth: usize) -> Option<bool>;
}
