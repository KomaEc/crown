//! Construct use-def chain via ssa

use rustc_middle::{mir::Body, ty::TyCtxt};

use crate::ssa::consume::{ConsumeChain, Definitions};

pub struct DefUseChain(ConsumeChain);

// fn compute_definitions(body: &Body, tcx: TyCtxt) -> Definitions {

// }
