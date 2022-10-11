use rustc_middle::mir::{HasLocalDecls, Operand, Place};
use rustc_span::symbol::Ident;

use super::{place_vars, FatnessAnalysis};
use crate::type_qualifier::flow_insensitive::{ConstraintSystem, Infer, StructFieldsVars, Var};

pub fn libc_call<'tcx>(
    destination: &Place<'tcx>,
    _args: &Vec<Operand<'tcx>>,
    callee: Ident,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    match callee.as_str() {
        // malloc is skipped
        // "malloc" => {},
        // free is skipped
        // "free" => {},
        "calloc" => calloc_call(destination, local_decls, locals, struct_fields, database),
        // realloc is skipped
        // "realloc" => {},
        _ => {}
    }
}

fn calloc_call<'tcx>(
    destination: &Place<'tcx>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.end > dest_vars.start);
    database.bottom(dest_vars.start);
}
