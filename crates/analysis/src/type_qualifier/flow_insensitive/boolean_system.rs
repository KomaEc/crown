use common::data_structure::vec_vec::VecVec;
use petgraph::{algo::TarjanScc, graph::NodeIndex};
use rustc_index::vec::{Idx, IndexVec};

use super::{BooleanLattice, ConstraintSystem, Var};

pub struct BooleanSystem<Value: BooleanLattice> {
    constraint_graph: petgraph::graph::DiGraph<(), ()>,
    solved: once_cell::unsync::OnceCell<VecVec<Var>>,
    _lattice: std::marker::PhantomData<*const Value>,
}

impl<Value: BooleanLattice> BooleanSystem<Value> {
    pub const TOP: Var = Var::from_u32(1);
    pub const BOTTOM: Var = Var::from_u32(0);

    pub fn new(universe: &IndexVec<Var, Value>) -> Self {
        let mut constraint_graph = petgraph::graph::Graph::new();
        constraint_graph.reserve_nodes(universe.len());
        constraint_graph.reserve_edges(universe.len() * 3);
        for _ in 0..universe.len() {
            constraint_graph.add_node(());
        }
        let mut system = Self {
            constraint_graph,
            solved: once_cell::unsync::OnceCell::new(),
            _lattice: std::marker::PhantomData,
        };
        system.guard(Self::TOP, Self::BOTTOM);
        for var in universe.indices() {
            system.guard(Self::TOP, var);
            system.guard(var, Self::BOTTOM);
        }
        system
    }

    pub fn solve(&self) -> &VecVec<Var> {
        self.solved.get_or_init(|| {
            let mut tarjan_scc = TarjanScc::new();
            let mut vars = VecVec::with_capacity(1, self.constraint_graph.node_count());
            tarjan_scc.run(&self.constraint_graph, |nodes| {
                vars.push_vec(nodes.iter().map(|node| Var::new(node.index())))
            });
            vars.done()
        })
    }
}

impl<Value: BooleanLattice> BooleanSystem<Value> {
    pub fn greatest_model(&self, model: &mut IndexVec<Var, Value>) {
        model.raw.fill(true.into());
        let sccs = self.solve();
        let bottom_valued_vars = &sccs[0];
        for &var in bottom_valued_vars {
            model[var] = false.into()
        }
        assert_eq!(Into::into(model[Self::BOTTOM]), false);
    }
}

impl<L: BooleanLattice> ConstraintSystem for BooleanSystem<L> {
    type Domain = L;

    fn top(&mut self, var: Var) {
        self.guard(var, Self::TOP);
    }

    fn bottom(&mut self, var: Var) {
        self.guard(Self::BOTTOM, var);
    }

    #[inline]
    fn guard(&mut self, guard: Var, guarded: Var) {
        tracing::debug!("emit constraint {:?} ⊒ {:?}", guard, guarded);
        self.constraint_graph.add_edge(
            NodeIndex::new(guard.index()),
            NodeIndex::new(guarded.index()),
            (),
        );
    }
}
