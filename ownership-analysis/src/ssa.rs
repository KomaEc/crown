use std::marker::PhantomData;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        BasicBlock, BasicBlockData, Body, Local, Location, Place, Statement, Terminator,
    },
    ty::TyCtxt,
};

use crate::{def_use::IsDefUse, ssa::body_ext::BodyExt};

use self::body_ext::PhiNodeInsertionPoints;

pub mod body_ext;
pub mod pretty;
pub mod rename;

#[derive(Clone, Debug)]
pub enum RichLocation {
    /// All Locals are assumed to be defined upon entry. This is an abstract location.
    Entry,
    Mir(Location),
    Phi(BasicBlock),
}

/*
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

pub struct RenameState {
    count: IndexVec<Local, usize>,
    stack: IndexVec<Local, Vec<usize>>,
}

impl RenameState {
    pub fn new(body: &Body) -> Self {
        let universe = &body.local_decls;
        RenameState {
            count: IndexVec::from_elem(0, universe),
            stack: IndexVec::from_elem(vec![0], universe),
        }
    }
    #[inline(always)]
    pub fn define(&mut self, var: Local) -> SSAIdx {
        self.count[var] += 1;
        let idx = self.count[var];
        self.stack[var].push(idx);
        idx.into()
    }
    #[inline(always)]
    pub fn r#use(&self, var: Local) -> SSAIdx {
        SSAIdx::from(
            *self.stack[var]
                .last()
                .expect(&format!("can't find definition for {:?}", var)),
        )
    }
}

pub trait Rename<'tcx> {
    type DefUse: IsDefUse;

    fn rename(
        &mut self,
        body: &Body<'tcx>,
        tcx: TyCtxt<'tcx>,
    ) {
        let insertion_points = body.compute_phi_node(tcx);
        let mut rename_state = RenameState::new(body);

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
                StackPopper::<Self::DefUse>(&mut rename_state.stack, body, tcx, PhantomData)
                    .visit_basic_block_data(bb, &body.basic_blocks()[bb]);
            } else {
                self.rename_basic_block_data(
                    body,
                    bb,
                    &body.basic_blocks()[bb],
                    &insertion_points,
                    &mut rename_state,
                );
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
        rename_state: &mut RenameState,
    ) {
        tracing::debug!("Renaming {:?}", block);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for local in insertion_points[block].locals() {
            let i = rename_state.define(local);
            self.ssa_name_handler()
                .handle_def_at_phi_node(local, i, block);
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block,
                statement_index: index,
            };
            self.rename_statement(statement, location, rename_state);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block,
                statement_index: index,
            };
            self.rename_terminator(terminator, location, rename_state);
        }

        for succ in body.successors(block) {
            let pos = body.predecessors()[succ]
                .iter()
                .position(|&pred| pred == block)
                .unwrap();
            for local in insertion_points[succ].locals() {
                let i = rename_state.r#use(local);
                self.ssa_name_handler()
                    .handle_use_at_phi_node(local, i, succ, pos);
            }
        }
    }

    fn rename_statement(
        &mut self,
        statement: &Statement<'tcx>,
        location: Location,
        rename_state: &mut RenameState,
    );

    fn rename_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        location: Location,
        rename_state: &mut RenameState,
    );
}

struct StackPopper<'me, 'tcx, DefUse: IsDefUse>(
    &'me mut IndexVec<Local, Vec<usize>>,
    &'me Body<'tcx>,
    TyCtxt<'tcx>,
    PhantomData<*const DefUse>,
);

impl<'me, 'tcx, DefUse: IsDefUse> Visitor<'tcx> for StackPopper<'me, 'tcx, DefUse> {
    fn visit_place(&mut self, place: &Place<'tcx>, context: PlaceContext, _location: Location) {
        if DefUse::categorise_for_place(place, context, self.1, self.2)
            .map_or(false, IsDefUse::defining)
        {
            let ssa_idx = self.0[place.local].pop();
            if let Some(ssa_idx) = ssa_idx {
                tracing::debug!("poping {:?}^{}", place.local, ssa_idx);
            }
        }
    }

    fn visit_local(&mut self, &local: &Local, context: PlaceContext, _location: Location) {
        if DefUse::categorise_for_local(local, &self.1.local_decls, context)
            .map_or(false, IsDefUse::defining)
        {
            let ssa_idx = self.0[local].pop();
            if let Some(ssa_idx) = ssa_idx {
                tracing::debug!("poping {:?}^{}", local, ssa_idx);
            }
        }
    }
}

*/
