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

// fn prune(read_only_params: &mut ReadOnlyParams, body: &Body, alias_result: &AliasResult) {
//     for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
//         let BasicBlockData { statements, .. } = bb_data;
//         let mut index = 0;
//         let mut deref_copy: Option<Local> = None;
//         for statement in statements {
//             let location = Location {
//                 block: bb,
//                 statement_index: index,
//             };
//             if let StatementKind::Assign(box (_, rvalue)) = &statement.kind
//                 && let Rvalue::CopyForDeref(place) = rvalue
//             {
//                 let base = deref_copy.take().or(Some(place.local));
//                 deref_copy = base;
//                 index += 1;
//                 continue;
//             }
//             if let Some(base) = deref_copy.take() {
//                 let StatementKind::Assign(box (lhs, _)) = &statement.kind else { unreachable!() };

//                 // if deref copy is mutated
//                 if matches!(
//                     body.local_decls[lhs.local].local_info,
//                     Some(box LocalInfo::DerefTemp)
//                 ) {
//                     let location_of = alias_result.local_locations(&body.source.def_id());
//                     for arg in body.args_iter() {
//                         if alias_result
//                             .may_alias(location_of[base.index()], location_of[arg.index()])
//                         {
//                             read_only_params.remove(&arg);
//                         }
//                     }
//                 }

//                 index += 1;
//                 continue;
//             }

//             // otherwise, prune normally

//             index += 1;
//         }
//     }
// }

// common::macros::newtype_index! {
//     pub struct Mu {
//         DEBUG_FORMAT = "µ_{}"
//     }
// }

// pub struct Immutability {
//     struct_fields: StructFieldsInfo<Mu>,
//     fn_locals: ()
// }
