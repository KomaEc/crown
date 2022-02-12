use crate::{
    ssa::{body_ext::PhiNodeInserted, rename::NewNameHandler},
    LocationMap,
};
use log::debug;
use rustc_index::vec::IndexVec;
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
    pub names_for_phi_nodes:
        IndexVec<BasicBlock, SmallVec<[(Local, usize, SmallVec<[usize; 2]>); 2]>>,
}

impl SSANameMap {
    pub fn new<'tcx>(body: &Body<'tcx>, insertion_points: &PhiNodeInserted) -> Self {
        let names_for_phi_nodes = body
            .basic_blocks()
            .indices()
            .map(|bb| {
                insertion_points[bb]
                    .iter()
                    .map(|&local| {
                        let uses = smallvec![0; body.predecessors()[bb].len()];
                        (local, 0, uses)
                    })
                    .collect::<SmallVec<_>>()
            })
            .collect::<IndexVec<_, _>>();
        SSANameMap {
            names: LocationMap::new(body),
            names_for_phi_nodes,
        }
    }
}

impl NewNameHandler for SSANameMap {
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        debug_assert!(self.names[location].0.is_none());
        self.names[location].0 = Some((local, idx));
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.names_for_phi_nodes[block]
            .iter_mut()
            .find(|(l, _, _)| *l == local)
            .map(|(_, def, _)| *def = idx)
            .expect("initialised")
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
        if !self.names[location].1.iter().any(|&(lo, _)| lo == local) {
            self.names[location].1.push((local, idx))
        }
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.names_for_phi_nodes[block]
            .iter_mut()
            .find(|(l, _, _)| *l == local)
            .map(|(_, _, uses)| uses[pos] = idx)
            .expect("initialised")
    }
}

macro_rules! make_new_name_debug_handler (
    ($Type: ident, $macro: ident) => {
        pub struct $Type;

        impl NewNameHandler for $Type {
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

make_new_name_debug_handler!(LogNewName, debug);
make_new_name_debug_handler!(PrintStdNewName, println);

impl<R1: NewNameHandler, R2: NewNameHandler> NewNameHandler for (R1, R2) {
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
        self.0.handle_use(local, idx, location);
        self.1.handle_use(local, idx, location)
    }

    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.0.handle_use_at_phi_node(local, idx, block, pos);
        self.1.handle_use_at_phi_node(local, idx, block, pos)
    }
}

impl<R1: NewNameHandler, R2: NewNameHandler, R3: NewNameHandler> NewNameHandler for (R1, R2, R3) {
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location);
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block)
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
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

impl<R1: NewNameHandler, R2: NewNameHandler, R3: NewNameHandler, R4: NewNameHandler> NewNameHandler
    for (R1, R2, R3, R4)
{
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
        self.0.handle_def(local, idx, location);
        self.1.handle_def(local, idx, location);
        self.2.handle_def(local, idx, location);
        self.3.handle_def(local, idx, location);
    }

    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.handle_def_at_phi_node(local, idx, block);
        self.1.handle_def_at_phi_node(local, idx, block);
        self.2.handle_def_at_phi_node(local, idx, block);
        self.3.handle_def_at_phi_node(local, idx, block);
    }

    fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
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
