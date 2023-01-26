use alias::AliasResult;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{
        visit::{NonMutatingUseContext, PlaceContext, Visitor},
        Body, Local, Location, Place, Rvalue, TerminatorKind,
    },
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;
use smallvec::{smallvec, SmallVec};

use crate::{
    call_graph::CallGraph, lattice::Lattice,
    type_qualifier::flow_insensitive::mutability::MutabilityResult,
};

pub type OutputParams = FxHashMap<DefId, FxHashSet<Local>>;

pub fn show_output_params(
    crate_data: &common::CrateData,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) {
    fn show_set<T: std::fmt::Debug>(set: impl Iterator<Item = T>) -> String {
        set.map(|x| format!("{:?}", x))
            .collect::<Vec<_>>()
            .join(", ")
    }

    let output_params = compute_output_params(crate_data, alias_result, mutability_result);

    for (did, noalias_params) in output_params {
        let noalias_params_str = show_set(noalias_params.into_iter());
        println!(
            "@{}: {noalias_params_str}",
            crate_data.tcx.def_path_str(did)
        );
    }
}

pub fn compute_output_params(
    crate_data: &common::CrateData,
    alias_result: &AliasResult,
    mutability_result: &MutabilityResult,
) -> OutputParams {
    let call_graph = CallGraph::new(crate_data.tcx, &crate_data.fns);

    let tcx = crate_data.tcx;

    let mut nonintervening = FxHashMap::default();
    nonintervening.reserve(crate_data.fns.len());
    for nodes in call_graph.sccs() {
        nonintervening.extend(nodes.iter().map(|&node| {
            let body = tcx.optimized_mir(node);
            (node, smallvec![true; body.arg_count])
        }));

        loop {
            let mut changed = false;

            for did in nodes {
                let body = tcx.optimized_mir(*did);
                let original = nonintervening[did].clone();
                let mut nc = NoninterferenceChecker {
                    arg_count: body.arg_count,
                    this: original,
                    nointervening: &nonintervening,
                    call_args: &collect_call_args_mapping(body, tcx),
                };
                nc.visit_body(body);
                let new = nc.this;
                changed = nonintervening.get_mut(did).unwrap().meet(&new);
            }

            if !changed {
                break;
            }
        }
    }

    nonintervening
        .into_iter()
        .map(|(did, arg_values)| {
            (
                did,
                arg_values
                    .iter()
                    .enumerate()
                    .filter_map(|(arg_idx, nonintervening)| {
                        nonintervening.then_some(Local::from(arg_idx + 1))
                    })
                    .collect(),
            )
        })
        .collect()
}

struct NoninterferenceChecker<'me> {
    arg_count: usize,
    this: SmallVec<[bool; 4]>,
    nointervening: &'me FxHashMap<DefId, SmallVec<[bool; 4]>>,
    call_args: &'me CallArgsMapping,
}

impl<'me, 'tcx> Visitor<'tcx> for NoninterferenceChecker<'me> {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        let Some((callee, idx)) = self.call_args.get(&place.local) else { return self.super_assign(place, rvalue, location) };
        assert!(place.as_local().is_some());
        let callee = self.nointervening[callee][*idx];

        if let Rvalue::Use(operand) = rvalue {
            if let Some(rhs) = operand.place() {
                if !rhs.is_indirect() && (1..self.arg_count + 1).contains(&rhs.local.index()) {
                    self.this[rhs.local.index() - 1].meet(&callee);
                }
            }
            return;
        }

        self.visit_rvalue(rvalue, location);
    }

    fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
        if let Rvalue::Cast(_, operand, _) = rvalue {
            if let Some(local) = operand.place().and_then(|place| place.as_local()) {
                if (1..self.arg_count + 1).contains(&local.index()) {
                    self.this[local.index() - 1].meet(&false);
                    return;
                }
            }
        }
        self.super_rvalue(rvalue, location);
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _: Location) {
        if !(1..self.arg_count + 1).contains(&place.local.index()) {
            return;
        }
        if place.is_indirect() || !context.is_use() {
            return;
        }
        let local = place.local;
        if context.is_place_assignment()
            || matches!(
                context,
                PlaceContext::NonMutatingUse(
                    NonMutatingUseContext::Copy | NonMutatingUseContext::Move
                )
            )
        {
            self.this[local.index() - 1].meet(&false);
        }
    }
}

/// mapping call arg temp to callee and index of parameter
type CallArgsMapping = FxHashMap<Local, (DefId, usize)>;

fn collect_call_args_mapping(body: &Body, tcx: TyCtxt) -> CallArgsMapping {
    let mut call_args_mapping = FxHashMap::default();
    for bb_data in body.basic_blocks.iter() {
        let Some(terminator) = &bb_data.terminator else { continue; };
        if let TerminatorKind::Call { func, args, .. } = &terminator.kind {
            let Some(func) = func.constant() else { continue };
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else { unreachable!() };
            let Some(local_did) = callee.as_local() else { continue };
            let rustc_hir::Node::Item(_) = tcx.hir().find_by_def_id(local_did).unwrap() else { continue };

            for (idx, arg) in args.iter().enumerate() {
                let Some(arg) = arg.place() else { continue };
                let arg = arg.as_local().unwrap();
                call_args_mapping.insert(arg, (callee, idx));
            }
        }
    }

    call_args_mapping
}
