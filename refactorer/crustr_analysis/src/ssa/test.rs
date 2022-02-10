use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{BasicBlock, Local, Location};
use std::str;

use crate::def_use::BorrowckDefUse;
use crate::ssa::body_ext::BodyExt;
use crate::ssa::rename::{implementation::PrintStdRename, RenameHandler, Renamer};

#[test]
fn test_phi_node_insertion_point() {
    compiler_interface::run_compiler_with_input_str_with_single_func(
        TEST_PROGRAM,
        |tcx, fn_did| {
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
        },
    );
}

#[test]
fn test_rename() {
    compiler_interface::run_compiler_with_input_str_with_single_func(TEST_PROGRAM, |tcx, fn_did| {
        let body = tcx.optimized_mir(fn_did);
        let insertion_points = body.compute_phi_node::<BorrowckDefUse>(tcx);

        struct TestProgramSpec;
        impl RenameHandler for TestProgramSpec {
            fn rename_def(&mut self, local: Local, idx: usize, location: Location) {
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

            fn rename_def_at_phi_node(&mut self, local: Local, idx: usize, block: BasicBlock) {
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

            fn rename_use(&mut self, local: Local, idx: usize, location: Location) {
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
                    } else {
                        assert!(false)
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

            fn rename_use_at_phi_node(
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

        let mut renamer = Renamer::<BorrowckDefUse, (PrintStdRename, TestProgramSpec)>::new(
            body,
            &insertion_points,
            (PrintStdRename, TestProgramSpec),
        );
        renamer.visit_body(body);
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
