use pointer_analysis::PointerAnalysis;
use rustc_ast_pretty::pprust::item_to_string;
use rustc_hir::OwnerNode;
use rustc_interface::interface::Compiler;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::ty::WithOptConstParam;
use std::borrow::Borrow;
use log;
// use transform::complex_place_reporter::ComplexPlaceReporter;
use crustr_analysis::place_tracer::PlaceTracer;
use crustr_analysis::unused_ptr_decl::UnusedPointerDecl;

pub fn run(compiler: &Compiler) {
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
    })
}