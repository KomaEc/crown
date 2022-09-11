use rustc_middle::ty::Ty;
use rustc_type_ir::TyKind;

use crate::CrateCtxt;

pub trait TyExt<'tcx> {
    fn is_tracked_ptr(self) -> bool;
}

impl<'tcx> TyExt<'tcx> for Ty<'tcx> {
    #[inline]
    fn is_tracked_ptr(self) -> bool {
        self.is_unsafe_ptr() || self.is_region_ptr() || self.is_box()
    }
}

#[inline]
pub fn ty_contains_ptr(ty: Ty, crate_ctxt: &CrateCtxt) -> bool {
    ty_ptr_measure(ty, crate_ctxt) > 0
}

pub fn ty_ptr_measure(mut ty: Ty, crate_ctxt: &CrateCtxt) -> u32 {
    while let TyKind::Array(inner_ty, _) = ty.kind() {
        ty = *inner_ty
    }

    if ty.is_tracked_ptr() {
        return 1;
    }

    // Notice: this has to be in accordance with struct topology
    let TyKind::Adt(adt_def, _) = ty.kind() else { return 0 };
    // if !adt_def.is_struct() || !struct_topology.contains(&adt_def.did()) {
    //     return 0;
    // }
    let total_offset = crate_ctxt
        .struct_topology()
        .struct_size(&adt_def.did())
        // .map(Offset::index)
        .unwrap_or(0);
    return total_offset;
}
