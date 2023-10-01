use common::data_structure::{
    assoc::AssocExt,
    vec_vec::{VecVec, VecVecBuilder},
};
use rustc_index::{bit_set::BitSet, IndexVec};
use rustc_middle::mir::{visit::Visitor, BasicBlock, BasicBlockData, Body, Local, Location};
use smallvec::SmallVec;

use super::{
    dom::compute_dominance_frontier,
    join_points::{JoinPoints, PhiNode},
    LocationMap, RichLocation, SSAIdx,
};

#[derive(Clone, Debug)]
pub struct Update<T> {
    pub r#use: T,
    pub def: T,
}

pub enum UseKind<T> {
    Use(T),
    Def(Update<T>),
    /// Peek the definition of `T`, and the definition is guaranteed to be
    /// located within the same basic block
    LocalPeek(T),
}
use UseKind::*;

#[cfg(not(debug_assertions))]
const _: () = assert!(8 == std::mem::size_of::<UseKind<SSAIdx>>());

pub struct DefUseChain {
    /// `Location -> Local -> UseKind<SSAIdx>`
    pub uses: LocationMap<SmallVec<[(Local, UseKind<SSAIdx>); 2]>>,

    /// `Local -> SSAIdx -> RichLocation`
    pub def_locs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,

    pub join_points: JoinPoints<PhiNode>,
}

impl DefUseChain {
    pub fn initialise<'tcx, L: LocationBuilder<'tcx>>(body: &Body<'tcx>, location_builder: L) -> Self {
        let uses = DefUseBuilder::build(body, location_builder);
        let def_locs =
            IndexVec::from_elem(IndexVec::from([RichLocation::Entry]), &body.local_decls);
        let dominance_frontier = compute_dominance_frontier(body);
        let mut def_sites = IndexVec::from_elem(
            BitSet::new_empty(body.basic_blocks.len()),
            &body.local_decls,
        );
        for (location, uses) in uses.iter_enumerated() {
            for &(local, ref use_kind) in uses.iter() {
                if let Def(..) = use_kind {
                    def_sites[local].insert(location.block);
                }
            }
        }
        let join_points = JoinPoints::new(body, &dominance_frontier, &def_sites);

        DefUseChain {
            uses,
            def_locs,
            join_points,
        }
    }

    pub fn pure_uses(&self, location: Location) -> impl Iterator<Item = Local> + '_ {
        self.uses
            .get_location(location)
            .iter()
            .filter_map(|(local, use_kind)| {
                if let Use(..) = use_kind {
                    Some(*local)
                } else {
                    None
                }
            })
    }

    pub fn peeked_loc(&self, local: Local, location: Location) -> Location {
        let Some(use_kind) = self.uses.get_location(location).get_by_key(&local) else {
            panic!("nonexisting peek {:?} @ {:?}", local, location)
        };
        let LocalPeek(ssa_idx) = *use_kind else {
            panic!("nonexisting peek {:?} @ {:?}", local, location)
        };
        match self.def_locs[local][ssa_idx] {
            RichLocation::Mir(loc) => loc,
            _ => unreachable!(),
        }
    }

    pub fn def_loc(&self, local: Local, location: Location) -> RichLocation {
        let Some(use_kind) = self.uses.get_location(location).get_by_key(&local) else {
            panic!("nonexisting use {:?} @ {:?}", local, location)
        };
        let Use(ssa_idx) = *use_kind else {
            panic!("nonexisting use {:?} @ {:?}", local, location)
        };
        self.def_locs[local][ssa_idx]
    }

    pub fn phi_node_def_locs(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = RichLocation> + '_ {
        let phi_node = self.join_points[block]
            .data
            .get_by_key(&local)
            .unwrap_or_else(|| panic!("phi node {:?} does not exist @ {:?}", local, block));

        phi_node
            .rhs
            .iter()
            .copied()
            .filter(|&ssa_idx| ssa_idx != phi_node.lhs)
            .map(move |ssa_idx| self.def_locs[local][ssa_idx])
    }
}

pub struct DefUseBuilder<L> {
    uses: VecVecBuilder<SmallVec<[(Local, UseKind<SSAIdx>); 2]>>,
    location_builder: L,
}

impl<'tcx, L: LocationBuilder<'tcx>> DefUseBuilder<L> {
    pub fn build(
        body: &Body<'tcx>,
        location_builder: L,
    ) -> LocationMap<SmallVec<[(Local, UseKind<SSAIdx>); 2]>> {
        let mut builder = DefUseBuilder {
            uses: VecVec::with_indices_capacity(body.basic_blocks.len()),
            location_builder,
        };
        builder.visit_body(body);
        LocationMap {
            map: builder.uses.complete(),
        }
    }

    fn visit_body(&mut self, body: &Body<'tcx>) {
        for (bb, data) in body.basic_blocks.iter_enumerated() {
            self.visit_basic_block_data(bb, data);
        }
    }

    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block,
                statement_index: index,
            };
            self.location_builder.visit_statement(statement, location);
            let location_uses = self.location_builder.retrieve();
            self.uses.push_element(location_uses);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block,
                statement_index: index,
            };
            self.location_builder.visit_terminator(terminator, location);
            let location_uses = self.location_builder.retrieve();
            self.uses.push_element(location_uses);
        }
        self.uses.complete_cur_vec();
    }
}

pub trait LocationBuilder<'tcx>: Visitor<'tcx> {
    fn retrieve(&mut self) -> SmallVec<[(Local, UseKind<SSAIdx>); 2]>;
}
