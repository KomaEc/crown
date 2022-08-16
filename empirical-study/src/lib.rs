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

use orc_common::OrcInput;
use orc_taint_analysis::taint_results;
use rustc_middle::mir::{visit::Visitor, Location, Rvalue};

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
            compute_percentage_of_non_address_taking_functions,
            compute_percentage_of_non_self_referential_structs
        ];
    }

    fn compute_percentage_of_non_address_taking_functions(&self);
    fn compute_percentage_of_non_self_referential_structs(&self);
}

impl<'tcx, Input: OrcInput<'tcx>> EmpiricalStudy<'tcx> for Input {
    fn compute_percentage_of_non_address_taking_functions(&self) {
        struct Vis;
        impl<'tcx> Visitor<'tcx> for Vis {
            fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, _location: Location) {
                if let Rvalue::AddressOf(rustc_ast::Mutability::Mut, _) = rvalue {
                    panic!("{:?} to be catched", rvalue)
                }
            }
        }
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let n_address_taking_functions = self
            .functions()
            .iter()
            .filter(|&did| {
                let body = self.tcx().optimized_mir(did);
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Vis.visit_body(body)))
                    .is_ok()
            })
            .count();
        std::panic::set_hook(prev_hook);
        let percentage = format!(
            "{:.1}%",
            100.0 * n_address_taking_functions as f64 / self.functions().len() as f64
        );
        let table = vec![vec![
            self.functions().len().cell().justify(Justify::Right),
            n_address_taking_functions.cell().justify(Justify::Right),
            percentage.cell().justify(Justify::Right),
        ]]
        .table()
        .title(vec![
            "# Functions".cell().bold(true),
            "# Functions without &mut".cell().bold(true),
            "Percentage".cell().bold(true),
        ])
        .bold(true);

        assert!(print_stdout(table).is_ok());
    }

    fn compute_percentage_of_non_self_referential_structs(&self) {
        let taint_results = taint_results(self);
        let maybe_self_referential_structs = taint_results.maybe_self_referential_structs();
        for &did in self.structs() {
            let adt_def = self.tcx().adt_def(did);
            println!("{:?}", adt_def);
            let field_defs = &adt_def.variants().raw[0].fields;
            maybe_self_referential_structs.get(&did).map(|pairs| {
                println!(
                    "aliasing fields: {}",
                    pairs
                        .iter()
                        .map(|&(f, g)| format!("({}, {})", field_defs[f].name, field_defs[g].name))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            });
        }

        let percentage = format!(
            "{:.1}%",
            100.0 * maybe_self_referential_structs.len() as f64 / self.structs().len() as f64
        );
        let table = vec![vec![
            self.structs().len().cell().justify(Justify::Right),
            maybe_self_referential_structs
                .len()
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
}
