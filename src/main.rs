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

use std::{fs, path::PathBuf, time::Instant};

use analysis::{
    ownership::AnalysisKind, type_qualifier::output_params::show_output_params, CrateCtxt,
};
use anyhow::{bail, Result};
use clap::Parser;
use tracing_subscriber::EnvFilter;
use utils::{
    compiler_interface::{compiler_config, run_compiler_with_config, Program},
    rewrite::RewriteMode,
};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,

    /// Path to lib.rs or main.rs of crate to work on
    path: PathBuf,
}

#[derive(Parser)]
enum Command {
    Preprocess {
        #[clap(value_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
    },
    ExplicitAddr {
        #[clap(value_enum, default_value_t = RewriteMode::Diff)]
        rewrite_mode: RewriteMode,
    },
    Analyse {
        results_path: Option<PathBuf>,
    },
    Ownership,
    Taint,
    Alias,
    OutputParams,
    Mutability,
    Fatness,
    VerifyRustcProperties,
    ShowMir {
        #[clap(long, short)]
        function: Option<String>,
    },
    ShowDefUse {
        #[clap(long, short)]
        function: Option<String>,
    },
}

fn preprocess(path: &PathBuf, rewrite_mode: RewriteMode) -> Result<()> {
    for preprocess in preprocess::PREPROCESSES {
        let config = compiler_config(path.clone().into(), compiler_args());
        run_compiler_with_config(config, |program| preprocess(program.tcx, rewrite_mode));
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
    run_compiler_with_config(
        compiler_config(args.path.into(), compiler_args()),
        |program| run(args.cmd, program),
    )
}

fn compiler_args() -> Vec<String> {
    let project_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let extra_deps_dir = project_dir.join("extra_deps");
    let mut args = [
        "rustc",
        "-C",
        "opt-level=3",
        "--cap-lints",
        "allow",
        "-L",
        extra_deps_dir.to_str().unwrap(),
    ]
    .map(|s| s.to_owned())
    .to_vec();
    for file in fs::read_dir(&extra_deps_dir).expect("build script not working") {
        let absolute_path = file.unwrap().path();
        let lib_name = absolute_path.file_name().unwrap();
        let lib_name = lib_name.to_str().unwrap().split("-").next().unwrap();
        args.push("--extern".to_owned());
        args.push(match lib_name {
            "libcrown_annotation" => {
                format!("crown_annotation={}", absolute_path.to_str().unwrap())
            }
            "liblibc" => format!("libc={}", absolute_path.to_str().unwrap()),
            "libc2rust_bitfields" => {
                format!("c2rust_bitfields={}", absolute_path.to_str().unwrap())
            }
            "libc2rust_bitfields_derive" => format!(
                "c2rust_bitfields_derive={}",
                absolute_path.to_str().unwrap()
            ),
            #[cfg(target_arch = "x86_64")]
            "libf128" => format!("f128={}", absolute_path.to_str().unwrap()),
            #[cfg(target_arch = "x86_64")]
            "libf128_internal" => format!("f128_internal={}", absolute_path.to_str().unwrap()),
            "libnum_traits" => format!("num_traits={}", absolute_path.to_str().unwrap()),
            _ => continue,
        })
    }
    args.push("--cap-lints".to_owned());
    args.push("allow".to_owned());
    args
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

fn run(cmd: Command, input: Program<'_>) -> Result<()> {
    // let input = utils::compiler_interface::Program::new(tcx);

    match cmd {
        Command::Preprocess { .. } => unreachable!(),
        Command::ShowMir { ref function } => {
            if let Some(fn_name) = function {
                let Some(&did) = input
                    .fns
                    .iter()
                    .find(|did| input.tcx.def_path_str(**did).ends_with(fn_name))
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
        Command::ShowDefUse { ref function } => {
            if let Some(fn_name) = function {
                let Some(&did) = input
                    .fns
                    .iter()
                    .find(|did| input.tcx.def_path_str(**did).ends_with(fn_name))
                else {
                    bail!("no such function!")
                };

                let body = input.tcx.optimized_mir(did);
                let def_use_chain = analysis::flow::def_use::DefUseChain::vanilla(body);
                analysis::flow::def_use::display_def_use_chain(body, &def_use_chain);
            } else {
                input.fns.iter().for_each(|&fn_did| {
                    let body = input.tcx.optimized_mir(fn_did);
                    let def_use_chain = analysis::flow::def_use::DefUseChain::vanilla(body);
                    analysis::flow::def_use::display_def_use_chain(body, &def_use_chain);
                });
            }
        }
        Command::VerifyRustcProperties => {
            rustc_properties::verify(&input);
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
        Command::OutputParams => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            show_output_params(&input, &mutability_result)
        }
        Command::Mutability => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            mutability_result.print_results(&input)
        }
        Command::Fatness => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            let output_params = analysis::type_qualifier::output_params::compute_output_params(
                &input,
                &mutability_result,
            );
            let crate_ctxt = CrateCtxt::new(&input);
            let ownership_schemes = time("construct ownership scheme", || {
                analysis::ownership::whole_program::WholeProgramAnalysis::analyze(
                    crate_ctxt,
                    &output_params,
                )
            })?;
            ownership_schemes.trace(input.tcx);

            let ownership_result = ownership_schemes.solidify(&input);
            ownership_result.print_results(&input);

            let fatness_result =
                analysis::type_qualifier::flow_insensitive::fatness::fatness_analysis(
                    &input,
                    &ownership_result,
                );
            fatness_result.print_results(&input)
        }
        Command::Ownership => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            let output_params = analysis::type_qualifier::output_params::compute_output_params(
                &input,
                &mutability_result,
            );
            let analysis_result = analysis::flow::ownership::analyse(&input, &output_params);
            for body in input.bodies() {
                print!("{}: ", input.tcx.def_path_str(body.source.def_id()));
                println!("{}", analysis_result.body_summary_str(body));
            }
            for body in input.bodies() {
                print!("{}: ", input.tcx.def_path_str(body.source.def_id()));
                println!("{}", analysis_result.fn_sig_str(body));
            }
        }
        Command::Analyse { results_path } => {
            let mutability_result =
                analysis::type_qualifier::flow_insensitive::mutability::mutability_analysis(&input);
            let output_params = analysis::type_qualifier::output_params::compute_output_params(
                &input,
                &mutability_result,
            );
            let crate_ctxt = CrateCtxt::new(&input);
            let ownership_schemes =
                analysis::ownership::whole_program::WholeProgramAnalysis::analyze(
                    crate_ctxt,
                    &output_params,
                )?;

            let ownership_result = ownership_schemes.solidify(&input);

            let fatness_result =
                analysis::type_qualifier::flow_insensitive::fatness::fatness_analysis(
                    &input,
                    &ownership_result,
                );

            if let Some(results_path) = results_path {
                let fatness_data = serde_json::to_string(&fatness_result.make_data(&input))?;
                let mutability_data = serde_json::to_string(&mutability_result.make_data(&input))?;
                let ownership_data = serde_json::to_string(&ownership_result.make_data(&input))?;
                fs::write(results_path.join("fatness.json"), fatness_data)?;
                fs::write(results_path.join("mutability.json"), mutability_data)?;
                fs::write(results_path.join("ownership.json"), ownership_data)?;
                let statistics =
                    serde_json::to_string(&analysis::statistics::CrateStatistics::new(
                        &input,
                        &fatness_result,
                        &mutability_result,
                        &ownership_result,
                    ))?;
                fs::write(results_path.join("statistics.json"), statistics)?;
            }
        }
        Command::ExplicitAddr { rewrite_mode } => {
            preprocess::use_explicit_addr(input.tcx, rewrite_mode)
        }
    }
    Ok(())
}
