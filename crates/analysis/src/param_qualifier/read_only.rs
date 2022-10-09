use alias::AliasResult;
use rustc_hash::FxHashSet;
use rustc_middle::mir::{
    visit::{MutatingUseContext, PlaceContext, Visitor},
    Body, Local,
};

pub type ReadOnlyParams = FxHashSet<Local>;

/// This is unsound. A sound one should be interprocedural, as call arguments
/// require fixpoint computation.
pub fn read_only_params(body: &Body, alias_result: &AliasResult) -> ReadOnlyParams {
    let mut read_only_params = body
        .args_iter()
        .filter(|&arg| !body.local_decls[arg].ty.is_primitive_ty())
        .collect::<ReadOnlyParams>();

    struct Prune<'me, 'tcx>(&'me mut ReadOnlyParams, &'me Body<'tcx>, &'me AliasResult);
    impl<'me, 'tcx> Visitor<'tcx> for Prune<'me, 'tcx> {
        fn visit_local(
            &mut self,
            a: Local,
            context: PlaceContext,
            _location: rustc_middle::mir::Location,
        ) {
            if let PlaceContext::MutatingUse(mutating_use_context) = context {
                if let MutatingUseContext::Store = mutating_use_context {
                    return;
                }
                let alias_result = self.2;
                let body = self.1;
                let location_of = alias_result.local_locations(&body.source.def_id());
                for b in body.local_decls.indices() {
                    if alias_result.may_alias(location_of[a.index()], location_of[b.index()]) {
                        self.0.remove(&b);
                    }
                }
            }
        }
    }

    Prune(&mut read_only_params, body, alias_result).visit_body(body);

    read_only_params
}
