use alias::{alias_results, AliasResult};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_index::bit_set::BitSet;
use rustc_middle::{
    mir::{Body, Local, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;
use utils::compiler_interface::Program;

use super::flow_insensitive::mutability::{Mutability, MutabilityResult};
use crate::{call_graph::CallGraph, flow::ownership::copies::collect_copies};

pub type OutputParams = FxHashMap<DefId, BitSet<Local>>;

pub fn show_output_params(program: &Program, mutability_result: &MutabilityResult) {
    fn show_set<T: std::fmt::Debug>(set: impl Iterator<Item = T>) -> String {
        set.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }
    let output_params = compute_output_params(program, mutability_result);

    for (did, noalias_params) in output_params {
        let noalias_params_str = show_set(noalias_params.iter());
        println!("@{}: {noalias_params_str}", program.tcx.def_path_str(did));
    }
}

pub fn compute_output_params(
    program: &Program,
    mutability_result: &MutabilityResult,
) -> OutputParams {
    let mut output_params = FxHashMap::default();
    output_params.reserve(program.fns.len());
    let mut copies = FxHashMap::default();
    copies.reserve(program.fns.len());

    let alias_result = alias_results(program);

    for &did in program.fns.iter() {
        let body = program.tcx.optimized_mir(did);
        output_params.insert(did, conservative(body, &alias_result, mutability_result));
        copies.insert(did, collect_copies(body));
    }

    let tcx = program.tcx;
    for scc in CallGraph::new(program.tcx, &program.fns).sccs() {
        loop {
            let mut changed = false;
            for &def_id in scc {
                let body = tcx.optimized_mir(def_id);
                changed = changed
                    || iterate(
                        body,
                        copies.get(&def_id).unwrap(),
                        &alias_result,
                        mutability_result,
                        &mut output_params,
                        tcx,
                    );
            }
            if !changed {
                break;
            }
        }
    }

    output_params
}

fn conservative(
    body: &Body,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> BitSet<Local> {
    let location_of = alias_result.local_locations(&body.source.def_id());
    let fn_result = mutability_result.fn_results(&body.source.def_id());

    let mut output_params = BitSet::new_empty(body.local_decls.len());
    for local in body.args_iter().filter(|&arg| {
        matches!(fn_result
            .local_result(arg)
            .first(),
            Some(&mutability)
            if mutability == Mutability::Mut
        )
    }) {
        output_params.insert(local);
    }

    for arg in body.args_iter() {
        for local in body.local_decls.indices() {
            if arg == local {
                continue;
            }
            if alias_result.may_alias(location_of[arg.index()], location_of[local.index()]) {
                tracing::debug!(
                    "@{:?}: {arg:?} removed because it aliases another pointer {local:?}",
                    body.source.def_id()
                );
                output_params.remove(arg);
            }
        }
    }
    output_params
}

fn iterate(
    body: &Body,
    copies: &BitSet<Local>,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
    known_facts: &mut OutputParams,
    tcx: TyCtxt,
) -> bool {
    let location_of = alias_result.local_locations(&body.source.def_id());
    let fn_result = mutability_result.fn_results(&body.source.def_id());
    let transitive_output_position_temporaries =
        transitive_output_position_temporaries(known_facts, copies, body, tcx);
    let output_params = known_facts.get_mut(&body.source.def_id()).unwrap();

    let mut changed = false;
    for arg in body.args_iter().filter(|&arg| {
        matches!(fn_result
            .local_result(arg)
            .first(),
            Some(&mutability)
            if mutability == Mutability::Mut
        )
    }) {
        if body
            .local_decls
            .indices()
            .filter(|&local| arg != local)
            .filter(|&local| !transitive_output_position_temporaries.contains(local))
            .all(|local| {
                !alias_result.may_alias(location_of[arg.index()], location_of[local.index()])
            })
        {
            tracing::debug!(
                "@{:?}: {arg:?} added because it aliases a transitive output position temporary",
                body.source.def_id()
            );
            changed = changed || output_params.insert(arg);
        }
    }

    changed
}

fn transitive_output_position_temporaries(
    known_facts: &OutputParams,
    copies: &BitSet<Local>,
    body: &Body,
    tcx: TyCtxt,
) -> BitSet<Local> {
    let mut transitive_output_temporaries = BitSet::new_empty(body.local_decls.len());
    for bb_data in body.basic_blocks.iter() {
        let Some(terminator) = &bb_data.terminator else {
            continue;
        };
        if let TerminatorKind::Call { func, args, .. } = &terminator.kind {
            let Some(func) = func.constant() else {
                continue;
            };
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else {
                unreachable!()
            };
            let Some(local_did) = callee.as_local() else {
                continue;
            };
            let rustc_hir::Node::Item(_) = tcx.hir().find_by_def_id(local_did).unwrap() else {
                continue;
            };

            for arg in known_facts
                .get(&callee)
                .unwrap()
                .iter()
                .map(|local| local.index() - 1)
                .map(|index| &args[index])
                .filter_map(|arg| arg.place().and_then(|place| place.as_local()))
                .filter(|&arg| copies.contains(arg))
            {
                transitive_output_temporaries.insert(arg);
            }
        }
    }
    transitive_output_temporaries
}
