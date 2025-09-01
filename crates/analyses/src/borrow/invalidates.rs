use rustc_index::bit_set::SparseBitMatrix;
use rustc_middle::{
    mir::{
        Body, CopyNonOverlapping, InlineAsmOperand, Location, NonDivergingIntrinsic, Operand,
        Place, Rvalue, Statement, StatementKind, Terminator, TerminatorKind, visit::Visitor,
    },
    ty::TyCtxt,
};
use rustc_mir_dataflow::points::{DenseLocationMap, PointIndex};

use crate::borrow::{
    BorrowSet, Loan,
    places_conflict::{AccessDepth, PlaceConflictBias, places_conflict},
};

pub(crate) type Invalidates = SparseBitMatrix<PointIndex, Loan>;

pub fn compute_invalidates<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    borrow_set: &BorrowSet<'tcx>,
    location_map: &DenseLocationMap,
) -> Invalidates {
    let mut invalidates = SparseBitMatrix::new(borrow_set.loans.len());

    LoanInvalidatesGenerator {
        facts: &mut invalidates,
        tcx,
        body,
        borrow_set,
        location_map,
    }
    .visit_body(body);

    invalidates
}

struct LoanInvalidatesGenerator<'g, 'tcx> {
    facts: &'g mut Invalidates,
    tcx: TyCtxt<'tcx>,
    body: &'g Body<'tcx>,
    borrow_set: &'g BorrowSet<'tcx>,
    location_map: &'g DenseLocationMap,
}

impl<'g, 'tcx> LoanInvalidatesGenerator<'g, 'tcx> {
    fn deeply_access_place(&mut self, location: Location, place: Place<'tcx>) {
        self.check_access_for_conflict(location, place, AccessDepth::Deep);
    }

    fn shallowly_access_place(&mut self, location: Location, place: Place<'tcx>) {
        self.check_access_for_conflict(location, place, AccessDepth::Shallow);
    }

    fn check_access_for_conflict(
        &mut self,
        location: Location,
        place: Place<'tcx>,
        access_depth: AccessDepth,
    ) {
        let Some(borrows_for_place_base) = self.borrow_set.local_map.row(place.local) else {
            return;
        };

        let point_index = self.location_map.point_from_location(location);

        for loan in borrows_for_place_base.iter() {
            let borrow_data = &self.borrow_set.loans[loan];
            if places_conflict(
                self.tcx,
                self.body,
                borrow_data.borrowed,
                place,
                access_depth,
                PlaceConflictBias::Overlap,
            ) {
                self.facts.insert(point_index, loan);
            }
        }
    }

    /// Simulates consumption of an operand.
    fn consume_operand(&mut self, location: Location, operand: &Operand<'tcx>) {
        match *operand {
            Operand::Copy(place) => {
                self.deeply_access_place(location, place);
            }
            Operand::Move(place) => {
                self.deeply_access_place(location, place);
            }
            Operand::Constant(_) => {}
        }
    }

    // Simulates consumption of an rvalue
    fn consume_rvalue(&mut self, location: Location, rvalue: &Rvalue<'tcx>) {
        match rvalue {
            &Rvalue::Ref(_ /*rgn*/, _, place) => {
                self.deeply_access_place(location, place);
            }

            &Rvalue::RawPtr(_, place) => {
                self.deeply_access_place(location, place);
            }

            Rvalue::ThreadLocalRef(_) => {}

            Rvalue::Use(operand)
            | Rvalue::Repeat(operand, _)
            | Rvalue::UnaryOp(_ /*un_op*/, operand)
            | Rvalue::Cast(_ /*cast_kind*/, operand, _ /*ty*/)
            | Rvalue::ShallowInitBox(operand, _ /*ty*/) => self.consume_operand(location, operand),

            &Rvalue::CopyForDeref(place) => {
                let op = &Operand::Copy(place);
                self.consume_operand(location, op);
            }

            &(Rvalue::Len(place) | Rvalue::Discriminant(place)) => {
                self.deeply_access_place(location, place);
            }

            Rvalue::BinaryOp(_bin_op, box (operand1, operand2)) => {
                self.consume_operand(location, operand1);
                self.consume_operand(location, operand2);
            }

            Rvalue::NullaryOp(_op, _ty) => {}

            Rvalue::Aggregate(_, operands) => {
                for operand in operands {
                    self.consume_operand(location, operand);
                }
            }

            Rvalue::WrapUnsafeBinder(op, _) => {
                self.consume_operand(location, op);
            }
        }
    }
}

/// Visits the whole MIR and generates `invalidates()` facts.
/// Most of the code implementing this was stolen from `borrow_check/mod.rs`.
impl<'g, 'tcx> Visitor<'tcx> for LoanInvalidatesGenerator<'g, 'tcx> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        match &statement.kind {
            StatementKind::Assign(box (lhs, rhs)) => {
                self.consume_rvalue(location, rhs);

                self.shallowly_access_place(location, *lhs);
            }
            StatementKind::FakeRead(box (_, _)) => {
                // Only relevant for initialized/liveness/safety checks.
            }
            StatementKind::Intrinsic(box NonDivergingIntrinsic::Assume(op)) => {
                self.consume_operand(location, op);
            }
            StatementKind::Intrinsic(box NonDivergingIntrinsic::CopyNonOverlapping(CopyNonOverlapping {
                src,
                dst,
                count,
            })) => {
                self.consume_operand(location, src);
                self.consume_operand(location, dst);
                self.consume_operand(location, count);
            }
            // Only relevant for mir typeck
            StatementKind::AscribeUserType(..)
            // Only relevant for liveness and unsafeck
            | StatementKind::PlaceMention(..)
            // Doesn't have any language semantics
            | StatementKind::Coverage(..)
            // Does not actually affect borrowck
            | StatementKind::StorageLive(..) => {}
            StatementKind::StorageDead(local) => {
                self.shallowly_access_place(
                    location,
                    Place::from(*local),
                );
            }
            StatementKind::ConstEvalCounter
            | StatementKind::Nop
            | StatementKind::Retag { .. }
            | StatementKind::Deinit(..)
            | StatementKind::BackwardIncompatibleDropHint { .. }
            | StatementKind::SetDiscriminant { .. } => {
                unreachable!("Statement not allowed in this MIR phase")
            }
        }

        self.super_statement(statement, location);
    }

    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        match &terminator.kind {
            TerminatorKind::SwitchInt { discr, targets: _ } => {
                self.consume_operand(location, discr);
            }
            TerminatorKind::Drop {
                place: drop_place,
                target: _,
                unwind: _,
                replace: _,
                drop: _,
                async_fut: _,
            } => {
                self.deeply_access_place(location, *drop_place);
            }
            TerminatorKind::Call {
                func,
                args,
                destination,
                target: _,
                unwind: _,
                call_source: _,
                fn_span: _,
            } => {
                self.consume_operand(location, func);
                for arg in args {
                    self.consume_operand(location, &arg.node);
                }
                self.deeply_access_place(location, *destination);
            }
            TerminatorKind::TailCall { func, args, .. } => {
                self.consume_operand(location, func);
                for arg in args {
                    self.consume_operand(location, &arg.node);
                }
            }
            TerminatorKind::Assert {
                cond,
                expected: _,
                msg,
                target: _,
                unwind: _,
            } => {
                self.consume_operand(location, cond);
                use rustc_middle::mir::AssertKind;
                if let AssertKind::BoundsCheck { len, index } = &**msg {
                    self.consume_operand(location, len);
                    self.consume_operand(location, index);
                }
            }
            TerminatorKind::Yield { .. } => {
                unimplemented!()
            }
            TerminatorKind::UnwindResume
            | TerminatorKind::Return
            | TerminatorKind::CoroutineDrop => {
                // Invalidate all borrows of local places
                let borrow_set = self.borrow_set;
                let point_index = self.location_map.point_from_location(location);
                for (i, data) in borrow_set.loans.iter_enumerated() {
                    if !data.borrowed.is_indirect() {
                        self.facts.insert(point_index, i);
                    }
                }
            }
            TerminatorKind::InlineAsm {
                asm_macro: _,
                template: _,
                operands,
                options: _,
                line_spans: _,
                targets: _,
                unwind: _,
            } => {
                for op in operands {
                    match op {
                        InlineAsmOperand::In { reg: _, value } => {
                            self.consume_operand(location, value);
                        }
                        InlineAsmOperand::Out {
                            reg: _,
                            late: _,
                            place,
                            ..
                        } => {
                            if let &Some(place) = place {
                                self.deeply_access_place(location, place);
                            }
                        }
                        InlineAsmOperand::InOut {
                            reg: _,
                            late: _,
                            in_value,
                            out_place,
                        } => {
                            self.consume_operand(location, in_value);
                            if let &Some(out_place) = out_place {
                                self.deeply_access_place(location, out_place);
                            }
                        }
                        InlineAsmOperand::Const { value: _ }
                        | InlineAsmOperand::SymFn { value: _ }
                        | InlineAsmOperand::SymStatic { def_id: _ }
                        | InlineAsmOperand::Label { target_index: _ } => {}
                    }
                }
            }
            TerminatorKind::Goto { target: _ }
            | TerminatorKind::UnwindTerminate(_)
            | TerminatorKind::Unreachable
            | TerminatorKind::FalseEdge {
                real_target: _,
                imaginary_target: _,
            }
            | TerminatorKind::FalseUnwind {
                real_target: _,
                unwind: _,
            } => {
                // no data used, thus irrelevant to borrowck
            }
        }

        self.super_terminator(terminator, location);
    }
}
