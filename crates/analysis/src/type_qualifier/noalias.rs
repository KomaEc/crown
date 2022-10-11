use alias::AliasResult;
use rustc_hash::FxHashSet;
use rustc_index::vec::Idx;
use rustc_middle::mir::{Body, Local};

use super::flow_insensitive::mutability::MutabilityResult;

pub type NoAliasParams = FxHashSet<Local>;

pub fn unsound_noalias_params(body: &Body, alias_result: &AliasResult) -> NoAliasParams {
    let mut noalias_params = body
        .args_iter()
        .filter(|&arg| !body.local_decls[arg].ty.is_primitive_ty())
        .collect::<NoAliasParams>();

    let location_of = alias_result.local_locations(&body.source.def_id());

    // too conservative!!! `is_null()`, cast by `assume`, etc..
    for arg in body.args_iter().map(|arg| arg.index()) {
        for (local, local_decl) in body.local_decls.iter_enumerated() {
            if local_decl.ty.is_primitive_ty() {
                continue;
            }
            let local = local.index();
            if local == arg {
                continue;
            }
            if alias_result.may_alias(location_of[arg], location_of[local]) {
                noalias_params.remove(&Local::new(arg));
            }
        }
    }

    noalias_params
}

/// A pointer parameter is marked `noalias` if and only if it is guaranteed
/// that there is no other pointers in-scope being alias of this parameter, and
/// that this parameter is only mutated through first-level indirection.
pub fn conservative_noalias_params(
    body: &Body,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> NoAliasParams {
    todo!()
}
