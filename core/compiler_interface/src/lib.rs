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
use rustc_errors::registry;
use rustc_hir::def_id::LocalDefId;
use rustc_hir::OwnerNode;
use rustc_interface::{interface::Compiler, Config};
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use std::path::PathBuf;
use std::process;
use std::str;

/// Run compiler with input source string that consists of a single function.
/// This is mainly used for testing
pub fn run_compiler_with_input_str_with_single_func<F>(input: &'static str, f: F)
where
    F: FnOnce(TyCtxt, LocalDefId) + Send,
{
    rustc_interface::run_compiler(Config::with_input_str(input), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();
                let fn_dids = hir_krate
                    .owners
                    .iter_enumerated()
                    .filter_map(|(did, owner_info)| {
                        if let Some(owner_info) = owner_info {
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
