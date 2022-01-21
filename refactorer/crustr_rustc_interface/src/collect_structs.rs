use rustc_hir::{ItemKind, OwnerNode};
use rustc_interface::interface::Compiler;

use crate::CompilerRunnable;

pub struct CollectStructInfo;

impl CompilerRunnable for CollectStructInfo {
    type Output = ();

    fn run(compiler: &Compiler) -> Self::Output {
        compiler.enter(|queries| {
            // let ast_krate = queries.parse().unwrap().take();
            queries.global_ctxt().unwrap().take().enter(|tcx| {
                // Every compilation contains a single crate.
                let hir_krate = tcx.hir().krate();
                let mut structs = Vec::new();
                for (did, owner) in hir_krate.owners.iter_enumerated() {
                    if let Some(owner_info) = owner {
                        if let OwnerNode::Item(item) = owner_info.node() {
                            if let ItemKind::Struct(variant_data, generics) = &item.kind {
                                let def_id = item.def_id;
                                assert_eq!(def_id, did);

                                println!("{:?}", variant_data);

                                structs.push(did)
                            }
                        }
                    }
                }
            })
        })
    }
}
