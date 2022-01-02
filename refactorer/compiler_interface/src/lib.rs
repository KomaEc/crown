#![feature(rustc_private)]

extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
// extern crate rustc_borrowck;

use pointer_analysis::PointerAnalysis;
use rustc_ast_pretty::pprust::item_to_string;
use rustc_errors::registry;
use rustc_hir::OwnerNode;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::ty::WithOptConstParam;
use rustc_session::config;
use std::borrow::Borrow;
// use rustc_span::source_map;
use std::path;
use std::process;
use std::str;

use log;
// use transform::complex_place_reporter::ComplexPlaceReporter;
use transform::place_tracer::PlaceTracer;
use transform::unused_ptr_decl::UnusedPointerDecl;

pub fn run(input_file_name: String) {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    let config = rustc_interface::Config {
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            ..config::Options::default()
        },
        input: config::Input::File(input_file_name.into()),
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
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // TODO: add this to -Z unpretty
            let ast_krate = queries.parse().unwrap().take();
            for item in ast_krate.items {
                println!("{}", item_to_string(&item));
            }

            // Analyze the crate and inspect the types under the cursor.
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                // Every compilation contains a single crate.
                let hir_krate = tcx.hir().krate();

                log::info!("... logging mir bodies for top-level functions");

                // Collect promoted mir bodies of all top-level functions
                let top_level_function_def_ids = hir_krate
                    .owners
                    .iter()
                    .filter_map(|owner_info| {
                        let owner_info = owner_info.as_ref()?;
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                                let def_id = item.def_id;
                                // calculate mir bodies in advance
                                let (body, promoted_bodies) =
                                    tcx.mir_promoted(WithOptConstParam::unknown(def_id));
                                assert!(
                                    promoted_bodies.borrow().borrow().is_empty(),
                                    "Promoted bodies are not handled"
                                );
                                // let body = body.steal();

                                let mut w = String::new();
                                if let Ok(_) = rustc_middle::mir::pretty::write_mir_fn(
                                    tcx,
                                    body.borrow().borrow(),
                                    &mut |_, _| Ok(()),
                                    unsafe { &mut w.as_mut_vec() },
                                ) {
                                    log::info!("... logging body:\n{}", &w);
                                } else {
                                    panic!("Error in writing mir");
                                }
                                return Some(def_id);
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>();

                log::info!("Start tracing places ...");
                for &local_def_id in top_level_function_def_ids.iter() {
                    let (body, _) = tcx.mir_promoted(WithOptConstParam::unknown(local_def_id));
                    let body_ref = body.borrow();

                    log::info!("... tracing places for {:?}", local_def_id);
                    let mut tracer = PlaceTracer::new(&top_level_function_def_ids, tcx);
                    tracer.visit_body(body_ref.borrow());
                }
                log::info!("Done\n");

                log::info!("Collecting body refs ...");
                let body_refs = top_level_function_def_ids
                    .into_iter()
                    .map(|did| {
                        let (body, _) = tcx.mir_promoted(WithOptConstParam::unknown(did));
                        body.borrow()
                    })
                    .collect::<Vec<_>>();
                // .iter().map(|body: &std::cell::Ref<_>| &**body).collect::<Vec<_>>();

                let mut bodies = body_refs
                    .iter()
                    .map(|body: &std::cell::Ref<_>| &**body)
                    .collect::<Vec<_>>();
                bodies.sort_by_key(|body| body.source.instance.def_id());

                log::info!("Start pointer analysis ...");
                let andersen_result = PointerAnalysis::new_analysis(bodies.as_slice(), tcx)
                    .into_constraint_generation()
                    .generate_constraints()
                    .proceed_to_solving_by_andersen()
                    .solve()
                    .finish();
                andersen_result.dump_pts_sets_to_log();
                // andersen_result.report_ptr_alias();

                log::info!("Start unused pointer decl analysis ...");
                UnusedPointerDecl::new(&bodies, tcx, andersen_result).analyze();

                log::info!("Done\n");
            })
        });
    });
}
