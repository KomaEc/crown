use rustc_middle::mir::Local;
use smallvec::SmallVec;

use super::{LocationMap, SSAIdx};

#[derive(Clone, Debug)]
pub struct Update<T> {
    pub r#use: T,
    pub def: T,
}

pub struct UpdateChain {
    /// `Location -> Local -> Update<SSAIdx>`
    pub updates: LocationMap<SmallVec<[(Local, Update<SSAIdx>); 2]>>,

    /// `Location -> Local -> Update<SSAIdx>`
    pub peeks: (),

    /// `Local -> SSAIdx -> RichLocation`
    pub def_locs: (),
}
