//! The set of constants used in this analysis

/// Dominance frontier small vec size hint
pub const SIZE_DOM_FRONTIER: usize = 3;

/// Size hint for phi nodes per block
pub const NUM_PHI_NODES: usize = 3;

/// Size hint for phi node
pub const SIZE_PHI_NODE: usize = 3;

/// Size hint for number of definitions of a local within a block
pub const NUM_DEFS_PER_BLOCK: usize = 4;
