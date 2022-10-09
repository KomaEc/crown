use alias::AliasResult;
use rustc_hash::FxHashSet;
use rustc_index::vec::Idx;
use rustc_middle::mir::{Body, Local};

pub type NoAliasParams = FxHashSet<Local>;

pub fn conservative_unique_params(body: &Body, alias_result: &AliasResult) -> NoAliasParams {
    let mut unique_params = body
        .args_iter()
        .filter(|&arg| !body.local_decls[arg].ty.is_primitive_ty())
        .collect::<NoAliasParams>();

    let location_of = alias_result.local_locations(&body.source.def_id());

    for arg1 in body.args_iter().map(|arg| arg.index()) {
        for arg2 in arg1 + 1..body.arg_count + 1 {
            if alias_result.may_alias(location_of[arg1], location_of[arg2]) {
                unique_params.remove(&Local::new(arg1));
                unique_params.remove(&Local::new(arg2));
            }
        }
    }

    unique_params
}
