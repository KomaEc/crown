use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Operand, Place, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_span::{Ident, source_map::Spanned};
use rustc_type_ir::TyKind::FnDef;

pub trait TerminatorExt<'tcx> {
    fn call(&self) -> Option<(&Operand<'tcx>, &[Spanned<Operand<'tcx>>], Place<'tcx>)>;
}

impl<'tcx> TerminatorExt<'tcx> for Terminator<'tcx> {
    fn call(&self) -> Option<(&Operand<'tcx>, &[Spanned<Operand<'tcx>>], Place<'tcx>)> {
        match &self.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => Some((func, args, *destination)),
            TerminatorKind::TailCall { func, args, .. } => {
                Some((func, args, Place::return_place()))
            }
            _ => None,
        }
    }
}

pub enum CallKind {
    FreeStanding(DefId),
    Extern(Ident),
    Library(DefId),
    Impl(DefId),
    Closure,
    Dynamic,
}

impl CallKind {
    pub fn new<'tcx>(tcx: TyCtxt<'tcx>, func: &Operand<'tcx>) -> CallKind {
        if let Some(func) = func.constant() {
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else {
                unreachable!()
            };

            if let Some(local_did) = callee.as_local() {
                match tcx.hir_node_by_def_id(local_did) {
                    rustc_hir::Node::Item(_) => return CallKind::FreeStanding(callee),
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        return CallKind::Extern(foreign_item.ident);
                    }
                    rustc_hir::Node::ImplItem(_) => return CallKind::Impl(callee),
                    rustc_hir::Node::TraitItem(_) => return CallKind::Dynamic,
                    _ => unreachable!(),
                }
            } else {
                return CallKind::Library(callee);
            }
        } else {
            return CallKind::Closure;
        }
    }
}
