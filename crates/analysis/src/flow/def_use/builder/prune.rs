//! Dead code elimination here is necessary! The reason is that we have a set of
//! copy locals during ownership analysis, which are guaranteed to have short live-range.
//! Since they are copies, they should not be joined with any variables (e.g. entry)!

use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    BasicBlock, Body, Local, Location, Place, Rvalue,
};
use utils::data_structure::assoc::AssocExt;

use super::DefUseChainBuilder;
use crate::flow::{
    def_use::{Def, DefUseChain, Inspect},
    join_points::PhiNode,
    ownership::copies::DerefCopiesCollector,
    state::SSAState,
    RichLocation, SSAIdx,
};

pub fn dead_code_elimination(body: &Body, mut def_use_chain: DefUseChain) -> DefUseChain {
    let mut deref_copies = BitSet::new_empty(body.local_decls.len());
    DerefCopiesCollector(&mut deref_copies).visit_body(body);
    prune_deref_copies(body, &mut deref_copies, &mut def_use_chain);

    let DefUseChain {
        uses,
        mut def_locs,
        mut join_points,
    } = def_use_chain;

    let mut useful: IndexVec<BasicBlock, BitSet<Local>> = IndexVec::from_elem_n(
        BitSet::new_empty(body.local_decls.len()),
        body.basic_blocks.len(),
    );

    use std::collections::VecDeque;
    let mut queue = uses
        .iter_enumerated()
        .flat_map(|(_, uses)| {
            uses.iter().copied().map(|(local, use_kind)| {
                (
                    local,
                    match use_kind {
                        Inspect(ssa_idx) => ssa_idx,
                        Def(update) => update.r#use,
                    },
                )
            })
        })
        .collect::<VecDeque<_>>();

    while let Some((local, ssa_idx)) = queue.pop_front() {
        let def_loc = def_locs[local][ssa_idx];
        if let RichLocation::Phi(bb) = def_loc {
            if useful[bb].insert(local) {
                let phi_node = join_points[bb].get_by_key(&local).unwrap();
                for ssa_idx in phi_node.rhs.iter().copied() {
                    queue.push_back((local, ssa_idx));
                }
            }
        }
    }

    for (bb, phi_nodes) in join_points.iter_enumerated_mut() {
        let replacement = std::mem::take(&mut phi_nodes.data);
        phi_nodes.data = replacement
            .into_iter()
            .filter(|&(local, _)| useful[bb].contains(local))
            .map(|(local, _)| (local, PhiNode::default()))
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

fn prune_deref_copies(
    body: &Body,
    deref_copies: &mut BitSet<Local>,
    def_use_chain: &mut DefUseChain,
) {
    PruneDerefCopies {
        deref_copies,
        def_use_chain,
    }
    .visit_body(body);
    TransitivelyPruneDerefCopies {
        deref_copies,
        def_use_chain,
    }
    .visit_body(body);
    PruneUselessDerefCopiesDefinition {
        deref_copies,
        def_use_chain,
    }
    .visit_body(body)
}

struct PruneDerefCopies<'this> {
    deref_copies: &'this mut BitSet<Local>,
    def_use_chain: &'this DefUseChain,
}

impl Visitor<'_> for PruneDerefCopies<'_> {
    fn visit_place(&mut self, place: &Place<'_>, _: PlaceContext, location: Location) {
        if self.deref_copies.contains(place.local) && place.is_indirect() {
            if !self.def_use_chain.uses[location].contains_key(&place.local) {
                let local = place.local;
                assert!(self.deref_copies.remove(local));
            }
        }
    }
}

struct TransitivelyPruneDerefCopies<'this> {
    deref_copies: &'this mut BitSet<Local>,
    def_use_chain: &'this DefUseChain,
}

impl Visitor<'_> for TransitivelyPruneDerefCopies<'_> {
    fn visit_assign(&mut self, place: &Place<'_>, rvalue: &Rvalue<'_>, location: Location) {
        if let Rvalue::CopyForDeref(rhs) = rvalue {
            if !self.deref_copies.contains(place.local) {
                let mut local = rhs.local;
                let mut location = location;
                loop {
                    if self.deref_copies.contains(local) {
                        assert!(self.deref_copies.remove(local));

                        let mut def_loc = self.def_use_chain.def_loc(local, location);
                        loop {
                            match def_loc {
                                RichLocation::Mir(loc) => {
                                    location = loc;
                                    let r#use = &self.def_use_chain.uses[location];
                                    assert_eq!(r#use.len(), 2);
                                    assert_ne!(local, r#use[1].0);
                                    local = r#use[1].0;
                                    break;
                                }
                                RichLocation::Phi(bb) => {
                                    let phi_node = self.def_use_chain.join_points[bb]
                                        .get_by_key(&local)
                                        .unwrap();
                                    // at most two definitions for a deref copy
                                    assert_eq!(phi_node.rhs.len(), 2);
                                    let ssa_idx = if phi_node.rhs[0] == SSAIdx::INIT {
                                        phi_node.rhs[1]
                                    } else {
                                        assert_eq!(phi_node.rhs[1], SSAIdx::INIT);
                                        phi_node.rhs[0]
                                    };
                                    def_loc = self.def_use_chain.def_locs[local][ssa_idx];
                                }
                                RichLocation::Entry => unreachable!(),
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

struct PruneUselessDerefCopiesDefinition<'this> {
    deref_copies: &'this BitSet<Local>,
    def_use_chain: &'this mut DefUseChain,
}

impl Visitor<'_> for PruneUselessDerefCopiesDefinition<'_> {
    fn visit_assign(&mut self, place: &Place<'_>, rvalue: &Rvalue<'_>, location: Location) {
        if let Rvalue::CopyForDeref(_) = rvalue {
            if !self.deref_copies.contains(place.local) {
                self.def_use_chain.uses[location].clear();
            }
        }
    }
}
