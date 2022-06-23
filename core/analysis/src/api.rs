use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::{Local, Location, Field};

pub trait AnalysisResults {
    fn local_result(&self, func: LocalDefId, local: Local, ptr_depth: usize) -> Option<bool>;
    fn local_result_at(
        &self,
        func: LocalDefId,
        local: Local,
        loc: Location,
        ptr_depth: usize,
    ) -> Option<bool>;
    fn field_result(&self, def_id: LocalDefId, field: Field, ptr_depth: usize) -> Option<bool>;
}
