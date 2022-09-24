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

use anyhow::{bail, Result};
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
use orc_common::rewrite::RewriteMode;
use orc_ownership_analysis::CrateCtxt;
use usage_analysis::{fatness, mutability, null};

// // Set up Jemalloc
// use jemallocator::Jemalloc;

// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

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
    Preprocess {
        #[clap(arg_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
    },
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
    CrashMe,
    Rewrite {
        #[clap(arg_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
    },
    VerifyRustcProperties,
    /// Perform empirical studies and show results.
    EmpiricalStudy,
    /// Pretty print Mir despite compilation error
    ShowMir {
        #[clap(long, short)]
        function: Option<String>,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Cli::parse();
    if let Command::Preprocess { rewrite_mode } = args.cmd {
        for preprocess in orc_preprocess::PREPROCESSES {
            let config = compiler_config(args.path.clone());
            rustc_interface::run_compiler(config, move |compiler| {
                compiler.enter(|queries| {
                    queries
                        .global_ctxt()
                        .borrow_mut()
                        .unwrap()
                        .peek_mut()
                        .enter(|tcx| preprocess(tcx, rewrite_mode))
                })
            });
            if !matches!(rewrite_mode, RewriteMode::InPlace) {
                println!("{rewrite_mode} mode only supports one round of rewrite");
                break;
            }
        }
        return Ok(());
    }
    rustc_interface::run_compiler(compiler_config(args.path), move |compiler| {
        compiler.enter(|queries| {
            queries
                .global_ctxt()
                .borrow_mut()
                .unwrap()
                .peek_mut()
                .enter(|tcx| run(&args.cmd, tcx))
        })
    })
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

fn run(cmd: &Command, tcx: TyCtxt<'_>) -> Result<()> {
    let mut fns = Vec::new();
    let mut structs = Vec::new();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        match item.kind {
            ItemKind::Fn(..) => fns.push(item.def_id.to_def_id()),
            ItemKind::Struct(..) => structs.push(item.def_id.to_def_id()),
            _ => {}
        };
    }

    let local_fns = fns
        .iter()
        .copied()
        .filter_map(DefId::as_local)
        .collect::<Vec<_>>();
    let local_structs = structs
        .iter()
        .copied()
        .filter_map(DefId::as_local)
        .collect::<Vec<_>>();

    // let input = (tcx, functions, structs);
    let input = orc_common::CrateData {
        tcx,
        fns,
        structs,
    };

    match *cmd {
        Command::Preprocess { .. } => unreachable!(),
        Command::ShowMir { ref function } => {
            if let Some(def_path_str) = function {
                let Some(&did) = input
                    .fns
                    .iter()
                    .find(|did| input.tcx.def_path_str(**did) == *def_path_str)
                    else {
                        bail!("no such function!")
                    };

                let body = input.tcx.optimized_mir(did);
                rustc_middle::mir::pretty::write_mir_fn(
                    input.tcx,
                    body,
                    &mut |_, _| Ok(()),
                    &mut std::io::stdout(),
                )
                .unwrap();
            } else {
                input.fns.iter().for_each(|&fn_did| {
                    let body = input.tcx.optimized_mir(fn_did);
                    rustc_middle::mir::pretty::write_mir_fn(
                        input.tcx,
                        body,
                        &mut |_, _| Ok(()),
                        &mut std::io::stdout(),
                    )
                    .unwrap();
                });
            }
        }
        Command::EmpiricalStudy => {
            time("empirical study", || input.perform_empirical_study());
        }
        Command::VerifyRustcProperties => {
            let program = time("construct call graph and struct topology", || {
                CrateCtxt::from(input)
            });
            program.verify();
            println!("verification success");
        }
        Command::CrashMe => {
            let program = CrateCtxt::from(input);
            time("crash me with pure rename", || program.pure_rename());
            time("crash me with inference and solve", || {
                program.standalone_solve()
            })?;
            time("crash me with whole program analysis", || {
                program.whole_program_analysis()
            })?;
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
                let results = time("null analysis", || {
                    null::CrateResults::collect(tcx, &local_fns, &local_structs)
                });
                results.show(tcx);
            }

            if ownership {
                let Some(ownership) = time("ownership analysis", || {
                    ownership_analysis::ownership_analysis::InterSummary::collect(tcx, &local_structs, &local_fns)
                }) else { bail!("ownership analysis failed") };
                ownership.show_result();
            }

            if taint {
                time("taint analysis", || {
                    orc_taint_analysis::report_results(&input)
                });
            }

            if mutability {
                let Some(ownership) = time("ownership analysis", || {
                    ownership_analysis::ownership_analysis::InterSummary::collect(tcx, &local_structs, &local_fns)
                }) else { bail!("ownership analysis failed") };
                let mutability = time("mutability analysis", || {
                    mutability::CrateResults::collect(tcx, &local_fns, &local_structs, &ownership)
                });
                mutability.show(tcx);
            }

            if array {
                let results = fatness::CrateResults::collect(tcx, &local_fns, &local_structs);
                results.show(tcx);
            }
        }
        Command::Rewrite { rewrite_mode } => {
            let Some(ownership) = time("ownership analysis", || {
                ownership_analysis::ownership_analysis::InterSummary::collect(tcx, &local_structs, &local_fns)
            }) else { bail!("ownership analysis failed") };
            let mutability = time("mutability analysis", || {
                mutability::CrateResults::collect(tcx, &local_fns, &local_structs, &ownership)
            });
            let fatness = time("fatness analysis", || {
                fatness::CrateResults::collect(tcx, &local_fns, &local_structs)
            });
            let null = time("null analysis", || {
                null::CrateResults::collect(tcx, &local_fns, &local_structs)
            });
            time("rewrite", || {
                refactor::rewrite::rewrite(
                    tcx,
                    &ownership,
                    &mutability,
                    &fatness,
                    &null,
                    &local_fns,
                    &local_structs,
                    rewrite_mode,
                )
            })
        }
    }
    Ok(())
}
