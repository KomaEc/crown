//! State for analysis steps

use orc_common::data_structure::assoc::AssocExt;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{Body, Local, LocalInfo, Location};

use super::{
    body_ext::DominanceFrontier,
    def::{Consume, ConsumeChain, Definitions},
    join_points::{JoinPoints, PhiNode},
};

orc_common::macros::newtype_index! {
    pub(crate) struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl SSAIdx {
    pub(crate) const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

pub(crate) struct SSAState {
    pub(crate) name_state: NameState,
    pub(crate) join_points: JoinPoints<PhiNode>,
    /// TODO: initialise
    pub(crate) consume_chain: ConsumeChain,
}

impl SSAState {
    pub(crate) fn new<'tcx>(
        body: &Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        definitions: Definitions,
    ) -> Self {
        let name_state = NameState::new(body);
        let join_points = JoinPoints::new(body, dominance_frontier, &definitions);
        let consume_chain = ConsumeChain::new(body, definitions);
        SSAState {
            name_state,
            join_points,
            consume_chain,
        }
    }
}

impl SSAState {
    #[inline]
    pub(crate) fn try_consume_at(&mut self, local: Local, location: Location) -> Option<Consume> {
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        let consume = self.consume_chain.consumes[location.block.index()][location.statement_index]
            .get_mut(&local)?;
        let old_ssa_idx = self.name_state.get_name(local);
        let new_ssa_idx = self.name_state.generate_fresh_name(local);
        tracing::debug!(
            "consuming {:?} at {:?}, use: {:?}, def: {:?}",
            local,
            location,
            old_ssa_idx,
            new_ssa_idx
        );
        consume.r#use = old_ssa_idx;
        consume.def = new_ssa_idx;
        let consume = consume.clone();
        // tracing::debug!("consume chain before: {:?}", &self.consume_chain.consumes[location.block.index()]);
        assert_eq!(
            new_ssa_idx,
            self.consume_chain.locs[local].push(location.into())
        );
        Some(consume)
    }

    #[inline]
    pub(crate) fn consume_at(&mut self, local: Local, location: Location) -> Consume {
        self.try_consume_at(local, location)
            .unwrap_or_else(|| panic!("{:?} isn't defined at {:?}", local, location))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NameState {
    count: IndexVec<Local, SSAIdx>,
    stack: IndexVec<Local, Vec<SSAIdx>>,
}

impl NameState {
    fn new<'tcx>(body: &Body<'tcx>) -> Self {
        let count = IndexVec::from_elem(SSAIdx::INIT, &body.local_decls);
        let stack = IndexVec::from_fn_n(
            |local| {
                matches!(
                    body.local_decls[local].local_info.as_deref(),
                    Some(LocalInfo::DerefTemp)
                )
                .then(|| Vec::new())
                .unwrap_or_else(|| vec![SSAIdx::INIT])
            },
            body.local_decls.len(),
        );
        // let stack = IndexVec::from_elem(vec![SSAIdx::INIT], &body.local_decls);
        NameState { count, stack }
    }

    #[inline]
    pub(crate) fn generate_fresh_name(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline]
    pub(crate) fn get_name(&self, var: Local) -> SSAIdx {
        self.try_get_name(var).unwrap_or_else(|| {
            panic!(
                "internal error: cannot find fresh name supply for {:?}",
                var
            )
        })
    }

    /// Get the newest version for a variable. If `None` is returned,
    /// this variable is uninitialised.
    #[inline]
    pub(crate) fn try_get_name(&self, var: Local) -> Option<SSAIdx> {
        self.stack[var].last().copied()
    }

    #[inline]
    pub(crate) fn pop(&mut self, var: Local) -> SSAIdx {
        let ssa_idx = self.stack[var]
            .pop()
            .unwrap_or_else(|| panic!("internal error: poping non existing version for {:?}", var));
        // tracing::debug!("popping {:?}~{:?}", var, ssa_idx);
        ssa_idx
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use smallvec::smallvec;

//     #[test]
//     fn test1() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![12u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![4u32.into(), 5u32.into()]]),
//             n_defs: vec![
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//                 smallvec![(0u32.into(), 1.try_into().unwrap())],
//             ],
//         };

//         assert_eq!(state.get_name(0), 5u32.into());
//         assert_eq!(state.generate_fresh_name(0), 13u32.into());
//         assert_eq!(state.get_name(0), 13u32.into());

//         assert_eq!(&state.n_defs[0][..], [(0u32.into(), 1.try_into().unwrap())]);
//         assert_eq!(&state.n_defs[1][..], [(0u32.into(), 2.try_into().unwrap())]);

//         state.remove_names(1);

//         assert_eq!(state.get_name(0), 4u32.into());
//         assert_eq!(state.n_defs.len(), 1);
//     }

//     #[test]
//     fn test2() {
//         let mut state: NameState<u32> = NameState {
//             count: IndexVec::from_raw(vec![0u32.into()]),
//             stack: IndexVec::from_raw(vec![vec![0u32.into()]]),
//             n_defs: vec![],
//         };

//         state.enter_new_block();
//         assert_eq!(state.get_name(0), SSAIdx::INIT);
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 1u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 3u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());

//         state.enter_new_block();
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 4u32.into());
//         let _ = state.generate_fresh_name(0);
//         assert_eq!(state.get_name(0), 5u32.into());

//         state.remove_names(1);
//         assert_eq!(state.get_name(0), 2u32.into());
//     }
// }
