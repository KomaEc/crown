#![feature(rustc_private)]

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use analysis::{
    ownership::Ownership,
    statistics::CrateStatistics,
    type_qualifier::{
        flow_insensitive::{fatness::Fatness, mutability::Mutability},
        serialize::QualifierData,
    },
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
        Body, Local, VarDebugInfoContents,
    },
    ty::TyCtxt,
};
use rustc_session::config;
use rustc_span::symbol::Symbol;

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
extern crate rustc_span;
extern crate rustc_target;

extern crate either;

#[derive(Parser)]
struct Cli {
    rewritten: PathBuf,
    stat_dir: PathBuf,
    #[clap(long)]
    output_csv: Option<PathBuf>,
}

fn mock_statistics(
    tcx: TyCtxt,
    fatness: QualifierData<Fatness>,
    mutability: QualifierData<Mutability>,
    ownership: QualifierData<Ownership>,
) -> CrateStatistics {
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

    let mut statistics = CrateStatistics::default();

    for &did in &fns {
        let body = tcx.optimized_mir(did);
        let def_path_str = tcx.def_path_str(did);
        let fn_name = def_path_str.rsplit("::").next().unwrap();
        if fn_name == "borrow"
            || fn_name == "borrow_mut"
            || fn_name == "owned_as_ref"
            || fn_name == "owned_as_mut"
            || fn_name == "option_to_raw"
            || fn_name == "_ref_eq"
            || fn_name == "_ref_ne"
            || fn_name == "main"
        {
            continue;
        }
        let fn_name = &def_path_str;

        let user_idents = body
            .var_debug_info
            .iter()
            .filter_map(|debug_info| match debug_info.value {
                VarDebugInfoContents::Place(place) => {
                    let local = place
                        .as_local()
                        .expect("user variable should be mapped to a local");
                    Some((local, debug_info.name))
                }
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        // gather ptr count
        for (local, local_decl) in body.local_decls.iter_enumerated() {
            if local_decl.is_user_variable() && local_decl.ty.is_unsafe_ptr() {
                let var_name = user_idents[&local];
                let is_owning = ownership.fn_data[fn_name][var_name.as_str()][0].is_owning();
                let is_mutable = mutability.fn_data[fn_name][var_name.as_str()][0].is_mutable();
                let is_thin = fatness.fn_data[fn_name][var_name.as_str()][0].is_ptr();
                if is_mutable && is_thin {
                    statistics.num_non_arr_mut_unsafe_ptrs += 1;
                    statistics.num_mut_unsafe_ptrs += 1;
                    statistics.num_non_arr_unsafe_ptrs += 1;
                } else if is_mutable {
                    statistics.num_mut_unsafe_ptrs += 1;
                } else {
                    statistics.num_non_arr_unsafe_ptrs += 1;
                }
                statistics.num_unsafe_ptrs += 1;
                if is_owning {
                    statistics.num_owning_ptrs_detected += 1;
                }
            }
        }

        // gather unsafe usages count
        MockCountUnsafeUsages {
            body,
            tcx,
            fatness: &fatness,
            mutability: &mutability,
            statistics: &mut statistics,
            user_ident: &user_idents,
        }
        .visit_body(body);
    }

    statistics
}

struct MockCountUnsafeUsages<'me, 'tcx> {
    body: &'me Body<'tcx>,
    tcx: TyCtxt<'tcx>,
    fatness: &'me QualifierData<Fatness>,
    mutability: &'me QualifierData<Mutability>,
    statistics: &'me mut CrateStatistics,
    user_ident: &'me HashMap<Local, Symbol>,
}

impl<'me, 'tcx> Visitor<'tcx> for MockCountUnsafeUsages<'me, 'tcx> {
    fn visit_place(
        &mut self,
        place: &rustc_middle::mir::Place<'tcx>,
        context: PlaceContext,
        _location: rustc_middle::mir::Location,
    ) {
        if matches!(context, PlaceContext::NonUse(..)) {
            return;
        }
        if self.body.local_decls[place.local].is_user_variable()
            && place.is_indirect()
            && self.body.local_decls[place.local].ty.is_unsafe_ptr()
        {
            // // verbose output
            // {
            //     let span = match self.body.stmt_at(location) {
            //         either::Either::Left(stmt) => stmt.source_info.span,
            //         either::Either::Right(term) => term.source_info.span,
            //     };

            //     println!("unsafe usage @ {:?}", span);
            // }
            let def_path_str = self.tcx.def_path_str(self.body.source.def_id());
            let fn_name = &def_path_str;
            let local = place.local;
            let var_name = self.user_ident[&local];
            let is_mutable = self.mutability.fn_data[fn_name][var_name.as_str()][0].is_mutable();
            let is_thin = self.fatness.fn_data[fn_name][var_name.as_str()][0].is_ptr();
            if is_mutable && is_thin {
                self.statistics.num_non_arr_mut_unsafe_usages += 1;
                self.statistics.num_mut_unsafe_usages += 1;
                self.statistics.num_non_arr_unsafe_usages += 1;
            } else if is_mutable {
                self.statistics.num_mut_unsafe_usages += 1;
            } else if is_thin {
                self.statistics.num_non_arr_unsafe_usages += 1;
            }
            self.statistics.num_unsafe_usages += 1;
        }
    }
}

fn into_csv_row_data(original: CrateStatistics, new: CrateStatistics) -> [[String; 3]; 4] {
    let original_ptrs_data = [
        original.num_unsafe_ptrs,
        original.num_non_arr_mut_unsafe_ptrs,
        original.num_unsafe_usages,
        original.num_non_arr_mut_unsafe_usages,
    ];

    let new_ptrs_data = [
        new.num_unsafe_ptrs,
        new.num_non_arr_mut_unsafe_ptrs,
        new.num_unsafe_usages,
        new.num_non_arr_mut_unsafe_usages,
    ];

    let comparison = original_ptrs_data
        .into_iter()
        .zip(new_ptrs_data)
        .map(|(original, new)| {
            let fixed = format!("{:.1}%", 100.0 * (original - new) as f64 / original as f64);
            [original.to_string(), new.to_string(), fixed]
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    comparison
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.stat_dir.exists() {
        bail!("expect analysis results")
    }

    let mut fatness: Option<QualifierData<Fatness>> = None;
    let mut mutability: Option<QualifierData<Mutability>> = None;
    let mut ownership: Option<QualifierData<Ownership>> = None;
    let mut statistics: Option<CrateStatistics> = None;
    for file in fs::read_dir(args.stat_dir.as_path()).context("expect dir")? {
        let json_path = file.context("how")?.path();

        if json_path.ends_with("fatness.json") {
            fatness = Some(serde_json::from_str(&fs::read_to_string(json_path)?)?);
        } else if json_path.ends_with("mutability.json") {
            mutability = Some(serde_json::from_str(&fs::read_to_string(json_path)?)?);
        } else if json_path.ends_with("ownership.json") {
            ownership = Some(serde_json::from_str(&fs::read_to_string(json_path)?)?);
        } else if json_path.ends_with("statistics.json") {
            statistics = Some(serde_json::from_str(&fs::read_to_string(json_path)?)?);
        } else {
            bail!("what is it? {}", json_path.to_string_lossy())
        }
    }

    let fatness = fatness.context("expect fatness.json")?;
    let mutability = mutability.context("expect mutability.json")?;
    let ownership = ownership.context("expect ownership.json")?;
    let statistics = statistics.context("expect statistics.json")?;

    let mock_statistics = run_compiler(compiler_config(args.rewritten)?, move |tcx| {
        mock_statistics(tcx, fatness, mutability, ownership)
    });

    let original = statistics;
    let new = mock_statistics;

    if let Some(output_path) = args.output_csv {
        let row = into_csv_row_data(original, new);
        let row_iter = row.into_iter().flatten();
        fs::write(output_path, row_iter.collect::<Vec<_>>().join(","))?;
    } else {
        let original = vec![
            original.num_unsafe_ptrs,
            original.num_non_arr_mut_unsafe_ptrs,
            original.num_unsafe_usages,
            original.num_non_arr_mut_unsafe_usages,
        ];

        let new = vec![
            new.num_unsafe_ptrs,
            new.num_non_arr_mut_unsafe_ptrs,
            new.num_unsafe_usages,
            new.num_non_arr_mut_unsafe_usages,
        ];

        let table = vec![original, new]
            .table()
            .title(vec![
                "# Unsafe Ptrs".cell().bold(true),
                "# Unsafe Mut && Thin Ptrs".cell().bold(true),
                "# Unsafe Usages".cell().bold(true),
                "# Unsafe Mut && Thin Usages".cell().bold(true),
            ])
            .bold(true);

        print_stdout(table)?;
    }

    Ok(())
}

fn run_compiler<R: Send>(
    config: Config,
    run: impl for<'tcx> FnOnce(TyCtxt<'tcx>) -> R + Send,
) -> R {
    rustc_interface::run_compiler(config, move |compiler| {
        compiler.enter(|queries| queries.global_ctxt().unwrap().enter(|tcx| run(tcx)))
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
    let mut args = ["rustc", "-L", extra_deps_dir.to_str().unwrap()]
        .map(|s| s.to_owned())
        .to_vec();
    for file in fs::read_dir(&extra_deps_dir).expect("build script not working") {
        let absolute_path = file.unwrap().path();
        let lib_name = absolute_path.file_name().unwrap();
        let lib_name = lib_name.to_str().unwrap().split("-").next().unwrap();
        args.push("--extern".to_owned());
        args.push(match lib_name {
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
