use std::ops::Range;

use rustc_hir::def_id::DefId;
use rustc_span::symbol::Ident;

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
    pub fn libc_call(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: DefId,
        callee_name: Ident,
    ) {
        match callee_name.as_str() {
            "malloc" => return self.call_malloc(destination),
            "realloc" => return self.call_realloc(destination, args),
            "calloc" => return self.call_calloc(destination),
            "free" => return self.call_free(args),
            "memset" => return self.call_memset(destination, args, callee),
            _ => {}
        }

        self.unknown_call(destination, args)
    }

    fn call_malloc(&mut self, destination: Option<Consume<Range<Var>>>) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn call_calloc(&mut self, destination: Option<Consume<Range<Var>>>) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
    }

    fn call_realloc(&mut self, destination: Option<Consume<Range<Var>>>, args: &CallArgs) {
        let destination = destination.as_ref().unwrap();
        <Analysis as InferMode>::source(self, destination.clone());
        let (arg, is_ref) = args[0].clone().unwrap();
        assert!(!is_ref);
        <Analysis as InferMode>::sink(self, arg);
    }

    fn call_free(&mut self, args: &CallArgs) {
        assert_eq!(args.len(), 1);
        let Some((arg, is_ref)) = args[0].clone() else { return };
        assert!(!is_ref);
        <Analysis as InferMode>::sink(self, arg);
    }

    fn call_memset(
        &mut self,
        destination: Option<Consume<Range<Var>>>,
        args: &CallArgs,
        callee: DefId,
    ) {
        let fn_sig = self.tcx.fn_sig(callee);
        let dest_ty = fn_sig.output().skip_binder();
        let arg_ty = fn_sig.inputs().skip_binder()[0];
        assert_eq!(dest_ty, arg_ty);
        let ty = dest_ty;

        let destination = destination.as_ref().cloned().unwrap();
        // <Analysis as InferMode>::source(self, destination.clone());
        let (arg, is_ref) = args[0].clone().unwrap();
        assert!(!is_ref);
        // <Analysis as InferMode>::sink(self, arg);

        // FIXME working around type cast
        let mut arg = arg;
        arg.r#use = arg.r#use.start..arg.r#use.start + 1u32;
        arg.def = arg.def.start..arg.def.start + 1u32;
        let arg = arg;

        <Analysis as InferMode>::transfer::<false>(self, ty, destination, arg)
    }
}
