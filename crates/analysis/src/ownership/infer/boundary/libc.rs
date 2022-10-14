use std::ops::Range;

use rustc_span::symbol::Ident;

use crate::{
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
    pub fn libc_call(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &<Analysis as InferMode<'infercx, 'db, 'tcx>>::CallArgs,
        callee: Ident,
    ) {
        match callee.as_str() {
            "malloc" => self.call_malloc(destination),
            "free" => self.call_free(args),
            // "malloc" => self.call_malloc(caller),
            // "free" => self.call_free(caller),
            _ => {}
        }
    }

    fn call_malloc(&mut self, destination: Option<Consume<Range<Var>>>) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn call_free(&mut self, args: &<Analysis as InferMode<'infercx, 'db, 'tcx>>::CallArgs) {
        assert_eq!(args.len(), 1);
        let (arg, is_ref) = args[0].clone().unwrap();
        assert!(!is_ref);
        <Analysis as InferMode>::sink(self, arg);
    }
}
