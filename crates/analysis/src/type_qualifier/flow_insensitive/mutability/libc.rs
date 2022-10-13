use rustc_middle::mir::{HasLocalDecls, Operand, Place};
use rustc_span::symbol::Ident;

use super::{conservative_call, MutabilityAnalysis};
use crate::type_qualifier::flow_insensitive::{Infer, StructFieldsVars, Var};

pub fn libc_call<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    callee: Ident,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    match callee.as_str() {
        "strlen" => return call_strlen(),
        // malloc is skipped
        // "malloc" => return call_malloc(destination, local_decls, locals, struct_fields, database),
        // free is skipped
        // "free" => return call_free(args, local_decls, locals, struct_fields, database),
        // "calloc" => calloc_call(destination, local_decls, locals, struct_fields, database),
        // realloc is skipped
        // "realloc" => {},
        _ => {}
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

fn call_strlen<'tcx>() {
    // unconstrained
}

// fn call_malloc<'tcx>(
//     destination: &Place<'tcx>,
//     local_decls: &impl HasLocalDecls<'tcx>,
//     locals: &[Var],
//     struct_fields: &StructFieldsVars,
//     database: &mut <MutabilityAnalysis as Infer>::L,
// ) {
//     let dest_vars = place_vars::<true>(destination, local_decls, locals, struct_fields, database);
//     assert!(dest_vars.end > dest_vars.start);
//     database.bottom(dest_vars.start);
// }

// fn call_free<'tcx>(
//     args: &Vec<Operand<'tcx>>,
//     local_decls: &impl HasLocalDecls<'tcx>,
//     locals: &[Var],
//     struct_fields: &StructFieldsVars,
//     database: &mut <MutabilityAnalysis as Infer>::L,
// ) {
//     let arg = &args[0];
//     let Some(arg) = arg.place() else { return; };
//     let arg_vars = place_vars::<false>(&arg, local_decls, locals, struct_fields, database);
//     database.bottom(arg_vars.start);
// }
