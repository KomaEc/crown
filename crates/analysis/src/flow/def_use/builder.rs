pub mod prune;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::IndexVec;
use rustc_middle::mir::{BasicBlock, BasicBlockData, Body, Location};

use super::DefUseChain;
use crate::flow::{def_use::UseKind, state::SSAState, RichLocation};

pub struct DefUseChainBuilder<'build, 'tcx> {
    body: &'build Body<'tcx>,
    pub def_use_chain: DefUseChain,
    ssa_state: SSAState,
}

impl<'build, 'tcx> DefUseChainBuilder<'build, 'tcx> {
    pub fn new(body: &'build Body<'tcx>, def_use_chain: DefUseChain, ssa_state: SSAState) -> Self {
        DefUseChainBuilder {
            body,
            def_use_chain,
            ssa_state,
        }
    }

    pub fn run(&mut self) {
        tracing::debug!("Running {:?}", self.body.source.def_id());
        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], &self.body.basic_blocks);
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks.indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if let Some(dom) = dom {
                children[dom].push(bb);
            } else {
                root = bb;
            }
        });
        assert_eq!(root, BasicBlock::from_u32(0), "expect root");

        enum State {
            ToVisit,
            ToPopNames,
        }
        let mut recursion = vec![(root, State::ToVisit)];

        while let Some((bb, state)) = recursion.pop() {
            match state {
                State::ToVisit => {
                    self.walk_basic_block(bb);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for local in self.def_use_chain.uses[bb]
                        .iter()
                        .flatten()
                        .filter(|(_, use_kind)| matches!(use_kind, UseKind::Def(..)))
                        .map(|(local, _)| *local)
                        .chain(
                            self.def_use_chain.join_points[bb]
                                .iter()
                                .map(|(local, _)| *local),
                        )
                    {
                        let ssa_idx = self.ssa_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }
    }

    fn walk_basic_block(&mut self, bb: BasicBlock) {
        tracing::debug!("Walking {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = &self.body.basic_blocks[bb];

        // process phi-node definition
        for (local, phi_node) in self.def_use_chain.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.ssa_state.fresh_name(local);
            phi_node.lhs = ssa_idx;
            assert_eq!(
                self.def_use_chain.def_locs[local].push(RichLocation::Phi(bb)),
                ssa_idx
            );
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx);
        }

        let mut index = 0;
        for _ in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            index += 1;

            self.build_location(location);
        }

        if let Some(..) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };

            self.build_location(location);
        }

        // process phi-nodes uses
        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.def_use_chain.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.ssa_state.get_name(local);
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx);
            }
        }
    }

    pub fn build_location(&mut self, location: Location) {
        for &mut (local, ref mut use_kind) in self.def_use_chain.uses[location].iter_mut() {
            let r#use = self.ssa_state.get_name(local);
            match use_kind {
                UseKind::Inspect(ssa_idx) /* | UseKind::LocalPeek(ssa_idx) */ => {
                    *ssa_idx = r#use;
                }
                UseKind::Def(update) => {
                    let def = self.ssa_state.fresh_name(local);
                    tracing::debug!(
                        "updating {:?} at {:?}, use: {:?}, def: {:?}",
                        local,
                        location,
                        r#use,
                        def
                    );
                    update.r#use = r#use;
                    update.def = def;
                    assert_eq!(
                        def,
                        self.def_use_chain.def_locs[local].push(location.into())
                    );
                }
            }
        }
    }
}
