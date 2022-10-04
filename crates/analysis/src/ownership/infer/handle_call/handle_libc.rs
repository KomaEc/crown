use std::ops::Range;

use rustc_span::symbol::Ident;

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
    pub fn handle_libc_call(&mut self, caller: &FnSig<Option<Consume<Range<Var>>>>, callee: Ident) {
        match callee.as_str() {
            "malloc" => self.handle_malloc(caller),
            "free" => self.handle_free(caller),
            _ => {}
        }
    }

    fn handle_malloc(&mut self, caller: &FnSig<Option<Consume<Range<Var>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        let destination = destination.as_ref().unwrap();
        assert_eq!(args.len(), 1);
        assert!(args[0].is_none());
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn handle_free(&mut self, caller: &FnSig<Option<Consume<Range<Var>>>>) {
        let FnSig {
            ret: destination,
            args,
        } = caller;
        assert!(destination.is_none());
        assert_eq!(args.len(), 1);
        let arg = args[0].clone().unwrap();
        <Analysis as InferMode>::sink(self, arg);
    }
}
