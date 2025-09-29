//! utils for working with HIR and MIR

use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::{query::IntoQueryParam, ty::TyCtxt};
use rustc_span::Symbol;

#[inline]
pub fn def_id_to_symbol(id: impl IntoQueryParam<DefId>, tcx: TyCtxt<'_>) -> Option<Symbol> {
    let key = tcx.def_key(id);
    let (DefPathData::ValueNs(name) | DefPathData::TypeNs(name)) = key.disambiguated_data.data
    else {
        return None;
    };
    Some(name)
}

#[inline]
pub fn with_tcx<R, F: for<'tcx> FnOnce(TyCtxt<'tcx>) -> R>(f: F) -> R {
    rustc_middle::ty::tls::with_opt(|tcx| f(tcx.unwrap()))
}

#[inline]
pub fn fmt_def_id(
    f: &mut std::fmt::Formatter<'_>,
    key: impl IntoQueryParam<DefId>,
) -> std::fmt::Result {
    let def_id = key.into_query_param();
    rustc_middle::ty::tls::with_opt(|opt_tcx| {
        if let Some(tcx) = opt_tcx {
            write!(f, "{}", tcx.def_path_str(def_id))
        } else {
            write!(f, "{}", def_id.index.index())
        }
    })
}

pub mod ast_to_hir;
pub mod hir_to_thir;
pub mod thir_to_mir;

pub use ast_to_hir::*;
pub use hir_to_thir::*;
pub use thir_to_mir::*;

pub struct IrMappings<'a> {
    pub ast_to_hir: &'a AstToHir,
    pub hir_to_thir: &'a HirToThir,
}
