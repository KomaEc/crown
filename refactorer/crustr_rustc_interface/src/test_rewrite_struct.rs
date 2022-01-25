use rustc_hir::{intravisit, ItemKind, OwnerNode};
use rustc_interface::interface::Compiler;

use crustr_rewrite::Rewriter;

use crate::CompilerRunnable;

pub struct CollectStructInfo;

impl CompilerRunnable for CollectStructInfo {
    type Output = ();

    fn run(compiler: &Compiler) -> Self::Output {
        compiler.enter(|queries| {
            // let ast_krate = queries.parse().unwrap().take();
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                let mut vis = crustr_passes::initial_rewrite::StructRewriteVisitor {
                    tcx,
                    rewriter: Rewriter::default(),
                };

                // Every compilation contains a single crate.
                let hir_krate = tcx.hir().krate();
                let mut structs = Vec::new();
                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(variant_data, _generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                // println!("{:?}", variant_data);

                                println!("{}", item.ident);

                                // vis.visit_item(item);
                                intravisit::walk_struct_def(&mut vis, variant_data);

                                structs.push(did)
                            }
                        }
                    }
                }

                vis.rewriter.over_rewrite();
            })
        })
    }
}
