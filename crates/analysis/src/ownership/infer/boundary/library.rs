use std::ops::Range;

use rustc_hir::def_id::DefId;

use crate::{
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
    pub fn library_call(
        &mut self,
        _destination: Option<Consume<Range<Var>>>,
        args: &<Analysis as InferMode<'infercx, 'db, 'tcx>>::CallArgs,
        callee: DefId,
    ) {
        let def_path = self.fn_ctxt.tcx.def_path(callee);
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
                        self.call_is_null(args);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn call_is_null(&mut self, args: &<Analysis as InferMode<'infercx, 'db, 'tcx>>::CallArgs) {
        assert_eq!(args.len(), 1);
        if let Some((arg, is_ref)) = args[0].clone() {
            assert!(!is_ref);
            <Analysis as InferMode>::lend(self, arg)
        }
    }
}
