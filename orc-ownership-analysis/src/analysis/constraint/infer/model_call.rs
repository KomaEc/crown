use std::ops::Range;

use itertools::izip;
use rustc_hir::def_id::DefId;

use crate::analysis::{
    constraint::{infer::InferCtxt, Database, OwnershipSig},
    def::Consume,
    AnalysisKind, FnSig, WholeProgram,
};

pub mod model_libc;
pub mod model_library;

pub trait ModelCall: AnalysisKind + Sized {
    fn model_call<'infercx, 'tcx: 'infercx, DB: Database + 'infercx>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: DefId,
    );
}

impl<K: AnalysisKind> ModelCall for K {
    default fn model_call<'infercx, 'tcx: 'infercx, DB: Database + 'infercx>(
        _: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        _: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        _: DefId,
    ) {
    }
}

impl ModelCall for WholeProgram {
    fn model_call<'infercx, 'tcx: 'infercx, DB: Database + 'infercx>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB, Self>,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: DefId,
    ) {
        let callee = &infer_cx.inter_ctxt[&callee];

        #[cfg(debug_assertions)]
        assert_eq!(callee.iter().count(), caller.iter().count());

        let mut callee_caller = callee.iter().zip(caller.iter());

        // dest = ret ~> rho(dest) = 0, rho(dest') = rho(ret)
        let (ret, dest) = callee_caller.next().unwrap();
        if let Some(ret) = ret.clone() {
            if let Some(Consume {
                r#use: dest_use,
                def: dest_def,
            }) = dest.clone()
            {
                for (ret, dest_use, dest_def) in izip!(ret, dest_use, dest_def) {
                    infer_cx
                        .database
                        .push_assume::<crate::analysis::constraint::Debug>((), dest_use, false);
                    infer_cx
                        .database
                        .push_equal::<crate::analysis::constraint::Debug>((), dest_def, ret);
                }
            }
        } else {
            assert!(dest.is_none())
        }

        // para = arg ~> rho(para') + rho(arg') = rho(arg)
        for (para, arg) in callee_caller {
            if let Some(para) = para.clone() {
                if let Some(Consume {
                    r#use: arg_use,
                    def: arg_def,
                }) = arg.clone()
                {
                    for (para, arg_use, arg_def) in izip!(para, arg_use, arg_def) {
                        infer_cx
                            .database
                            .push_linear::<crate::analysis::constraint::Debug>(
                                (),
                                para,
                                arg_def,
                                arg_use,
                            );
                    }
                }
            } else {
                assert!(arg.is_none())
            }
        }
    }
}
