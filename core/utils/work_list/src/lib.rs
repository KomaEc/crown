#![feature(rustc_private)]

extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_serialize;

use rustc_data_structures::graph::{DirectedGraph, WithNumNodes, WithSuccessors};
use rustc_index::{bit_set::BitSet, vec::Idx};
use std::collections::VecDeque;

pub struct WorkList<'me, CV, G>
where
    CV: Idx,
    G: ?Sized + DirectedGraph<Node = CV> + WithNumNodes + WithSuccessors,
{
    pub dependency_graph: &'me G,
    in_list: BitSet<CV>,
    work_list: VecDeque<CV>,
}

impl<'me, CV, G> WorkList<'me, CV, G>
where
    CV: Idx,
    G: ?Sized + DirectedGraph<Node = CV> + WithNumNodes + WithSuccessors,
{
    pub fn new(dependency_graph: &'me G, nodes: impl Iterator<Item = CV>) -> Self {
        let ret = WorkList {
            dependency_graph,
            in_list: BitSet::new_filled(dependency_graph.num_nodes()),
            work_list: VecDeque::from_iter(nodes),
        };
        debug_assert!(ret.work_list.len() == ret.in_list.domain_size());
        ret
    }

    pub fn extract(&mut self) -> Option<CV> {
        let cv = self.work_list.pop_front()?;
        self.in_list.remove(cv);
        Some(cv)
    }

    pub fn insert(&mut self, cv: CV) -> bool {
        self.in_list.contains(cv) && {
            self.work_list.push_back(cv);
            true
        }
    }
}
