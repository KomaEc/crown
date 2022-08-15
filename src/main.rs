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

use clap::Parser;
use rustc_errors::registry;
use rustc_feature::UnstableFeatures;
use rustc_hir::{def_id::DefId, ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use std::{borrow::BorrowMut, path::PathBuf, time::Instant};
use tracing_subscriber::EnvFilter;

use empirical_study::EmpiricalStudy;
use orc_ownership_analysis::CrateInfo;
use orc_refactor::rewriter::RewriteMode;
use usage_analysis::{fatness, null};

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

        #[clap(long, short = 'T')]
        taint: bool,

        #[clap(long, short = 'M')]
        mutability: bool,

        #[clap(long, short)]
        all: bool,
    },
    Rewrite {
        #[clap(arg_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
    },
    Playground {
        #[clap(long)]
        mir: bool,
        #[clap(long)]
        addr: bool,
    },
    EmpiricalStudy,
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
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: rustc_interface::interface::parse_check_cfg(vec![]),
        input: config::Input::File(input_path),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
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

    let local_fns = functions
        .iter()
        .copied()
        .filter_map(DefId::as_local)
        .collect::<Vec<_>>();
    let local_structs = structs
        .iter()
        .copied()
        .filter_map(DefId::as_local)
        .collect::<Vec<_>>();

    let program = time("construct call graph and struct topology", || {
        CrateInfo::new(tcx, functions, structs)
    });

    match *cmd {
        Command::EmpiricalStudy => {
            program.perform_empirical_study();
        }
        Command::Playground { mir, addr } => {
            if mir {
                program.print_mir();
            }
            if addr {
                program.compute_percentage_of_non_address_taking_functions();
            }
            program.verify_shape_of_place();
            program.assert_assign_simple();
        }
        Command::Analyse {
            null,
            array,
            ownership,
            taint,
            mutability,
            all: _,
        } => {
            if null {
                let results = null::CrateResults::collect(tcx, &local_fns, &local_structs);
                results.show(tcx);
            }

            if ownership {
                todo!("ownership analysis");
            }

            if taint {
                orc_taint_analysis::run_steensgaard(&program)
            }

            if mutability {
                todo!("ownership analysis");
            }

            if array {
                let results = fatness::CrateResults::collect(tcx, &local_fns, &local_structs);
                results.show(tcx);
            }
        }
        Command::Rewrite { rewrite_mode: _ } => {
            todo!("ownership analysis");
        }
    }
}
