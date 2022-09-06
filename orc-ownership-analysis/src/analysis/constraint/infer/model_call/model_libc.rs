use std::ops::Range;

use rustc_span::symbol::Ident;

use crate::analysis::{
    constraint::{
        infer::{InferCtxt, Mode, WithCtxt},
        Database, OwnershipSig,
    },
    def::Consume,
};

impl<'infercx, 'tcx: 'infercx, DB: Database + 'infercx> InferCtxt<'infercx, 'tcx, DB> {
    pub(crate) fn model_libc_call(
        &mut self,
        func_sig: &(
            Option<Consume<Range<OwnershipSig>>>,
            Vec<Option<Consume<Range<OwnershipSig>>>>,
        ),
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.model_malloc(func_sig),
            "free" => self.model_free(func_sig),
            _ => {}
        }
    }

    fn model_malloc(
        &mut self,
        (destination, args): &(
            Option<Consume<Range<OwnershipSig>>>,
            Vec<Option<Consume<Range<OwnershipSig>>>>,
        ),
    ) {
        let destination = destination.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert!(args[0].is_none());
        <WithCtxt as Mode>::source(self, destination.clone());
    }

    fn model_free(
        &mut self,
        (destination, args): &(
            Option<Consume<Range<OwnershipSig>>>,
            Vec<Option<Consume<Range<OwnershipSig>>>>,
        ),
    ) {
        assert!(destination.is_none());
        assert_eq!(args.len(), 1);
        let arg = args[0].clone().unwrap();
        <WithCtxt as Mode>::sink(self, arg);
    }
}
