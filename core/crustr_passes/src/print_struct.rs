use rustc_hir::{
    intravisit::{nested_filter, Visitor},
    FieldDef,
};
use rustc_middle::{hir::map::Map, ty::TyCtxt};

pub struct PrintStruct<'tcx> {
    pub tcx: TyCtxt<'tcx>,
}

impl<'tcx> Visitor<'tcx> for PrintStruct<'tcx> {
    type Map = Map<'tcx>;
    type NestedFilter = nested_filter::None;

    /*
    fn nested_visit_map(&mut self) -> NestedVisitorMap<Self::Map> {
        return NestedVisitorMap::OnlyBodies(self.tcx.hir());
    }
    */

    fn visit_field_def(&mut self, field_def: &'tcx FieldDef<'tcx>) {
        let hir_id = field_def.hir_id;
        let did = self.tcx.hir().local_def_id(hir_id);
        let ty = self.tcx.type_of(did);
        println!(
            "visiting field def {} : {} at {:?}",
            field_def.ident, ty, field_def.span
        );
        let walk = ty
            .walk()
            .map(|ty| format!("{}", ty))
            .collect::<Vec<_>>()
            .join(", ");
        println!("... with type decomposed as {}", walk);
    }
}
