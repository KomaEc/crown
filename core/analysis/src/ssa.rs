use rustc_middle::mir::{BasicBlock, Location};

pub mod body_ext;
pub mod rename;
// pub mod pretty;
#[cfg(test)]
mod test;

#[derive(Clone, Debug)]
pub enum RichLocation {
    /// All Locals are assumed to be defined upon entry. This is an abstract location.
    Entry,
    Mir(Location),
    Phi(BasicBlock),
}
