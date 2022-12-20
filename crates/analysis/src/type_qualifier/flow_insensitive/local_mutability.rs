use rustc_middle::{
    mir::{HasLocalDecls, Location, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;

use super::{
    mutability::{
        conservative_call, libc::libc_call, library::library_call, MutabilityLikeAnalysis,
    },
    AnalysisResult, FnLocalsVars, Infer, StructFieldsVars, Var,
};

pub fn local_mutability_analysis(crate_data: &common::CrateData) -> LocalMutabilityResult {
    LocalMutabilityResult::new(crate_data)
}

pub type LocalMutabilityResult = AnalysisResult<LocalMutabilityAnalysis>;

pub struct LocalMutabilityAnalysis;

impl MutabilityLikeAnalysis for LocalMutabilityAnalysis {}

impl Infer for LocalMutabilityAnalysis {
    fn infer_terminator<'tcx>(
        terminator: &Terminator<'tcx>,
        _location: Location,
        local_decls: &impl HasLocalDecls<'tcx>,
        locals: &[Var],
        _fn_locals: &FnLocalsVars,
        struct_fields: &StructFieldsVars,
        database: &mut <Self as Infer>::L,
        tcx: TyCtxt<'tcx>,
    ) {
        let TerminatorKind::Call {
            func,
            args,
            destination,
            ..
        } = &terminator.kind else { return };

        if let Some(func) = func.constant() {
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else { unreachable!() };

            if let Some(local_did) = callee.as_local() {
                match tcx.hir().find_by_def_id(local_did).unwrap() {
                    // this crate
                    rustc_hir::Node::Item(_) => {
                        return;
                    }
                    // extern
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        libc_call::<Self>(
                            destination,
                            args,
                            foreign_item.ident,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        );
                        return;
                    }
                    // in libxml2.rust/src/xmlschemastypes.rs/{} impl_xmlSchemaValDate/set_mon
                    rustc_hir::Node::ImplItem(_) => { /* TODO */ }
                    _ => unreachable!(),
                }
            } else {
                library_call::<Self>(
                    destination,
                    args,
                    callee,
                    local_decls,
                    locals,
                    struct_fields,
                    database,
                    tcx,
                );
                return;
            }
        } else {
            conservative_call::<Self>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
    }
}
