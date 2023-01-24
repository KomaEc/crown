#![feature(rustc_private)]

use std::{
    borrow::BorrowMut,
    collections::HashSet,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_table::{print_stdout, Cell, Style, Table};
use rustc_errors::registry;
use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::Config;
use rustc_middle::{
    mir::{
        visit::{PlaceContext, Visitor},
        Body, Local,
    },
    ty::TyCtxt,
};
use rustc_session::config;

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

#[derive(Parser)]
struct Cli {
    original: PathBuf,
    new: PathBuf,
}

fn body_stat(body: &Body) -> (usize, usize) {
    let mut unsafe_ptr_cnt = 0;
    let mut user_vars = HashSet::new();
    for (local, local_decl) in body.local_decls.iter_enumerated() {
        if local_decl.is_user_variable() && local_decl.ty.is_unsafe_ptr() {
            unsafe_ptr_cnt += 1;
            user_vars.insert(local);
        }
    }

    struct UnsafeUsage<'me, 'tcx> {
        user_vars: &'me HashSet<Local>,
        num_unsafe_usages: &'me mut usize,
        body: &'me Body<'tcx>,
    }

    impl<'tcx> Visitor<'tcx> for UnsafeUsage<'_, 'tcx> {
        fn visit_place(
            &mut self,
            place: &rustc_middle::mir::Place<'tcx>,
            context: PlaceContext,
            _: rustc_middle::mir::Location,
        ) {
            if matches!(context, PlaceContext::NonUse(..)) {
                return;
            }
            if self.user_vars.contains(&place.local)
                && place.is_indirect()
                && self.body.local_decls[place.local].ty.is_unsafe_ptr()
            {
                *self.num_unsafe_usages += 1;
            }
        }
    }

    let mut num_unsafe_usages = 0;
    UnsafeUsage {
        user_vars: &user_vars,
        num_unsafe_usages: &mut num_unsafe_usages,
        body,
    }
    .visit_body(body);

    (unsafe_ptr_cnt, num_unsafe_usages)
}

struct Statistics {
    num_unsafe_ptrs: usize,
    num_unsafe_usages: usize,
    num_unsafe_ptr_free_fns: usize,
    num_fns: usize,
}

fn get_statistics(tcx: TyCtxt) -> Statistics {
    let mut fns = Vec::new();
    let mut structs = Vec::new();

    for maybe_owner in tcx.hir().krate().owners.iter() {
        let Some(owner) = maybe_owner.as_owner() else { continue };
        let OwnerNode::Item(item) = owner.node() else { continue };
        match item.kind {
            ItemKind::Fn(..) => fns.push(item.owner_id.def_id.to_def_id()),
            ItemKind::Struct(..) => structs.push(item.owner_id.def_id.to_def_id()),
            _ => {}
        };
    }

    let fns = fns;
    // let structs = structs;

    let mut num_unsafe_ptrs = 0;
    let mut num_unsafe_usages = 0;
    let mut num_unsafe_ptr_free_fns = 0;
    let mut num_fns = 0;
    for did in fns {
        let body = tcx.optimized_mir(did);
        let def_path_str = tcx.def_path_str(did);
        if def_path_str.ends_with("borrow")
            || def_path_str.ends_with("borrow_mut")
            || def_path_str.ends_with("owned_as_ref")
            || def_path_str.ends_with("owned_as_mut")
            || def_path_str.ends_with("option_to_raw")
            || def_path_str.ends_with("_ref_eq")
            || def_path_str.ends_with("_ref_ne")
        {
            continue;
        }
        let (fn_num_unsafe_ptrs, fn_num_unsafe_usages) = body_stat(body);
        if fn_num_unsafe_ptrs == 0 {
            num_unsafe_ptr_free_fns += 1;
        }
        num_unsafe_ptrs += fn_num_unsafe_ptrs;
        num_unsafe_usages += fn_num_unsafe_usages;
        num_fns += 1;
    }

    Statistics {
        num_unsafe_ptrs,
        num_unsafe_usages,
        num_unsafe_ptr_free_fns,
        num_fns,
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let original = run_compiler(compiler_config(args.original)?, get_statistics);

    let new = run_compiler(compiler_config(args.new)?, get_statistics);

    let table = vec![
        vec![
            original.num_unsafe_ptrs,
            original.num_unsafe_usages,
            original.num_fns - original.num_unsafe_ptr_free_fns,
        ],
        vec![
            new.num_unsafe_ptrs,
            new.num_unsafe_usages,
            new.num_fns - new.num_unsafe_ptr_free_fns,
        ],
    ]
    .table()
    .title(vec![
        "# Unsafe Ptrs".cell().bold(true),
        "# Unsafe Usages".cell().bold(true),
        "# Fns with Unsafe Ptrs".cell().bold(true),
    ])
    .bold(true);

    print_stdout(table)?;

    Ok(())
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

    let project_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let extra_deps_dir = project_dir.join("extra_deps");

    let args = [
        "rustc",
        "-L",
        extra_deps_dir.to_str().unwrap(),
        "--extern",
        "c2rust_bitfields",
        "c2rust_bitfields_derive",
        "libc",
        #[cfg(target_arch = "x86_64")]
        "f128",
        #[cfg(target_arch = "x86_64")]
        "f128_internal",
        "num_traits",
        "--cap-lints",
        "allow",
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
        lint_caps: rustc_hash::FxHashMap::default(),
        parse_sess_created: None,
        register_lints: None,
        override_queries: None,
        make_codegen_backend: None,
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
    })
}
