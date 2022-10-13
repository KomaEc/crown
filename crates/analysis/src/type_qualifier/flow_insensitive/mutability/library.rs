use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{HasLocalDecls, Operand, Place},
    ty::TyCtxt,
};

use super::{conservative_call, place_vars, EnsureNoDeref, MutCtxt, MutabilityAnalysis};
use crate::type_qualifier::flow_insensitive::{ConstraintSystem, Infer, StructFieldsVars, Var};

pub fn library_call<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    callee: DefId,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
    tcx: TyCtxt<'tcx>,
) {
    let def_path = tcx.def_path(callee);
    // if it is a library call in core::ptr
    if def_path
        .data
        .get(0)
        .map(|d| match d.data {
            rustc_hir::definitions::DefPathData::TypeNs(s) if s.as_str() == "ptr" => true,
            _ => false,
        })
        .is_some()
    {
        // if it is core::ptr::<..>::..
        if let Some(d) = def_path.data.get(3) {
            match d.data {
                rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "is_null" => {
                    call_is_null(
                        destination,
                        args,
                        local_decls,
                        locals,
                        struct_fields,
                        database,
                    );
                    return;
                }
                rustc_hir::definitions::DefPathData::ValueNs(s) if s.as_str() == "offset" => {
                    call_offset(
                        destination,
                        args,
                        local_decls,
                        locals,
                        struct_fields,
                        database,
                    );
                    return;
                }
                _ => {}
            }
        }

        conservative_call(
            destination,
            args,
            local_decls,
            locals,
            struct_fields,
            database,
        );
    }
}

fn call_is_null<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_offset<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    if let Some(arg) = args[0].place() {
        let arg_vars =
            place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());
        let mut dest_arg = dest_vars.zip(arg_vars);

        if let Some((dest, arg)) = dest_arg.next() {
            database.guard(dest, arg)
        }
        for (dest, arg) in dest_arg {
            database.guard(arg, dest);
            database.guard(dest, arg);
        }
    }
}
