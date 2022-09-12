use crate::analysis::{def::RichLocation, state::SSAState};
use orc_common::data_structure::assoc::AssocExt;
use rustc_hash::FxHashSet;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body};

use super::infer::{Pure, Renamer};

pub fn pruned(body: &Body, ssa_state: SSAState) -> SSAState {
    let mut rn = Renamer::new(body, ssa_state);
    rn.go::<Pure, ()>(());
    let Renamer { mut state, .. } = rn;
    dead_code_elimination(body, &mut state);
    state
}

fn dead_code_elimination(body: &Body, ssa_state: &mut SSAState) {
    let mut stack = Vec::with_capacity(body.local_decls.len());
    // let mut useful = BitSet::new_empty(body.local_decls.len());
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
        }

        recursion.extend(children[bb].iter().rev().copied());
    }

    // usefulness propagation phase
    while let Some((local, r#use)) = stack.pop() {
        let loc = ssa_state.consume_chain.locs[local][r#use];
        let RichLocation::Phi(bb) = loc else { continue };
        let phi_nodes = &ssa_state.join_points[bb];
        let phi_node = phi_nodes.get(&local).unwrap();
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
