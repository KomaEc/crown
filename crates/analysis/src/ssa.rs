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

pub trait AnalysisResults<'a> {
    type Value: 'a;
    type Param: 'a;
    type FnSig: Iterator<Item = Option<Self::Param>>;
    type FnResult: FnResult<'a, LocalResult = &'a [Self::Value]>;

    fn fn_result(&'a self, r#fn: DefId) -> Option<Self::FnResult>;

    fn fn_sig(&'a self, r#fn: DefId) -> Self::FnSig;
    fn print_fn_sigs(&'a self, tcx: TyCtxt, fns: &[DefId]);
}
