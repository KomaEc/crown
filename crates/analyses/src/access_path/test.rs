use rustc_abi::FieldIdx;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Local, Place, ProjectionElem},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::Adt;

use crate::access_path::{KLimited, ctxt::AccessPathsCx};

#[test]
fn test_some_random_structs() {
    const PROGRAM: &str = "
    struct s {
        f: t,
        g: *mut i32,
        h: *mut s,
    }
    struct t {
        f: *mut i32,
        g: u
    }
    struct u {
        f: v,
        g: w,
        h: x
    }
    struct v {
        f: i32,
    }
    struct w {
        f: *mut i32
    }
    struct x;
    ";

    utils::rustc::run_compiler(PROGRAM, |program| {
        let apcx = AccessPathsCx::new(&program);

        macro_rules! define_structs {
            ($( $x: ident ),*) => {
                $(
                    let &$x = program
                        .structs
                        .iter()
                        .find(|&&did| {
                        let stringify!($x) = program.tcx.def_path_str(did).as_str() else { return false };
                        true
                    })
                    .unwrap();
                )*
            };
        }
        define_structs!(s, t, u, v, w, x);

        let get_offsets = |k_limit: usize, did: DefId| {
            apcx.start_offsets(KLimited::new(k_limit, did))
                .collect::<Vec<_>>()
        };

        assert_eq!(get_offsets(0, s), [0, 0, 0, 0]);
        assert_eq!(get_offsets(1, s), [0, 2, 3, 4]);
        assert_eq!(get_offsets(2, s), [0, 2, 3, 8]);
        assert_eq!(get_offsets(3, s), [0, 2, 3, 12]);
        assert_eq!(get_offsets(3, t), [0, 1, 2]);
        assert_eq!(get_offsets(3, u), [0, 0, 1, 1]);
        assert_eq!(get_offsets(3, v), [0, 0]);
        assert_eq!(get_offsets(3, w), [0, 1]);
        assert_eq!(get_offsets(3, x), [0]);
    });
}

#[test]
fn test_bst_offsets() {
    const PROGRAM: &str =
        "struct Node { data: Data, left: *mut Node, right: *mut Node } struct Data;";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let apcx = AccessPathsCx::new(&program);

        let &node = program
            .structs
            .iter()
            .find(|&&did| {
                let "Node" = program.tcx.def_path_str(did).as_str() else {
                    return false;
                };
                true
            })
            .unwrap();

        let get_offsets = |k_limit: usize, did: DefId| {
            apcx.start_offsets(KLimited::new(k_limit, did))
                .collect::<Vec<_>>()
        };

        assert_eq!(get_offsets(0, node), [0, 0, 0, 0]);
        assert_eq!(get_offsets(1, node), [0, 0, 1, 2]);
        assert_eq!(get_offsets(2, node), [0, 0, 3, 6]);
        assert_eq!(get_offsets(3, node), [0, 0, 7, 14]);
    })
}

#[test]
fn test_bst_encode() {
    const PROGRAM: &str = "
        struct Node { left: *mut Node, right: *mut Node }
        unsafe fn f(input: *mut Node) {
            (*input).left = core::ptr::null_mut();
        }";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let apcx = AccessPathsCx::new(&program);

        let body = &*program
            .tcx
            .mir_drops_elaborated_and_const_checked(
                program.functions.first().unwrap().expect_local(),
            )
            .borrow();
        let tcx = program.tcx;

        let input = Place::from(Local::from_u32(1));
        let star_node = input.ty(body, tcx).ty;

        // encoding of `input`
        assert_eq!(
            apcx.encode(KLimited::new(3, input), body, tcx).projections,
            KLimited::new(3, (0..7).into()),
        );

        // encoding of `*input`
        assert_eq!(
            apcx.encode(
                KLimited::new(3, input.project_deeper(&[ProjectionElem::Deref], tcx)),
                body,
                tcx
            )
            .projections,
            KLimited::new(2, (1..7).into()),
        );

        // encoding of `(*input).left`
        assert_eq!(
            apcx.encode(
                KLimited::new(
                    3,
                    input.project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(0), star_node)
                        ],
                        tcx
                    )
                ),
                body,
                tcx
            )
            .projections,
            KLimited::new(2, (1..4).into()),
        );

        // encoding of `(*input).right`
        assert_eq!(
            apcx.encode(
                KLimited::new(
                    3,
                    input.project_deeper(
                        &[
                            ProjectionElem::Deref,
                            ProjectionElem::Field(FieldIdx::from_u32(1), star_node)
                        ],
                        tcx
                    )
                ),
                body,
                tcx
            )
            .projections,
            KLimited::new(2, (4..7).into()),
        );
    })
}

#[test]
fn test_bst_matcher() {
    const PROGRAM: &str = "
        struct Node { left: *mut Node, right: *mut Node }";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let tcx = program.tcx;
        let apcx = AccessPathsCx::new(&program);

        let &node_did = program
            .structs
            .iter()
            .find(|&&did| {
                let "Node" = program.tcx.def_path_str(did).as_str() else {
                    return false;
                };
                true
            })
            .unwrap();
        let node = program.tcx.type_of(node_did).skip_binder();
        let Adt(adt_def, subst_ref) = node.kind() else {
            unreachable!()
        };
        let ty_star_node = adt_def
            .all_fields()
            .next()
            .unwrap()
            .ty(program.tcx, &subst_ref);

        // let node_struct_index = StructIndex::from_u32(0);

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 0, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 4]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 2, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 8]
        );

        assert_eq!(
            apcx.lift(KLimited::new(3, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2, 5, 8, 9, 12]
        );
    })
}

#[test]
fn test_weird_bst_matcher() {
    const PROGRAM: &str = "
        struct Node { left: *mut *mut Node, right: *mut *mut Node }";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let tcx = program.tcx;
        let apcx = AccessPathsCx::new(&program);

        let &node_did = program
            .structs
            .iter()
            .find(|&&did| {
                let "Node" = program.tcx.def_path_str(did).as_str() else {
                    return false;
                };
                true
            })
            .unwrap();
        let node = program.tcx.type_of(node_did).skip_binder();
        let Adt(adt_def, subst_ref) = node.kind() else {
            unreachable!()
        };
        let ty_star_node = adt_def
            .all_fields()
            .next()
            .unwrap()
            .ty(program.tcx, &subst_ref)
            .builtin_deref(true)
            .unwrap();

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 0, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 3]
        );

        assert_eq!(
            apcx.lift(KLimited::new(1, ty_star_node), 3, tcx)
                .collect::<Vec<_>>(),
            [0]
        );

        assert_eq!(
            apcx.lift(KLimited::new(3, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2, 5, 6]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 2, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 5]
        );
    })
}

#[test]
fn test_weirder_bst_matcher() {
    const PROGRAM: &str = "
        struct Node { left: *mut Node, right: *mut *mut Node }";
    utils::rustc::run_compiler(PROGRAM, |program| {
        let tcx = program.tcx;
        let apcx = AccessPathsCx::new(&program);

        let &node_did = program
            .structs
            .iter()
            .find(|&&did| {
                let "Node" = program.tcx.def_path_str(did).as_str() else {
                    return false;
                };
                true
            })
            .unwrap();
        let node = program.tcx.type_of(node_did).skip_binder();
        let Adt(adt_def, subst_ref) = node.kind() else {
            unreachable!()
        };
        let ty_star_node = adt_def
            .all_fields()
            .next()
            .unwrap()
            .ty(program.tcx, &subst_ref);

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 0, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2]
        );

        assert_eq!(
            apcx.lift(KLimited::new(3, ty_star_node), 0, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 4]
        );

        assert_eq!(
            apcx.lift(KLimited::new(1, ty_star_node), 3, tcx)
                .collect::<Vec<_>>(),
            [0]
        );

        assert_eq!(
            apcx.lift(KLimited::new(3, ty_star_node), 1, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 2, 5, 7, 8]
        );

        assert_eq!(
            apcx.lift(KLimited::new(2, ty_star_node), 2, tcx)
                .collect::<Vec<_>>(),
            [0, 1, 7]
        );
    })
}

#[test]
fn smoke_test_libtree() {
    utils::rustc::run_compiler(utils::rustc::SourceCode::Libtree, |program| {
        let apcx = AccessPathsCx::new(&program);
        let tcx = program.tcx;

        use rustc_middle::mir::{
            HasLocalDecls, Location,
            visit::{PlaceContext, Visitor},
        };
        struct Vis<'tcx, 'this, D: HasLocalDecls<'tcx>> {
            tcx: TyCtxt<'tcx>,
            apcx: &'this AccessPathsCx,
            local_decls: &'this D,
        }
        const K_LIMIT: usize = 3;
        impl<'tcx, 'apcx, D: HasLocalDecls<'tcx>> Visitor<'tcx> for Vis<'tcx, 'apcx, D> {
            fn visit_place(
                &mut self,
                place: &Place<'tcx>,
                _context: PlaceContext,
                _location: Location,
            ) {
                let encoded =
                    self.apcx
                        .encode(KLimited::new(K_LIMIT, place), self.local_decls, self.tcx);

                if encoded.projections.k_limit < K_LIMIT {
                    let lifted = self
                        .apcx
                        .lift(
                            encoded
                                .projections
                                .map(|_| place.ty(self.local_decls, self.tcx).ty),
                            3 - encoded.projections.k_limit,
                            self.tcx,
                        )
                        .collect::<Vec<_>>();
                    assert_eq!(
                        lifted.len(),
                        self.apcx.size_of(
                            KLimited::new(
                                encoded.projections.k_limit,
                                place.ty(self.local_decls, self.tcx).ty
                            ),
                            self.tcx
                        )
                    );
                }
            }
        }

        for did in &program.functions {
            let body = &*tcx
                .mir_drops_elaborated_and_const_checked(did.expect_local())
                .borrow();
            Vis {
                tcx,
                apcx: &apcx,
                local_decls: body,
            }
            .visit_body(body)
        }
    })
}
