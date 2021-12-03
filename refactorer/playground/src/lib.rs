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

use alias_analysis::andersen::{AndersenAnalysis, AndersenResult};
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

                log::trace!("Iterating over each crate");

                // Collect promoted mir bodies of all top-level functions
                let _body_def_ids = hir_krate
                    .owners
                    .iter()
                    .filter_map(|owner_info| {
                        let owner_info = owner_info.as_ref()?;
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                                log::trace!("For top-level function:");
                                let def_id = item.def_id;
                                let (body, _promoted_bodies) =
                                    tcx.mir_promoted(WithOptConstParam::unknown(def_id));
                                // let body = body.steal();

                                // let mut w = String::new();
                                let mut w = std::io::stdout();
                                if let Ok(_) = rustc_middle::mir::pretty::write_mir_fn(
                                    tcx,
                                    body.borrow().borrow(),
                                    &mut |_, _| Ok(()),
                                    &mut w,
                                ) {
                                    log::trace!("Done!");
                                } else {
                                    log::error!("Error in writing mir");
                                }

                                let body_ref = body.borrow();
                                let mut tracer = PlaceTracer;
                                tracer.visit_body(body_ref.borrow());

                                //let mut reporter =
                                //    ComplexPlaceReporter::new(tcx, body_ref.borrow());
                                //reporter.visit_body(body_ref.borrow());

                                AndersenAnalysis::new(body_ref.borrow(), tcx)
                                    .run()
                                    .log_debug();

                                // should not panic!
                                // let _ = tcx.optimized_mir(def_id);
                                return Some(def_id);
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>();

                /*
                // Iterate over the top-level items in the crate, looking for the main function.
                for owner_info in hir_krate.owners.iter() {

                    // Assume that functions are all on top-level
                    if let Some(owner_info) = owner_info {

                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let rustc_hir::ItemKind::Fn(_, _, body_id) = item.kind {

                                log::trace!("For top-level function:");
                                let expr = &tcx.hir().body(body_id).value;
                                if let rustc_hir::ExprKind::Block(block, _) = expr.kind {
                                    if let rustc_hir::StmtKind::Local(local) = block.stmts[0].kind {

                                        if let Some(expr) = local.init {
                                            let hir_id = expr.hir_id; // hir_id identifies the string "local"
                                            let def_id = tcx.hir().local_def_id(item.hir_id());
                                            let ty = tcx.typeck(def_id).node_type(hir_id);
                                            log::info!("{:?}: {:?}", local.span, ty);
                                        }
                                    }
                                }
                                let def_id = item.def_id;
                                let (body, _promoted_bodies) = tcx.mir_promoted(WithOptConstParam::unknown(def_id));
                                let body = body.steal();


                                let mut w = String::new();
                                if let Ok(_) = rustc_middle::mir::pretty::write_mir_fn(tcx, &body, &mut |_, _| Ok(()), unsafe { w.as_mut_vec() }) {
                                    log::info!("{}\nDone!", w);
                                } else {
                                    log::error!("Error in writing mir");
                                }
                            }
                        }
                    }

                }

                */
            })
        });
    });
}
