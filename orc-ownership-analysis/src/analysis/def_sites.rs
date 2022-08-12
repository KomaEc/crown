use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Body, Local};

pub type DefSites = IndexVec<Local, BitSet<BasicBlock>>;

pub fn initial_def_sites<'tcx>(body: &Body<'tcx>) -> DefSites {
    todo!()
}
