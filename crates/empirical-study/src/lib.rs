#![feature(rustc_private)]

extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_data_structures;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_type_ir;

use alias::taint_results;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

/// A set of empirical studies to be conducted on
/// input program
pub trait EmpiricalStudy<'tcx> {
    fn perform_empirical_study(&self) {
        macro_rules! perform {
            ($ ($func:ident), *) => {
                $ (
                    println!("Performing Study: {}", stringify!($func));
                    self.$func();
                ) *
            };
        }

        perform![
            compute_percentage_of_non_self_referential_structs,
            report_maybe_owning_fields,
            report_maybe_analysable_structs
        ];
    }

    fn compute_percentage_of_non_self_referential_structs(&self);
    fn report_maybe_owning_fields(&self);
    fn report_maybe_analysable_structs(&self);
}

// impl<'tcx, Input: OrcInput<'tcx>> EmpiricalStudy<'tcx> for Input {

impl<'tcx> EmpiricalStudy<'tcx> for common::CrateData<'tcx> {
    fn compute_percentage_of_non_self_referential_structs(&self) {
        let taint_results = taint_results(self);
        let aliasing_field_pairs = taint_results.aliasing_field_pairs();
        for (did, aliasing_field_pairs) in &aliasing_field_pairs {
            if !aliasing_field_pairs.is_empty() {
                let adt_def = self.tcx.adt_def(did);
                println!("{:?}", adt_def);
                let field_defs = &adt_def.variants().raw[0].fields;
                println!(
                    "aliasing fields: {}",
                    aliasing_field_pairs
                        .iter()
                        .map(|&(f, g)| format!("({}, {})", field_defs[f].name, field_defs[g].name))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }

        let num_maybe_self_referential_structs = aliasing_field_pairs
            .values()
            .filter(|pairs| !pairs.is_empty())
            .count();

        let percentage = format!(
            "{:.1}%",
            100.0 * num_maybe_self_referential_structs as f64 / self.structs.len() as f64
        );
        let table = vec![vec![
            self.structs.len().cell().justify(Justify::Right),
            num_maybe_self_referential_structs
                .cell()
                .justify(Justify::Right),
            percentage.cell().justify(Justify::Right),
        ]]
        .table()
        .title(vec![
            "# Struct Definitions".cell().bold(true),
            "# Maybe Self Referential Struct Definitions"
                .cell()
                .bold(true),
            "Percentage".cell().bold(true),
        ])
        .bold(true);

        assert!(print_stdout(table).is_ok());
    }

    fn report_maybe_owning_fields(&self) {
        let taint_results = taint_results(self);
        let maybe_owning_fields = taint_results.maybe_owning_fields();
        for (did, maybe_owning_fields) in maybe_owning_fields {
            if !maybe_owning_fields.is_empty() {
                let adt_def = self.tcx.adt_def(did);
                println!("{:?}", adt_def);
                let field_defs = &adt_def.variants().raw[0].fields;

                println!(
                    "maybe owning fields: {}",
                    maybe_owning_fields
                        .iter()
                        .map(|&f| field_defs[f].name.as_str().to_owned())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
    }

    fn report_maybe_analysable_structs(&self) {
        let taint_results = taint_results(self);
        let maybe_owning_fields = taint_results.maybe_owning_fields();
        let aliasing_field_pairs = taint_results.aliasing_field_pairs();

        for (did, maybe_owning_fields) in maybe_owning_fields {
            let aliasing_field_pairs = aliasing_field_pairs.get(&did).unwrap();
            let mut aliasing_fields = aliasing_field_pairs
                .iter()
                .flat_map(|&(x, y)| [x, y])
                .collect::<Vec<_>>();
            aliasing_fields.sort();
            aliasing_fields.dedup();

            let maybe_analysable = maybe_owning_fields
                .iter()
                .any(|&field| !aliasing_fields.contains(&field));

            if maybe_analysable {
                let hir_id = self
                    .tcx
                    .hir()
                    .local_def_id_to_hir_id(did.as_local().unwrap());
                println!(
                    "{}",
                    rustc_hir_pretty::id_to_string(&self.tcx.hir(), hir_id)
                );
            }
        }
    }
}
