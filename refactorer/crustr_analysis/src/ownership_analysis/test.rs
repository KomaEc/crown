use rustc_errors::registry;
use rustc_hir::OwnerNode;
use rustc_interface::Config;
use rustc_middle::mir::{BasicBlock, Local};
use rustc_session::config;
use rustc_span::FileName;
use std::path::PathBuf;
use std::process;
use std::str;

use crate::ownership_analysis::body_ext::BodyExt;

#[test]
fn test_phi_node_insertion_point() {
    rustc_interface::run_compiler(config_setup(), |compiler| {
        compiler.enter(|queries| {
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let hir_krate = tcx.hir().krate();
                let fn_dids = hir_krate
                    .owners
                    .iter_enumerated()
                    .filter_map(|(did, owner_info)| {
                        if let Some(owner_info) = owner_info {
                            if let OwnerNode::Item(item) = owner_info.node() {
                                if let rustc_hir::ItemKind::Fn(_, _, _) = item.kind {
                                    assert_eq!(item.def_id, did);

                                    return Some(did);
                                }
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>();
                assert_eq!(fn_dids.len(), 1);
                let fn_did = fn_dids[0];
                let body = tcx.optimized_mir(fn_did);

                let mut w = String::new();
                if let Ok(_) =
                    rustc_middle::mir::pretty::write_mir_fn(tcx, body, &mut |_, _| Ok(()), unsafe {
                        &mut w.as_mut_vec()
                    })
                {
                    // assert_eq!(w, "");
                    println!("{}", w);
                } else {
                    panic!("Error in writing mir");
                }

                let dominators = body.dominators();
                for bb in body.basic_blocks().indices() {
                    let idom = dominators.immediate_dominator(bb);
                    println!("{:?} idom {:?}", idom, bb);
                }

                let dominance_frontier = body.dominance_frontier();
                assert_eq!(dominance_frontier[BasicBlock::from_u32(0)].as_slice(), &[]);
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(1)].as_slice(),
                    &[BasicBlock::from_u32(1)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(2)].as_slice(),
                    &[BasicBlock::from_u32(1)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(3)].as_slice(),
                    &[BasicBlock::from_u32(7)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(4)].as_slice(),
                    &[BasicBlock::from_u32(7)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(5)].as_slice(),
                    &[BasicBlock::from_u32(7)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(6)].as_slice(),
                    &[BasicBlock::from_u32(7)]
                );
                assert_eq!(
                    dominance_frontier[BasicBlock::from_u32(7)].as_slice(),
                    &[BasicBlock::from_u32(1)]
                );
                assert!(dominance_frontier[BasicBlock::from_u32(8)].is_empty());

                let insertion_points = body
                    .compute_phi_node::<crate::ownership_analysis::def_use::BorrowckDefUse>(tcx);
                assert_eq!(
                    insertion_points[BasicBlock::from_u32(1)].as_slice(),
                    &[Local::from_u32(0), Local::from_u32(2)]
                );
                assert_eq!(
                    insertion_points[BasicBlock::from_u32(7)].as_slice(),
                    &[Local::from_u32(0), Local::from_u32(2)]
                );
                // bb that is not a join point must not have phi nodes inserted
                assert!(insertion_points[BasicBlock::from_u32(0)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(2)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(3)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(4)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(5)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(6)].is_empty());
                assert!(insertion_points[BasicBlock::from_u32(8)].is_empty());
            })
        })
    })
}

/// This is an example program in the "Tiger book"
/// Compiling under `cargo test`:
/// BasicBlock in mir <-> Number in the book
/// bb0               <-> 1
/// bb1               <-> 2
/// bb2               <-> 3
/// bb3 + bb4         <-> 5
/// bb5 + bb6         <-> 6
/// bb7               <-> 7
/// bb8               <-> 4
///
/// user debug:
/// i => _1
/// j => _0
/// k => _2
const TEST_PROGRAM: &'static str = "
fn f() -> u32 {
    let i = 1;
    let mut j = 1;
    let mut k = 0;
    while k < 100 {
        if j < 20 {
            j = i;
            k += 1;
        } else {
            j = k;
            k += 2;
        }
        assert!(true, \"Introduce a new block, this assertion is optimised away\")
    }
    return j
}";

fn config_setup() -> Config {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    Config {
        opts: config::Options {
            maybe_sysroot: Some(PathBuf::from(sysroot)),
            ..config::Options::default()
        },
        input: config::Input::Str {
            name: FileName::Custom("main.rs".to_string()),
            input: TEST_PROGRAM.to_string(),
        },
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
    }
}
