#![feature(let_else)]
#![feature(rustc_private)]

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

// use analysis::null_analysis::NullAnalysisResults;
use clap::Parser;
use core::Program;
use rustc_errors::registry;
use rustc_feature::UnstableFeatures;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use std::{borrow::BorrowMut, path::PathBuf, time::Instant};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,

    /// Path to lib.rs or main.rs of crate to work on
    #[clap(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Parser)]
enum Command {
    Analyse {
        #[clap(long, short)]
        null: bool,

        #[clap(long, short = 'A')]
        array: bool,

        #[clap(long, short = 'O')]
        ownership: bool,

        #[clap(long, short = 'M')]
        mutability: bool,

        #[clap(long, short = 'p')]
        pretty_mir: bool,

        #[clap(long, short)]
        all: bool,
    },
    Rewrite {
        #[clap(arg_enum, default_value_t = core::rewriter::RewriteMode::Print)]
        rewrite_mode: core::rewriter::RewriteMode,
    },
    Playground {
        #[clap(long)]
        mir: bool,
    },
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    rustc_interface::run_compiler(compiler_config(args.path), move |compiler| {
        compiler.enter(|queries| {
            queries
                .global_ctxt()
                .borrow_mut()
                .unwrap()
                .peek_mut()
                .enter(|tcx| {
                    run(&args.cmd, tcx);
                })
        });
    });
}

fn compiler_config(input_path: PathBuf) -> Config {
    let out = std::process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = std::str::from_utf8(&out.stdout).unwrap().trim();
    Config {
        opts: config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot)),
            unstable_features: UnstableFeatures::from_environment(None),
            ..config::Options::default()
        },
        input: config::Input::File(input_path),
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
        crate_cfg: rustc_hash::FxHashSet::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        crate_check_cfg: rustc_interface::interface::parse_check_cfg(vec![]),
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    }
}

fn time<T>(label: &str, f: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let ret = f();
    tracing::info!(
        task = label,
        "task finished in {}ms",
        (Instant::now() - start).as_millis()
    );
    ret
}

fn run(cmd: &Command, tcx: TyCtxt<'_>) {
    let mut functions = Vec::new();
    let mut structs = Vec::new();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        match item.kind {
            ItemKind::Fn(..) => functions.push(item.def_id.to_def_id()),
            ItemKind::Struct(..) => structs.push(item.def_id.to_def_id()),
            _ => {}
        };
    }

    let program = Program::new(tcx, functions, structs);

    match *cmd {
        Command::Playground { mir } => {
            if mir {
                program.print_mir();
            }
            program.inspect_nested_places();
        }
        Command::Analyse { .. } => {
            // if *pretty_mir {
            //     refactor::show_mir(tcx, top_level_fns.clone())
            // }

            // if *ownership {
            //     refactor::show_ownership_analysis_results(
            //         tcx,
            //         top_level_struct_defs.clone(),
            //         top_level_fns.clone(),
            //     )
            // }

            // if *mutability {
            //     refactor::show_mutability_analysis_results(
            //         tcx,
            //         top_level_struct_defs.clone(),
            //         top_level_fns.clone(),
            //     )
            // }

            // if *array {
            //     refactor::print_fat_thin_analysis_results(tcx, top_level_struct_defs, top_level_fns)
            // }
        }
        Command::Rewrite { .. } => {
            // let ownership_analysis = time("ownership analysis", || {
            //     refactor::ownership_analysis(tcx, &top_level_struct_defs, &top_level_fns)
            // });
            // let mutability_analysis = time("mutability analysis", || {
            //     refactor::mutability_analysis(tcx, &top_level_struct_defs, &top_level_fns)
            // });
            // let fatness_analysis = time("fatness analysis", || {
            //     refactor::fatness_analysis(tcx, &top_level_struct_defs, &top_level_fns)
            // });
            // time("rewrite", || {
            //     refactor::rewrite::rewrite(
            //         tcx,
            //         &ownership_analysis,
            //         &mutability_analysis,
            //         &fatness_analysis,
            //         &top_level_struct_defs,
            //         rewrite_mode,
            //     )
            // });
        }
    }
}
