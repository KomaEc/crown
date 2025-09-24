use rustc_ast::{
    HasNodeId, ItemKind,
    ast::*,
    mut_visit::{self, MutVisitor},
    ptr::P,
};
use rustc_span::symbol::Ident;
use utils::rustc::RustProgram;

use crate::decision::{FnLocalDecisions, PtrDecision};
use utils::ir_util::IrMappings;

pub(super) struct TransformVisitor<'tcx, 'a> {
    rust_program: &'a RustProgram<'tcx>,
    fn_local_decs: &'a FnLocalDecisions,
    ir_mappings: IrMappings<'a>,
    pub updated: bool,
}

impl TransformVisitor<'_, '_> {
    pub fn new<'tcx, 'a>(
        rust_program: &'a RustProgram<'tcx>,
        fn_local_decs: &'a FnLocalDecisions,
        ir_mappings: IrMappings<'a>,
    ) -> TransformVisitor<'tcx, 'a> {
        TransformVisitor {
            rust_program,
            fn_local_decs,
            ir_mappings,
            updated: false,
        }
    }
}

impl MutVisitor for TransformVisitor<'_, '_> {
    fn visit_item(&mut self, item: &mut Item) {
        let node_id = item.node_id();
        let ItemKind::Fn(box fn_item) = &mut item.kind else {
            return mut_visit::walk_item(self, item);
        };

        let hir_item = self
            .ir_mappings
            .ast_to_hir
            .get_item(node_id, self.rust_program.tcx)
            .unwrap_or_else(|| panic!("Failed to find HIR item for Item {:?}", item.span));

        let def_id = hir_item.owner_id.def_id.to_def_id();
        let fn_dec = self.fn_local_decs.local_data(&def_id);

        // Currently intra-procedural borrow inference: skip return type, only consider parameters
        for (idx, param) in fn_item.sig.decl.inputs.iter_mut().enumerate() {
            let local_idx = idx + 1;
            let local_dec = &fn_dec[local_idx][0];
            match local_dec {
                PtrDecision::OptMutRef => {
                    // println!("Output parameter {:#?}", param);
                    let TyKind::Ptr(ptr_mut_ty) =
                        std::mem::replace(&mut param.ty.kind, TyKind::Infer)
                    else {
                        panic!("Expected pointer type for parameter {:#?}", param);
                    };
                    assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut);

                    // Create Option<&mut T>
                    let inner_ref = P(Ty {
                        id: rustc_ast::DUMMY_NODE_ID,
                        kind: TyKind::Ref(None, ptr_mut_ty),
                        span: rustc_span::DUMMY_SP,
                        tokens: None,
                    });

                    // Create the Option path
                    let option_path = Path {
                        span: rustc_span::DUMMY_SP,
                        segments: vec![PathSegment {
                            ident: Ident::from_str("Option"),
                            id: rustc_ast::DUMMY_NODE_ID,
                            args: Some(P(GenericArgs::AngleBracketed(AngleBracketedArgs {
                                span: rustc_span::DUMMY_SP,
                                args: [AngleBracketedArg::Arg(GenericArg::Type(inner_ref))].into(),
                            }))),
                        }]
                        .into(),
                        tokens: None,
                    };

                    param.ty.kind = TyKind::Path(None, option_path);
                    self.updated = true;
                }
                PtrDecision::MutRef => {
                    match param.ty.kind {
                        TyKind::Ptr(..) => {
                            let TyKind::Ptr(ptr_mut_ty) =
                                std::mem::replace(&mut param.ty.kind, TyKind::Infer)
                            else {
                                unreachable!()
                            };
                            param.ty.kind = TyKind::Ref(None, ptr_mut_ty);
                            self.updated = true;
                        }
                        TyKind::Path(..) => {
                            param.ty = Ty {
                                id: rustc_ast::DUMMY_NODE_ID,
                                kind: TyKind::Ref(
                                    None,
                                    MutTy {
                                        ty: param.ty.clone(),
                                        mutbl: rustc_ast::Mutability::Mut,
                                    },
                                ),
                                span: rustc_span::DUMMY_SP,
                                tokens: None,
                            }
                            .into();
                        }
                        _ => panic!("Expected pointer type for parameter {:#?}", param),
                    };
                    // assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut); Not necessarily
                }
                PtrDecision::AsIs => {}
            }
        }

        mut_visit::walk_item(self, item);
    }
}
