#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(min_specialization)]
#![feature(crate_visibility_modifier)]
#![feature(bool_to_option)]

pub mod array_analysis;
pub mod call_graph;
pub mod def_use;
pub mod liveness_analysis;
pub mod ownership_analysis;
pub mod pointer_analysis;
pub mod ssa;
#[cfg(test)]
pub mod test;
pub mod toy_analysis;

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

use rustc_index::vec::IndexVec;
use rustc_middle::mir::{BasicBlock, Body, Location};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct LocationMap<T> {
    /// Location-indexed (BasicBlock for outer index, index within BB
    /// for inner index) map.
    pub(crate) map: IndexVec<BasicBlock, Vec<T>>,
}

impl<T> Index<Location> for LocationMap<T> {
    type Output = T;
    fn index(&self, index: Location) -> &Self::Output {
        &self.map[index.block][index.statement_index]
    }
}

impl<T> IndexMut<Location> for LocationMap<T> {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.map[index.block][index.statement_index]
    }
}

impl<T> LocationMap<T>
where
    T: Default + Clone,
{
    crate fn new(body: &Body<'_>) -> Self {
        LocationMap {
            map: body
                .basic_blocks()
                .iter()
                .map(|block| vec![T::default(); block.statements.len() + 1])
                .collect(),
        }
    }
}

rustc_index::newtype_index! {
    pub struct FieldDefIdx {
        DEBUG_FORMAT = "field_def ({})"
    }
}
