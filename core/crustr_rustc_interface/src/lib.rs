#![feature(rustc_private)]

use rustc_errors::registry;
use rustc_feature::UnstableFeatures;
use rustc_interface::{interface::Compiler, Config};
use rustc_session::config;
use std::path::PathBuf;
use std::process;
use std::str;

extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_feature;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
// extern crate rustc_borrowck;

pub mod rewrite_struct;
pub mod toy_run;

pub fn config_setup(input_path: PathBuf) -> Config {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    Config {
        opts: config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot)),
            unstable_features: UnstableFeatures::from_environment(None),
            ..config::Options::default()
        },
        input: config::Input::File(input_path),
        /*
        input: config::Input::Str {
            name: source_map::FileName::Custom("main.rs".to_string()),
            input: "fn f<'a>() -> &'a str { let local = String::from(\"local\"); &local }"
                .to_string(),
        },
        */
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
pub trait CompilerRunnable {
    type Output: Send;
    fn run(compiler: &Compiler) -> Self::Output;
}

#[inline]
pub fn run_compiler_with_config<Runnable>(config: Config) -> <Runnable as CompilerRunnable>::Output
where
    Runnable: CompilerRunnable,
{
    rustc_interface::run_compiler(config, Runnable::run)
}
