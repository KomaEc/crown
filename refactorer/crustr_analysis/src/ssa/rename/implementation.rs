use rustc_middle::mir::{BasicBlock, Local, Location};

use crate::ssa::rename::RenameHandler;
use log::debug;

impl<R1: RenameHandler, R2: RenameHandler> RenameHandler for (R1, R2) {
    fn rename_def(&mut self, local: Local, idx: usize, location: Location) {
        self.0.rename_def(local, idx, location);
        self.1.rename_def(local, idx, location);
    }

    fn rename_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
        self.0.rename_def_at_phi_node(local, idx, block);
        self.1.rename_def_at_phi_node(local, idx, block)
    }

    fn rename_use(&mut self, local: Local, idx: usize, location: Location) {
        self.0.rename_use(local, idx, location);
        self.1.rename_use(local, idx, location)
    }

    fn rename_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
        self.0.rename_use_at_phi_node(local, idx, block, pos);
        self.1.rename_use_at_phi_node(local, idx, block, pos)
    }
}

macro_rules! make_rename_debug (
    ($Rename: ident, $macro: ident) => {
        pub struct $Rename;

        impl RenameHandler for $Rename {
            fn rename_def(&mut self, local: Local, idx: usize, location: Location) {
                $macro!(
                    "rename definition of {:?} with {} at {:?}",
                    local,
                    idx,
                    location
                )
            }

            fn rename_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
                $macro!(
                    "rename definition of {:?} with {} at phi node of {:?}",
                    local,
                    idx,
                    block
                )
            }

            fn rename_use(&mut self, local: Local, idx: usize, location: Location) {
                $macro!("rename use of {:?} with {} at {:?}", local, idx, location)
            }

            fn rename_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {
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

make_rename_debug!(LogRename, debug);
make_rename_debug!(PrintStdRename, println);
