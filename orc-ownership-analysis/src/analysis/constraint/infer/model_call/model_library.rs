use std::ops::Range;

use rustc_hir::def_id::DefId;

use crate::analysis::{
    constraint::{
        infer::{InferCtxt, Mode, WithCtxt},
        Database, OwnershipSig,
    },
    def::Consume,
    FnSig,
};

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx> InferCtxt<'infercx, 'tcx, DB> {
    pub fn model_library_call(
        &mut self,
        fn_sig: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
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
                        self.model_is_null(fn_sig);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn model_is_null(&mut self, fn_sig: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig { args, .. } = fn_sig;
        assert_eq!(args.len(), 1);
        let arg = args.first().map(Option::as_ref).flatten().cloned().unwrap();
        <WithCtxt as Mode>::borrow(self, arg)
    }
}
