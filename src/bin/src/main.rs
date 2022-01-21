#![feature(rustc_private)]

extern crate rustc_interface;

use crustr_rustc_interface::{config_setup, run_compiler_with_config};
use std::env;

fn main() {
    env_logger::init();

    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Expect appropriate arguments");
    }
    let input_file_name: &str = &args[1];
    // crustr_rustc_interface::toy_run::run(input_file_name.to_string());

    let config = config_setup(input_file_name.into());
    run_compiler_with_config::<crustr_rustc_interface::collect_structs::CollectStructInfo>(config)
}
