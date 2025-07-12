//! Auxilary definitions for MIR

use std::hash::Hash;

use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{Operand, Place, Terminator, TerminatorKind, visit::Visitor},
    ty::TyCtxt,
};
use rustc_span::{Ident, source_map::Spanned};
use rustc_type_ir::TyKind::FnDef;
use utils::{
    dsa::fixed_shape::VecVec,
    petgraph::{algo::TarjanScc, prelude::DiGraphMap},
    rustc::RustProgram,
};

pub trait TerminatorExt<'tcx> {
    fn call(&self) -> Option<(&Operand<'tcx>, &[Spanned<Operand<'tcx>>], Place<'tcx>)>;
}

impl<'tcx> TerminatorExt<'tcx> for Terminator<'tcx> {
    fn call(&self) -> Option<(&Operand<'tcx>, &[Spanned<Operand<'tcx>>], Place<'tcx>)> {
        match &self.kind {
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => Some((func, args, *destination)),
            TerminatorKind::TailCall { func, args, .. } => {
                Some((func, args, Place::return_place()))
            }
            _ => None,
        }
    }
}

pub enum CallKind {
    FreeStanding(DefId),
    Extern(Ident),
    Library(DefId),
    Impl(DefId),
    Closure,
    Dynamic,
}

impl CallKind {
    pub fn new<'tcx>(tcx: TyCtxt<'tcx>, func: &Operand<'tcx>) -> CallKind {
        if let Some(func) = func.constant() {
            let ty = func.ty();
            let &FnDef(callee, _) = ty.kind() else {
                unreachable!()
            };

            if let Some(local_did) = callee.as_local() {
                match tcx.hir_node_by_def_id(local_did) {
                    rustc_hir::Node::Item(_) => return CallKind::FreeStanding(callee),
                    rustc_hir::Node::ForeignItem(foreign_item) => {
                        return CallKind::Extern(foreign_item.ident);
                    }
                    rustc_hir::Node::ImplItem(_) => return CallKind::Impl(callee),
                    rustc_hir::Node::TraitItem(_) => return CallKind::Dynamic,
                    _ => unreachable!(),
                }
            } else {
                return CallKind::Library(callee);
            }
        } else {
            return CallKind::Closure;
        }
    }
}

pub struct CallGraphPostOrder(pub VecVec<DefId>);

impl CallGraphPostOrder {
    pub fn new(program: &RustProgram) -> Self {
        let mut graph = DiGraphMap::new();
        for &did in &program.functions {
            graph.add_node(CxDefId::new(program.tcx, did));
        }

        for &did in &program.functions {
            StaticCallGraphBuilder {
                tcx: program.tcx,
                caller: did,
                graph: &mut graph,
            }
            .visit_body(program.tcx.optimized_mir(did));
        }

        let mut tarjan_scc = TarjanScc::new();
        let mut post_order = VecVec::with_indices_capacity(program.functions.len());
        tarjan_scc.run(&graph, |nodes| {
            post_order.push_vec(nodes.iter().map(|cxdid| cxdid.did))
        });
        let post_order = post_order.complete();

        CallGraphPostOrder(post_order)
    }

    pub fn sccs(&self) -> impl Iterator<Item = &[DefId]> {
        self.0.iter()
    }
}

/// FIXME: Hmmmm does it make sense at all?
/// Is it possible that two [`DefId`] can have the same name?
#[derive(Clone, Copy)]
struct CxDefId<'tcx> {
    tcx: TyCtxt<'tcx>,
    did: DefId,
}

impl<'tcx> CxDefId<'tcx> {
    fn new(tcx: TyCtxt<'tcx>, did: DefId) -> Self {
        CxDefId { tcx, did }
    }
}

impl Hash for CxDefId<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.did.hash(state);
    }
}

impl PartialEq for CxDefId<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.did == other.did
    }
}

impl Eq for CxDefId<'_> {}

impl PartialOrd for CxDefId<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tcx
            .item_name(self.did)
            .partial_cmp(&other.tcx.item_name(other.did))
    }
}

impl<'tcx> Ord for CxDefId<'tcx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tcx
            .item_name(self.did)
            .cmp(&other.tcx.item_name(other.did))
    }
}

struct StaticCallGraphBuilder<'me, 'tcx> {
    tcx: TyCtxt<'tcx>,
    caller: DefId,
    /// We use the `def_path_str` of [`DefId`] as keys
    graph: &'me mut DiGraphMap<CxDefId<'tcx>, ()>,
}

impl<'me, 'tcx> Visitor<'tcx> for StaticCallGraphBuilder<'me, 'tcx> {
    fn visit_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        let Some((func, ..)) = terminator.call() else {
            return;
        };
        let Some(func_constant) = func.constant() else {
            return;
        };
        let ty = func_constant.ty();
        let &FnDef(callee, _generic_args) = ty.kind() else {
            unreachable!("what could it be? {}", ty)
        };
        if !self.graph.contains_node(CxDefId::new(self.tcx, callee)) {
            return;
        }

        self.graph.add_edge(
            CxDefId::new(self.tcx, self.caller),
            CxDefId::new(self.tcx, callee),
            (),
        );
    }
}
