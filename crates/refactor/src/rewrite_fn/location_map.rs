use std::ops::{Index, IndexMut};

use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Location};

#[derive(Debug)]
pub struct LocationMap<T>(pub IndexVec<BasicBlock, Vec<T>>);

impl<T> Index<Location> for LocationMap<T> {
    type Output = T;
    fn index(&self, index: Location) -> &Self::Output {
        &self.0[index.block][index.statement_index]
    }
}

impl<T> IndexMut<Location> for LocationMap<T> {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.0[index.block][index.statement_index]
    }
}

impl<T> LocationMap<T>
where
    T: Default + Clone,
{
    pub fn new(body: &Body<'_>) -> Self {
        LocationMap(
            body.basic_blocks
                .iter()
                .map(|block| vec![T::default(); block.statements.len() + 1])
                .collect(),
        )
    }
}
