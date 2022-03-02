#![feature(rustc_private)]
#![feature(bool_to_option)]

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

// pub mod initial_rewrite;
// pub mod print_struct;
// #[cfg(test)]
// mod test;

use crustr_analysis::{call_graph::CallGraph, array_analysis::CrateSummary, def_use::BorrowckDefUse, ssa::rename::handler::LogSSAName};
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;

pub fn array_analysis<'tcx>(tcx: TyCtxt<'tcx>, structs: Vec<LocalDefId>, funcs: Vec<LocalDefId>) {
    let bodies = funcs
        .iter()
        .map(|&fn_did| {
            let body = tcx.optimized_mir(fn_did);
            rustc_middle::mir::pretty::write_mir_fn(
                tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();
            fn_did.to_def_id()
        })
        .collect::<Vec<_>>();

    let adt_defs = structs
        .into_iter()
        .map(|did| did.to_def_id())
        .collect::<Vec<_>>();

        let call_graph = CallGraph::new(tcx, bodies.into_iter());
        let mut crate_summary = CrateSummary::new(tcx, &adt_defs, call_graph);
        crate_summary.debug();
        crate_summary.infer::<BorrowckDefUse, LogSSAName>(LogSSAName);
        // assert_eq!(crate_summary.call_graph.num_nodes(), 1);
    
        crate_summary.iterate_to_fixpoint().unwrap();
        let solutions = crate_summary.lambda_ctxt.lambda_map.assumptions;
    
        log::debug!("All constraints:");
        for constraint in crate_summary.constraints {
            log::debug!("{}", constraint)
        }
    
        for (lambda, solution) in solutions.into_iter_enumerated() {
            log::debug!(
                "{: <7} = {: <2}, with source data {}",
                &format!("{:?}", lambda),
                solution
                    .map(|fat| { fat.then_some("1").unwrap_or("0") })
                    .unwrap_or("?"),
                crate_summary.lambda_ctxt.lambda_map.data_map[lambda]
            )
        }
}

/* 
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
*/