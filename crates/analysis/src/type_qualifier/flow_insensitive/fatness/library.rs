use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{HasLocalDecls, Operand, Place},
    ty::TyCtxt,
};

use super::{place_vars, FatnessAnalysis};
use crate::type_qualifier::flow_insensitive::{
    ConstraintSystem, StructFields, Var, WithConstraintSystem,
};

pub fn library_call<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    callee: DefId,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <FatnessAnalysis as WithConstraintSystem>::DB,
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
            if let rustc_hir::definitions::DefPathData::ValueNs(s) = d.data {
                match s.as_str() {
                    "offset" => {
                        return call_offset(
                            destination,
                            args,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        )
                    }
                    "offset_from" => {
                        return call_offset_from(
                            destination,
                            args,
                            local_decls,
                            locals,
                            struct_fields,
                            database,
                        )
                    }
                    "addr" => {
                        // no constraint
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn call_offset<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <FatnessAnalysis as WithConstraintSystem>::DB,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    if let Some(arg) = args[0].place() {
        let arg_vars = place_vars(&arg, local_decls, locals, struct_fields);
        let mut dest_arg = dest_vars.zip(arg_vars);

        if let Some((_, arg)) = dest_arg.next() {
            database.bottom(arg);
        }
        for (dest, arg) in dest_arg {
            database.guard(arg, dest);
            database.guard(dest, arg);
        }
    }
}

fn call_offset_from<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <FatnessAnalysis as WithConstraintSystem>::DB,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.is_empty());
    // no constraint on args
    for ptr in args.iter().filter_map(|arg| arg.place()) {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}
