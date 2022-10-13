use rustc_middle::mir::{HasLocalDecls, Operand, Place};
use rustc_span::symbol::Ident;

use super::{conservative_call, EnsureNoDeref, MutabilityAnalysis};
use crate::type_qualifier::flow_insensitive::{
    mutability::{place_vars, MutCtxt},
    ConstraintSystem, Infer, StructFieldsVars, Var,
};

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

fn call_strlen<'tcx>(
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

fn call_strcmp<'tcx>(
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

fn call_strncat<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    call_memcpy(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    );
}

fn call_strstr<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    // assert!(dest_vars.is_empty());
    // no constraint on args
    let ([haystack, needle], _) = args.split_array_ref();
    if let Some(haystack) = haystack.place() {
        let haystack =
            place_vars::<EnsureNoDeref>(&haystack, local_decls, locals, struct_fields, &mut ());

        let mut lhs_rhs = dest_vars.zip(haystack);
        if let Some((lhs, rhs)) = lhs_rhs.next() {
            database.guard(lhs, rhs);
        }
        for (lhs, rhs) in lhs_rhs {
            database.guard(lhs, rhs);
            database.guard(rhs, lhs)
        }
    }
    if let Some(needle) = needle.place() {
        let needle =
            place_vars::<EnsureNoDeref>(&needle, local_decls, locals, struct_fields, &mut ());
        let _ = needle;
    }
}

fn call_memcpy<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    // assert!(dest_vars.is_empty());
    // no constraint on args
    let ([memcpy_dest, _, _], _) = args.split_array_ref();
    if let Some(memcpy_dest) = memcpy_dest.place() {
        let memcpy_dest =
            place_vars::<EnsureNoDeref>(&memcpy_dest, local_decls, locals, struct_fields, &mut ());

        assert!(memcpy_dest.end > memcpy_dest.start);
        database.bottom(memcpy_dest.start);

        let mut lhs_rhs = dest_vars.zip(memcpy_dest);
        if let Some((lhs, rhs)) = lhs_rhs.next() {
            database.guard(lhs, rhs);
        }
        for (lhs, rhs) in lhs_rhs {
            database.guard(lhs, rhs);
            database.guard(rhs, lhs)
        }
    }
}

fn call_memmove<'tcx>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFieldsVars,
    database: &mut <MutabilityAnalysis as Infer>::L,
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
    database: &mut <MutabilityAnalysis as Infer>::L,
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
