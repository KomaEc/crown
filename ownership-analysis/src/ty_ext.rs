use rustc_middle::ty::Ty;

pub trait TyExt {
    fn is_ptr_but_not_fn_ptr(&self) -> bool;
}

impl<'tcx> TyExt for Ty<'tcx> {
    fn is_ptr_but_not_fn_ptr(&self) -> bool {
        self.is_unsafe_ptr() || self.is_region_ptr()
        // self.is_any_ptr() && {
        //     if self.is_fn_ptr() {
        //         panic!("fn ptr is not supported")
        //     } else {
        //         true
        //     }
        // }
    }
}
