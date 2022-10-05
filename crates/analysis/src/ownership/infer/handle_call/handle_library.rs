use std::ops::Range;

use rustc_hir::def_id::DefId;

use crate::{
    call_graph::FnSig,
    ownership::{infer::InferCtxt, AnalysisKind},
    ssa::{
        constraint::{infer::InferMode, Var},
        consume::Consume,
    },
};

impl<'infercx, 'db, 'tcx, Analysis> InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db>,
{
    pub fn handle_library_call(
        &mut self,
        caller: &FnSig<Option<Consume<Range<Var>>>>,
        callee: DefId,
    ) {
        let def_path = self.crate_ctxt.tcx.def_path(callee);
        // if it is a library call in core::ptr
        if def_path
            .data
            .get(0)
            .map(|d| match d.data {
                rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                _ => false,
            })
            .is_some()
        {
            // if it is core::ptr::<..>::..
            if let Some(d) = def_path.data.get(3) {
                match d.data {
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                        self.handle_is_null(caller);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_is_null(&mut self, caller: &FnSig<Option<Consume<Range<Var>>>>) {
        let FnSig { args, .. } = caller;
        assert_eq!(args.len(), 1);
        let arg = args.first().and_then(Option::as_ref).cloned().unwrap();
        <Analysis as InferMode>::borrow(self, arg)
    }
}
