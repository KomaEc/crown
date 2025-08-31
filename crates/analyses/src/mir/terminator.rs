use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Operand, Place, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_span::{Ident, source_map::Spanned};
use rustc_type_ir::TyKind::FnDef;

pub struct MirFunctionCall<'call, 'tcx> {
    pub(crate) func: CallKind,
    pub(crate) args: &'call [Spanned<Operand<'tcx>>],
    pub(crate) destination: Place<'tcx>,
}

pub trait TerminatorExt<'tcx> {
    fn as_call<'a>(&'a self, tcx: TyCtxt<'tcx>) -> Option<MirFunctionCall<'a, 'tcx>>;
}

impl<'tcx> TerminatorExt<'tcx> for Terminator<'tcx> {
    fn as_call<'a>(&'a self, tcx: TyCtxt<'tcx>) -> Option<MirFunctionCall<'a, 'tcx>> {
        match &self.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => Some(MirFunctionCall {
                func: CallKind::new(tcx, func),
                args,
                destination: *destination,
            }),
            TerminatorKind::TailCall { func, args, .. } => Some(MirFunctionCall {
                func: CallKind::new(tcx, func),
                args,
                destination: Place::return_place(),
            }),
            _ => None,
        }
    }
}

pub enum CallKind {
    FreeStanding(DefId),
    /// Extern functions in **C2Rust** generated programs all come from _libc_
    LibC(Ident),
    /// Library calls mostly come from _stdlib_
    RustLib(DefId),
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
                        return CallKind::LibC(foreign_item.ident);
                    }
                    rustc_hir::Node::ImplItem(_) => return CallKind::Impl(callee),
                    rustc_hir::Node::TraitItem(_) => return CallKind::Dynamic,
                    _ => unreachable!(),
                }
            } else {
                return CallKind::RustLib(callee);
            }
        } else {
            return CallKind::Closure;
        }
    }

    pub fn did(&self) -> Option<DefId> {
        match self {
            CallKind::FreeStanding(def_id) => Some(*def_id),
            CallKind::LibC(_) => None,
            CallKind::RustLib(def_id) => Some(*def_id),
            CallKind::Impl(def_id) => Some(*def_id),
            CallKind::Closure => None,
            CallKind::Dynamic => None,
        }
    }
}
