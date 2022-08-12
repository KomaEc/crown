use crate::CrateInfo;
use once_cell::sync::OnceCell;
use rustc_errors::registry;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_session::config;
use std::path::PathBuf;
use std::process;
use std::str;

static TRACING_SUB_FMT_SET: OnceCell<()> = OnceCell::new();

pub(crate) fn init_logger() {
    TRACING_SUB_FMT_SET.get_or_init(|| tracing_subscriber::fmt::init());
}

/// A simpler input type than `rustc_session::config::Input`
pub(crate) enum Input {
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

impl Into<rustc_session::config::Input> for Input {
    fn into(self) -> rustc_session::config::Input {
        match self {
            Input::File(path) => rustc_session::config::Input::File(path),
            Input::Str(str) => rustc_session::config::Input::Str {
                name: rustc_span::FileName::Custom("main.rs".to_string()),
                input: str.to_string(),
            },
        }
    }
}

pub(crate) fn run_compiler_with(input: Input, f: impl FnOnce(CrateInfo) + Send) {
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
        diagnostic_output: rustc_session::DiagnosticOutput::Default,
        crate_cfg: rustc_hash::FxHashSet::default(),
        input_path: None,
        output_dir: None,
        output_file: None,
        file_loader: None,
        crate_check_cfg: rustc_interface::interface::parse_check_cfg(vec![]),
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    };

    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
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
                let program = CrateInfo::new(tcx, functions, structs);
                f(program)
            })
        })
    })
}
