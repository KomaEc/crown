use std::{
    path::{Path, PathBuf},
    str,
};

use anyhow::{bail, Context, Result};
use rustc_errors::registry;
use rustc_hir::{def_id::DefId, ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::{mir::Body, ty::TyCtxt};
use rustc_session::{config, EarlyErrorHandler};

pub struct Program<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub fns: Vec<DefId>,
    pub structs: Vec<DefId>,
}

impl<'tcx> Program<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        let mut fns = Vec::new();
        let mut structs = Vec::new();

        for maybe_owner in tcx.hir().krate().owners.iter() {
            let Some(owner) = maybe_owner.as_owner() else {
                continue;
            };
            let OwnerNode::Item(item) = owner.node() else {
                continue;
            };
            match item.kind {
                ItemKind::Fn(..) => fns.push(item.owner_id.def_id.to_def_id()),
                ItemKind::Struct(..) => structs.push(item.owner_id.def_id.to_def_id()),
                _ => {}
            };
        }
        Self { tcx, fns, structs }
    }

    pub fn bodies(&self) -> impl Iterator<Item = &Body<'tcx>> + '_ {
        self.fns.iter().map(|def_id| self.tcx.optimized_mir(def_id))
    }
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

pub fn compiler_config(input: Input, args: Vec<String>) -> Config {
    let sysroot_path = SYSROOT_PATH
        .get_or_try_init(|| rustc_sysroot())
        .unwrap()
        .to_owned();
    let mut early_error_handler = EarlyErrorHandler::new(Default::default());
    let matches = rustc_driver::handle_options(&early_error_handler, &args)
        .context("what?")
        .unwrap();
    let opts = rustc_session::config::build_session_options(&mut early_error_handler, &matches);

    Config {
        opts: config::Options {
            maybe_sysroot: Some(sysroot_path),
            ..opts
        },
        crate_cfg: rustc_hash::FxHashSet::default(),
        crate_check_cfg: rustc_interface::interface::parse_check_cfg(&early_error_handler, vec![]),
        input: input.into(),
        output_dir: None,
        output_file: None,
        file_loader: None,
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        ice_file: None,
        locale_resources: &[],
        expanded_args: vec![],
    }
}

pub const OPT_LEVEL: &str = "opt-level=3";

const SYSROOT_PATH: once_cell::unsync::OnceCell<PathBuf> = once_cell::unsync::OnceCell::new();

pub fn run_compiler_with_config<R: Send>(
    config: Config,
    f: impl for<'tcx> FnOnce(Program<'tcx>) -> R + Send,
) -> R {
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            queries
                .global_ctxt()
                .unwrap()
                .enter(|tcx| f(Program::new(tcx)))
        })
    })
}

pub fn run_compiler<R: Send>(
    input: Input,
    f: impl for<'tcx> FnOnce(Program<'tcx>) -> R + Send,
) -> R {
    let args = ["rustc", "-C", OPT_LEVEL, "--cap-lints", "allow"]
        .map(|s| s.to_owned())
        .to_vec();
    let config = compiler_config(input, args);
    run_compiler_with_config(config, f)
}
