use std::marker::PhantomData;

use crate::{
    def_use::IsDefUse,
    ssa::{body_ext::PhiNodeInsertionPoints, rename::SSANameHandler},
    ty_ext::TyExt,
    LocationMap,
};
use log::debug;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::mir::{BasicBlock, Body, Local, Location};
use smallvec::{smallvec, SmallVec};

/// A map that associated each local with its renames
pub struct SSANameMap {
    /// Location -> Local -> usize
    pub names: LocationMap<(
        /* defs */ Option<(Local, usize)>,
        /* uses */ SmallVec<[(Local, usize); 2]>,
    )>,
    /// BasicBlock -> Local -> (usize, [usize])
    pub names_for_phi_nodes: PhiNodeInsertionPoints<(usize, SmallVec<[usize; 2]>)>, // IndexVec<BasicBlock, SmallVec<[(Local, usize, SmallVec<[usize; 2]>); 2]>>,
}

impl SSANameMap {
    pub fn new<'tcx, DefUse: IsDefUse>(
        body: &Body<'tcx>,
        insertion_points: &PhiNodeInsertionPoints<PhantomData<*const DefUse>>,
    ) -> Self {
        let names_for_phi_nodes = insertion_points
            .iter_enumerated()
            .map(|(bb,bb_insertion_points)| {
                bb_insertion_points.repack(|_, _| {
                    let uses = smallvec![0; body.predecessors()[bb].len()];
                    (0, uses)
                })
            })
            .collect::<IndexVec<_, _>>()
            .into();

        SSANameMap {
            names: LocationMap::new(body),
            names_for_phi_nodes,
        }
    }

    pub fn lookup_def(&self, local: Local, location: Location) -> Option<usize> {
        let (this_local, idx) = self.names[location].0?;
        (this_local == local).then_some(idx)
    }

    pub fn lookup_def_in_phi_node(&self, local: Local, block: BasicBlock) -> Option<usize> {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .find_map(|(this_local, &(idx, _))| (this_local == local).then_some(idx))
    }

    pub fn lookup_use(&self, local: Local, location: Location) -> Option<usize> {
        self.names[location]
            .1
            .iter()
            .find_map(|&(this_local, idx)| (this_local == local).then_some(idx))
    }

    pub fn lookup_use_at_phi_node(
        &self,
        local: Local,
        block: BasicBlock,
    ) -> impl Iterator<Item = usize> + '_ {
        self.names_for_phi_nodes[block]
            .iter_enumerated()
            .find_map(|(this_local, (_, vec))| (this_local == local).then(|| vec.iter()))
            .unwrap_or_else(|| (&[]).iter())
            .map(|&idx| idx)
    }
}

impl SSANameHandler for SSANameMap {
    type Output = ();
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        debug_assert!(self.names[location].0.is_none());
        self.names[location].0 = Some((local, idx));
    }

    /*
    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.names_for_phi_nodes[block][local].0 = idx;
        .iter_mut()
        .find(|(l, _, _)| *l == local)
        .map(|(_, def, _)| *def = idx)
        .expect("initialised")
    }
    */

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
        if !self.names[location].1.iter().any(|&(lo, _)| lo == local) {
            self.names[location].1.push((local, idx))
        }
    }

    /*
    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.names_for_phi_nodes[block][local].1[pos] = idx
            .iter_mut()
            .find(|(l, _, _)| *l == local)
            .map(|(_, _, uses)| uses[pos] = idx)
            .expect("initialised")
    }
    */
}

macro_rules! make_new_name_debug_handler (
    ($Type: ident, $macro: ident) => {
        #[derive(Clone)]
        pub struct $Type;

        impl SSANameHandler for $Type {
            type Output = ();

            fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
                $macro!(
                    "rename definition of {:?} with {} at {:?}",
                    local,
                    idx,
                    location
                )
            }

            fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
                $macro!(
                    "rename definition of {:?} with {} at phi node of {:?}",
                    local,
                    idx,
                    block
                )
            }

            fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
                $macro!("rename use of {:?} with {} at {:?}", local, idx, location)
            }

            fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
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

/// Generate constraint variables for locals of pointer type (but not
/// function pointer).
/// Note that all renames of a local must preserves type, therefore
/// the `map` field supports linear lookup.
/// During renaming, all uses are processed before defs, therefore
/// it does not handle renames at uses.
pub struct LocalSimplePtrCVMap<'me, 'tcx, CV: Idx> {
    body: &'me Body<'tcx>,
    pub map: IndexVec<Local, Vec<CV>>,
    pub rev_map: IndexVec<CV, (Local, usize)>,
}

/// Assume all locals are defined at entry (this is not right for
/// locals that are not function arguments)
impl<'me, 'tcx, CV: Idx> LocalSimplePtrCVMap<'me, 'tcx, CV> {
    pub fn new(body: &'me Body<'tcx>) -> Self {
        let mut rev_map = IndexVec::new();
        let map = body
            .local_decls
            .iter_enumerated()
            .map(|(local, local_decl)| {
                // if it is local simple pointer type
                if local_decl.ty.is_ptr_of_concerned() {
                    let idx = rev_map.push((local, 0));
                    vec![idx]
                } else {
                    vec![]
                }
            })
            .collect::<IndexVec<_, _>>();
        LocalSimplePtrCVMap { body, map, rev_map }
    }

    #[inline]
    fn gen_def(&mut self, local: Local, idx: usize) {
        let ty = self.body.local_decls[local].ty;
        if ty.is_ptr_of_concerned() {
            let cv = self.rev_map.push((local, idx));
            assert_eq!(self.map[local].len(), idx);
            self.map[local].push(cv);
        }
    }
}

impl<'me, 'tcx, CV: Idx> SSANameHandler for LocalSimplePtrCVMap<'me, 'tcx, CV> {
    type Output = ();
    fn handle_def(&mut self, local: Local, idx: usize, _location: Location) {
        self.gen_def(local, idx)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, _block: BasicBlock) {
        self.gen_def(local, idx)
    }

    fn handle_use(&mut self, _local: Local, _idx: usize, _location: Location) {}
}

impl SSANameHandler for () {
    type Output = ();

    fn handle_def(&mut self, _local: Local, _idx: usize, _location: Location) {}

    fn handle_use(&mut self, _local: Local, _idx: usize, _location: Location) {}
}

impl<H: SSANameHandler> SSANameHandler for &mut H {
    type Output = H::Output;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        (*self).handle_def(local, idx, location)
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        (*self).handle_use(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        (*self).handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        (*self).handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<H1: SSANameHandler, H2: SSANameHandler> SSANameHandler for (H1, H2) {
    type Output = H2::Output;

    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<H1: SSANameHandler, H2: SSANameHandler, H3: SSANameHandler> SSANameHandler for (H1, H2, H3) {
    type Output = H3::Output;

    fn handle_def(
        &mut self,
        local: Local,
        idx: usize,
        location: Location,
    ) -> <H3 as SSANameHandler>::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(
        &mut self,
        local: Local,
        idx: usize,
        location: Location,
    ) -> <H3 as SSANameHandler>::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location);
        self.2.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos);
        self.2.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<H1: SSANameHandler, H2: SSANameHandler, H3: SSANameHandler, H4: SSANameHandler> SSANameHandler
    for (H1, H2, H3, H4)
{
    type Output = H4::Output;

    fn handle_def(
        &mut self,
        local: Local,
        idx: usize,
        location: Location,
    ) -> <H4 as SSANameHandler>::Output {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location);
        self.3.handle_def(local, idx, location)
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block);
        self.3.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(
        &mut self,
        local: Local,
        idx: usize,
        location: Location,
    ) -> <H4 as SSANameHandler>::Output {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location);
        self.2.handle_use(local, idx, location);
        self.3.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos);
        self.2.handle_use_at_phi_node(local, idx, block, pos);
        self.3.handle_use_at_phi_node(local, idx, block, pos)
    }
}
