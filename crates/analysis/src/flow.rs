use common::data_structure::vec_vec::VecVec;
use rustc_middle::mir::{BasicBlock, Location};

mod dom;
pub mod infer;
mod join_points;
pub mod ownership;
mod state;
mod update;
pub mod use_def;

common::macros::newtype_index! {
    #[debug_format = "{}"]
    pub struct SSAIdx {
    }
}

impl Default for SSAIdx {
    fn default() -> Self {
        Self::INIT
    }
}

impl SSAIdx {
    pub const INIT: Self = SSAIdx::from_u32(0);
}

impl std::ops::AddAssign<usize> for SSAIdx {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RichLocation {
    Entry,
    Phi(BasicBlock),
    Mir(Location),
}

impl From<Location> for RichLocation {
    fn from(location: Location) -> Self {
        RichLocation::Mir(location)
    }
}

pub struct LocationMap<T> {
    map: VecVec<T>,
}

impl<T> LocationMap<T> {
    pub fn get_block(&self, block: BasicBlock) -> &[T] {
        &self.map[block.index()]
    }

    pub fn get_location(&self, location: Location) -> &T {
        let Location {
            block,
            statement_index,
        } = location;
        &self.map[block.index()][statement_index]
    }
}
