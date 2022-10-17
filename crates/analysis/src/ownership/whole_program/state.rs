use either::Either;
use rustc_data_structures::sso::SsoHashSet;
use rustc_hash::FxHashMap;
use rustc_middle::mir::{BasicBlockData, Body, Location, Rvalue, StatementKind};

use super::WholeProgramInterCtxt;
use crate::{
    call_graph::FnSig,
    ownership::{infer::FnSummary, Ownership, Param},
    ssa::{
        constraint::{initialize_local, Database, Gen, Z3Database},
        consume::{initial_definitions, Consume},
        dom::compute_dominance_frontier,
        state::SSAState,
        FnResult,
    },
    type_qualifier::noalias::NoAliasParams,
    CrateCtxt,
};

pub(super) fn initial_inter_ctxt(
    crate_ctxt: &CrateCtxt,
    noalias_params: &NoAliasParams,
    gen: &mut Gen,
    database: &mut Z3Database,
) -> WholeProgramInterCtxt {
    let mut fn_sigs = FxHashMap::default();
    fn_sigs.reserve(crate_ctxt.fns().len());
    for &did in crate_ctxt.call_graph.fns() {
        let noalias_params = &noalias_params[&did];
        let body = crate_ctxt.tcx.optimized_mir(did);
        let fn_sig = {
            let mut local_decls = body.local_decls.iter_enumerated();
            let (_, return_local_decl) = local_decls.next().unwrap();
            let ret = initialize_local(return_local_decl, gen, database, crate_ctxt.with_allowed_depth(1))
                .map(|sigs| Param::Normal(sigs));

            let args = local_decls
                .take(body.arg_count)
                .map(|(local, local_decl)| {
                    if noalias_params.contains(&local) {
                        let r#use = initialize_local(local_decl, gen, database, crate_ctxt.with_allowed_depth(1));
                        let def = initialize_local(local_decl, gen, database, crate_ctxt.with_allowed_depth(1));
                        r#use.zip(def).map(|(r#use, def)| {
                            database.push_assume::<crate::ssa::constraint::Debug>(
                                (),
                                r#use.start,
                                true,
                            );
                            database.push_assume::<crate::ssa::constraint::Debug>(
                                (),
                                def.start,
                                true,
                            );
                            Param::Output(Consume { r#use, def })
                        })
                    } else {
                        initialize_local(local_decl, gen, database, crate_ctxt.with_allowed_depth(1))
                            .map(|sigs| Param::Normal(sigs))
                    }
                })
                .collect();

            FnSig { ret, args }
        };
        println!("generating signatures for {:?}: {:?}", did, fn_sig);
        fn_sigs.insert(did, fn_sig);
    }

    fn_sigs
}

#[inline]
pub(super) fn initial_ssa_state<'tcx>(crate_ctxt: &CrateCtxt<'tcx>, body: &Body<'tcx>) -> SSAState {
    let dominance_frontier = compute_dominance_frontier(body);
    let definitions = initial_definitions(body, crate_ctxt);
    SSAState::new(body, &dominance_frontier, definitions)
}

pub(super) fn refine_state(body: &Body, fn_summary: FnSummary, model: &[Ownership]) -> SSAState {
    let ownership_transferred_locations =
        compute_ownership_transferred_locations(body, &fn_summary, model);

    let FnSummary {
        fn_body_sig,
        mut ssa_state,
    } = fn_summary;

    let consumes = &mut ssa_state.consume_chain.consumes;
    // we have to do this awkwardly as lending iterator is not ready
    for bb in 0..consumes.len() {
        for (statement_index, consumes) in consumes[bb].iter_mut().enumerate() {
            for &mut (local, ref mut consume) in consumes.iter_mut() {
                if consume.is_use() {
                    let location = Location {
                        block: bb.into(),
                        statement_index,
                    };
                    let Either::Left(_) = body.stmt_at(location) else {
                        unreachable!("function args and return are assumed to be local. rustc changes this property somehow")
                    };
                    let outter_most = fn_body_sig[local][consume.r#use].start;
                    if matches!(model[outter_most.index()], Ownership::Owning)
                        || ownership_transferred_locations.contains(&location)
                    {
                        consume.enable_def();
                    }
                }
            }
        }
    }
    ssa_state.name_state.reset();
    ssa_state.join_points.reset();
    ssa_state.consume_chain.reset();
    ssa_state
}

pub fn compute_ownership_transferred_locations(
    body: &Body,
    fn_summary: &FnSummary,
    model: &[Ownership],
) -> SsoHashSet<Location> {
    fn ownership_transferred(consume: Consume<&[Ownership]>) -> bool {
        for (&r#use, &def) in consume.r#use.iter().zip(consume.def.iter()) {
            if r#use == def {
                continue;
            }
            if r#use == Ownership::Owning || def == Ownership::Owning {
                return true;
            }
        }
        false
    }

    let mut ownership_transferred_locations: SsoHashSet<Location> = SsoHashSet::default();
    let fn_result = (fn_summary, model);

    for (bb, bb_data) in body.basic_blocks.iter_enumerated() {
        let BasicBlockData { statements, .. } = bb_data;
        let mut index = 0;
        let mut deref_copy: Option<Location> = None;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            if matches!(&statement.kind, StatementKind::Assign(box (_, rvalue)) if matches!(rvalue, Rvalue::CopyForDeref(_)))
            {
                let base_location = deref_copy.take().or(Some(location));
                deref_copy = base_location;
                index += 1;
                continue;
            }
            if let Some(base_location) = deref_copy.take() {
                let StatementKind::Assign(box (_, _)) = &statement.kind else { unreachable!() };

                let location_result = fn_result
                    .location_results(base_location)
                    .chain(fn_result.location_results(location));
                for (_, ownership_status) in location_result {
                    if ownership_transferred(ownership_status) {
                        ownership_transferred_locations.insert(location);
                        ownership_transferred_locations.insert(base_location);
                    }
                }

                index += 1;
                continue;
            }

            let location_result = fn_result.location_results(location);
            for (_, ownership_status) in location_result {
                if ownership_transferred(ownership_status) {
                    ownership_transferred_locations.insert(location);
                }
            }

            index += 1;
        }
    }

    ownership_transferred_locations
}
