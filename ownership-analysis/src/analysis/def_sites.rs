use rustc_index::{bit_set::BitSet, vec::IndexVec};
use rustc_middle::mir::{BasicBlock, Local};

pub type DefSites = IndexVec<Local, BitSet<BasicBlock>>;
