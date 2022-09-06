use orc_common::data_structure::vec_array::VecArray;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{visit::Visitor, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;

// pub(crate) struct FnSig<T> {
//     output: T,
//     inputs: Vec<T>,
// }

pub struct CallGraph {
    graph: DiGraphMap<DefId, ()>,
    // /// (sccs + post_order): Vec<Vec<DefId>>,
    // sccs: Vec<usize>,
    // post_order: Vec<DefId>,
    post_order: VecArray<DefId>,
    // func_sigs: FxHashMap<DefId, >
}

impl CallGraph {
    pub fn new(tcx: TyCtxt, functions: &[DefId]) -> Self {
        let mut graph = DiGraphMap::new();
        for did in functions {
            graph.add_node(*did);
        }
        for did in functions {
            CallGraphConstruction {
                caller: *did,
                graph: &mut graph,
            }
            .visit_body(tcx.optimized_mir(did));
        }

        let mut tarjan_scc = TarjanScc::new();
        let mut post_order = VecArray::new(functions.len());
        // let mut sccs = vec![0];
        // let mut post_order = Vec::with_capacity(functions.len());
        tarjan_scc.run(&graph, |nodes| {
            // post_order.extend(nodes);
            // sccs.push(post_order.len());
            post_order.push_array(nodes.iter().copied())
        });
        let post_order = post_order.done();
        CallGraph {
            graph,
            // sccs,
            post_order,
        }
    }

    #[inline]
    pub fn functions(&self) -> &[DefId] {
        // &self.post_order
        self.post_order.everything()
    }

    #[inline]
    pub fn function_count(&self) -> usize {
        self.graph.node_count()
    }

    #[inline]
    pub fn num_sccs(&self) -> usize {
        // self.sccs.len() - 1
        self.post_order.array_count()
    }

    #[inline]
    pub fn sccs(&self) -> impl Iterator<Item = &[DefId]> {
        // self.sccs
        //     .array_windows()
        //     .map(|&[start, end]| &self.post_order[start..end])
        self.post_order.iter()
    }
}

struct CallGraphConstruction<'me> {
    caller: DefId,
    graph: &'me mut DiGraphMap<DefId, ()>,
}

impl<'me, 'tcx> Visitor<'tcx> for CallGraphConstruction<'me> {
    fn visit_terminator(
        &mut self,
        terminator: &Terminator<'tcx>,
        _location: rustc_middle::mir::Location,
    ) {
        let TerminatorKind::Call {
            func,
            ..
        } = &terminator.kind else {
            return
        };
        let Some(func_constant) = func.constant() else {
            // tracing::warn!("closures or function pointers are now ignored");
            return
        };
        let ty = func_constant.ty();
        let &FnDef(callee, _generic_args) = ty.kind() else {
            unreachable!("what could it be? {}", ty)
        };
        // // local defined functions: libc externs or user functions
        // let Some(_) = callee.as_local() else { return };
        if !self.graph.contains_node(callee) {
            return;
        }

        self.graph.add_edge(self.caller, callee, ());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use orc_common::test_infra::init_logger;

    const TEST_PROGRAMS: &'static str = "
        fn f() {
            assert!(true);
            if cond() {
                g()
            }
        }
        fn g() {
            h()
        }
        fn h() {
            if !cond() {
                f();
            }
            if cond() {
                g();
            }
            l()
        }
        fn m() {
            g()
        }
        fn l() {}
        #[inline(never)]
        fn cond() -> bool {
            true
        }
        ";

    #[test]
    fn test() {
        init_logger();
        orc_common::test_infra::run_compiler_with(TEST_PROGRAMS.into(), |tcx, functions, _| {
            let call_graph = CallGraph::new(tcx, &functions[..]);
            for &caller in call_graph.functions() {
                for callee in call_graph.graph.neighbors(caller) {
                    tracing::debug!("{:?} ---> {:?}", caller, callee)
                }
            }

            assert_eq!(call_graph.num_sccs(), 4);

            let mut post_order_sorted = call_graph
                .sccs()
                .map(|nodes| nodes.to_vec())
                .collect::<Vec<_>>();

            for group in post_order_sorted.iter_mut() {
                group.sort();
            }

            let ([c1, c2, c3, c4], empty) = post_order_sorted.split_array_ref();
            assert!(empty.is_empty());

            let (&[f, g, h, m, l, cond], empty) = functions.split_array_ref();
            assert!(empty.is_empty());

            assert_eq!(*c1, vec![cond]);
            assert_eq!(*c2, vec![l]);
            assert_eq!(*c3, vec![f, g, h]);
            assert_eq!(*c4, vec![m]);
        });
    }
}
