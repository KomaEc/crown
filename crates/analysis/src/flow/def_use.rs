use common::data_structure::assoc::AssocExt;
use rustc_index::IndexVec;
use rustc_middle::mir::{BasicBlock, Local, Location};
use smallvec::SmallVec;

use super::{
    join_points::{JoinPoints, PhiNode},
    LocationMap, SSAIdx, RichLocation,
};

#[derive(Clone, Debug)]
pub struct Update<T> {
    pub r#use: T,
    pub def: T,
}

pub enum UseKind<T> {
    Use(T),
    Def(Update<T>),
    /// Peek the definition of `T`
    Peek(T),
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
