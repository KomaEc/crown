pub(crate) mod size_of;
pub(crate) mod start_offset;

use rustc_index::IndexVec;

use crate::access_path::{KLimited, struct_lookup::StructIndex};

pub type IndirectionGraph = IndexVec<StructIndex, Vec<(usize, Option<StructIndex>)>>;

pub trait SizeOfable {
    fn size_of(&self, struct_index: KLimited<StructIndex>) -> usize;

    fn start_offset(&self, struct_index: KLimited<StructIndex>, field_idx: usize) -> usize;

    fn start_offsets(&self, struct_index: KLimited<StructIndex>)
    -> impl IntoIterator<Item = usize>;
}
