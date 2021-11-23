use rustc_infer::traits::query::CanonicalProjectionGoal;
use rustc_middle::mir::{
    visit::Visitor, BasicBlock, BasicBlockData, Body, Local, Location, Operand, Place, PlaceRef,
    ProjectionElem, Rvalue, SourceInfo, Statement, StatementKind, Terminator, TerminatorKind,
    UserTypeProjection,
};

use log;
use rustc_middle::ty::TyCtxt;

pub struct PlaceSimplifier<'tcx, 'ps> {
    tcx: TyCtxt<'tcx>,
    body: &'ps mut Body<'tcx>,
}

impl<'tcx, 'ps> PlaceSimplifier<'tcx, 'ps> {}

use rustc_errors::{struct_span_err, DiagnosticBuilder, DiagnosticId};
use rustc_span::MultiSpan;
pub struct ComplexPlaceReporter<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> ComplexPlaceReporter<'tcx> {
    fn place_nested_level(place: &Place<'tcx>) -> usize {
        place.projection.len()
    }

    fn rvalue_is_complex(rvalue: &Rvalue<'tcx>) -> bool {
        match rvalue {
            Rvalue::Use(Operand::Copy(place)) | Rvalue::Use(Operand::Move(place)) => {
                Self::place_nested_level(place) >= 2
            }
            Rvalue::Ref(_, _, path) => Self::place_nested_level(path) >= 1,
            Rvalue::AddressOf(_, path) => Self::place_nested_level(path) >= 1,
            Rvalue::Cast(..) => {
                unimplemented!()
            }
            Rvalue::Discriminant(place) => Self::place_nested_level(place) >= 2,
            _ => false,
        }
    }

    fn struct_span_err_with_code<S: Into<MultiSpan>>(
        &self,
        sp: S,
        msg: &str,
        code: DiagnosticId,
    ) -> DiagnosticBuilder<'tcx> {
        self.tcx.sess.struct_span_err_with_code(sp, msg, code)
    }
}

impl<'tcx> Visitor<'tcx> for ComplexPlaceReporter<'tcx> {
    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        let Statement { source_info, kind } = statement;

        if let StatementKind::Assign(box (place, rvalue)) = kind {
            if ComplexPlaceReporter::rvalue_is_complex(rvalue) {
                let mut err = struct_span_err!(
                    self,
                    source_info.span,
                    ComplexRvalue,
                    "rvalue {:?} is complex!",
                    rvalue
                );
                err.emit();
            }

            if ComplexPlaceReporter::place_nested_level(place) >= 2 {
                // let local = place.local_or_deref_local();
                let mut err = struct_span_err!(
                    self,
                    source_info.span,
                    ComplexPlace,
                    "place {:?} is complex!",
                    place
                );
                err.emit();
            }
        }

        self.super_statement(statement, location);
    }
}
