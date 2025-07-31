use rustc_hir::LangItem;
use rustc_middle::ty::{Ty, TyCtxt, TyKind};

pub trait TyExt {
    fn is_non_unit_tuple(&self) -> bool;

    /// is of the form `Option<unsafe extern "C" fn(...) -> ...>`
    fn is_c2rust_fn_ptr(&self, tcx: TyCtxt) -> bool;

    /// Bruh...
    fn from_libtree(&self) -> bool;
}

impl TyExt for Ty<'_> {
    fn is_non_unit_tuple(&self) -> bool {
        matches!(self.kind(), TyKind::Tuple(list) if list.len() > 0)
    }

    fn is_c2rust_fn_ptr(&self, tcx: TyCtxt) -> bool {
        match self.kind() {
            TyKind::Adt(adt_def, generics) => {
                let option_did = tcx.lang_items().get(LangItem::Option).unwrap();
                (adt_def.did() == option_did) && generics[0].as_type().unwrap().is_fn_ptr()
            }
            _ => false,
        }
    }

    fn from_libtree(&self) -> bool {
        format!("{self}").starts_with("src::libtree::")
    }
}

pub trait TyGate {
    fn gated(&self, tcx: TyCtxt);
}

impl TyGate for Ty<'_> {
    #[cfg(debug_assertions)]
    fn gated(&self, tcx: TyCtxt) {
        let ty = self;
        if ty.is_non_unit_tuple()
            || (ty.is_enum() && !ty.is_c_void(tcx) && !ty.is_c2rust_fn_ptr(tcx))
            || ty.is_union() && !ty.from_libtree()
        {
            unimplemented!("only user-defined structs are allowed, found {self}")
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn gated(&self, _: TyCtxt) {}
}
