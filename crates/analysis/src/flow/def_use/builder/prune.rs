//! Dead code elimination here is necessary! The reason is that we have a set of
//! copy locals during ownership analysis, which are guaranteed to have short live-range.
//! Since they are copies, they should not be joined with any variables (e.g. entry)!

use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::mir::{BasicBlock, Body, Local};

use super::DefUseChainBuilder;
use crate::flow::{
    def_use::{Def, DefUseChain, Inspect},
    state::SSAState,
    RichLocation,
};

pub fn dead_code_elimination(body: &Body, def_use_chain: DefUseChain) -> DefUseChain {
    let DefUseChain {
        uses,
        mut def_locs,
        mut join_points,
    } = def_use_chain;

    let mut useful: IndexVec<BasicBlock, BitSet<Local>> = IndexVec::from_elem_n(
        BitSet::new_empty(body.local_decls.len()),
        body.basic_blocks.len(),
    );

    for (_, uses) in uses.iter_enumerated() {
        for &(local, use_kind) in uses.iter() {
            let r#use = match use_kind {
                Inspect(ssa_idx) => ssa_idx,
                Def(update) => update.r#use,
            };
            let def_loc = def_locs[local][r#use];
            if let RichLocation::Phi(bb) = def_loc {
                useful[bb].insert(local);
            }
        }
    }

    for (bb, phi_nodes) in join_points.iter_enumerated_mut() {
        let replacement = std::mem::take(&mut phi_nodes.data);
        phi_nodes.data = replacement
            .into_iter()
            .filter(|&(local, _)| useful[bb].contains(local))
            .collect();
    }

    // reset def locs
    for locs in def_locs.iter_mut() {
        locs.raw.clear();
        locs.push(RichLocation::Entry);
    }

    let def_use_chain = DefUseChain {
        uses,
        def_locs,
        join_points,
    };

    let ssa_state = SSAState::new(body.local_decls.len());
    let mut builder = DefUseChainBuilder::new(body, def_use_chain, ssa_state);
    builder.run();
    builder.def_use_chain
}
