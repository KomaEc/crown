#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

pub mod initial_rewrite;

use rustc_middle::ty::TyCtxt;

pub trait Pass<'tcx> {
    type Input;
    type Output;

    fn tcx(&'tcx self) -> TyCtxt<'tcx>;

    fn run_pass(&mut self, input: Self::Input) -> Self::Output;
}

pub struct Chain<T, O> {
    this: T,
    other: O,
}

impl<'tcx, T, O> Pass<'tcx> for Chain<T, O>
where
    T: 'tcx + Pass<'tcx>,
    O: 'tcx + Pass<'tcx, Input = T::Output>,
{
    type Input = T::Input;
    type Output = O::Output;

    fn tcx(&'tcx self) -> TyCtxt<'tcx> {
        self.this.tcx()
    }

    fn run_pass(&mut self, input: Self::Input) -> Self::Output {
        self.other.run_pass(self.this.run_pass(input))
    }
}
