use alias::AliasResult;
use rustc_hash::FxHashSet;
use rustc_index::vec::Idx;
use rustc_middle::mir::{Body, Local};

use super::flow_insensitive::mutability::{Mutability, MutabilityResult};

pub type NoAliasParams = FxHashSet<Local>;

pub fn show_noalias_params(
    crate_ctxt: &crate::CrateCtxt,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) {
    fn show_set<T: std::fmt::Debug>(set: impl Iterator<Item = T>) -> String {
        set.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }

    for &did in crate_ctxt.fns() {
        let body = crate_ctxt.tcx.optimized_mir(did);

        let noalias_params = conservative_noalias_params(body, alias_result, mutability_result);
        let noalias_params_str = show_set(noalias_params.iter());

        println!(
            "@{}: {noalias_params_str}",
            crate_ctxt.tcx.def_path_str(did)
        );
    }
}

/// A pointer parameter is marked `noalias` if and only if it is guaranteed
/// that there is no other pointers in-scope being alias of this parameter
pub fn conservative_noalias_params(
    body: &Body,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> NoAliasParams {
    let mut noalias_params = body
        .args_iter()
        .filter(|&arg| !body.local_decls[arg].ty.is_primitive_ty())
        .collect::<NoAliasParams>();

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
