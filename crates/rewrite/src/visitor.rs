use std::{
    fmt::Write as _,
    ops::{Deref, DerefMut},
};

use etrace::some_or;
use rustc_abi::FieldIdx;
use rustc_ast::{
    HasNodeId, ItemKind,
    ast::*,
    mut_visit::{self, MutVisitor},
    ptr::P,
    token::TokenKind,
    tokenstream::{TokenStream, TokenTree},
};
use rustc_ast_pretty::pprust;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_hir::ItemKind as HirItemKind;
use rustc_middle::{query::Key, ty::TyCtxt};
use rustc_span::{Span, Symbol, def_id::LocalDefId, sym::format, symbol::Ident};
use utils::rustc::{RustProgram, RustProgramWithMappings};

use crate::Analysis;
use crate::decision::{FnLocalDecisions, PtrDecision};
use utils::ir_util::{AstToHirMapper, IrMappings};

pub(super) struct TransformVisitor<'tcx, 'a, 'b> {
    rust_program: &'a RustProgramWithMappings<'tcx>,
    fn_local_decs: &'b FnLocalDecisions,
    pub updated: bool,
}

impl TransformVisitor<'_, '_, '_> {
    pub fn new<'tcx, 'a, 'b>(
        rust_program: &'a RustProgramWithMappings<'tcx>,
        fn_local_decs: &'b FnLocalDecisions,
    ) -> TransformVisitor<'tcx, 'a, 'b> {
        TransformVisitor {
            rust_program,
            fn_local_decs,
            updated: false,
        }
    }
}

impl MutVisitor for TransformVisitor<'_, '_, '_> {
    fn visit_item(&mut self, item: &mut Item) {
        let node_id = item.node_id();
        let ItemKind::Fn(box fn_item) = &mut item.kind else {
            return mut_visit::walk_item(self, item);
        };

        let hir_item = self
            .rust_program
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
                    let TyKind::Ptr(ptr_mut_ty) =
                        std::mem::replace(&mut param.ty.kind, TyKind::Infer)
                    else {
                        panic!("Expected pointer type for parameter");
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
                    let TyKind::Ptr(ptr_mut_ty) =
                        std::mem::replace(&mut param.ty.kind, TyKind::Infer)
                    else {
                        panic!("Expected pointer type for parameter");
                    };
                    assert!(ptr_mut_ty.mutbl == rustc_ast::Mutability::Mut);
                    param.ty.kind = TyKind::Ref(None, ptr_mut_ty);
                    self.updated = true;
                }
                PtrDecision::AsIs => {}
            }
        }

        mut_visit::walk_item(self, item);
    }
}
