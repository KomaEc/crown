use std::ops::Range;

use rustc_span::symbol::Ident;

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
    pub fn model_libc_call(
        &mut self,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.model_malloc(caller),
            "free" => self.model_free(caller),
            _ => {}
        }
    }

    fn model_malloc(&mut self, caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        let destination = destination.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert!(args[0].is_none());
        <Kind as Mode>::source(self, destination.clone());
    }

    fn model_free(&mut self, caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        assert!(destination.is_none());
        assert_eq!(args.len(), 1);
        let arg = args[0].clone().unwrap();
        <Kind as Mode>::sink(self, arg);
    }
}
