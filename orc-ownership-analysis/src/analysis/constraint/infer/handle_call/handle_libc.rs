use std::ops::Range;

use rustc_span::symbol::Ident;

use crate::analysis::{
    constraint::{
        infer::{InferCtxt, InferMode},
        Database, OwnershipSig,
    },
    consume::Consume,
    AnalysisKind, FnSig,
};

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx, Kind: AnalysisKind<'infercx>>
    InferCtxt<'infercx, 'tcx, DB, Kind>
{
    pub fn handle_libc_call(
        &mut self,
        caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>,
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.handle_malloc(caller),
            "free" => self.handle_free(caller),
            _ => {}
        }
    }

    fn handle_malloc(&mut self, caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        let destination = destination.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert!(args[0].is_none());
        <Kind as InferMode<_>>::source(self, destination.clone());
    }

    fn handle_free(&mut self, caller: &FnSig<Option<Consume<Range<OwnershipSig>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        assert!(destination.is_none());
        assert_eq!(args.len(), 1);
        let arg = args[0].clone().unwrap();
        <Kind as InferMode<_>>::sink(self, arg);
    }
}
