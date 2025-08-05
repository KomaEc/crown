pub mod generation;
pub mod watcher;

use crate::alias::steensgaard::AbstractLocation;

pub type Constraint = crate::alias::constraint::Constraint<AbstractLocation>;
