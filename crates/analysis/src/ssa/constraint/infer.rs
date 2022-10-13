use std::borrow::BorrowMut;

use rustc_data_structures::graph::WithSuccessors;
use rustc_hir::def_id::DefId;
use rustc_index::vec::IndexVec;
use rustc_middle::{
    mir::{
        AggregateKind, BasicBlock, BasicBlockData, Body, CastKind, Local, Location,
        NonDivergingIntrinsic, Operand, Place, Rvalue, Statement, StatementKind, Terminator,
        TerminatorKind, RETURN_PLACE,
    },
    ty::Ty,
};
use rustc_type_ir::TyKind;

use crate::ssa::{
    consume::{Consume, RichLocation},
    join_points::PhiNode,
    state::{SSAIdx, SSAState},
};

pub trait InferMode<'infercx, 'db, 'tcx> {
    type Ctxt;

    type LocalSig: Clone + std::fmt::Debug;

    type CallArgs<T>: std::fmt::Debug
    where
        T: std::fmt::Debug;

    fn collect_call_args(
        call_args: impl Iterator<Item = Option<Consume<Self::LocalSig>>>,
    ) -> Self::CallArgs<Option<Consume<Self::LocalSig>>>;

    fn define_phi_node(infer_cx: &mut Self::Ctxt, local: Local, ty: Ty<'tcx>, def: SSAIdx);

    fn join_phi_nodes<'a>(
        infer_cx: &'a mut Self::Ctxt,
        phi_nodes: impl Iterator<Item = (Local, &'a mut PhiNode)>,
    );

    fn interpret_consume(
        infer_cx: &mut Self::Ctxt,
        body: &Body<'tcx>,
        place: &Place<'tcx>,
        consume: Option<Consume<SSAIdx>>,
    ) -> Option<Consume<Self::LocalSig>>;

    fn copy_for_deref(infer_cx: &mut Self::Ctxt, consume: Option<Consume<Self::LocalSig>>);

    fn transfer<const ENSURE_MOVE: bool>(
        infer_cx: &mut Self::Ctxt,
        lhs_result: Consume<Self::LocalSig>,
        rhs_result: Consume<Self::LocalSig>,
    );

    fn cast<const ENSURE_MOVE: bool>(
        infer_cx: &mut Self::Ctxt,
        lhs: Consume<Self::LocalSig>,
        rhs: Consume<Self::LocalSig>,
    );

    fn borrow(infer_cx: &mut Self::Ctxt, consume: Consume<Self::LocalSig>) {
        Self::assume(infer_cx, consume.r#use, false);
        Self::assume(infer_cx, consume.def, false);
    }

    fn source(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>) {
        Self::assume(infer_cx, result.r#use, false);
        Self::assume(infer_cx, result.def, true)
    }

    fn sink(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>) {
        Self::assume(infer_cx, result.r#use, true);
        Self::assume(infer_cx, result.def, false)
    }

    /// Impose no constraint on a definition. Constraints are still emitted
    /// because the old value of lhs must be non-owning
    fn unknown_source(infer_cx: &mut Self::Ctxt, result: Consume<Self::LocalSig>) {
        Self::assume(infer_cx, result.r#use, false)
    }

    fn unknown_sink(_: &mut Self::Ctxt, _: Consume<Self::LocalSig>);

    fn cast_to_c_void(
        infer_cx: &mut Self::Ctxt,
        consume: Consume<Self::LocalSig>,
    ) -> Consume<Self::LocalSig>;

    fn assume(infer_cx: &mut Self::Ctxt, result: Self::LocalSig, value: bool);

    fn finalize(infer_cx: &mut Self::Ctxt, local: Local, r#use: SSAIdx);

    fn call(
        infer_cx: &mut Self::Ctxt,
        args: Self::CallArgs<Option<Consume<Self::LocalSig>>>,
        callee: &Operand,
    );

    fn r#return(infer_cx: &mut Self::Ctxt, ssa_idx: Option<SSAIdx>, r#fn: DefId);
}
#[derive(Debug)]
pub enum Pure {}
impl<'infercx, 'db, 'tcx: 'infercx> InferMode<'infercx, 'db, 'tcx> for Pure {
    type Ctxt = ();

    type LocalSig = ();

    type CallArgs<T> = () where T: std::fmt::Debug;

    fn collect_call_args(call_args: impl Iterator<Item = Option<Consume<Self::LocalSig>>>) {
        for _ in call_args {}
    }

    fn define_phi_node((): &mut Self::Ctxt, _: Local, _: Ty, _: SSAIdx) {}

    fn join_phi_nodes<'a>(
        (): &'a mut Self::Ctxt,
        _: impl Iterator<Item = (Local, &'a mut PhiNode)>,
    ) {
    }

    #[inline]
    fn interpret_consume(
        (): &mut Self::Ctxt,
        _: &Body<'tcx>,
        _: &Place<'tcx>,
        _: Option<Consume<SSAIdx>>,
    ) -> Option<Consume<()>> {
        None
    }

    fn copy_for_deref((): &mut Self::Ctxt, _: Option<Consume<Self::LocalSig>>) {}

    #[inline]
    fn transfer<const ENSURE_MOVE: bool>(
        (): &mut Self::Ctxt,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
    ) {
    }

    #[inline]
    fn cast<const ENSURE_MOVE: bool>(
        (): &mut Self::Ctxt,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
        Consume { r#use: (), def: () }: Consume<Self::LocalSig>,
    ) {
    }

    fn unknown_sink((): &mut Self::Ctxt, _: Consume<Self::LocalSig>) {}

    fn assume((): &mut Self::Ctxt, (): Self::LocalSig, _: bool) {}

    fn finalize((): &mut Self::Ctxt, _: Local, _: SSAIdx) {}

    fn call((): &mut Self::Ctxt, (): (), _: &Operand) {}

    fn r#return((): &mut Self::Ctxt, _: Option<SSAIdx>, _: DefId) {}

    fn cast_to_c_void(
        (): &mut Self::Ctxt,
        consume: Consume<Self::LocalSig>,
    ) -> Consume<Self::LocalSig> {
        consume
    }
}

pub struct Renamer<'rn, 'tcx> {
    body: &'rn Body<'tcx>,
    pub state: SSAState,
}

#[inline]
pub fn consume_place_at<'rn, 'db, 'tcx, Infer>(
    place: &Place<'tcx>,
    body: &Body<'tcx>,
    location: Location,
    rn: &mut Renamer<'rn, 'tcx>,
    infer_cx: &mut Infer::Ctxt,
) -> Option<Consume<Infer::LocalSig>>
where
    'tcx: 'rn,
    Infer: InferMode<'rn, 'db, 'tcx>,
{
    let consume = rn.state.try_consume_at(place.local, location);
    Infer::interpret_consume(infer_cx, body, place, consume)
}

impl<'rn, 'tcx: 'rn> Renamer<'rn, 'tcx> {
    pub fn new(body: &'rn Body<'tcx>, state: SSAState) -> Self {
        Renamer { body, state }
    }

    pub fn go<'db, Infer>(&mut self, mut infer_cx: impl BorrowMut<Infer::Ctxt>)
    where
        Infer: InferMode<'rn, 'db, 'tcx> + 'rn,
    {
        tracing::debug!("Renaming {:?}", self.body.source.def_id());

        let dominators = self.body.basic_blocks.dominators();
        let mut children = IndexVec::from_elem(vec![], &self.body.basic_blocks);
        let mut root = BasicBlock::from_u32(0);
        self.body.basic_blocks.indices().for_each(|bb| {
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
                    self.go_basic_block::<Infer>(
                        infer_cx.borrow_mut(),
                        bb,
                        &self.body.basic_blocks[bb],
                    );
                    recursion.push((bb, State::ToPopNames));
                    recursion.extend(children[bb].iter().rev().map(|&bb| (bb, State::ToVisit)));
                }
                State::ToPopNames => {
                    for &(local, _) in self
                        .state
                        .consume_chain
                        .of_block(bb)
                        .iter()
                        .flatten()
                        .filter(|(_, consume)| !consume.is_use())
                    {
                        let ssa_idx = self.state.name_state.pop(local);
                        tracing::debug!("popping at {:?}: {:?}~{:?}", bb, local, ssa_idx);
                    }
                }
            }
        }

        Infer::join_phi_nodes(
            infer_cx.borrow_mut(),
            self.state.join_points.phi_nodes_mut(),
        );
    }

    fn go_basic_block<'db, Infer>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        bb: BasicBlock,
        data: &BasicBlockData<'tcx>,
    ) where
        Infer: InferMode<'rn, 'db, 'tcx>,
    {
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
            tracing::debug!("defining {:?} at Phi({:?}), def: {:?}", local, bb, ssa_idx);
            Infer::define_phi_node(infer_cx, local, self.body.local_decls[local].ty, ssa_idx);
        }

        let mut index = 0;
        for statement in statements {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_statement::<Infer>(infer_cx.borrow_mut(), statement, location);
            index += 1;
        }

        if let Some(terminator) = terminator {
            let location = Location {
                block: bb,
                statement_index: index,
            };
            self.go_terminator::<Infer>(infer_cx, terminator, location);
        }

        for succ in self.body.basic_blocks.successors(bb) {
            for (local, phi_node) in self.state.join_points[succ].iter_enumerated_mut() {
                let ssa_idx = self.state.name_state.get_name(local);
                phi_node.rhs.push(ssa_idx);
                tracing::debug!("using {:?} at Phi({:?}), use: {:?}", local, succ, ssa_idx)
            }
        }
    }

    fn go_statement<'db, Infer>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        statement: &Statement<'tcx>,
        location: Location,
    ) where
        Infer: InferMode<'rn, 'db, 'tcx>,
    {
        match &statement.kind {
            StatementKind::Assign(box (place, rvalue)) => {
                self.go_assign::<Infer>(infer_cx, place, rvalue, location)
            }
            StatementKind::SetDiscriminant { .. } => {
                tracing::debug!("ignoring SetDiscriminant statement {:?}", statement)
            }
            StatementKind::Deinit(..) => {
                tracing::debug!("ignoring Deinit statement {:?}", statement)
            }
            StatementKind::Intrinsic(box intrinsic) => {
                assert!(matches!(intrinsic, NonDivergingIntrinsic::Assume(..)))
            }
            StatementKind::AscribeUserType(_, _)
            | StatementKind::StorageLive(_)
            | StatementKind::StorageDead(_)
            | StatementKind::Retag(_, _)
            | StatementKind::FakeRead(_)
            | StatementKind::Coverage(_)
            | StatementKind::Nop => {
                unreachable!("statement {:?} is not assumed to appear", statement)
            }
        }
    }

    fn go_terminator<'db, Infer>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        terminator: &Terminator<'tcx>,
        location: Location,
    ) where
        Infer: InferMode<'rn, 'db, 'tcx>,
    {
        match &terminator.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => {
                tracing::debug!("processing terminator {:?}", terminator.kind);

                let call_args = std::iter::once(Some(destination))
                    .chain(args.iter().map(|arg| match arg {
                        Operand::Move(arg) | Operand::Copy(arg) => Some(arg),
                        Operand::Constant(..) => None,
                    }))
                    .map(|place| {
                        place.and_then(|place| {
                            consume_place_at::<Infer>(place, self.body, location, self, infer_cx)
                        })
                    });

                let call_args = Infer::collect_call_args(call_args);

                // println!("{:?}", self.body.source_info(location).span);
                // println!("{:?}", fn_sig);
                Infer::call(infer_cx, call_args, func);
            }
            TerminatorKind::Return => {
                tracing::debug!("processing terminator {:?}", terminator.kind);

                Infer::r#return(
                    infer_cx,
                    self.state.name_state.try_get_name(RETURN_PLACE),
                    self.body.source.def_id(),
                );

                // finalize!
                // note that return place should not be finalized!!
                for local in self.state.consume_chain.to_finalize() {
                    let r#use = self.state.name_state.get_name(local);
                    tracing::debug!("finalizing {:?}~{:?}", local, r#use);
                    Infer::finalize(infer_cx, local, r#use);
                }
            }
            _ => {}
        }
    }

    fn go_assign<'db, Infer>(
        &mut self,
        infer_cx: &mut Infer::Ctxt,
        place: &Place<'tcx>,
        rvalue: &Rvalue<'tcx>,
        location: Location,
    ) where
        Infer: InferMode<'rn, 'db, 'tcx>,
    {
        tracing::debug!("processing assignment {:?} = {:?}", place, rvalue);

        let lhs = place;
        let rhs = rvalue;

        match rhs {
            Rvalue::Use(Operand::Constant(_)) => {
                if let Some(lhs_consume) =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx)
                {
                    Infer::unknown_source(infer_cx, lhs_consume);
                    tracing::debug!("constant pointer rvalue {:?}", rhs)
                }
            }

            Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
                // even tho lhs is a pointer, it does not necessarily have an entry!
                // this is because we limit the nested level of pointers
                if let Some(lhs_consume) =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx)
                {
                    Infer::unknown_source(infer_cx, lhs_consume);
                    tracing::debug!("untrusted pointer source: raw address {:?}", operand)
                }
            }

            Rvalue::Cast(
                CastKind::PointerExposeAddress,
                Operand::Copy(rhs) | Operand::Move(rhs),
                _,
            ) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
                if let Some(rhs_consume) =
                    consume_place_at::<Infer>(rhs, self.body, location, self, infer_cx)
                {
                    // correctness?
                    Infer::unknown_sink(infer_cx, rhs_consume);
                    tracing::debug!("untrusted pointer sink: address {:?}", lhs);
                }
            }

            Rvalue::Cast(_, Operand::Constant(box constant), _) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(
                    lhs_consume.is_none(),
                    "TODO: constant pointer {:?}",
                    constant
                )
            }

            Rvalue::Use(operand @ Operand::Copy(rhs) | operand @ Operand::Move(rhs)) => {
                let lhs_consume =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx);
                let rhs_consume =
                    consume_place_at::<Infer>(rhs, self.body, location, self, infer_cx);

                match (lhs_consume, rhs_consume) {
                    (None, None) => {}
                    (None, Some(rhs_consume)) => Infer::unknown_sink(infer_cx, rhs_consume),
                    (Some(lhs_consume), None) => Infer::unknown_source(infer_cx, lhs_consume),
                    (Some(lhs_consume), Some(rhs_consume)) => {
                        if operand.is_move() {
                            Infer::transfer::<true>(infer_cx, lhs_consume, rhs_consume)
                        } else {
                            Infer::transfer::<false>(infer_cx, lhs_consume, rhs_consume)
                        }
                    }
                }
            }

            Rvalue::Cast(_, operand @ Operand::Move(rhs) | operand @ Operand::Copy(rhs), ty) => {
                let lhs_consume =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx);
                let mut rhs_consume =
                    consume_place_at::<Infer>(rhs, self.body, location, self, infer_cx);

                if let TyKind::RawPtr(pointee_ty) = ty.kind() {
                    let pointee_ty = pointee_ty.ty;
                    if &format!("{pointee_ty}") == "libc::c_void" {
                        rhs_consume =
                            rhs_consume.map(|consume| Infer::cast_to_c_void(infer_cx, consume));
                    }
                }
                let rhs_consume = rhs_consume;

                match (lhs_consume, rhs_consume) {
                    (None, None) => {}
                    (None, Some(rhs_consume)) => Infer::unknown_sink(infer_cx, rhs_consume),
                    (Some(lhs_consume), None) => Infer::unknown_source(infer_cx, lhs_consume),
                    (Some(lhs_consume), Some(rhs_consume)) => {
                        if operand.is_move() {
                            Infer::cast::<true>(infer_cx, lhs_consume, rhs_consume)
                        } else {
                            Infer::cast::<false>(infer_cx, lhs_consume, rhs_consume)
                        }
                    }
                }
            }

            Rvalue::CopyForDeref(rhs) => {
                /* TODO */
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
                let rhs_consume =
                    consume_place_at::<Infer>(rhs, self.body, location, self, infer_cx);
                Infer::copy_for_deref(infer_cx, rhs_consume);
                tracing::debug!("deref_copy is ignored")
            }

            Rvalue::Ref(_, _, _) | Rvalue::AddressOf(_, _) => {
                if let Some(lhs_consume) =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx)
                {
                    /* TODO */
                    // correctness???
                    Infer::borrow(infer_cx, lhs_consume)
                }
            }

            Rvalue::Repeat(Operand::Copy(rhs) | Operand::Move(rhs), _) => {
                let _ = consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx);
                let _ = consume_place_at::<Infer>(rhs, self.body, location, self, infer_cx);

                /* TODO */
            }

            Rvalue::Aggregate(box AggregateKind::Array(_), rhs_vec) => {
                let _ = consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx);

                for rhs in rhs_vec {
                    let Some(rhs) = rhs.place() else { continue };
                    let _ = consume_place_at::<Infer>(&rhs, self.body, location, self, infer_cx);
                }

                /* TODO */
            }

            Rvalue::Repeat(..) | Rvalue::Aggregate(..) => {
                // handle cases like _1 = [0 as *mut _; 50] or _1 = [move _12, move _13]

                // TODO note that vars in those rvalues are not counted as
                // definitions nor pure uses. If these are to be taken care
                // of, logic in initial_definition needs to be taken care of
                // as well

                let lhs_consume =
                    consume_place_at::<Infer>(lhs, self.body, location, self, infer_cx);
                if let Some(_) = lhs_consume { /* TODO */ }
            }

            Rvalue::BinaryOp(_, _) | Rvalue::CheckedBinaryOp(_, _) | Rvalue::UnaryOp(_, _) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
            }
            Rvalue::NullaryOp(_, _)
            | Rvalue::Discriminant(_)
            | Rvalue::Len(_)
            | Rvalue::ShallowInitBox(_, _)
            | Rvalue::ThreadLocalRef(_) => {
                let lhs_consume = self.state.try_consume_at(lhs.local, location);
                assert!(lhs_consume.is_none());
            }
        }
    }
}
