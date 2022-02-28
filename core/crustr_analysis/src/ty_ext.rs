use rustc_middle::ty::TyS;

pub trait TyExt {
    fn is_ptr_of_concerned(&self) -> bool;
}

impl<'tcx> TyExt for TyS<'tcx> {
    fn is_ptr_of_concerned(&self) -> bool {
        self.is_any_ptr() && {
            if self.is_fn_ptr() {
                panic!("fn ptr is not supported")
            } else {
                true
            }
        }
    }
}
