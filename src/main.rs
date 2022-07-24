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
extern crate rustc_target;

use analysis::{fatness_analysis, mutability_analysis, null_analysis};
use clap::Parser;
use rustc_errors::registry;
use rustc_feature::UnstableFeatures;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use std::{borrow::BorrowMut, path::PathBuf, time::Instant};
use tracing_subscriber::EnvFilter;

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
        #[clap(arg_enum, default_value_t = rewriter::RewriteMode::Print)]
        rewrite_mode: rewriter::RewriteMode,
    },
}

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
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
        Command::Analyse {
            null,
            array,
            ownership,
            mutability,
            pretty_mir,
            all: _,
        } => {
            if *null {
                let results = null_analysis::CrateResults::collect(
                    tcx,
                    &top_level_fns,
                    &top_level_struct_defs,
                );
                results.show(tcx);
            }

            if *pretty_mir {
                refactor::show_mir(tcx, top_level_fns.clone())
            }

            if *ownership {
                refactor::show_ownership_analysis_results(
                    tcx,
                    top_level_struct_defs.clone(),
                    top_level_fns.clone(),
                )
            }

            if *mutability {
                let results = mutability_analysis::CrateResults::collect(
                    tcx,
                    &top_level_fns,
                    &top_level_struct_defs,
                );
                results.show(tcx);
            }

            if *array {
                let results = fatness_analysis::CrateResults::collect(
                    tcx,
                    &top_level_fns,
                    &top_level_struct_defs,
                );
                results.show(tcx);
            }
        }
        &Command::Rewrite { rewrite_mode } => {
            let ownership_analysis = time("ownership analysis", || {
                refactor::ownership_analysis(tcx, &top_level_struct_defs, &top_level_fns)
            });
            let mutability_analysis = time("mutability analysis", || {
                mutability_analysis::CrateResults::collect(
                    tcx,
                    &top_level_fns,
                    &top_level_struct_defs,
                )
            });
            let fatness_analysis = time("fatness analysis", || {
                fatness_analysis::CrateResults::collect(tcx, &top_level_fns, &top_level_struct_defs)
            });
            let null_analysis = time("null analysis", || {
                null_analysis::CrateResults::collect(tcx, &top_level_fns, &top_level_struct_defs)
            });
            time("rewrite", || {
                refactor::rewrite::rewrite(
                    tcx,
                    &ownership_analysis,
                    &mutability_analysis,
                    &fatness_analysis,
                    &null_analysis,
                    &top_level_struct_defs,
                    rewrite_mode,
                )
            });
        }
    }
}
