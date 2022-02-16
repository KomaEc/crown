pub mod handler;
pub mod impls;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    BasicBlock, BasicBlockData, Body, Local, Location, Statement, Terminator,
};

use crate::{def_use::DefUseCategorisable, ssa::body_ext::PhiNodeInserted};

use std::marker::PhantomData;

pub struct SSARenameState<ProgramVar: Idx> {
    count: IndexVec<ProgramVar, usize>,
    stack: IndexVec<ProgramVar, Vec<usize>>,
}

impl<ProgramVar: Idx> SSARenameState<ProgramVar> {
    pub fn new<T>(universe: &IndexVec<ProgramVar, T>) -> Self {
        SSARenameState {
            count: IndexVec::from_elem(0, universe),
            stack: IndexVec::from_elem(vec![0], universe),
        }
    }
}

pub trait HasSSARenameState<ProgramVar: Idx> {
    fn state(&mut self) -> &mut SSARenameState<ProgramVar>;
    #[inline(always)]
    fn define(&mut self, var: ProgramVar) -> usize {
        self.state().define(var)
    }
    #[inline(always)]
    fn r#use(&mut self, var: ProgramVar) -> usize {
        self.state().r#use(var)
    }
}

impl<ProgramVar: Idx> HasSSARenameState<ProgramVar> for SSARenameState<ProgramVar> {
    #[inline(always)]
    fn state(&mut self) -> &mut SSARenameState<ProgramVar> {
        self
    }

    #[inline(always)]
    fn define(&mut self, var: ProgramVar) -> usize {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx
    }

    #[inline(always)]
    fn r#use(&mut self, var: ProgramVar) -> usize {
        *self.stack[var].last().unwrap()
    }
}

#[allow(unused_variables)]
pub trait SSANameHandler {
    type Output: Clone + Copy + PartialEq + Eq;
    fn handle_def(&mut self, local: Local, idx: usize, location: Location) -> Self::Output;
    fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {}
    fn handle_use(&mut self, local: Local, idx: usize, location: Location) -> Self::Output;
    fn handle_use_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock, pos: usize) {}
}

pub trait HasSSANameHandler {
    type Handler: SSANameHandler;
    fn ssa_name_handler(&mut self) -> &mut Self::Handler;
}

pub trait SSARename<'tcx>: HasSSARenameState<Local> + HasSSANameHandler {
    type DefUse: DefUseCategorisable;

    fn rename_body(&mut self, body: &Body<'tcx>, insertion_points: &PhiNodeInserted) {
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
                StackPopper::<Self::DefUse>(&mut self.state().stack, PhantomData)
                    .visit_basic_block_data(bb, &body.basic_blocks()[bb]);
            } else {
                self.rename_basic_block_data(body, bb, &body.basic_blocks()[bb], insertion_points);
                visitor_stack.push((bb, true));
                visitor_stack.extend(children[bb].iter().rev().map(|&bb| (bb, false)));
            }
        }
    }

    fn rename_basic_block_data(
        &mut self,
        body: &Body<'tcx>,
        block: BasicBlock,
        data: &BasicBlockData<'tcx>,
        insertion_points: &PhiNodeInserted,
    ) {
        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for &local in &insertion_points[block] {
            let i = self.define(local);
            self.ssa_name_handler()
                .handle_def_at_phi_node(local, i, block);
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block,
                statement_index: index,
            };
            self.rename_statement(statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block,
                statement_index: index,
            };
            self.rename_terminator(terminator, location);
        }

        for succ in body.successors(block) {
            let pos = body.predecessors()[succ]
                .iter()
                .position(|&pred| pred == block)
                .unwrap();
            for &local in &insertion_points[succ] {
                let i = self.r#use(local);
                self.ssa_name_handler()
                    .handle_use_at_phi_node(local, i, succ, pos);
            }
        }
    }

    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location);

    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location);
}

struct StackPopper<'me, DefUse: DefUseCategorisable>(
    &'me mut IndexVec<Local, Vec<usize>>,
    PhantomData<*const DefUse>,
);

impl<'me, 'tcx, DefUse: DefUseCategorisable> Visitor<'tcx> for StackPopper<'tcx, DefUse> {
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, _location: Location) {
        if DefUse::categorize(context).map_or(false, |def_use| DefUse::defining(def_use)) {
            self.0[local].pop();
        }
    }
}
