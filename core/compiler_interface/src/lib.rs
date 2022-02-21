#![feature(rustc_private)]

pub mod config_ext;

extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use config_ext::ConfigExt;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::ItemKind;
use rustc_hir::OwnerNode;
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use std::path::PathBuf;
use std::str;

/// A simpler input type than `rustc_session::config::Input`
pub enum Input {
    File(PathBuf),
    Str(&'static str)
}

impl From<PathBuf> for Input {
    fn from(path: PathBuf) -> Self {
        Input::File(path)
    }
}

impl From<&'static str> for Input {
    fn from(str: &'static str) -> Self {
        Input::Str(str)
    }
}

impl Into<rustc_session::config::Input> for Input {
    fn into(self) -> rustc_session::config::Input {
        match self {
            Input::File(path) => rustc_session::config::Input::File(path),
            Input::Str(str) => rustc_session::config::Input::Str {
                name: rustc_span::FileName::Custom("main.rs".to_string()),
                input: str.to_string(),
            },
        }
    }
}

/// Run compiler with input source string that consists of a single function.
/// This is mainly used for testing
pub fn run_compiler_with_single_func<F>(input: Input, f: F)
where
    F: FnOnce(TyCtxt, LocalDefId) + Send,
{
    rustc_interface::run_compiler(Config::with_input(input.into()), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();
                let fn_dids = hir_krate
                    .owners
                    .iter_enumerated()
                    .filter_map(|(did, owner_info)| {
                        if let Some(owner_info) = owner_info.as_owner() {
                            if let OwnerNode::Item(item) = owner_info.node() {
                                if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                                    assert_eq!(item.def_id, did);

                                    return Some(did);
                                }
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                assert_eq!(fn_dids.len(), 1);
                let fn_did = fn_dids[0];

                f(tcx, fn_did)
            })
        })
    })
}

/*
pub fn run_compiler_with_file_with_single_func<F>(input: PathBuf, f: F)
where
    F: FnOnce(TyCtxt, LocalDefId) + Send,
{
    rustc_interface::run_compiler(Config::with_input_file(input), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();
                let fn_dids = hir_krate
                    .owners
                    .iter_enumerated()
                    .filter_map(|(did, owner_info)| {
                        if let Some(owner_info) = owner_info.as_owner() {
                            if let OwnerNode::Item(item) = owner_info.node() {
                                if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                                    assert_eq!(item.def_id, did);

                                    return Some(did);
                                }
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                assert_eq!(fn_dids.len(), 1);
                let fn_did = fn_dids[0];

                f(tcx, fn_did)
            })
        })
    })
}
*/

pub fn run_compiler_with_all_structs<F>(input: Input, f: F)
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>, Vec<LocalDefId>) + Send,
{
    rustc_interface::run_compiler(Config::with_input(input.into()), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();

                let mut struct_defs = vec![];

                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner.as_owner() {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(_variant_data, _generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                struct_defs.push(def_id)
                            }
                        }
                    }
                }

                f(tcx, struct_defs)
            })
        })
    })
}

pub fn run_compiler_with_struct_defs_and_single_func<F>(input: Input, f: F)
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>, Vec<LocalDefId>, LocalDefId) + Send,
{
    rustc_interface::run_compiler(Config::with_input(input.into()), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();

                let mut struct_defs = vec![];
                let mut fn_dids = vec![];

                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner.as_owner() {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(_variant_data, _generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                struct_defs.push(def_id)
                            } else if let ItemKind::Fn(_, _, _) = item.kind {
                                assert_eq!(item.def_id, did);

                                fn_dids.push(did);
                            }
                        }
                    }
                }

                assert_eq!(fn_dids.len(), 1);

                f(tcx, struct_defs, fn_dids[0])
            })
        })
    })
}

pub fn run_compiler_with_struct_defs_and_funcs<F>(input: Input, f: F)
where
    F: for<'tcx> FnOnce(TyCtxt<'tcx>, Vec<LocalDefId>, Vec<LocalDefId>) + Send,
{
    rustc_interface::run_compiler(Config::with_input(input.into()), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();

                let mut struct_defs = vec![];
                let mut fn_dids = vec![];

                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner.as_owner() {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(_variant_data, _generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                struct_defs.push(def_id)
                            } else if let ItemKind::Fn(_, _, _) = item.kind {
                                assert_eq!(item.def_id, did);

                                fn_dids.push(did);
                            }
                        }
                    }
                }

                f(tcx, struct_defs, fn_dids)
            })
        })
    })
}
