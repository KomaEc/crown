use itertools::izip;
use rustc_ast::{
    HasNodeId, ItemKind,
    ast::*,
    mut_visit::{self, MutVisitor},
    ptr::P,
};
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Ty as MirTy, TyKind as MirTyKind};
use rustc_span::{sym::expect, symbol::Ident};
use utils::{ir_util::map_thir_to_mir, rustc::RustProgram};

use crate::decision::{FnLocalDecisions, PtrKind};
use utils::ast_util;
use utils::ir_util::IrMappings;

pub(crate) struct TransformVisitor<'tcx, 'a> {
    rust_program: &'a RustProgram<'tcx>,
    fn_local_decs: &'a FnLocalDecisions,
    ir_mappings: IrMappings<'a>,
    pub updated: bool,
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
        let local_def_id = hir_item.owner_id.def_id;
        let def_id = local_def_id.to_def_id();
        let mir_body = self
            .rust_program
            .tcx
            .mir_drops_elaborated_and_const_checked(local_def_id)
            .borrow();

        // 1. Rewrite function signature
        let local_input_decls = mir_body
            .args_iter()
            .map(|local| mir_body.local_decls[local].clone())
            .collect::<Vec<_>>();

        let fn_dec = self.fn_local_decs.local_data(&def_id);

        // Currently intra-procedural borrow inference: skip return type, only consider parameters
        for (idx, (local_decl, param)) in
            izip!(local_input_decls, fn_item.sig.decl.inputs.iter_mut()).enumerate()
        {
            let local_idx = idx + 1;
            let ty_res = utils::ty!("{}", local_decl.ty.to_string()); // resolved type (no type alias)
            self.rewrite_ty(&mut param.ty, ty_res, &fn_dec[local_idx][0]);
        }
    }
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

    fn rewrite_ty(&mut self, ty: &mut Ty, ty_res: Ty, dec: &Option<PtrKind>) {
        match dec {
            Some(PtrKind::OptMutRef) => {
                let ptr_mut_ty = expect_ptr(ty, ty_res);
                // Create Option<&mut T>
                assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut);

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

                ty.kind = TyKind::Path(None, option_path);
                self.updated = true;
            }
            Some(PtrKind::MutRef) => {
                let ptr_mut_ty = expect_ptr(ty, ty_res);
                ty.kind = TyKind::Ref(None, ptr_mut_ty);
                self.updated = true;
                // Not necessarily; TODO: immutable reference for immutable raw pointers
                // assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut);
            }
            None | Some(PtrKind::MutRaw) => {}
        }
    }
}

fn expect_ptr(ty: &mut Ty, ty_res: Ty) -> MutTy {
    match std::mem::replace(&mut ty.kind, TyKind::Infer) {
        TyKind::Ptr(ptr) => ptr,
        _ => match ty_res.kind {
            TyKind::Ptr(ptr) => ptr,
            _ => panic!("Expected pointer type for type {:#?}", ty),
        },
    }
}
