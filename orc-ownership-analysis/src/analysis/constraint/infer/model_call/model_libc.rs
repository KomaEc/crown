use std::ops::Range;

use rustc_span::symbol::Ident;

use crate::analysis::{
    constraint::{
        infer::{InferCtxt, Mode},
        Database, OwnershipSig,
    },
    def::Consume,
    FnSig, AnalysisKind,
};

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx, Kind: AnalysisKind> InferCtxt<'infercx, 'tcx, DB, Kind> {
    pub fn model_libc_call(
        &mut self,
        fn_sig: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.model_malloc(fn_sig),
            "free" => self.model_free(fn_sig),
            _ => {}
        }
    }

    fn model_malloc(&mut self, fn_sig: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = fn_sig;
        let destination = destination.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert!(args[0].is_none());
        <Kind as Mode>::source(self, destination.clone());
    }

    fn model_free(&mut self, fn_sig: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = fn_sig;
        assert!(destination.is_none());
        assert_eq!(args.len(), 1);
        let arg = args[0].clone().unwrap();
        <Kind as Mode>::sink(self, arg);
    }
}
