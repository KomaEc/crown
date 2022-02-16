use crustr_rewrite::Rewriter;
use rustc_hir::{
    intravisit::{self, nested_filter, Visitor},
    FieldDef, Mutability, Ty, TyKind,
};
use rustc_middle::{hir::map::Map, ty::TyCtxt};

const OPTION_NON_NULL_RAW_MUT_PREFIX: &'static str = "Option<crustr_ptr::NonNullRawMut<";
const OPTION_NON_NULL_RAW_MUT_SUFFIX: &'static str = ">";

struct TypeRewriteVisitor<'trv, 'tcx> {
    tcx: TyCtxt<'tcx>,
    rewriter: &'trv mut Rewriter,
}

impl<'trv, 'tcx> Visitor<'tcx> for TypeRewriteVisitor<'trv, 'tcx> {
    type Map = Map<'tcx>;

    type NestedFilter = nested_filter::None;

    /*
    fn nested_visit_map(&mut self) -> Self::Map {
        return NestedVisitorMap::OnlyBodies(self.tcx.hir());
    }
    */

    fn visit_ty(&mut self, typ: &'tcx Ty<'tcx>) {
        // log::debug!("visiting {} at {:?}", rustc_hir_pretty::ty_to_string(typ), typ.span);

        match typ.kind {
            TyKind::Ptr(ref mutable_type) => {
                // transform *mut only
                if let Mutability::Mut = mutable_type.mutbl {
                    log::debug!("visiting *mut at {:?}", typ.span);

                    // the span of the prefix *mut
                    let prefix_span = typ.span.until(mutable_type.ty.span);

                    self.rewriter.make_suggestion(
                        self.tcx,
                        prefix_span,
                        "rewriting *mut into Option<crustr_ptr::NonNullRawMut<".to_owned(),
                        OPTION_NON_NULL_RAW_MUT_PREFIX.to_owned(),
                    );

                    self.rewriter.make_suggestion(
                        self.tcx,
                        typ.span.shrink_to_hi(),
                        "adding suffix".to_owned(),
                        OPTION_NON_NULL_RAW_MUT_SUFFIX.to_owned(),
                    );
                }
            }
            _ => {}
        }

        intravisit::walk_ty(self, typ);
    }
}

pub struct StructRewriteVisitor<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub rewriter: Rewriter,
}

impl<'tcx> Visitor<'tcx> for StructRewriteVisitor<'tcx> {
    type Map = Map<'tcx>;

    type NestedFilter = nested_filter::None;

    /*
    fn nested_visit_map(&mut self) -> NestedVisitorMap<Self::Map> {
        return NestedVisitorMap::OnlyBodies(self.tcx.hir());
    }
    */

    fn visit_field_def(&mut self, field_def: &'tcx FieldDef<'tcx>) {
        // log::debug!("visiting field def {} : {} at {:?}", field_def.ident, rustc_hir_pretty::ty_to_string(field_def.ty), field_def.span);

        let mut ty_vis = TypeRewriteVisitor {
            tcx: self.tcx,
            rewriter: &mut self.rewriter,
        };
        ty_vis.visit_ty(field_def.ty);
    }
}
