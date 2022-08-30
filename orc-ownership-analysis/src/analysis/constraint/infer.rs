use std::ops::Range;

use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        visit::Visitor, BasicBlock, BasicBlockData, Body, CastKind, Constant, Local, Location,
        Operand, Place, Rvalue, Statement, StatementKind,
    },
    ty::TyCtxt,
};

use crate::{
    analysis::{
        def::Definitions,
        state::{SSAIdx, SSAState},
        ty_ext::TyExt,
    },
    struct_topology::StructTopology,
};

use super::{CadicalDatabase, Database, OwnershipSig};

/// Should it hold 'tcx at all?
pub(crate) struct InferCtxt<'me, DB = CadicalDatabase>
where
    DB: Database,
{
    database: &'me mut DB,
    local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
    // store: VecArray<Range<OwnershipSig>>,
    // TODO: signatures for the function that is analysed (and perhaps
    // those in the same connected component)
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<'me, DB>
    where
        Self: 'me,
        DB: Database + 'me;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<'me, DB: Database + 'me> = ();
}
impl Mode for WithCtxt {
    type Ctxt<'me, DB> = InferCtxt<'me, DB> where Self: 'me, DB: Database + 'me;
}

pub(crate) struct Renamer<'me, 'tcx: 'me> {
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    struct_topology: &'me StructTopology,
    state: SSAState,
    definitions: Definitions,
}

impl<'me, 'tcx: 'me> Renamer<'me, 'tcx> {
    pub(crate) fn go<M: Mode>(&mut self) {
        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], self.body.basic_blocks());
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks().indices().for_each(|bb| {
            let dom = dominators.immediate_dominator(bb);
            if dom != bb {
                children[dom].push(bb)
            } else {
                root = bb;
            }
        });
        assert_eq!(root, BasicBlock::from_u32(0));

        enum State {
            ToVisit,
            ToPopNames,
        }

        let mut recursion = vec![(root, State::ToVisit)];

        while let Some((bb, state)) = recursion.pop() {
            match state {
                State::ToVisit => {
                    // self.state.name_state.enter_new_block();
                    self.go_basic_block::<M>(bb, &self.body.basic_blocks()[bb]);
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for local in self.definitions.of_block(bb).iter().flatten().copied() {
                        self.state.name_state.pop(local);
                    }
                }
            }
        }
    }

    pub(crate) fn go_basic_block<M: Mode>(&mut self, bb: BasicBlock, data: &BasicBlockData) {
        tracing::debug!("Renaming {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for (local, phi_node) in self.state.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.state.name_state.generate_fresh_name(local);
            phi_node.lhs = ssa_idx;
            todo!()
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            // go statement
            todo!();
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            // go terminator
            todo!()
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                phi_node.rhs.push(ssa_idx)
            }
        }

        todo!()
    }

    fn go_statement<M: Mode>(&mut self, statement: &Statement<'tcx>, location: Location) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<M>(place, rvalue, location)
            }
            StatementKind::SetDiscriminant { .. } => {
                tracing::debug!("ignoring SetDiscriminant statement {:?}", statement)
            }
            StatementKind::Deinit(..) => {
                tracing::debug!("ignoring Deinit statement {:?}", statement)
            }
            StatementKind::AscribeUserType(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            | StatementKind::CopyNonOverlapping(_)
            | StatementKind::Nop => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }

    // fn try_define_place(&mut self, place: &Place<'tcx>, location: Location) -> Option<SSAIdx> {
    //     if self
    //         .definitions
    //         .of_location(location)
    //         .contains(&place.local)
    //     {
    //         let name_state = &mut self.state.name_state;
    //         let old_ssa_idx = name_state.get_name(place.local);
    //         let new_ssa_idx = name_state.generate_fresh_name(place.local);
    //         tracing::debug!("fresh name for {:?}: {:?}", place.local, new_ssa_idx);
    //         todo!();
    //         return Some(old_ssa_idx);
    //     }
    //     None
    // }

    fn go_assign<M: Mode>(
        &mut self,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        // TODO: this completely skips ptr to addr cast.
        let ty = lhs.ty(self.body, self.tcx).ty;
        if !ty.contains_ptr(self.struct_topology) {
            return;
        }

        // let def_of_loc = self.definitions.of_location(location);

        // if def_of_loc.contains(&lhs.local) {
        //     let ssa_idx = self.state.name_state.generate_fresh_name(lhs.local);
        //     tracing::debug!("fresh name for {:?}: {:?}", lhs.local, ssa_idx);
        // }

        match rhs {
            Rvalue::Use(Operand::Constant(..)) => {
                tracing::debug!("ignoring rvalue {:?}", rhs)
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, ..) => {
                tracing::debug!("ignoring rvalue {:?}", rhs)
            }

            Rvalue::Cast(CastKind::PointerExposeAddress, ..) => {
                unreachable!("lhs shoud be a pointer")
            }

            Rvalue::Cast(_, Operand::Constant(_constant), _) => {
                todo!("constant pointer {:?}", _constant)
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                // if def_of_loc.contains(&rhs.local) {
                //     let ssa_idx = self.state.name_state.generate_fresh_name(rhs.local);
                //     tracing::debug!("fresh name for {:?}: {:?}", rhs.local, ssa_idx);
                // }
            }

            Rvalue::CopyForDeref(rhs) => {}

            Rvalue::Ref(_, _, rhs) | Rvalue::AddressOf(_, rhs) => {
                let lhs = lhs
                    .as_local()
                    .expect("we assume that rustc guarantees the lhs of `p = &q` being local");
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                unreachable!("{:?}: {ty} cannot contain ptr", rhs)
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Aggregate(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Repeat(_, _) => unreachable!("rvalue {:?} is not assumed to appear", rhs),
        }
    }

    // fn go_place<M: Mode>(&mut self, place: &Place, location: Location) {
    //     if self
    //         .definitions
    //         .of_location(location)
    //         .contains(&place.local)
    //     {
    //         let ssa_idx = self.state.name_state.generate_fresh_name(place.local);
    //         tracing::debug!("fresh name for {:?}: {:?}", place.local, ssa_idx);
    //     }
    // }
}

pub(crate) enum SimplifiedAssignment<'me, 'tcx> {
    RhsPlace(Local, &'me Place<'tcx>),
    RhsConstant(Local, &'me Constant<'tcx>),
    /// lhs place must contain projections
    LhsPlace(&'me Place<'tcx>, Local),
    Addr(Local, &'me Place<'tcx>),
    DerefCopy(Local, &'me Place<'tcx>),
}

impl<'me, 'tcx> SimplifiedAssignment<'me, 'tcx> {
    pub(crate) fn from_assign(place: &'me Place<'tcx>, rvalue: &'me Rvalue<'tcx>) -> Self {
        todo!()
    }
}
