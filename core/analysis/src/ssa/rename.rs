pub mod handler;
pub mod implementations;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::{Idx, IndexVec};
use rustc_middle::mir::{
    visit::{PlaceContext, Visitor},
    BasicBlock, BasicBlockData, Body, Local, LocalDecls, Location, Statement, Terminator,
};

use crate::{def_use::IsDefUse, ssa::body_ext::PhiNodeInsertionPoints};

use std::marker::PhantomData;

rustc_index::newtype_index! {
    pub struct SSAIdx {
        DEBUG_FORMAT = "{}"
    }
}

impl std::fmt::Display for SSAIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl SSAIdx {
    pub const ENTRY: Self = SSAIdx::from_u32(0);
}

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

pub trait HasSSARenameState<ProgramVar: Idx + std::fmt::Debug> {
    fn ssa_state(&mut self) -> &mut SSARenameState<ProgramVar>;
}

impl<ProgramVar: Idx> SSARenameState<ProgramVar> {
    #[inline(always)]
    pub fn define(&mut self, var: ProgramVar) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx.into()
    }
    #[inline(always)]
    pub fn r#use(&self, var: ProgramVar) -> SSAIdx {
        SSAIdx::from(*self.stack[var].last().expect(&format!("can't find definition for {:?}", var)))
    }
}

impl<ProgramVar: Idx> HasSSARenameState<ProgramVar> for SSARenameState<ProgramVar> {
    #[inline(always)]
    fn ssa_state(&mut self) -> &mut SSARenameState<ProgramVar> {
        self
    }
}

/// Types that implement `SSANameHandler` maybe handlers that are generic to
/// `DefUse`.
#[allow(unused_variables)]
pub trait SSANameHandler {
    type Output = ();
    fn handle_def(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output;
    fn handle_def_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock) {}
    fn handle_use(&mut self, local: Local, idx: SSAIdx, location: Location) -> Self::Output;
    fn handle_use_at_phi_node(&mut self, local: Local, idx: SSAIdx, block: BasicBlock, pos: usize) {
    }
}

/// Types that implement `HasSSANameHandler` should be the inference engine
/// with a specific `DefUse`
pub trait HasSSANameHandler {
    type Handler: SSANameHandler;
    type DefUse: IsDefUse;
    fn ssa_name_handler(&mut self) -> &mut Self::Handler;
}

pub trait SSARename<'tcx>: HasSSARenameState<Local> + HasSSANameHandler {
    #[inline]
    fn define_local(
        &mut self,
        local: Local,
        location: Location,
    ) -> <<Self as HasSSANameHandler>::Handler as SSANameHandler>::Output {
        let ssa_idx = self.ssa_state().define(local);
        self.ssa_name_handler().handle_def(local, ssa_idx, location)
    }

    #[inline]
    fn use_local(
        &mut self,
        local: Local,
        location: Location,
    ) -> <<Self as HasSSANameHandler>::Handler as SSANameHandler>::Output {
        let ssa_idx = self.ssa_state().r#use(local);
        self.ssa_name_handler().handle_use(local, ssa_idx, location)
    }

    fn rename_body(
        &mut self,
        body: &Body<'tcx>,
        insertion_points: &PhiNodeInsertionPoints<PhantomData<*const Self::DefUse>>,
    ) {
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
        debug_assert_eq!(root, BasicBlock::from_u32(0));
        let mut visitor_stack = vec![(root, false)];

        while let Some((bb, to_pop_stack)) = visitor_stack.pop() {
            if to_pop_stack {
                StackPopper::<Self::DefUse>(
                    &mut self.ssa_state().stack,
                    &body.local_decls,
                    PhantomData,
                )
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
        insertion_points: &PhiNodeInsertionPoints<PhantomData<*const Self::DefUse>>,
    ) {


        log::debug!("Renaming {:?}", block);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for local in insertion_points[block].locals() {
            let i = self.ssa_state().define(local);
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
            for local in insertion_points[succ].locals() {
                let i = self.ssa_state().r#use(local);
                self.ssa_name_handler()
                    .handle_use_at_phi_node(local, i, succ, pos);
            }
        }
    }

    fn rename_statement(&mut self, statement: &Statement<'tcx>, location: Location);

    fn rename_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location);
}

struct StackPopper<'me, 'tcx, DefUse: IsDefUse>(
    &'me mut IndexVec<Local, Vec<usize>>,
    &'me LocalDecls<'tcx>,
    PhantomData<*const DefUse>,
);

impl<'me, 'tcx, DefUse: IsDefUse> Visitor<'tcx> for StackPopper<'me, 'tcx, DefUse> {
    fn visit_local(&mut self, &local: &Local, context: PlaceContext, _location: Location) {
        if DefUse::categorize_finely(local, self.1, context).map_or(false, IsDefUse::defining) {
            let ssa_idx = self.0[local].pop();
            if let Some(ssa_idx) = ssa_idx {
                log::debug!("poping {:?}^{}", local, ssa_idx);
            }
        }
    }
}
