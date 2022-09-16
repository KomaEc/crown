use std::ops::Range;

use rustc_hir::def_id::DefId;

use crate::analysis::{
    constraint::{
        infer::{InferCtxt, Mode},
        Database, OwnershipSig,
    },
    def::Consume,
    AnalysisKind, FnSig,
};

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx, Kind: AnalysisKind>
    InferCtxt<'infercx, 'tcx, DB, Kind>
{
    pub fn handle_library_call(
        &mut self,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
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
                    // if it is core::ptr::<..>::offset
                    // rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "offset" => {
                    //     tracing::debug!("modelling ptr offset");
                    //     self.model_ptr_offset(args, destination, location);
                    //     return;
                    // }
                    // if it is core::ptr::<..>::is_null
                    rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                        self.handle_is_null(caller);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn handle_is_null(&mut self, caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig { args, .. } = caller;
        assert_eq!(args.len(), 1);
        let arg = args.first().and_then(Option::as_ref).cloned().unwrap();
        <Kind as Mode>::borrow(self, arg)
    }
}
