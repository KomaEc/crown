use rustc_middle::ty::TyCtxt;
use rustc_type_ir::TyKind;
use utils::itertools::Itertools;

use crate::type_qualifier::TypeQualifiers;

fn display_slice<T: std::fmt::Display>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|value| format!("{value}"))
        .collect::<Vec<_>>()
        .join(" ")
}

impl<Qualifier: std::fmt::Display> TypeQualifiers<Qualifier> {
    pub fn pretty(&self, tcx: TyCtxt) -> String {
        let mut functions = self.pretty_functions(tcx).collect::<Vec<_>>();
        let mut structs = self.pretty_structs(tcx).collect::<Vec<_>>();

        functions.sort_by(|fst, snd| fst.0.cmp(&snd.0));
        structs.sort_by(|fst, snd| fst.0.cmp(&snd.0));

        let functions = functions
            .into_iter()
            .map(|(function_def_path, signature)| format!("{function_def_path}: {signature}"))
            .join("\n");

        let structs = structs
            .into_iter()
            .map(|(struct_def_path, fields)| format!("{struct_def_path} {{\n{fields}\n}}"))
            .join("\n");

        functions + "\n" + &structs
    }

    fn pretty_functions(&self, tcx: TyCtxt) -> impl Iterator<Item = (String, String)> {
        self.fn_locals.0.did_idx.keys().map(move |did| {
            let mut function_facts = self.function_facts(did, tcx).map(display_slice);

            let ret = function_facts.next().unwrap();
            let args = function_facts.join(", ");

            let function_def_path = tcx.def_path_str(*did);

            // format!("{fn_path}: ({args}) -> {ret}")
            (function_def_path, format!("({args}) -> {ret}"))
        })
    }

    fn pretty_structs(&self, tcx: TyCtxt) -> impl Iterator<Item = (String, String)> {
        self.struct_fields.0.did_idx.keys().map(move |did| {
            let struct_facts = self.struct_facts(did);
            let struct_ty = tcx.type_of(*did).skip_binder();

            let TyKind::Adt(adt_def, _) = struct_ty.kind() else {
                unreachable!()
            };

            let struct_def_path = tcx.def_path_str(*did);

            let fields = adt_def
                .all_fields()
                .zip(struct_facts)
                .map(|(field_def, qualifiers)| {
                    format!(
                        "  {}: {},",
                        field_def.ident(tcx).as_str(),
                        display_slice(qualifiers)
                    )
                })
                .join("\n");

            (struct_def_path, fields)
        })
    }
}
