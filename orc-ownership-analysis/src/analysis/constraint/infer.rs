use std::{borrow::BorrowMut, ops::Range};

// use orc_common::pointer::BorrowMut;
use rustc_data_structures::graph::WithSuccessors;
use rustc_index::vec::IndexVec;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, CastKind, Local, Location, Operand, Place, Rvalue, Statement,
    StatementKind, Terminator, TerminatorKind, RETURN_PLACE,
};

use crate::{
    analysis::{
        body_ext::DominanceFrontier,
        constraint::{CadicalDatabase, Database, OwnershipSig},
        def::{Consume, Definitions, RichLocation},
        state::{SSAIdx, SSAState},
        ty_ext::TyExt,
    },
    CrateCtxt,
};

// pub(crate) trait Context {
//     fn f(x: impl Context) -> bool {
//         false
//     }
// }

// impl Context for () {}
// impl<'me, 'tcx: 'me, DB: Database> Context for InferCtxt<'me, 'tcx, DB> {}

pub(crate) struct InferCtxt<'infercx, 'tcx, DB = CadicalDatabase> {
    database: DB,
    crate_ctxt: &'infercx CrateCtxt<'tcx>,
    local_sig: IndexVec<Local, IndexVec<SSAIdx, Range<OwnershipSig>>>,
    next: OwnershipSig,
    // store: VecArray<Range<OwnershipSig>>,
    // TODO: signatures for the function that is analysed (and perhaps
    // those in the same connected component)
}

impl<'infercx, 'tcx, DB> InferCtxt<'infercx, 'tcx, DB>
where
    'tcx: 'infercx,
    DB: Database,
{
    pub(crate) fn new(crate_ctxt: &CrateCtxt<'tcx>, body: &Body<'tcx>, db: DB) -> Self {
        InferCtxt {
            database: db,
            crate_ctxt,
            local_sig: todo!(),
            next: OwnershipSig::MIN,
        }
    }

    // pub(crate) fn consume_place(&mut self, place: &Place<'tcx>, consume: Consume) {}
    // pub(crate) fn transfer(
    //     &mut self,
    //     lhs: &Place<'tcx>,
    //     lhs_consume: Consume,
    //     rhs: &Place<'tcx>,
    //     rhs_consume: Consume,
    // ) {
    // }
}

/// FIXME: is it the right way?
pub(crate) trait Mode {
    type Ctxt<'infercx, 'tcx, DB>
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;

    fn consume_place<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume,
    ) where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;
}
#[derive(Debug)]
pub(crate) struct Pure;
#[derive(Debug)]
pub(crate) struct WithCtxt;
impl Mode for Pure {
    type Ctxt<'infercx, 'tcx, DB> = ()
    where
        'tcx: 'infercx,
        DB: Database + 'infercx;

    #[inline]
    fn consume_place<'infercx, 'tcx, DB>(
        infer_cx: &mut Self::Ctxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume,
    ) where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
    }
}
impl Mode for WithCtxt {
    type Ctxt<'infercx, 'tcx, DB> = InferCtxt<'infercx, 'tcx, DB>
    where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx;

    #[inline]
    fn consume_place<'infercx, 'tcx, DB>(
        infer_cx: &mut InferCtxt<'infercx, 'tcx, DB>,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Consume,
    ) where
        Self: 'infercx,
        'tcx: 'infercx,
        DB: Database + 'infercx,
    {
        let base = place.local;
        let base_ty = body.local_decls[base].ty;
        let base_offset = infer_cx.crate_ctxt.ty_ptr_count(base_ty);

        let local_sig = infer_cx.local_sig[base][consume.r#use].clone();

        assert_eq!(base_offset, local_sig.end.index() - local_sig.start.index());

        // let field_offsets = struct_topology.field_offsets(did)


        // let mut ty = base_ty;
        // for proj_elem in place.projection {

        // }

        // for (base_place, proj_elem) in place.iter_projections() {

        // }
    }
}

pub(crate) struct Renamer<'rn, 'tcx: 'rn> {
    body: &'rn Body<'tcx>,
    state: SSAState,
}

impl<'rn, 'tcx: 'rn> Renamer<'rn, 'tcx> {
    pub(crate) fn new(
        body: &'rn Body<'tcx>,
        dominance_frontier: &DominanceFrontier,
        definitions: Definitions,
    ) -> Self {
        Renamer {
            body,
            state: SSAState::new(body, dominance_frontier, definitions),
        }
    }

    pub(crate) fn go<M: Mode + 'rn, DB: Database + 'rn>(
        &mut self,
        mut infer_cx: impl BorrowMut<M::Ctxt<'rn, 'tcx, DB>>,
    ) {
        tracing::debug!("Renaming {:?}", self.body.source.def_id());
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
                    self.go_basic_block::<M, _>(
                        infer_cx.borrow_mut(),
                        bb,
                        &self.body.basic_blocks()[bb],
                    );
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for &(local, _) in self.state.consume_chain.of_block(bb).iter().flatten() {
                        let ssa_idx = self.state.name_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }
    }

    fn go_basic_block<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        bb: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) {
        tracing::debug!("Renaming {:?}", bb);

        let BasicBlockData {
            statements,
            terminator,
            is_cleanup: _,
        } = data;

        for (local, phi_node) in self.state.join_points[bb].iter_enumerated_mut() {
            let ssa_idx = self.state.name_state.generate_fresh_name(local);
            phi_node.lhs = ssa_idx;
            assert_eq!(
                self.state.consume_chain.locs[local].push(RichLocation::Phi(bb)),
                ssa_idx
            );
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx)
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_statement::<M, _>(infer_cx.borrow_mut(), statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_terminator::<M, _>(infer_cx, terminator, location);
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                // .unwrap_or_else(|| panic!("{:?}: {}", local, self.body.local_decls[local].ty));
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx)
            }
        }
    }

    fn go_statement<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        statement: &Statement<'tcx>,
        location: Location,
    ) {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<M, _>(infer_cx, place, rvalue, location)
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

    /// TODO: handle return?
    fn go_terminator<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        terminator: &Terminator<'tcx>,
        location: Location,
    ) {
        match &terminator.kind {
            TerminatorKind::Call {
                args, destination, ..
            } => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                let destination = destination.as_local().unwrap();
                let _ = self.state.try_consume_at(destination, location);
                for arg in args {
                    match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => {
                            let _ = self.state.try_consume_at(arg.local, location);
                        }
                        _ => continue,
                    }
                }
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);
                let return_place = RETURN_PLACE;
                let _ = self.state.try_consume_at(return_place, location);
                // finalise!
                for local in self.state.consume_chain.to_finalise() {
                    let _ = self.state.name_state.get_name(local);
                }
            }
            _ => {}
        }
    }

    fn go_assign<M: Mode, DB: Database + 'rn>(
        &mut self,
        infer_cx: &mut M::Ctxt<'rn, 'tcx, DB>,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        let lhs_consume = self.state.try_consume_at(lhs.local, location);
        //.map(|consume| M::consume_place(infer_cx, self.body, place, consume));

        match rhs {
            Rvalue::Use(Operand::Constant(_)) => {
                // assert!(lhs_consume.is_none(), "pointers cannot be mir constants: {:?}", constant)
                // if lhs_consume.is_some() {
                //     tracing::debug!("constant pointer rvalue {:?}", rhs)
                // }
                if let Some(lhs_consume) = lhs_consume {
                    M::consume_place(infer_cx, self.body, lhs, lhs_consume);
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                // even tho lhs is a pointer, it does not necessarily have an entry!
                // this is because we limit the nested level of pointers
                if let Some(lhs_consume) = lhs_consume {
                    M::consume_place(infer_cx, self.body, lhs, lhs_consume)
                }
                // lhs_consume.map(|lhs_consume| M::consume_place(infer_cx, self.body, lhs, lhs_consume));
                tracing::debug!("untrusted pointer source: raw address {:?}", operand)
            }

            Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Copy(rhs) | Operand::Move(rhs),
                _,
            ) => {
                let rhs_consume = self.state.consume_at(rhs.local, location);
                M::consume_place(infer_cx, self.body, rhs, rhs_consume);
                tracing::debug!("untrusted pointer sink: address {:?}", lhs);
            }

            Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                assert!(
                    lhs_consume.is_none(),
                    "TODO: constant pointer {:?}",
                    constant
                )
                // todo!("constant pointer {:?}, cast_kind: {:?}", constant)
            }

            Rvalue::Use(Operand::Move(rhs) | Operand::Copy(rhs))
            | Rvalue::Cast(_, Operand::Move(rhs) | Operand::Copy(rhs), _) => {
                let rhs_consume = self.state.try_consume_at(rhs.local, location);
                if let Some(consume) = lhs_consume {
                    M::consume_place(infer_cx, self.body, lhs, consume)
                }
                // lhs_consume.map(|consume| M::consume_place(infer_cx, self.body, lhs, consume));
                if let Some(consume) = rhs_consume {
                    M::consume_place(infer_cx, self.body, rhs, consume)
                }
                // rhs_consume.map(|consume| M::consume_place(infer_cx, self.body, rhs, consume));
                // let (Some(lhs_consume), Some(rhs_consume)) = (lhs_consume, rhs_consume) else { return };
                // tracing::error!("TODO");
            }

            Rvalue::CopyForDeref(rhs) => {
                tracing::warn!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                let lhs = lhs
                    .as_local()
                    .expect("we assume that rustc guarantees the lhs of `p = &q` being local");

                // tracing::error!("TODO");
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                // unreachable!("{:?}: {ty} cannot contain ptr", rhs)
                return;
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Aggregate(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_)
            | Rvalue::Repeat(_, _) => return, // unreachable!("rvalue {:?} is not assumed to appear", rhs),
        }
    }
}
