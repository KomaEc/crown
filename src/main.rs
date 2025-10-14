#![feature(rustc_private)]

extern crate once_cell;

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;
extern crate rustc_session;
extern crate rustc_target;

use analyses::mir_variable_grouping::SourceVarGroups;
use clap::Parser;
// use refactor::RefactorOptions;
use std::path::PathBuf;
use utils::{
    rewrite::RewriteMode,
    rustc::{RustProgram, RustProgramMapped, run_compiler, run_compiler_with_mapping},
};

use rustc_hir::{ItemKind, OwnerNode};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,

    /// Path to lib.rs or main.rs of crate to work on
    path: PathBuf,
}

#[derive(Parser, Debug, Clone)]
enum Command {
    Preprocess {
        #[clap(value_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
    },
    Rewrite {
        #[clap(long)]
        results_path: Option<PathBuf>,
        #[clap(value_enum, default_value_t = RewriteMode::Diff)]
        rewrite_mode: RewriteMode,
        // #[command(flatten)]
        // options: RefactorOptions,
    },
}

fn preprocess(path: &PathBuf, rewrite_mode: RewriteMode) -> Result<(), ()> {
    for preprocess in preprocess::PREPROCESSES {
        run_compiler(path.clone(), |program| {
            preprocess(program.tcx, rewrite_mode)
        });
    }
    Ok(())
}

fn run(
    dir: &std::path::Path,
    cmd: Command,
    rust_program_mapped: RustProgramMapped<'_>,
) -> Result<(), ()> {
    let rust_program = rust_program_mapped.rust_program;
    let tcx = rust_program.tcx;
    let mut functions = Vec::new();
    let mut structs = Vec::new();

    for maybe_owner in tcx.hir_crate(()).owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else {
            continue;
        };
        let OwnerNode::Item(item) = owner.node() else {
            continue;
        };
        match item.kind {
            ItemKind::Fn { .. } => functions.push(item.owner_id.def_id.to_def_id()),
            ItemKind::Struct(..) => structs.push(item.owner_id.def_id.to_def_id()),
            _ => {}
        };
    }

    let input = RustProgram {
        tcx,
        functions,
        structs,
    };

    match cmd {
        Command::Preprocess { .. } => unreachable!(),
        Command::Rewrite {
            results_path: _,
            rewrite_mode: _,
            // options,
        } => {
            let mutability_result =
                analyses::type_qualifier::foster::mutability::mutability_analysis(&input);
            let output_params =
                analyses::output_params::compute_output_params(&input, &mutability_result);

            let source_var_groups = SourceVarGroups::new(&input);
            let promoted_mut_refs = source_var_groups.postprocess_promoted_mut_refs(
                analyses::borrow::mutable_references_no_guarantee(&input),
            );

            // let analysis_results = refactor::Analysis::new(output_params, promoted_mut_refs);
            let analysis_results = rewrite::Analysis::new(output_params, promoted_mut_refs);
            // let refactor_options = options;
            // let _ = refactor::refactor(&input, &analysis_results, rewrite_mode, refactor_options);
            let input = RustProgram {
                tcx: input.tcx,
                functions: input.functions,
                structs: input.structs,
            };
            let res = rewrite::rewrite(
                dir,
                &input,
                rust_program_mapped.hir_to_thir,
                &analysis_results,
            );
            res.apply();
        }
    }
    Ok(())
}

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    if let Command::Preprocess { rewrite_mode } = args.cmd {
        preprocess(&args.path, rewrite_mode)?;
        return Ok(());
    }
    let dir = args.path.parent().unwrap();
    run_compiler_with_mapping(args.path.clone(), |rust_program| {
        run(dir, args.cmd.clone(), rust_program).unwrap()
    });
    Ok(())
}
