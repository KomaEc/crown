use rustc_middle::mir::{BasicBlock, Body, Local, Location};
use rustc_middle::ty::TyCtxt;
use std::str;

use crate::def_use::BorrowckDefUse;
use crate::ssa::body_ext::BodyExt;
use crate::ssa::rename::handler::{LocalSimplePtrCVMap, SSANameMap};
use crate::ssa::rename::{handler::PrintStdSSAName, implementations::PlainRenamer, SSANameHandler};

#[test]
fn test_phi_node_insertion_point() {
    compiler_interface::run_compiler_with_single_func(TEST_PROGRAMS[0].into(), |tcx, fn_did| {
        let body = tcx.optimized_mir(fn_did);
        rustc_middle::mir::pretty::write_mir_fn(
            tcx,
            body,
            &mut |_, _| Ok(()),
            &mut std::io::stdout(),
        )
        .unwrap();

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

        let insertion_points = body.compute_phi_node::<BorrowckDefUse>(tcx);
        assert_eq!(
            insertion_points[BasicBlock::from_u32(1)]
                .locals()
                .collect::<Vec<_>>()
                .as_slice(),
            &[Local::from_u32(0), Local::from_u32(2)]
        );
        assert_eq!(
            insertion_points[BasicBlock::from_u32(7)]
                .locals()
                .collect::<Vec<_>>()
                .as_slice(),
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
    });
}

#[test]
fn test_all() {
    for (&program, &spec) in std::iter::zip(TEST_PROGRAMS.iter(), TEST_PROGRAMS_SPECS.iter()) {
        compiler_interface::run_compiler_with_single_func(program.into(), |tcx, fn_did| {
            let body = tcx.optimized_mir(fn_did);
            rustc_middle::mir::pretty::write_mir_fn(
                tcx,
                body,
                &mut |_, _| Ok(()),
                &mut std::io::stdout(),
            )
            .unwrap();

            spec(tcx, body);
        })
    }
}

const TEST_PROGRAMS: &'static [&'static str] = &[
    // This is an example program in the "Tiger book"
    // Compiling under `cargo test`:
    // BasicBlock in mir <-> Number in the book
    // bb0               <-> 1
    // bb1               <-> 2
    // bb2               <-> 3
    // bb3 + bb4         <-> 5
    // bb5 + bb6         <-> 6
    // bb7               <-> 7
    // bb8               <-> 4
    //
    // user debug:
    // i => _1
    // j => _0
    // k => _2
    "
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
    }",
    "
    fn f() -> u32 {
        let mut local = 1;
        let r = &mut local;
        let mut x = 0;
        while x < 100 {
            if x < 20 {
                *r += 1;
                x += 2;
            } else {
                *r += 2;
                x += 1;
            }
        }
        return *r
    }",
];

const TEST_PROGRAMS_SPECS: &'static [for<'a, 'tcx> fn(TyCtxt<'tcx>, &'a Body<'tcx>)] =
    &[spec0, spec1];

fn spec0<'a, 'tcx>(tcx: TyCtxt<'tcx>, body: &'a Body<'tcx>) {
    let insertion_points = body.compute_phi_node::<BorrowckDefUse>(tcx);

    struct TestProgramSpec;
    impl SSANameHandler for TestProgramSpec {
        type Output = ();
        fn handle_def(&mut self, local: Local, idx: usize, location: Location) {
            if local == Local::from_usize(1) {
                // regular definitions for i, which occur only at entry block
                assert_eq!(location.block, BasicBlock::from_usize(0));
                assert_eq!(idx, 1);
            } else if local == Local::from_usize(0) {
                // regular definitions for j
                if location.block == BasicBlock::from_usize(0) {
                    assert_eq!(idx, 1)
                } else if location.block == BasicBlock::from_usize(3) {
                    assert_eq!(idx, 3)
                } else if location.block == BasicBlock::from_usize(5) {
                    assert_eq!(idx, 4)
                } else {
                    assert!(false)
                }
            } else if local == Local::from_usize(2) {
                // regular definitions for k
                if location.block == BasicBlock::from_usize(0) {
                    assert_eq!(idx, 1)
                } else if location.block == BasicBlock::from_usize(4) {
                    assert_eq!(idx, 3)
                } else if location.block == BasicBlock::from_usize(6) {
                    assert_eq!(idx, 4)
                } else {
                    assert!(false)
                }
            }
        }

        fn handle_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
            if local == Local::from_usize(0) {
                if block == BasicBlock::from_usize(1) {
                    assert_eq!(idx, 2)
                } else if block == BasicBlock::from_usize(7) {
                    assert_eq!(idx, 5)
                } else {
                    assert!(false)
                }
            } else if local == Local::from_usize(2) {
                if block == BasicBlock::from_usize(1) {
                    assert_eq!(idx, 2)
                } else if block == BasicBlock::from_usize(7) {
                    assert_eq!(idx, 5)
                } else {
                    assert!(false)
                }
            }
        }

        fn handle_use(&mut self, local: Local, idx: usize, location: Location) {
            if local == Local::from_usize(1) {
                // regular uses for i
                assert_eq!(location.block, BasicBlock::from_usize(3));
                assert_eq!(idx, 1);
            } else if local == Local::from_usize(0) {
                // regular uses for j
                if location.block == BasicBlock::from_usize(2) {
                    assert_eq!(idx, 2)
                } else if location.block == BasicBlock::from_usize(3) {
                    assert_eq!(idx, 2)
                } else if location.block == BasicBlock::from_usize(8) {
                    assert_eq!(idx, 5)
                } else {
                    assert!(false, "local {:?} of idx {} at {:?}", local, idx, location)
                }
            } else if local == Local::from_usize(2) {
                // regular uses for k
                if location.block == BasicBlock::from_usize(1) {
                    assert_eq!(idx, 2)
                } else if location.block == BasicBlock::from_usize(3) {
                    assert_eq!(idx, 2)
                } else if location.block == BasicBlock::from_usize(5) {
                    assert_eq!(idx, 2)
                } else {
                    assert!(false)
                }
            }
        }

        fn handle_use_at_phi_node(
            &mut self,
            local: Local,
            idx: usize,
            block: BasicBlock,
            pos: usize,
        ) {
            if local == Local::from_usize(0) {
                if block == BasicBlock::from_usize(1) {
                    if pos == 0 {
                        assert_eq!(idx, 1)
                    } else if pos == 1 {
                        assert_eq!(idx, 5)
                    } else {
                        assert!(false)
                    }
                } else if block == BasicBlock::from_usize(7) {
                    if pos == 0 {
                        assert_eq!(idx, 3)
                    } else if pos == 1 {
                        assert_eq!(idx, 4)
                    } else {
                        assert!(false)
                    }
                } else {
                    assert!(false)
                }
            } else if local == Local::from_usize(2) {
                if block == BasicBlock::from_usize(1) {
                    if pos == 0 {
                        assert_eq!(idx, 1)
                    } else if pos == 1 {
                        assert_eq!(idx, 5)
                    } else {
                        assert!(false)
                    }
                } else if block == BasicBlock::from_usize(7) {
                    if pos == 0 {
                        assert_eq!(idx, 3)
                    } else if pos == 1 {
                        assert_eq!(idx, 4)
                    } else {
                        assert!(false)
                    }
                } else {
                    assert!(false)
                }
            }
        }
    }

    let mut renamer = PlainRenamer::<
        BorrowckDefUse,
        (
            PrintStdSSAName,
            TestProgramSpec,
            SSANameMap,
            LocalSimplePtrCVMap<usize>,
        ),
    >::new(
        tcx,
        body,
        (
            PrintStdSSAName,
            TestProgramSpec,
            SSANameMap::new(body, &insertion_points),
            LocalSimplePtrCVMap::new(body),
        ),
    );
    renamer.rename();
    /*
    write_ssa_mir_fn(
        tcx,
        body,
        &renamer.ssa_name_handler.2,
        &mut std::io::stdout(),
    )
    .unwrap();
    */
}

fn spec1<'a, 'tcx>(tcx: TyCtxt<'tcx>, body: &'a Body<'tcx>) {
    let insertion_points = body.compute_phi_node::<BorrowckDefUse>(tcx);
    let mut renamer = PlainRenamer::<
        BorrowckDefUse,
        (PrintStdSSAName, SSANameMap, LocalSimplePtrCVMap<usize>),
    >::new(
        tcx,
        body,
        (
            PrintStdSSAName,
            SSANameMap::new(body, &insertion_points),
            LocalSimplePtrCVMap::new(body),
        ),
    );
    renamer.rename();
    /*
    write_ssa_mir_fn(
        tcx,
        body,
        &renamer.ssa_name_handler.1,
        &mut std::io::stdout(),
    )
    .unwrap();
    */
}
