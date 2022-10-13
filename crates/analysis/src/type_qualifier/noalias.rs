use alias::AliasResult;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::def_id::DefId;
use rustc_index::vec::Idx;
use rustc_middle::mir::{Body, Local};

use super::flow_insensitive::mutability::{Mutability, MutabilityResult};

pub type NoAliasParams = FxHashMap<DefId, FxHashSet<Local>>;

pub fn show_noalias_params(
    crate_data: &common::CrateData,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) {
    fn show_set<T: std::fmt::Debug>(set: impl Iterator<Item = T>) -> String {
        set.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }

    let noalias_params = compute_noalias_params(crate_data, alias_result, mutability_result);

    for (did, noalias_params) in noalias_params {
        let noalias_params_str = show_set(noalias_params.into_iter());
        println!(
            "@{}: {noalias_params_str}",
            crate_data.tcx.def_path_str(did)
        );
    }
}

pub fn compute_noalias_params(
    crate_data: &common::CrateData,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> NoAliasParams {
    let mut noalias_params = FxHashMap::default();
    noalias_params.reserve(crate_data.fns.len());

    for &did in &crate_data.fns {
        let body = crate_data.tcx.optimized_mir(did);
        noalias_params.insert(
            did,
            conservative_noalias_params(body, alias_result, mutability_result),
        );
    }

    noalias_params
}

/// A pointer parameter is marked `noalias` if and only if it is guaranteed
/// that there is no other pointers in-scope being alias of this parameter
fn conservative_noalias_params(
    body: &Body,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> FxHashSet<Local> {
    let mut noalias_params = body
        .args_iter()
        .filter(|&arg| !body.local_decls[arg].ty.is_primitive_ty())
        .collect::<FxHashSet<Local>>();

    let location_of = alias_result.local_locations(&body.source.def_id());
    let fn_result = mutability_result.fn_result(&body.source.def_id());

    for arg in body.args_iter().map(|arg| arg.index()) {
        for (local, local_decl) in body.local_decls.iter_enumerated() {
            if local_decl.ty.is_primitive_ty() {
                continue;
            }
            let local = local.index();
            if local == arg {
                continue;
            }
            if alias_result.may_alias(location_of[arg], location_of[local])
                && fn_result
                    .local_result(Local::new(local))
                    .first()
                    .is_some_and(|&&mutability| mutability == Mutability::Mut)
            {
                noalias_params.remove(&Local::new(arg));
            }
        }
    }

    noalias_params
}
