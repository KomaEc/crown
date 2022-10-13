use rustc_middle::mir::{HasLocalDecls, Operand, Place};
use rustc_span::symbol::Ident;

use super::{place_vars, FatnessAnalysis};
use crate::type_qualifier::flow_insensitive::{ConstraintSystem, Infer, StructFieldsVars, Var};

pub fn libc_call<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
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
        "strlen" => {
            return call_strlen(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strstr" => {
            return call_strstr(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strcmp" => {
            return call_strcmp(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strncat" => {
            return call_strncat(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memcpy" => {
            return call_memcpy(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memmove" => {
            return call_memmove(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memset" => {
            return call_memset(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "calloc" => call_calloc(
            destination,
            args,
            local_decls,
            locals,
            struct_fields,
            database,
        ),
        // realloc is skipped
        "realloc" => call_realloc(
            destination,
            args,
            local_decls,
            locals,
            struct_fields,
            database,
        ),
        _ => {}
    }
}

fn call_calloc<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.end > dest_vars.start);
    database.bottom(dest_vars.start);
    let _ = args;
}

fn call_realloc<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(!dest_vars.is_empty());
    database.bottom(dest_vars.start);
    let ptr = &args[0];
    if let Some(ptr) = ptr.place() {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}

fn call_strlen<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let _ = destination;
    let ptr = &args[0];
    if let Some(ptr) = ptr.place() {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}

fn call_strstr<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.end > dest_vars.start);
    database.bottom(dest_vars.start);

    for ptr in args.iter().filter_map(|arg| arg.place()) {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}

fn call_strcmp<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let _ = destination;

    for ptr in args.iter().filter_map(|arg| arg.place()) {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}

fn call_strncat<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.end > dest_vars.start);
    database.bottom(dest_vars.start);

    for ptr in args.iter().take(2).filter_map(|arg| arg.place()) {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}

fn call_memcpy<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    call_strncat(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    )
}

fn call_memmove<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    call_memcpy(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    )
}

fn call_memset<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <FatnessAnalysis as Infer>::L,
) {
    let dest_vars = place_vars(destination, local_decls, locals, struct_fields);
    assert!(dest_vars.end > dest_vars.start);
    database.bottom(dest_vars.start);

    let ptr = &args[0];
    if let Some(ptr) = ptr.place() {
        let ptr_vars = place_vars(&ptr, local_decls, locals, struct_fields);
        assert!(!ptr_vars.is_empty());
        database.bottom(ptr_vars.start);
    }
}
