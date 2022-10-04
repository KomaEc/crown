use rustc_middle::mir::{Local, Location};

use crate::ssa::consume::Consume;

pub mod constants;
pub mod constraint;
pub mod consume;
pub mod dom;
pub mod join_points;
pub mod state;

pub trait FnResult<'a> {
    type LocalResult;
    type LocationResults: Iterator<Item = (Local, Consume<Self::LocalResult>)>;

    fn local_result(&self, local: Local, location: Location) -> Option<Consume<Self::LocalResult>>;

    fn location_result(&'a self, location: Location) -> Self::LocationResults;
}
