use rustc_errors::{struct_span_err, DiagnosticBuilder, DiagnosticId};
use rustc_hir::Node::*;
use rustc_hir_pretty::id_to_string;
use rustc_middle::{
    mir::{visit::Visitor, Body, Location, Operand, Place, Rvalue, Statement, StatementKind},
    ty::TyCtxt,
};
use rustc_span::MultiSpan;

pub struct ComplexPlaceReporter<'cpr, 'tcx> {
    tcx: TyCtxt<'tcx>,
    body: &'cpr Body<'tcx>,
}

impl<'cpr, 'tcx> ComplexPlaceReporter<'cpr, 'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, body: &'cpr Body<'tcx>) -> Self {
        ComplexPlaceReporter { tcx, body }
    }

    fn place_nested_level(place: &'cpr Place<'tcx>) -> usize {
        place.projection.len()
    }

    fn rvalue_is_complex(rvalue: &'cpr Rvalue<'tcx>) -> bool {
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

impl<'cpr, 'tcx> Visitor<'tcx> for ComplexPlaceReporter<'cpr, 'tcx> {
    fn visit_body(&mut self, body: &Body<'tcx>) {
        for (scope, _) in body.source_scopes.iter_enumerated() {
            let hir_id = scope
                .lint_root(&body.source_scopes)
                .expect("Mir source scopes should have hir_id");
            let node = self.tcx.hir().get(hir_id);
            match node {
                Item(_) => log::trace!("Got item, hir_id: {}", hir_id),
                Param(_) => todo!(),
                ForeignItem(_) => todo!(),
                TraitItem(_) => todo!(),
                ImplItem(_) => todo!(),
                Variant(_) => todo!(),
                Field(_) => todo!(),
                AnonConst(_) => todo!(),
                Expr(_) => todo!(),
                Stmt(stmt) => log::trace!("Yeah! Got statement: {:?}", stmt),
                PathSegment(_) => todo!(),
                Ty(_) => todo!(),
                TraitRef(_) => todo!(),
                Binding(_) => todo!(),
                Pat(_) => todo!(),
                Arm(_) => todo!(),
                Block(_) => todo!(),
                rustc_hir::Node::Local(_) => todo!(),
                Ctor(_) => todo!(),
                Lifetime(_) => todo!(),
                GenericParam(_) => todo!(),
                Visibility(_) => todo!(),
                Crate(_) => todo!(),
                Infer(_) => todo!(),
            }
        }

        self.super_body(body);
    }

    fn visit_statement(&mut self, statement: &Statement<'tcx>, location: Location) {
        let Statement { source_info, kind } = statement;

        // let source_info = self.body.source_info(location);

        if let StatementKind::Assign(box (place, rvalue)) = kind {
            let hir = source_info
                .scope
                .lint_root(&self.body.source_scopes)
                .expect("complex places or rvalues come from user hir");
            /*
            let node = self.tcx.hir().get(hir);
            match node {
                Item(item) => log::trace!("Got item: {:?}", item),
                Param(_) => todo!(),
                ForeignItem(_) => todo!(),
                TraitItem(_) => todo!(),
                ImplItem(_) => todo!(),
                Variant(_) => todo!(),
                Field(_) => todo!(),
                AnonConst(_) => todo!(),
                Expr(_) => todo!(),
                Stmt(stmt) => log::trace!("Yeah! Got statement: {:?}", stmt),
                PathSegment(_) => todo!(),
                Ty(_) => todo!(),
                TraitRef(_) => todo!(),
                Binding(_) => todo!(),
                Pat(_) => todo!(),
                Arm(_) => todo!(),
                Block(_) => todo!(),
                rustc_hir::Node::Local(_) => todo!(),
                Ctor(_) => todo!(),
                Lifetime(_) => todo!(),
                GenericParam(_) => todo!(),
                Visibility(_) => todo!(),
                Crate(_) => todo!(),
                Infer(_) => todo!(),
            }
            */

            if ComplexPlaceReporter::rvalue_is_complex(rvalue) {
                // let node = self.tcx.hir().get(hir);
                let mut err = struct_span_err!(
                    self,
                    source_info.span,
                    ComplexRvalue,
                    "rvalue {:?} is complex! Originated from hir item:\n{}",
                    rvalue,
                    id_to_string(&self.tcx.hir(), hir)
                );
                err.emit();
            }

            if ComplexPlaceReporter::place_nested_level(place) >= 2 {
                // let node = self.tcx.hir().get(hir);
                let mut err = struct_span_err!(
                    self,
                    source_info.span,
                    ComplexPlace,
                    "place {:?} is complex! Originated from hir item:\n{}",
                    place,
                    id_to_string(&self.tcx.hir(), hir)
                );
                err.emit();
            }
        }

        self.super_statement(statement, location);
    }
}
