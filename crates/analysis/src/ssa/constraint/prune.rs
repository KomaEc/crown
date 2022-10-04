use crate::analysis::{
    def::RichLocation,
    state::{SSAIdx, SSAState},
};
use orc_common::data_structure::assoc::AssocExt;
use rustc_hash::FxHashSet;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local, TerminatorKind, RETURN_PLACE};

use super::infer::{Mode, Renamer};

pub fn prune(body: &Body, ssa_state: SSAState) -> SSAState {
    let mut rn = Renamer::new(body, ssa_state);
    let mut special_uses = Vec::with_capacity(body.local_decls.len());
    rn.go::<Prune, ()>(&mut special_uses);
    let Renamer { mut state, .. } = rn;
    dead_code_elimination(body, &mut state, &special_uses);
    state
}

fn dead_code_elimination(body: &Body, ssa_state: &mut SSAState, special_uses: &SpecialUses) {
    let mut stack = Vec::with_capacity(body.local_decls.len());
    let mut useful = FxHashSet::default();

    // initial marking phase
    let dominators = body.basic_blocks.dominators();
    let mut children = IndexVec::from_elem(vec![], &body.basic_blocks);
    let mut root = BasicBlock::from_u32(0);
    body.basic_blocks.indices().for_each(|bb| {
        let dom = dominators.immediate_dominator(bb);
        if dom != bb {
            children[dom].push(bb)
        } else {
            root = bb;
        }
    });
    assert_eq!(root, BasicBlock::from_u32(0));

    let mut recursion = vec![root];

    while let Some(bb) = recursion.pop() {
        let phi_nodes = &ssa_state.join_points[bb];
        for (local, phi_node) in phi_nodes.iter() {
            useful.remove(&(*local, phi_node.lhs));
        }

        for block_consumes in &ssa_state.consume_chain.consumes[bb.index()] {
            for &(local, consume) in block_consumes {
                let loc = ssa_state.consume_chain.locs[local][consume.r#use];
                if matches!(loc, RichLocation::Phi(_)) {
                    useful.insert((local, consume.r#use));
                    stack.push((local, consume.r#use));
                }
            }
            if let Some(terminator) = &body.basic_blocks[bb].terminator {
                if let TerminatorKind::Return = terminator.kind {
                    for &(local, ssa_idx) in special_uses {
                        let loc = ssa_state.consume_chain.locs[local][ssa_idx];
                        if matches!(loc, RichLocation::Phi(_)) {
                            useful.insert((local, ssa_idx));
                            stack.push((local, ssa_idx));
                        }
                    }
                }
            }
        }

        recursion.extend(children[bb].iter().rev().copied());
    }

    // usefulness propagation phase
    while let Some((local, r#use)) = stack.pop() {
        let loc = ssa_state.consume_chain.locs[local][r#use];
        let RichLocation::Phi(bb) = loc else { continue };
        let phi_nodes = &ssa_state.join_points[bb];
        let phi_node = phi_nodes.get_by_key(&local).unwrap();
        for &ssa_idx in &phi_node.rhs {
            if !useful.contains(&(local, ssa_idx)) {
                useful.insert((local, ssa_idx));
                stack.push((local, ssa_idx));
            }
        }
    }

    // final pruning phase
    for block_nodes in ssa_state.join_points.iter_mut() {
        block_nodes.retain(|(local, phi_node)| useful.contains(&(*local, phi_node.lhs)))
    }

    ssa_state.name_state.reset();
    ssa_state.consume_chain.reset();
}

enum Prune {}

type Var = (Local, SSAIdx);

type SpecialUses = Vec<Var>;

impl Mode for Prune {
    type Ctxt<'infercx, 'tcx, DB> = SpecialUses
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: super::Database + 'infercx;

    type Interpretation = ();

    type FnSig<T> = () where T: std::fmt::Debug;

    fn transform_fn_sig(
        fn_sig: impl Iterator<Item = Option<crate::analysis::def::Consume<Self::Interpretation>>>,
    ) -> Self::FnSig<Option<crate::analysis::def::Consume<Self::Interpretation>>> {
        for _ in fn_sig {}
    }

    fn define_phi_node<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Local,
        _: rustc_middle::ty::Ty<'tcx>,
        _: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn join_phi_nodes<'a, 'infercx, 'tcx, DB>(
        _: &'a mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: impl Iterator<Item = (Local, &'a mut crate::analysis::join_points::PhiNode)>,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn interpret_consume<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: &Body<'tcx>,
        _: &rustc_middle::mir::Place<'tcx>,
        _: crate::analysis::def::Consume<SSAIdx>,
    ) -> crate::analysis::def::Consume<Self::Interpretation>
    where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
        crate::analysis::def::Consume { r#use: (), def: () }
    }

    fn transfer<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: crate::analysis::def::Consume<Self::Interpretation>,
        _: crate::analysis::def::Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn unknown_sink<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: crate::analysis::def::Consume<Self::Interpretation>,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn assume<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Self::Interpretation,
        _: bool,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn finalise<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        local: Local,
        r#use: SSAIdx,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
        infer_cx.push((local, r#use));
    }

    fn handle_call<'infercx, 'tcx, DB>(
        _: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        _: Self::FnSig<Option<crate::analysis::def::Consume<Self::Interpretation>>>,
        _: &rustc_middle::mir::Operand,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
    }

    fn handle_output<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        ssa_idx: Option<SSAIdx>,
        _: rustc_hir::def_id::DefId,
    ) where
        'tcx: 'infercx,
        DB: super::Database + 'infercx,
    {
        if let Some(ssa_idx) = ssa_idx {
            infer_cx.push((RETURN_PLACE, ssa_idx));
        }
    }
}
