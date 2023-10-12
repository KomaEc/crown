use std::ops::Range;

use rustc_abi::FieldIdx;
use rustc_index::IndexVec;
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, BasicBlockData, BinOp, Body, Local, Location, NullOp, Operand,
        Place, Rvalue, Statement, Terminator, UnOp,
    },
    ty::{Const, TyCtxt},
};
use utils::data_structure::assoc::AssocExt;

use super::{
    access_path::Path,
    constraint::{Database, OwnershipToken},
    Ctxt,
};
use crate::flow::{
    def_use::{Def, DefUseChain, Inspect, Update, UseKind},
    // inference::{
    //     Engine, InferAssign, InferCall, InferIrrelevant, InferJoin, InferReturn, Inference,
    // },
    join_points::PhiNode,
    RichLocation,
    SSAIdx,
};

pub struct Analysis<'analysis, 'tcx, const K_LIMIT: usize, DB> {
    ctxt: &'analysis mut Ctxt<K_LIMIT, DB>,
    /// `Local -> SSAIdx -> first token`
    tokens: &'analysis IndexVec<Local, IndexVec<SSAIdx, OwnershipToken>>,
    flow_chain: &'analysis DefUseChain,
    body: &'analysis Body<'tcx>,
    tcx: TyCtxt<'tcx>,
}

type Base = (Local, UseKind<SSAIdx>);
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<Base>() == 12);
type ExpandedBase = Update<OwnershipToken>;
#[cfg(not(debug_assertions))]
const _: () = assert!(std::mem::size_of::<ExpandedBase>() == 8);

impl<'tcx, const K_LIMIT: usize, DB: Database> Analysis<'_, 'tcx, K_LIMIT, DB> {
    // Cached?
    fn base(&self, local: Local, ssa_idx: SSAIdx) -> OwnershipToken {
        // monotonicity constraints assumed
        self.tokens[local][ssa_idx]
    }

    /// If the path is a `Some`, then its size > 0
    fn path(&self, place: &Place<'tcx>, location: Location) -> Option<Path<Base>> {
        let path = self.ctxt.access_paths.path(place, self.body);
        let base = self.flow_chain.uses[location]
            .get_by_key(&place.local)
            .copied()?;
        (path.num_pointers_reachable() > 0)
            .then_some(Path::new((place.local, base), path.projections))
    }

    fn expand(&mut self, path: &Path<Base>) -> Path<ExpandedBase> {
        path.map_base(|(local, base)| {
            match base {
                Inspect(ssa_idx) => {
                    let def_loc = self.flow_chain.def_locs[local][ssa_idx];
                    match def_loc {
                        RichLocation::Entry => unreachable!("Inspecting entry definition. How?"),
                        RichLocation::Phi(block) => {
                            let phi_node = self.flow_chain.join_points[block]
                                .get_by_key(&local)
                                .expect("Definition location does not have phi node. How?");
                            unimplemented!(
                                "How to get the pre-state of {:?} at phi node {block:?}. Potentially, \
                                store two sets of tokens when defining phi-node. The first set represents \
                                the post-state, while the second set represents the pre-state and is to be \
                                unified with rhs of a phi node.",
                                phi_node.lhs
                            );
                        }
                        RichLocation::Mir(location) => {
                            let update = self.flow_chain.uses[location]
                                .get_by_key(&local)
                                .copied()
                                .and_then(|use_kind| use_kind.update())
                                .expect("Definition location does not define. How?");
                            update
                        }
                    }
                }
                Def(update) => update,
            }
            .map(|ssa_idx| self.tokens[local][ssa_idx])
        })
    }
}

impl<'tcx, const K_LIMIT: usize, DB: Database> Visitor<'tcx> for Analysis<'_, 'tcx, K_LIMIT, DB> {
    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &BasicBlockData<'tcx>) {
        for &(local, ref phi_node) in self.flow_chain.join_points[block].iter() {
            todo!(
                "Equate the ownership variables at phi-node {local:?}: {:?} = {}",
                phi_node.lhs,
                phi_node
                    .rhs
                    .iter()
                    .map(|ssa_idx| format!("{ssa_idx:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        self.super_basic_block_data(block, data);
    }

    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        let lhs = self.path(place, location);
        match rvalue {
            Rvalue::Use(operand) => match operand {
                Operand::Copy(rhs) => {}
                Operand::Move(_) => todo!(),
                Operand::Constant(_) => todo!(),
            },
            Rvalue::Repeat(_, _) => todo!(),
            Rvalue::Ref(_, _, _) => todo!(),
            Rvalue::ThreadLocalRef(_) => todo!(),
            Rvalue::AddressOf(_, _) => todo!(),
            Rvalue::Len(_) => todo!(),
            Rvalue::Cast(_, _, _) => todo!(),
            Rvalue::BinaryOp(_, _) => todo!(),
            Rvalue::CheckedBinaryOp(_, _) => todo!(),
            Rvalue::NullaryOp(_, _) => todo!(),
            Rvalue::UnaryOp(_, _) => todo!(),
            Rvalue::Discriminant(_) => todo!(),
            Rvalue::Aggregate(_, _) => todo!(),
            Rvalue::ShallowInitBox(_, _) => todo!(),
            Rvalue::CopyForDeref(_) => todo!(),
        }
        todo!("infer assignment {place:?} = {rvalue:?} at {location:?}")
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        self.super_statement(statement, location)
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        self.super_terminator(terminator, location)
    }
}
