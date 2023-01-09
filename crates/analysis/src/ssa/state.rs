//! State for analysis steps

use common::data_structure::assoc::AssocExt;
use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{Body, Local, Location};

use super::consume::Voidable;
use crate::ssa::{
    consume::{Consume, ConsumeChain, Definitions},
    dom::DominanceFrontier,
    join_points::{JoinPoints, PhiNode},
};

common::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct SSAIdx {
        // DEBUG_FORMAT = "{}"
    }
}

impl Default for SSAIdx {
    fn default() -> Self {
        Self::INIT
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
}

impl Voidable for SSAIdx {
    const VOID: Self = SSAIdx::MAX;

    #[inline]
    fn is_void(&self) -> bool {
        *self == Self::VOID
    }
}

impl std::ops::AddAssign<usize> for SSAIdx {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

pub struct SSAState {
    pub name_state: NameState,
    pub join_points: JoinPoints<PhiNode>,
    pub consume_chain: ConsumeChain,
}

impl SSAState {
    pub fn new<'tcx>(
        body: &Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        definitions: Definitions,
    ) -> Self {
        let name_state = NameState::new(body, &definitions.locals_with_defs);
        let join_points = JoinPoints::new(body, dominance_frontier, &definitions.def_sites);
        let consume_chain = ConsumeChain::new(body, definitions);
        SSAState {
            name_state,
            join_points,
            consume_chain,
        }
    }

    pub fn mk_dummy(mut self) -> Self {
        self.name_state.count.raw.clear();
        self.name_state.stack.raw.clear();
        self.join_points.data.raw.clear();
        self.consume_chain.mk_dummy();
        self
    }

    /// Try find valid consume at `location`
    #[inline]
    pub fn try_consume_at(&mut self, local: Local, location: Location) -> Option<Consume<SSAIdx>> {
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        let consume = self.consume_chain.consumes[location.block.index()][location.statement_index]
            .get_by_key_mut(&local)?;
        let old_ssa_idx = self.name_state.get_name(local);
        consume.r#use = old_ssa_idx;
        if consume.is_use() {
            return None;
        }
        let new_ssa_idx = self.name_state.generate_fresh_name(local);
        tracing::debug!(
            "consuming {:?} at {:?}, use: {:?}, def: {:?}",
            local,
            location,
            old_ssa_idx,
            new_ssa_idx
        );
        consume.def = new_ssa_idx;
        let consume = *consume;
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        assert_eq!(
            new_ssa_idx,
            self.consume_chain.locs[local].push(location.into())
        );
        Some(consume)
    }

    // /// Try find valid consume at `location`
    // #[inline]
    // pub fn try_consume_at(&mut self, local: Local, location: Location) -> Option<Consume<SSAIdx>> {
    //     // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
    //     let consume = self.consume_chain.consumes[location.block.index()][location.statement_index]
    //         .get_by_key_mut(&local)?;
    //     let old_ssa_idx = self.name_state.get_name(local);
    //     if self.consume_chain.proxy_temporaries.contains(&local) {
    //         if old_ssa_idx == SSAIdx::INIT {
    //             consume.r#use = old_ssa_idx;
    //             let new_ssa_idx = self.name_state.generate_fresh_name(local);
    //             consume.def = new_ssa_idx;
    //         } else {
    //             assert_eq!(old_ssa_idx, SSAIdx::INIT + 1);
    //             consume.r#use = SSAIdx::INIT;
    //             consume.def = SSAIdx::INIT + 1;
    //         }
    //     }
    //     consume.r#use = old_ssa_idx;
    //     if consume.is_use() {
    //         return None;
    //     }
    //     let new_ssa_idx = self.name_state.generate_fresh_name(local);
    //     tracing::debug!(
    //         "consuming {:?} at {:?}, use: {:?}, def: {:?}",
    //         local,
    //         location,
    //         old_ssa_idx,
    //         new_ssa_idx
    //     );
    //     consume.def = new_ssa_idx;
    //     let consume = *consume;
    //     // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
    //     assert_eq!(
    //         new_ssa_idx,
    //         self.consume_chain.locs[local].push(location.into())
    //     );
    //     Some(consume)
    // }
}

#[derive(Clone, Debug)]
pub struct NameState {
    count: IndexVec<Local, SSAIdx>,
    stack: IndexVec<Local, Vec<SSAIdx>>,
}

impl NameState {
    fn new<'tcx>(body: &Body<'tcx>, has_def: &BitSet<Local>) -> Self {
        let count = IndexVec::from_elem(SSAIdx::INIT, &body.local_decls);

        // Notice: this has to be in accordance with ConsumeChain.locs
        let stack = body
            .local_decls
            .indices()
            .map(|local| {
                has_def
                    .contains(local)
                    .then(|| vec![SSAIdx::INIT])
                    .unwrap_or_default()
                // .unwrap_or_else(Vec::new)
            })
            .collect();
        // let stack = IndexVec::from_elem(vec![SSAIdx::INIT], &body.local_decls);
        NameState { count, stack }
    }

    pub fn reset(&mut self) {
        self.count.raw.fill(SSAIdx::INIT);
        for stack in &mut self.stack {
            if !stack.is_empty() {
                stack.truncate(1);
            }
        }
    }

    #[inline]
    pub fn generate_fresh_name(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline]
    pub fn get_name(&self, var: Local) -> SSAIdx {
        self.try_get_name(var).unwrap_or_else(|| {
            panic!(
                "internal error: cannot find fresh name supply for {:?}",
                var
            )
        })
    }

    /// Get the newest version for a variable. If `None` is returned,
    /// this variable is uninitialized.
    #[inline]
    pub fn try_get_name(&self, var: Local) -> Option<SSAIdx> {
        self.stack[var].last().copied()
    }

    #[inline]
    pub fn pop(&mut self, var: Local) -> SSAIdx {
        // tracing::debug!("popping {:?}~{:?}", var, ssa_idx);
        self.stack[var]
            .pop()
            .unwrap_or_else(|| panic!("internal error: poping non existing version for {:?}", var))
    }
}
