use orc_common::data_structure::vec_array::VecArray;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{visit::Visitor, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;
use smallvec::SmallVec;

pub struct FnSig<T> {
    pub ret: T,
    pub args: SmallVec<[T; 4]>,
}

impl<T> FnSig<T> {
    pub fn new(ret: T, args: SmallVec<[T; 4]>) -> Self {
        FnSig { ret, args }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::once(&self.ret).chain(self.args.iter())
    }
}

impl<T> FromIterator<T> for FnSig<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let ret = iter.next().unwrap();
        let args = iter.collect();
        FnSig { ret, args }
    }
}

impl<T: Default> Default for FnSig<T> {
    fn default() -> Self {
        Self {
            ret: Default::default(),
            args: Default::default(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for FnSig<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        f.write_str(
            &self
                .args
                .iter()
                .map(|data| format!("{:?}", data))
                .collect::<Vec<_>>()
                .join(", "),
        )?;
        f.write_str(") -> ")?;
        f.write_fmt(format_args!("{:?}", self.ret))
    }
}

pub struct CallGraph {
    post_order: VecArray<DefId>,
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
        let mut post_order = VecArray::with_indices_capacity(functions.len());
        tarjan_scc.run(&graph, |nodes| post_order.push_array(nodes.iter().copied()));
        let post_order = post_order.done();
        CallGraph { post_order }
    }

    #[inline]
    pub fn fns(&self) -> &[DefId] {
        self.post_order.everything()
    }

    #[inline]
    pub fn sccs(&self) -> impl Iterator<Item = &[DefId]> {
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
            return
        };
        let ty = func_constant.ty();
        let &FnDef(callee, _generic_args) = ty.kind() else {
            unreachable!("what could it be? {}", ty)
        };
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

            assert_eq!(call_graph.sccs().count(), 4);

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
