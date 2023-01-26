use rustc_middle::mir::{HasLocalDecls, Operand, Place};
use rustc_span::symbol::Ident;

use super::{conservative_call, EnsureNoDeref, MutabilityLikeAnalysis};
use crate::type_qualifier::flow_insensitive::{
    mutability::{place_vars, MutCtxt},
    ConstraintSystem, StructFields, Var, WithConstraintSystem,
};

pub fn libc_call<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    callee: Ident,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    match callee.as_str() {
        "strlen" => {
            return call_strlen::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strstr" => {
            return call_strstr::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strcmp" | "strcasecmp" => {
            return call_strcmp::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strcat" => {
            return call_strcat::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strncat" => {
            return call_strncat::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memcpy" => {
            return call_memcpy::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memmove" => {
            return call_memmove::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "memset" => {
            return call_memset::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strchr" => {
            return call_strchr::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strrchr" => {
            return call_strrchr::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strncmp" | "strncasecmp" => {
            return call_strncmp::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "printf" | "fprintf" | "sprintf" => {
            return call_printf::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "scanf" => {
            return call_scanf::<1, M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "fscanf" | "sscanf" => {
            return call_scanf::<2, M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "strdup" => {
            return call_strdup::<M>(
                destination,
                args,
                local_decls,
                locals,
                struct_fields,
                database,
            )
        }
        "fdopen" | "fopen" | "atoi" | "atof" => return,
        _ => {}
    }

    conservative_call::<M>(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    );
}

fn call_strlen<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_strcmp<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_strcat<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    // assert!(dest_vars.is_empty());
    // no constraint on args
    let ([memcpy_dest, _], _) = args.split_array_ref();
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

fn call_strncat<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    call_memcpy::<M>(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    );
}

fn call_strstr<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
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

fn call_memcpy<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
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

fn call_memmove<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    call_memcpy::<M>(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    )
}

fn call_memset<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    call_memcpy::<M>(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    )
}

fn call_strchr<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    call_strrchr::<M>(
        destination,
        args,
        local_decls,
        locals,
        struct_fields,
        database,
    )
}

fn call_strrchr<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
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

fn call_strncmp<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_printf<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    let _ = args;
}

fn call_scanf<'tcx, const MUT_START: usize, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    assert!(dest_vars.is_empty());
    // no constraint on args
    for arg in &args[MUT_START..] {
        if let Some(arg) = arg.place() {
            let arg =
                place_vars::<EnsureNoDeref>(&arg, local_decls, locals, struct_fields, &mut ());

            assert!(arg.end > arg.start);
            database.bottom(arg.start);
        }
    }
}

fn call_strdup<'tcx, M: MutabilityLikeAnalysis>(
    destination: &Place<'tcx>,
    args: &Vec<Operand<'tcx>>,
    local_decls: &impl HasLocalDecls<'tcx>,
    locals: &[Var],
    struct_fields: &StructFields,
    database: &mut <M as WithConstraintSystem>::DB,
) {
    let dest_vars =
        place_vars::<MutCtxt>(destination, local_decls, locals, struct_fields, database);
    let _ = dest_vars;
    // no constraint on args
    let _ = args;
}
