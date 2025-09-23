//! utils for working with HIR and MIR

use crate::ast_util;
use etrace::some_or;
use rustc_ast::{Crate, visit::Visitor};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::{query::IntoQueryParam, ty::TyCtxt};
use rustc_parse;
use rustc_span::{FileName, RealFileName, Symbol};
use std::path::{Path, PathBuf};

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

pub use ast_to_hir::*;
pub use hir_to_thir::*;

pub struct IrMappings {
    pub ast_to_hir: AstToHir,
    pub hir_to_thir: HirToThir,
}

impl IrMappings {
    pub fn new(dir: &Path, tcx: TyCtxt<'_>) -> Self {
        let borrowed = tcx.resolver_for_lowering().borrow();
        let mut expanded_crate = borrowed.1.as_ref().clone();
        drop(borrowed);

        let mut path_to_mod_id = FxHashMap::default();
        tcx.hir_for_each_module(|mod_id| {
            let def_path = tcx.def_path(mod_id.to_def_id());
            let mut path = dir.to_path_buf();
            for data in def_path.data {
                let DefPathData::TypeNs(name) = data.data else {
                    panic!()
                };
                path.push(name.as_str());
            }
            path.set_extension("rs");
            path_to_mod_id.insert(path, mod_id);
        });

        let source_map = tcx.sess.source_map();
        let parse_sess = ast_util::new_parse_sess();

        for file in source_map.files().iter() {
            let p = match &file.name {
                FileName::Real(RealFileName::LocalPath(p)) => p.clone(),
                FileName::Custom(p) => PathBuf::from(p),
                _ => continue,
            };
            let src = some_or!(file.src.as_ref(), continue);
            let _name = p.file_name().unwrap().to_str().unwrap();
            if _name == "c2rust-lib.rs" || _name == "lib.rs" {
                continue;
            }
            let mut parser = rustc_parse::new_parser_from_source_str(
                &parse_sess,
                file.name.clone(),
                src.to_string(),
            )
            .unwrap();
            let mut krate = parser.parse_crate_mod().unwrap();
            let mod_id = path_to_mod_id[&p];
            let (module, _, _) = tcx.hir_get_module(mod_id);
            let mut mapper = AstToHirMapper::new(tcx);
            mapper.map_crate_to_mod(&mut krate, module, false);
            let mut checker = AstToHirChecker {
                tcx,
                ast_to_hir: mapper.ast_to_hir,
            };
            for item in &krate.items {
                checker.visit_item(item);
            }
        }

        let mut mapper = AstToHirMapper::new(tcx);
        let module = tcx.hir_root_module();
        mapper.map_crate_to_mod(&mut expanded_crate, module, true);

        let hir_to_thir = map_hir_to_thir(tcx);

        IrMappings {
            ast_to_hir: mapper.ast_to_hir,
            hir_to_thir,
        }
    }
}
