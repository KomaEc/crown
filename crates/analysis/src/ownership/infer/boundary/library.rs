use std::ops::Range;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::SubstsRef;

use crate::{
    ownership::{
        infer::{CallArgs, InferCtxt},
        AnalysisKind,
    },
    ssa::{
        constraint::{infer::InferMode, Var},
        consume::Consume,
    },
};

impl<'infercx, 'db, 'tcx, Analysis> InferCtxt<'infercx, 'db, 'tcx, Analysis>
where
    'tcx: 'infercx,
    Analysis: AnalysisKind<'infercx, 'db, 'tcx>,
{
    pub fn library_call(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: DefId,
        _substs: SubstsRef<'tcx>,
    ) {
        let def_path = self.tcx.def_path(callee);
        // if it is a library call in core::ptr
        if matches!(def_path
            .data
            .get(0)
            .map(|d| match d.data {
                rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
                _ => false,
            }), Some(cond) if cond)
        {
            // if it is core::ptr::<..>::..
            if let Some(d) = def_path.data.get(3) {
                if let rustc_hir::definitions::DefPathData::ValueNs(s) = d.data {
                    match s.as_str() {
                        "is_null" => {
                            self.call_is_null(args);
                            return;
                        }
                        "offset" => {
                            self.call_offset(destination, args);
                            return;
                        }
                        "addr" => {
                            self.call_addr(args);
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.unknown_call(destination, args)
    }

    /// Huge Hack right there
    pub fn call_addr(&mut self, args: &CallArgs) {
        assert_eq!(args.len(), 1);
        if let Some((arg, is_ref)) = args[0].clone() {
            assert!(!is_ref);
            let _ = arg; // ignore
        }
    }

    pub fn call_is_null(&mut self, args: &CallArgs) {
        assert_eq!(args.len(), 1);
        if let Some((arg, is_ref)) = args[0].clone() {
            assert!(!is_ref);
            <Analysis as InferMode>::lend(self, arg)
        }
    }

    /// limitation!!!
    /// TODO special casing offset. If a call to [`offset`] is followed immediately by
    /// a dereference, then this call is guaranteed to be a short-time borrow!!
    pub fn call_offset(&mut self, destination: Option<Consume<Range<Var>>>, args: &CallArgs) {
        let Some(dest) = destination else { return };
        let Some((arg, is_ref)) = args[0].clone() else { return };
        assert!(!is_ref, "offset(&mut x) is not supported");

        // let fn_sig = self.tcx.fn_sig(offset_did);
        // let ty = EarlyBinder(fn_sig.output().skip_binder()).subst(self.tcx, substs);

        <Analysis as InferMode>::lend(self, arg);
        <Analysis as InferMode>::borrow(self, dest);
    }
}
