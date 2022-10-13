#![feature(let_else)]
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

use std::{
    borrow::BorrowMut,
    path::{Path, PathBuf},
    time::Instant,
};

use analysis::{ownership::AnalysisKind, CrateCtxt};
use anyhow::{bail, Context, Result};
use clap::Parser;
use common::rewrite::RewriteMode;
use empirical_study::EmpiricalStudy;
use rustc_errors::registry;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;
use tracing_subscriber::EnvFilter;

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
    Analyse,
    Taint,
    Alias,
    NoAliasParams,
    Mutability,
    Fatness,
    Refactor,
    VerifyRustcProperties,
    /// Perform empirical studies and show results.
    EmpiricalStudy,
    /// Pretty print Mir despite compilation error
    ShowMir {
        #[clap(long, short)]
        function: Option<String>,
    },
}

fn run_compiler<R: Send>(
    config: Config,
    run: impl for<'tcx> FnOnce(TyCtxt<'tcx>) -> R + Send,
) -> R {
    rustc_interface::run_compiler(config, move |compiler| {
        compiler.enter(|queries| {
            queries
                .global_ctxt()
                .borrow_mut()
                .unwrap()
                .peek_mut()
                .enter(|tcx| run(tcx))
        })
    })
}

fn preprocess(path: &PathBuf, rewrite_mode: RewriteMode) -> Result<()> {
    for preprocess in preprocess::PREPROCESSES {
        let config = compiler_config(path.clone())?;
        run_compiler(config, |tcx| preprocess(tcx, rewrite_mode));
        if !matches!(rewrite_mode, RewriteMode::InPlace) {
            println!("{rewrite_mode} mode does not support multi-phase rewrite");
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Cli::parse();

    if let Command::Preprocess { rewrite_mode } = args.cmd {
        preprocess(&args.path, rewrite_mode)?;
        return Ok(());
    }
    if let Command::Refactor = args.cmd {
        preprocess(&args.path, RewriteMode::InPlace)?;
    }
    run_compiler(compiler_config(args.path)?, |tcx| run(&args.cmd, tcx))
}

/// Returns where the given rustc stores its sysroot source code.
fn rustc_sysroot() -> Result<PathBuf> {
    let mut rustc = std::process::Command::new("rustc");
    let output = rustc
        .args(["--print", "sysroot"])
        .output()
        .context("failed to determine sysroot")?;
    if !output.status.success() {
        bail!(
            "failed to determine sysroot; rustc said:\n{}",
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }

    let sysroot =
        std::str::from_utf8(&output.stdout).context("sysroot folder is not valid UTF-8")?;
    let sysroot = Path::new(sysroot.trim_end_matches('\n'));
    Ok(sysroot.to_path_buf())
}

const SYSROOT_PATH: once_cell::unsync::OnceCell<PathBuf> = once_cell::unsync::OnceCell::new();

fn compiler_config(input_path: PathBuf) -> Result<Config> {
    let sysroot_path = SYSROOT_PATH.get_or_try_init(|| rustc_sysroot())?.to_owned();

    let args = [
        "rustc",
        "-L",
        "dependency=./deps",
        "--extern",
        "c2rust_bitfields=./deps/libc2rust_bitfields-9912e7b3bbb08750.rlib",
        "--extern",
        "libc=./deps/liblibc-224625194917b41f.rlib",
    ]
    .map(|s| s.to_owned());

    let matches = rustc_driver::handle_options(&args).context("what?")?;
    let opts = rustc_session::config::build_session_options(&matches);

    Ok(Config {
        opts: config::Options {
            maybe_sysroot: Some(sysroot_path),
            ..opts
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
    })
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

    let input = common::CrateData { tcx, fns, structs };

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
        Command::Taint => {
            let alias = alias::taint_results(&input);
            alias.print_results();
        }
        Command::Alias => {
            let alias = alias::alias_results(&input);
            alias.print_results();
        }
        Command::NoAliasParams => {
            let alias_result = alias::alias_results(&input);
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            analysis::type_qualifier::noalias::show_noalias_params(
                &input,
                &alias_result,
                &mutability_result,
            );
        }
        Command::Mutability => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            mutability_result.print_fn_sigs(tcx, &input.fns)
        }
        Command::Fatness => {
            let fatness_result =
                analysis::type_qualifier::flow_insensitive::fatness::fatness_analysis(&input);
            fatness_result.print_fn_sigs(tcx, &input.fns)
        }
        Command::Analyse => {
            let mut crate_ctxt = CrateCtxt::from(input);
            let ownership_schemes = time("construct ownership scheme", || {
                analysis::ownership::WholeProgram::analyze(&mut crate_ctxt)
            })?;
            ownership_schemes.trace(tcx);
        }
        Command::Refactor => {
            let alias_result = alias::alias_results(&input);
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            let fatness_result =
                analysis::type_qualifier::flow_insensitive::fatness::fatness_analysis(&input);
            let mut crate_ctxt = CrateCtxt::from(input);
            let ownership_schemes = analysis::ownership::WholeProgram::analyze(&mut crate_ctxt)?;

            let _ = (
                alias_result,
                mutability_result,
                fatness_result,
                ownership_schemes,
            );
            todo!();
        }
    }
    Ok(())
}
