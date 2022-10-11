use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Local, Location},
    ty::TyCtxt,
};

use crate::ssa::consume::Consume;

pub mod constants;
pub mod constraint;
pub mod consume;
pub mod dom;
pub mod join_points;
pub mod state;

pub trait FnResult<'a> {
    type LocalResult;
    type LocationResults: Iterator<Item = (Local, Consume<Self::LocalResult>)>;

    fn local_result(&self, local: Local, location: Location) -> Option<Consume<Self::LocalResult>>;

    fn location_result(&'a self, location: Location) -> Self::LocationResults;
}

fn display_value<Value: std::fmt::Display>(value: &[Value]) -> String {
    "[".to_owned()
        + &value
            .iter()
            .map(|value| format!("{value}"))
            .collect::<Vec<_>>()
            .join(" ")
        + "]"
}

pub trait AnalysisResults<'a> {
    type Value: std::fmt::Display + 'a;
    type FnSig: Iterator<Item = Option<&'a [Self::Value]>>;
    type FnResult: FnResult<'a, LocalResult = &'a [Self::Value]>;

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult>;

    fn fn_sig(&'a self, r#fn: DefId) -> Self::FnSig;
    fn print_fn_sigs(&'a self, tcx: TyCtxt, fns: &[DefId]) {
        for &did in fns {
            let mut fn_sig = self.fn_sig(did);
            let ret = fn_sig.next().unwrap();
            let ret = if let Some(sig) = ret {
                // format!("{:?}", sig)
                display_value(sig)
            } else {
                "_".to_owned()
            };
            let args = fn_sig
                .map(|sig| {
                    if let Some(sig) = sig {
                        // format!("{:?}", sig)
                        display_value(sig)
                    } else {
                        "_".to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            let fn_path = tcx.def_path_str(did);
            println!("{fn_path}: ({args}) -> {ret}")
        }
    }
}