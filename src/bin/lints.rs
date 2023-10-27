#![feature(rustc_private)]

use std::{fs, path::PathBuf};

extern crate rustc_interface;
extern crate rustc_lint;

use analysis::lints::RawPointerPermission;
use rustc_lint::LintStore;
use rustc_tools::with_lints;

fn compiler_args() -> Vec<String> {
    let project_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let extra_deps_dir = project_dir.join("extra_deps");
    let mut args = [
        "rustc",
        "-C",
        "opt-level=3",
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
            "libnum_traits" => format!("num_traits={}", absolute_path.to_str().unwrap()),
            _ => {
                args.pop();
                continue;
            }
        })
    }
    args
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut compiler_args = compiler_args();
    compiler_args.extend(args);

    compiler_args.extend(
        "--crate-type lib -C link-arg=dynamic_lookup"
            .split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<_>>(),
    );

    with_lints(&compiler_args, vec![], |lint_store: &mut LintStore| {
        lint_store.register_late_pass(|tcx| Box::new(RawPointerPermission::new(tcx)));
    })
    .map(|_| ())
    .map_err(|_| ())
}
