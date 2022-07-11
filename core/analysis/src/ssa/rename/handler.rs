use std::marker::PhantomData;

use crate::{
    def_use::IsDefUse,
    ssa::{body_ext::PhiNodeInsertionPoints, rename::SSANameHandler, RichLocation},
    LocationMap,
};
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Local, Location};
use smallvec::{smallvec, SmallVec};
use tracing::debug;

use super::SSAIdx;

pub struct SSADefSites<DefUse: IsDefUse> {
    /// Invariant: SSAIdx::from_u32(0) -> RichLocation::Entry,
    /// and other indices are mapped to non-entry rich locations.
    pub defs: IndexVec<Local, IndexVec<SSAIdx, RichLocation>>,
    _marker: PhantomData<*const DefUse>,
}

impl<DefUse: IsDefUse> SSADefSites<DefUse> {
    pub fn new(body: &Body) -> Self {
        Self {
            defs: IndexVec::from_elem(
                IndexVec::from_raw(vec![RichLocation::Entry]),
                &body.local_decls,
            ),
            _marker: PhantomData,
        }
    }
}

impl<DefUse: IsDefUse> SSANameHandler for SSADefSites<DefUse> {
    type Output = ();

    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        assert_eq!(idx, self.defs[local].push(RichLocation::Mir(location)))
    }

    fn handle_use(&mut self, _local: Local, _idx: SSAIdx, _location: Location) -> Self::Output {}

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        assert_eq!(idx, self.defs[local].push(RichLocation::Phi(block)))
    }
}

/// A map that associated each local with its renames
pub struct SSANameSourceMap<DefUse: IsDefUse> {
    /// defs: Location -> Local -> SSAIdx
    /// uses: Location -> Local -> SSAIdx
    names: LocationMap<(
        /* defs */ SmallVec<[(Local, SSAIdx); 2]>,
        /* uses */ SmallVec<[(Local, SSAIdx); 2]>,
    )>,
    /// BasicBlock -> Local -> (SSAIdx, [SSAIdx])
    /// For phi node x_1 <- phi(x_2, x_3),
    /// we have an entry (1, [2, 3])
    names_for_phi_nodes: PhiNodeInsertionPoints<(SSAIdx, SmallVec<[SSAIdx; 2]>)>, // IndexVec<BasicBlock, SmallVec<[(Local, usize, SmallVec<[usize; 2]>); 2]>>,
    _marker: PhantomData<*const DefUse>,
}

impl<DefUse: IsDefUse> SSANameSourceMap<DefUse> {
    pub fn new(
        body: &Body,
        insertion_points: &PhiNodeInsertionPoints<PhantomData<*const DefUse>>,
    ) -> Self {
        let names_for_phi_nodes = insertion_points
            .iter_enumerated()
            .map(|(bb, bb_insertion_points)| {
                bb_insertion_points.repack(|_, _| {
                    let uses = smallvec![SSAIdx::MAX; body.basic_blocks.predecessors()[bb].len()];
                    (SSAIdx::MAX, uses)
                })
            })
            .collect::<IndexVec<_, _>>()
            .into();

        SSANameSourceMap {
            names: LocationMap::new(body),
            names_for_phi_nodes,
            _marker: PhantomData,
        }
    }

    pub fn defs(&self, location: Location) -> impl Iterator<Item = (Local, SSAIdx)> + '_ {
        self.names[location].0.iter().map(|&x| x)
    }

    pub fn try_def(&self, local: Local, location: Location) -> Option<SSAIdx> {
        self.names[location]
            .0
            .iter()
            .find_map(|&(this_local, idx)| (this_local == local).then_some(idx))
    }

    pub fn defs_at_phi_node(
        &self,
        block: BasicBlock,
    ) -> impl Iterator<Item = (Local, SSAIdx)> + '_ {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .map(|(local, &(ssa_idx, _))| (local, ssa_idx))
    }

    pub fn try_def_at_phi_node(&self, local: Local, block: BasicBlock) -> Option<SSAIdx> {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .find_map(|(this_local, &(idx, _))| (this_local == local).then_some(idx))
    }

    pub fn uses(&self, location: Location) -> impl Iterator<Item = (Local, SSAIdx)> + '_ {
        self.names[location].1.iter().map(|&x| x)
    }

    pub fn try_use(&self, local: Local, location: Location) -> Option<SSAIdx> {
        self.names[location]
            .1
            .iter()
            .find_map(|&(this_local, idx)| (this_local == local).then_some(idx))
    }

    pub fn uses_at_phi_node(
        &self,
        block: BasicBlock,
    ) -> impl Iterator<Item = (Local, &[SSAIdx])> + '_ {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .map(|(local, (_, uses))| (local, &uses[..]))
    }

    pub fn try_use_at_phi_node(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = SSAIdx> + '_ {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .find_map(|(this_local, (_, vec))| (this_local == local).then(|| vec.iter()))
            .unwrap_or_else(|| (&[]).iter())
            .map(|&idx| idx)
    }
}

impl<DefUse: IsDefUse> SSANameHandler for SSANameSourceMap<DefUse> {
    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) {
        // debug_assert!(self.names[location].0.is_none());
        // self.names[location].0 = Some((local, idx));
        if !self.names[location].0.iter().any(|&(lo, _)| lo == local) {
            self.names[location].0.push((local, idx))
        }
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) {
        if !self.names[location].1.iter().any(|&(lo, _)| lo == local) {
            self.names[location].1.push((local, idx))
        }
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        assert_eq!(self.names_for_phi_nodes[block][local].0, SSAIdx::MAX);
        self.names_for_phi_nodes[block][local].0 = idx;
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        assert_eq!(self.names_for_phi_nodes[block][local].1[pos], SSAIdx::MAX);
        self.names_for_phi_nodes[block][local].1[pos] = idx;
    }
}

macro_rules! make_new_name_debug_handler (
    ($Type: ident, $macro: ident) => {
        #[derive(Clone)]
        pub struct $Type;

        impl SSANameHandler for $Type {

            fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) {
                $macro!(
                    "rename definition of {:?} with {} at {:?}",
                    local,
                    idx,
                    location
                )
            }

            fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
                $macro!(
                    "rename definition of {:?} with {} at phi node of {:?}",
                    local,
                    idx,
                    block
                )
            }

            fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) {
                $macro!("rename use of {:?} with {} at {:?}", local, idx, location)
            }

            fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
                $macro!(
                    "rename use of {:?} with {} at phi node position {} of {:?}",
                    local,
                    idx,
                    pos,
                    block
                )
            }
        }
    }
);

make_new_name_debug_handler!(LogSSAName, debug);
make_new_name_debug_handler!(PrintStdSSAName, println);

impl SSANameHandler for () {
    type Output = ();

    fn handle_def(&mut self, _local: Local, _idx: SSAIdx, _location: Location) {}

    fn handle_use(&mut self, _local: Local, _idx: SSAIdx, _location: Location) {}
}

impl<H: SSANameHandler> SSANameHandler for &mut H {
    type Output = H::Output;
    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        (*self).handle_def(local, idx, location)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        (*self).handle_use(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        (*self).handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        (*self).handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<H1: SSANameHandler<Output = ()>, H2: SSANameHandler> SSANameHandler for (H1, H2) {
    type Output = H2::Output;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<H1: SSANameHandler<Output = ()>, H2: SSANameHandler<Output = ()>, H3: SSANameHandler>
    SSANameHandler for (H1, H2, H3)
{
    type Output = H3::Output;
    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location);
        self.2.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos);
        self.2.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<
        H1: SSANameHandler<Output = ()>,
        H2: SSANameHandler<Output = ()>,
        H3: SSANameHandler<Output = ()>,
        H4: SSANameHandler,
    > SSANameHandler for (H1, H2, H3, H4)
{
    type Output = H4::Output;

    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location);
        self.3.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block);
        self.3.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location);
        self.2.handle_use(local, idx, location);
        self.3.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos);
        self.2.handle_use_at_phi_node(local, idx, block, pos);
        self.3.handle_use_at_phi_node(local, idx, block, pos)
    }
}

/// `SSANameHandler` has a somehow 'applicative' interface
pub struct Product<H1, H2>
where
    H1: SSANameHandler,
    H2: SSANameHandler,
{
    h1: H1,
    h2: H2,
}

impl<H1, H2> SSANameHandler for Product<H1, H2>
where
    H1: SSANameHandler,
    H2: SSANameHandler,
{
    type Output = (H1::Output, H2::Output);

    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        (
            self.h1.handle_def(local, idx, location),
            self.h2.handle_def(local, idx, location),
        )
    }

    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output {
        (
            self.h1.handle_use(local, idx, location),
            self.h2.handle_use(local, idx, location),
        )
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {
        self.h1.handle_def_at_phi_node(local, idx, block);
        self.h2.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
        self.h1.handle_use_at_phi_node(local, idx, block, pos);
        self.h2.handle_use_at_phi_node(local, idx, block, pos)
    }
}
