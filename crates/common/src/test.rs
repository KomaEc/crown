use std::{path::PathBuf, process, str};

use once_cell::sync::OnceCell;
use rustc_errors::registry;
use rustc_hir::{def_id::DefId, ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::ty::TyCtxt;
use rustc_session::config;

static TRACING_SUB_FMT_SET: OnceCell<()> = OnceCell::new();

pub fn init_logger() {
    TRACING_SUB_FMT_SET.get_or_init(tracing_subscriber::fmt::init);
}

/// A simpler input type than `rustc_session::config::Input`
pub enum Input {
    File(PathBuf),
    Str(&'static str),
}

impl From<PathBuf> for Input {
    fn from(path: PathBuf) -> Self {
        Input::File(path)
    }
}

impl From<&'static str> for Input {
    fn from(str: &'static str) -> Self {
        Input::Str(str)
    }
}

impl From<Input> for rustc_session::config::Input {
    fn from(input: Input) -> Self {
        match input {
            Input::File(path) => rustc_session::config::Input::File(path),
            Input::Str(str) => rustc_session::config::Input::Str {
                name: rustc_span::FileName::Custom("main.rs".to_string()),
                input: str.to_string(),
            },
        }
    }
}

pub fn run_compiler_with(
    input: Input,
    f: impl for<'tcx> FnOnce(TyCtxt<'tcx>, Vec<DefId>, Vec<DefId>) + Send,
) {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    let config = Config {
        opts: config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot)),
            ..config::Options::default()
        },
        input: input.into(),
        crate_cfg: rustc_hash::FxHashSet::default(),
        output_dir: None,
        output_file: None,
        file_loader: None,
        crate_check_cfg: rustc_interface::interface::parse_check_cfg(vec![]),
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(rustc_error_codes::DIAGNOSTICS),
    };

    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().enter(|tcx| {
                let mut functions = Vec::new();
                let mut structs = Vec::new();
                for maybe_owner in tcx.hir().krate().owners.iter() {
                    let Some(owner) = maybe_owner.as_owner() else { continue };
                    let OwnerNode::Item(item) = owner.node() else { continue };
                    match item.kind {
                        ItemKind::Fn(..) => functions.push(item.owner_id.def_id.to_def_id()),
                        ItemKind::Struct(..) => structs.push(item.owner_id.def_id.to_def_id()),
                        _ => {}
                    };
                }
                f(tcx, functions, structs)
            })
        })
    })
}
