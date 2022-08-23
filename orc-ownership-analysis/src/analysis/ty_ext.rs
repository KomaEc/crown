use rustc_middle::ty::Ty;
use rustc_type_ir::TyKind;

use super::OwnershipAnalysisCtxt;

pub trait TyExt<'tcx> {
    fn contains_ptr<'octxt>(self, octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>) -> bool;
    fn is_tracked_ptr(self) -> bool;
}

impl<'tcx> TyExt<'tcx> for Ty<'tcx> {
    /// TODO: handle fn_ptr and tuple?
    fn contains_ptr<'octxt>(self, octxt: &OwnershipAnalysisCtxt<'octxt, 'tcx>) -> bool {
        if self.is_tracked_ptr() {
            return true;
        }

        let TyKind::Adt(adt_def, _) = self.kind() else { return false };
        let Some(total_offset) = octxt.program.struct_topology().struct_offset(&adt_def.did()) else { return false };
        return total_offset.as_usize() > 0;
    }

    #[inline]
    fn is_tracked_ptr(self) -> bool {
        self.is_unsafe_ptr() || self.is_region_ptr() || self.is_box()
    }
}
