use rustc_middle::ty::Ty;
use rustc_type_ir::TyKind;

use crate::CrateCtxt;

use super::place_ext::place_abs::Offset;

pub(crate) trait TyExt<'tcx> {
    // fn contains_ptr(self, struct_topology: &StructTopology) -> bool;
    // fn ptr_count(self, struct_topology: &StructTopology) -> usize;
    fn is_tracked_ptr(self) -> bool;
}

impl<'tcx> TyExt<'tcx> for Ty<'tcx> {
    // #[inline]
    // fn contains_ptr(self, struct_topology: &StructTopology) -> bool {
    //     self.ptr_count(struct_topology) > 0
    // }

    // /// TODO: handle fn_ptr and tuple?
    // fn ptr_count(mut self, struct_topology: &StructTopology) -> usize {
    //     while let TyKind::Array(inner_ty, _) = self.kind() {
    //         self = *inner_ty
    //     }

    //     if self.is_tracked_ptr() {
    //         return 1;
    //     }

    //     // Notice: this has to be in accordance with struct topology
    //     let TyKind::Adt(adt_def, _) = self.kind() else { return 0 };
    //     // if !adt_def.is_struct() || !struct_topology.contains(&adt_def.did()) {
    //     //     return 0;
    //     // }
    //     let total_offset = struct_topology
    //         .struct_size(&adt_def.did())
    //         .map(Offset::index)
    //         .unwrap_or(0);
    //     return total_offset;
    // }

    #[inline]
    fn is_tracked_ptr(self) -> bool {
        self.is_unsafe_ptr() || self.is_region_ptr() || self.is_box()
    }
}

impl<'tcx> CrateCtxt<'tcx> {
    pub(crate) fn ty_contains_ptr(&self, ty: Ty<'tcx>) -> bool {
        self.ty_ptr_count(ty) > 0
    }

    pub(crate) fn ty_ptr_count(&self, mut ty: Ty<'tcx>) -> usize {
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
        let total_offset = self
            .struct_topology()
            .struct_size(&adt_def.did())
            .map(Offset::index)
            .unwrap_or(0);
        return total_offset;
    }
}
