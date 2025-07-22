use rustc_index::IndexVec;

use crate::access_path::ty_post_order::StructIndex;

pub struct SizeOf {
    cache: Vec<IndexVec<StructIndex, usize>>,
}

impl SizeOf {}
