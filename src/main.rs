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

use clap::Parser;
use std::path::PathBuf;
use utils::{rewrite::RewriteMode, rustc::run_compiler};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    cmd: Command,

    /// Path to lib.rs or main.rs of crate to work on
    path: PathBuf,
}

#[derive(Parser, Debug)]
enum Command {
    Preprocess {
        #[clap(value_enum, default_value_t = RewriteMode::Print)]
        rewrite_mode: RewriteMode,
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

fn main() -> Result<(), ()> {
    let args = Cli::parse();

    let Command::Preprocess { rewrite_mode } = args.cmd;
    preprocess(&args.path, rewrite_mode)?;
    return Ok(());
}
