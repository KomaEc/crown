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

use analysis::{
    null_analysis::NullAnalysisResults,
};
use clap::Parser;
use rustc_errors::registry;
use rustc_feature::UnstableFeatures;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_index::vec::IndexVec;
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use std::{borrow::BorrowMut, path::PathBuf};

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

        #[clap(long, short)]
        all: bool,
    },
}

fn main() {
    env_logger::init();
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
        stderr: None,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    }
}

fn run(cmd: &Command, tcx: TyCtxt<'_>) {
    let top_level_fns = tcx
        .hir()
        .krate()
        .owners
        .iter()
        .filter_map(|maybe_owner| {
            let owner = maybe_owner.as_owner();
            let OwnerNode::Item(item) = owner.as_ref()?.node() else { return None };
            if !matches!(item.kind, ItemKind::Fn(_, _, _)) {
                return None;
            }
            Some(item.def_id)
        })
        .collect::<Vec<_>>();

    let top_level_struct_defs = tcx
        .hir()
        .krate()
        .owners
        .iter()
        .filter_map(|maybe_owner| {
            let owner = maybe_owner.as_owner();
            let OwnerNode::Item(item) = owner.as_ref()?.node() else { return None };
            if !matches!(item.kind, ItemKind::Struct(_, _)) {
                return None;
            }
            Some(item.def_id)
        })
        .collect::<Vec<_>>();

    match cmd {
        Command::Analyse { null, array, all } => {
            if *null || *all {
                let null_results_raw = top_level_fns
                    .iter()
                    .map(|def_id| (def_id, NullAnalysisResults::collect(tcx, *def_id)))
                    .collect::<Vec<_>>();
                let mut null_results = IndexVec::new();
                for (def_id, result) in null_results_raw.into_iter() {
                    null_results.insert(*def_id, result);
                }
                NullAnalysisResults::resolve_deps(&mut null_results);
                for result in null_results.iter().filter_map(Option::as_ref) {
                    println!("{result}");
                }
            }

            if *array || *all {
                refactor::print_array_analysis_results(tcx, top_level_struct_defs, top_level_fns)
            }
        }
    }
}
