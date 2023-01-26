use common::data_structure::vec_vec::VecVec;
use petgraph::{algo::TarjanScc, prelude::DiGraphMap};
use rustc_hash::FxHashMap;
use rustc_hir::def_id::DefId;
use rustc_middle::{
    mir::{visit::Visitor, Terminator, TerminatorKind},
    ty::TyCtxt,
};
use rustc_type_ir::TyKind::FnDef;
use smallvec::SmallVec;

use crate::lattice::{FlatSet, Lattice};

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Monotonicity {
    Alloc,
    Dealloc,
}

struct MonotonicityChecker<'me, 'tcx> {
    this: FlatSet<Monotonicity>,
    monotonicity: &'me FxHashMap<DefId, FlatSet<Monotonicity>>,
    tcx: TyCtxt<'tcx>,
}

impl<'me, 'tcx> Visitor<'tcx> for MonotonicityChecker<'me, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, _: rustc_middle::mir::Location) {
        let TerminatorKind::Call { func, .. } = &terminator.kind else { return; };
        let Some(func) = func.constant() else { return; };
        let ty = func.ty();
        let &FnDef(callee, _) = ty.kind() else { unreachable!() };
        let Some(local_did) = callee.as_local() else { return; };

        match self.tcx.hir().find_by_def_id(local_did).unwrap() {
            rustc_hir::Node::ForeignItem(foreign_item) => match foreign_item.ident.as_str() {
                "free" => {
                    self.this.meet(&FlatSet::Elem(Monotonicity::Dealloc));
                }
                "malloc" | "calloc" | "realloc" => {
                    self.this.meet(&FlatSet::Elem(Monotonicity::Alloc));
                }
                _ => {}
            },
            rustc_hir::Node::Item(..) => {
                let that = self.monotonicity[&callee];
                self.this.meet(&that);
            }
            _ => {}
        }
    }
}

pub struct CallGraph {
    post_order: VecVec<DefId>,
    ranked_by_n_alloc_deallocs: Vec<DefId>,
    monotonicity: FxHashMap<DefId, FlatSet<Monotonicity>>,
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
        let mut post_order = VecVec::with_indices_capacity(functions.len());
        tarjan_scc.run(&graph, |nodes| post_order.push_vec(nodes.iter().copied()));
        let post_order = post_order.done();

        let mut n_alloc_deallocs = rustc_hash::FxHashMap::default();
        n_alloc_deallocs.reserve(functions.len());
        for did in functions {
            struct Vis<'tcx>(TyCtxt<'tcx>, usize);
            impl<'tcx> Visitor<'tcx> for Vis<'tcx> {
                fn visit_terminator(
                    &mut self,
                    terminator: &Terminator<'tcx>,
                    _: rustc_middle::mir::Location,
                ) {
                    let tcx = self.0;
                    let TerminatorKind::Call { func, .. } = &terminator.kind else { return };
                    if let Some(func) = func.constant() {
                        let ty = func.ty();
                        let &FnDef(callee, _) = ty.kind() else { unreachable!() };
                        if let Some(local_did) = callee.as_local() {
                            if let rustc_hir::Node::ForeignItem(foreign_item) =
                                tcx.hir().find_by_def_id(local_did).unwrap()
                            {
                                match foreign_item.ident.as_str() {
                                    "free" => self.1 += 10,
                                    "malloc" | "calloc" | "realloc" => self.1 += 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            let mut vis = Vis(tcx, 0);
            let body = tcx.optimized_mir(*did);
            vis.visit_body(body);
            n_alloc_deallocs.insert(*did, vis.1);
        }
        let mut ranked_by_n_alloc_deallocs = functions.to_vec();
        ranked_by_n_alloc_deallocs
            .sort_by(|f, g| n_alloc_deallocs[f].cmp(&n_alloc_deallocs[g]).reverse());

        let mut monotonicity = FxHashMap::default();
        monotonicity.reserve(functions.len());
        tarjan_scc.run(&graph, |nodes| {
            monotonicity.extend(nodes.iter().map(|&node| (node, FlatSet::Top)));
            loop {
                let mut changed = false;

                for did in nodes {
                    let body = tcx.optimized_mir(*did);
                    let original = monotonicity[did];
                    let mut mc = MonotonicityChecker {
                        this: original,
                        monotonicity: &monotonicity,
                        tcx,
                    };
                    mc.visit_body(body);
                    let new = mc.this;
                    if original != new {
                        changed = true;
                        *monotonicity.get_mut(did).unwrap() = new;
                    }
                }

                if !changed {
                    break;
                }
            }
        });

        CallGraph {
            post_order,
            ranked_by_n_alloc_deallocs,
            monotonicity,
        }
    }

    pub fn monotonicity(&self, did: DefId) -> FlatSet<Monotonicity> {
        self.monotonicity[&did]
    }

    #[inline]
    pub fn fns(&self) -> &[DefId] {
        &self.ranked_by_n_alloc_deallocs
    }

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
    use common::test::init_logger;

    use super::*;

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
        common::test::run_compiler_with(TEST_PROGRAMS.into(), |tcx, functions, _| {
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
