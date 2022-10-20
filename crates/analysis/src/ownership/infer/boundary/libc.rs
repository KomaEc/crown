use std::ops::Range;

use rustc_span::symbol::Ident;

use crate::{
    ownership::{infer::{InferCtxt, CallArgs}, AnalysisKind},
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
    pub fn libc_call(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.call_malloc(destination),
            "realloc" => self.call_realloc(destination, args),
            "calloc" => self.call_calloc(destination),
            "free" => self.call_free(args),
            _ => {}
        }
    }

    fn call_malloc(&mut self, destination: Option<Consume<Range<Var>>>) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn call_calloc(&mut self, destination: Option<Consume<Range<Var>>>) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn call_realloc(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
    ) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
        let (arg, is_ref) = args[0].clone().unwrap();
        assert!(!is_ref);
        <Analysis as InferMode>::sink(self, arg);
    }

    fn call_free(&mut self, args: &CallArgs) {
        assert_eq!(args.len(), 1);
        let (arg, is_ref) = args[0].clone().unwrap();
        assert!(!is_ref);
        <Analysis as InferMode>::sink(self, arg);
    }
}
