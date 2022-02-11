pub mod impls;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    BasicBlock, BasicBlockData, Body, Local, Location, Place,
};

use crate::{def_use::DefUseCategorisable, ssa::body_ext::PhiNodeInserted};

use std::marker::PhantomData;

pub trait RenameHandler {
    fn handle_def(&mut self, local: Local, idx: usize, location: Location);
    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock);
    fn handle_use(&mut self, local: Local, idx: usize, location: Location);
    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize);
}

pub struct Renamer<'me, 'tcx, DefUse: DefUseCategorisable, R: RenameHandler> {
    body: &'me Body<'tcx>,
    insertion_points: &'me PhiNodeInserted,
    count: IndexVec<Local, usize>,
    stack: IndexVec<Local, Vec<usize>>,
    pub rename_handler: R,
    _marker: PhantomData<*const DefUse>,
}

impl<'me, 'tcx, DefUse: DefUseCategorisable, R: RenameHandler> Renamer<'me, 'tcx, DefUse, R> {
    pub fn new(body: &'me Body<'tcx>, insertion_points: &'me PhiNodeInserted, rename: R) -> Self {
        Renamer {
            body,
            insertion_points,
            count: IndexVec::from_elem(0, &body.local_decls),
            stack: IndexVec::from_elem(vec![0], &body.local_decls),
            rename_handler: rename,
            _marker: PhantomData,
        }
    }
}

impl<'me, 'tcx, DefUse: DefUseCategorisable, R: RenameHandler> Visitor<'tcx>
    for Renamer<'me, 'tcx, DefUse, R>
{
    fn visit_body(&mut self, body: &Body<'tcx>) {
        let dominators = body.dominators();
        let mut children = IndexVec::from_elem(vec![], body.basic_blocks());
        let mut root = BasicBlock::from_u32(0);
        body.basic_blocks().indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if dom != bb {
                children[dom].push(bb)
            } else {
                root = bb;
            }
        });
        // mir convention?
        assert_eq!(root, BasicBlock::from_u32(0));
        let mut visitor_stack = vec![(root, false)];

        while let Some((bb, to_pop_stack)) = visitor_stack.pop() {
            if to_pop_stack {
                struct StackPopper<'me, DefUse: DefUseCategorisable>(
                    &'me mut IndexVec<Local, Vec<usize>>,
                    PhantomData<*const DefUse>,
                );

                impl<'me, 'tcx, DefUse: DefUseCategorisable> Visitor<'tcx> for StackPopper<'tcx, DefUse> {
                    fn visit_place(
                        &mut self,
                        place: &Place<'tcx>,
                        context: PlaceContext,
                        _location: Location,
                    ) {
                        if DefUse::categorize(context)
                            .map_or(false, |def_use| DefUse::defining(def_use))
                        {
                            self.0[place.local].pop();
                        }
                    }
                }

                StackPopper::<DefUse>(&mut self.stack, PhantomData)
                    .visit_basic_block_data(bb, &body.basic_blocks()[bb]);
            } else {
                self.visit_basic_block_data(bb, &body.basic_blocks()[bb]);
                visitor_stack.push((bb, true));
                visitor_stack.extend(children[bb].iter().rev().map(|&bb| (bb, false)));
            }
        }
    }

    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for &local in &self.insertion_points[block] {
            // next available numbering
            self.count[local] += 1;
            let i = self.count[local];
            self.stack[local].push(i);
            self.rename_handler.handle_def_at_phi_node(local, i, block)
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block,
                statement_index: index,
            };
            self.visit_statement(statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block,
                statement_index: index,
            };
            self.visit_terminator(terminator, location);
        }

        for succ in self.body.successors(block) {
            let pos = self.body.predecessors()[succ]
                .iter()
                .position(|&pred| pred == block)
                .unwrap();
            for &local in &self.insertion_points[succ] {
                let &i = self.stack[local]
                    .last()
                    .expect(&format!("{:?} should be defined", local));
                self.rename_handler
                    .handle_use_at_phi_node(local, i, succ, pos);
            }
        }
    }

    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, location: Location) {
        if let Some(def_use) = DefUse::categorize(context) {
            if DefUse::defining(def_use) {
                self.count[place.local] += 1;
                let i = self.count[place.local];
                self.stack[place.local].push(i);
                self.rename_handler.handle_def(place.local, i, location);
            } else if DefUse::using(def_use) {
                let &i = self.stack[place.local]
                    .last()
                    .expect(&format!("{:?} should be defined", place.local));
                self.rename_handler.handle_use(place.local, i, location);
            }
        }
    }
}
