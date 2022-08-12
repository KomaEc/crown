use rustc_index::vec::{Idx, IndexVec};

use rustc_data_structures::graph::{
    DirectedGraph, GraphPredecessors, GraphSuccessors, WithNumEdges, WithNumNodes,
    WithPredecessors, WithSuccessors,
};

#[derive(Clone)]
pub struct Graph<Node: Idx, Edge: Idx> {
    pub nodes: IndexVec<Node, NodeData<Edge>>,
    pub edges: IndexVec<Edge, EdgeData<Node, Edge>>,
}

#[derive(Clone)]
pub struct NodeData<Edge: Idx> {
    pub first_edges: [Edge; 2],
}

#[derive(Clone)]
pub struct EdgeData<Node: Idx, Edge: Idx> {
    pub next_edges: [Edge; 2],
    pub source: Node,
    pub target: Node,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Outgoing,
    Incoming,
}

impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }

    #[inline(always)]
    pub fn index(self) -> usize {
        self as usize
    }
}

/// This is to be compatible with rustc_index
pub const INVALID_EDGE_INDEX: u32 = 0xFFFF_FF00;

impl<Node: Idx, Edge: Idx> Graph<Node, Edge> {
    pub fn new(num_nodes: usize, edge_pairs: impl Iterator<Item = (Node, Node)>) -> Self {
        let nodes = IndexVec::from_elem_n(
            NodeData {
                first_edges: [Edge::new(INVALID_EDGE_INDEX as usize); 2],
            },
            num_nodes,
        );
        let mut g = Graph {
            nodes,
            edges: IndexVec::new(),
        };
        for (src, tgt) in edge_pairs {
            g.add_edge(src, tgt);
        }
        g
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: IndexVec::with_capacity(nodes),
            edges: IndexVec::with_capacity(edges),
        }
    }

    #[inline]
    pub fn len_nodes(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn next_node_index(&self) -> Node {
        Node::new(self.nodes.len())
    }

    pub fn add_node(&mut self) -> Node {
        let idx = self.next_node_index();
        let node = NodeData {
            first_edges: [Edge::new(INVALID_EDGE_INDEX as usize); 2],
        };
        self.nodes.push(node);
        idx
    }

    pub fn next_edge_index(&self) -> Edge {
        Edge::new(self.edges.len())
    }

    pub fn add_edge(&mut self, source: Node, target: Node) -> Edge {
        let idx = self.next_edge_index();
        let next_out = self.nodes[source].first_edges[Direction::Outgoing.index()];
        let next_in = self.nodes[target].first_edges[Direction::Incoming.index()];
        let edge = EdgeData {
            next_edges: [next_out, next_in],
            source,
            target,
        };
        self.nodes[source].first_edges[Direction::Outgoing.index()] = idx;
        self.nodes[target].first_edges[Direction::Incoming.index()] = idx;
        self.edges.push(edge);
        idx
    }

    pub fn add_edge_without_dup(&mut self, source: Node, target: Node) -> Option<Edge> {
        if self.successor_nodes(source).any(|tgt| tgt == target) {
            None
        } else {
            Some(self.add_edge(source, target))
        }
    }

    pub fn nodes<'a>(&'a self) -> impl Iterator<Item = Node> + 'a {
        self.nodes.indices()
    }

    pub fn outgoing_edges<'a>(&'a self, source: Node) -> AdjacentEdges<'a, Node, Edge> {
        self.adjacent_edges(source, Direction::Outgoing)
    }

    pub fn incoming_edges<'a>(&'a self, source: Node) -> AdjacentEdges<'a, Node, Edge> {
        self.adjacent_edges(source, Direction::Incoming)
    }

    pub fn adjacent_edges<'a>(
        &'a self,
        source: Node,
        direction: Direction,
    ) -> AdjacentEdges<'a, Node, Edge> {
        AdjacentEdges {
            graph: self,
            direction,
            next_edge: self.nodes[source].first_edges[direction.index()],
        }
    }

    pub fn successor_nodes<'a>(&'a self, source: Node) -> SuccessorNodes<'a, Node, Edge> {
        self.outgoing_edges(source).targets()
    }

    pub fn predecessor_nodes<'a>(&'a self, source: Node) -> PredecessorNodes<'a, Node, Edge> {
        self.incoming_edges(source).sources()
    }

    pub fn depth_traverse<'a>(
        &'a self,
        start: Node,
        direction: Direction,
    ) -> DepthFirstTraversal<'a, Node, Edge> {
        DepthFirstTraversal::with_start_node(self, start, direction)
    }

    /*
    pub fn from_another_graph_rep<G>(g: G) -> Self
    where
        G: DirectedGraph<Node = Node> + WithNumNodes + WithSuccessors,
    {
        Self::new(
            g.num_nodes(),
            (0..g.num_nodes())
                .map(G::Node::new)
                .flat_map(|src| g.successors(src).map(move |tgt| (src, tgt))),
        )
    }
    */
}

// # Iterators

pub struct SuccessorNodes<'g, Node: Idx, Edge: Idx>(AdjacentEdges<'g, Node, Edge>);

impl<'g, Node: Idx, Edge: Idx> Iterator for SuccessorNodes<'g, Node, Edge> {
    type Item = Node;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, e)| e.target)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub struct PredecessorNodes<'g, Node: Idx, Edge: Idx>(AdjacentEdges<'g, Node, Edge>);

impl<'g, Node: Idx, Edge: Idx> Iterator for PredecessorNodes<'g, Node, Edge> {
    type Item = Node;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, e)| e.source)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub struct AdjacentEdges<'g, Node: Idx, Edge: Idx> {
    graph: &'g Graph<Node, Edge>,
    direction: Direction,
    next_edge: Edge,
}

impl<'g, Node: Idx, Edge: Idx> AdjacentEdges<'g, Node, Edge> {
    pub fn sources(self) -> PredecessorNodes<'g, Node, Edge> {
        PredecessorNodes(self)
    }

    pub fn targets(self) -> SuccessorNodes<'g, Node, Edge> {
        SuccessorNodes(self)
    }
}

impl<'g, Node: Idx, Edge: Idx> Iterator for AdjacentEdges<'g, Node, Edge> {
    type Item = (Edge, &'g EdgeData<Node, Edge>);

    fn next(&mut self) -> Option<Self::Item> {
        let edge_idx = self.next_edge;
        if edge_idx == Edge::new(INVALID_EDGE_INDEX as usize) {
            return None;
        }
        let edge = &self.graph.edges[edge_idx];
        self.next_edge = edge.next_edges[self.direction.index()];
        Some((edge_idx, edge))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.graph.len_edges()))
    }
}

/// Visiting order: FIFO
pub struct DepthFirstTraversal<'g, Node: Idx, Edge: Idx> {
    graph: &'g Graph<Node, Edge>,
    direction: Direction,
    visited: Vec<bool>,
    stack: Vec<Node>,
}

impl<'g, Node: Idx, Edge: Idx> DepthFirstTraversal<'g, Node, Edge> {
    pub fn with_start_node(
        graph: &'g Graph<Node, Edge>,
        start: Node,
        direction: Direction,
    ) -> Self {
        let mut visited = vec![false; graph.len_nodes()];
        visited[start.index()] = true;
        DepthFirstTraversal {
            graph,
            direction,
            visited,
            stack: vec![start],
        }
    }

    pub fn visit(&mut self, node: Node) {
        if !self.visited[node.index()] {
            self.visited[node.index()] = true;
            self.stack.push(node)
        }
    }
}

impl<'g, Node: Idx, Edge: Idx> Iterator for DepthFirstTraversal<'g, Node, Edge> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            for (_, edge) in self.graph.adjacent_edges(node, self.direction) {
                self.visit(edge.target_of_direction(self.direction))
            }
            node
        })
    }
}

impl<Node: Idx, Edge: Idx> EdgeData<Node, Edge> {
    pub fn target_of_direction(&self, direction: Direction) -> Node {
        match direction {
            Direction::Outgoing => self.target,
            Direction::Incoming => self.source,
        }
    }
}

impl<Node: Idx, Edge: Idx> DirectedGraph for Graph<Node, Edge> {
    type Node = Node;
}

impl<Node: Idx, Edge: Idx> WithNumNodes for Graph<Node, Edge> {
    fn num_nodes(&self) -> usize {
        self.len_nodes()
    }
}

impl<Node: Idx, Edge: Idx> WithNumEdges for Graph<Node, Edge> {
    fn num_edges(&self) -> usize {
        self.len_edges()
    }
}

impl<Node: Idx, Edge: Idx> WithSuccessors for Graph<Node, Edge> {
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter {
        self.successor_nodes(node)
    }
}

impl<'graph, Node: Idx, Edge: Idx> GraphSuccessors<'graph> for Graph<Node, Edge> {
    type Item = Node;
    type Iter = SuccessorNodes<'graph, Node, Edge>;
}

impl<Node: Idx, Edge: Idx> WithPredecessors for Graph<Node, Edge> {
    fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter {
        self.predecessor_nodes(node)
    }
}

impl<'graph, Node: Idx, Edge: Idx> GraphPredecessors<'graph> for Graph<Node, Edge> {
    type Item = Node;
    type Iter = PredecessorNodes<'graph, Node, Edge>;
}

/*
// # SCC

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCCIndex(pub usize);

const INVALID_SCC_INDEX: SCCIndex = SCCIndex(usize::MAX);

pub struct SCCs {
    scc_indices: Vec<SCCIndex>,
    num_component: usize,
}

impl SCCs {
    pub fn components(&self) -> Vec<Vec<NodeIndex>> {
        let mut res = vec![Vec::new(); self.num_component];
        for (i, &s) in self.scc_indices.iter().enumerate() {
            res[s.0].push(NodeIndex::new(i))
        }
        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time(usize);

const INVALID_TIME: Time = Time(usize::MAX);

impl std::ops::AddAssign<usize> for Time {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

pub struct TarjanSCC<'g> {
    graph: &'g Graph,
    low_link: Vec<Time>,
    timestamp: Time,
    discovery_time: Vec<Time>,
    stack: Vec<NodeIndex>,
    // visited: Vec<bool>,
    on_stack: Vec<bool>,

    component_cnt: usize,
    scc_indices: Vec<SCCIndex>,
}

impl<'g> TarjanSCC<'g> {
    fn new(graph: &'g Graph) -> Self {
        let num_nodes = graph.len_nodes();
        TarjanSCC {
            graph,
            low_link: vec![INVALID_TIME; num_nodes],
            timestamp: Time(0),
            discovery_time: vec![INVALID_TIME; num_nodes],
            stack: Vec::with_capacity(num_nodes),
            // visited: vec![false; num_nodes],
            on_stack: vec![false; num_nodes],
            component_cnt: 0,
            scc_indices: vec![INVALID_SCC_INDEX; num_nodes],
        }
    }

    /// Assuming `node` is not currently visited
    fn dfs_vertex(&mut self, v: NodeIndex) {
        // self.visited[v.0] = true;
        self.discovery_time[v.0] = self.timestamp;
        self.low_link[v.0] = self.timestamp;
        self.timestamp += 1;
        self.stack.push(v);
        self.on_stack[v.0] = true;

        // @Loop invariant:
        // Let V be the set of vertices that have been visited, let V' be the set of vertices that
        // are on stack. Then we have V' ⊂ V.
        // Let E be the set of edges that have been visited (note that, as long as target vertex is
        // touched, we count the corresponding edge as visited),
        // let E' be the set of edges in the current DFS tree. Then we have E' ⊂ E.
        //
        // At anytime, the loop invariant is represented by the following mutually dependent predicates:
        // 1. `low_link[v]` points to a vertex in V' (on stack), which is also the vertex of lowest preorder
        // in the current DFS tree (V, E'), that is reachable from `v` in the graph (V, E) by at most one
        // back edge or cross edge (note that, following tree edge will only increase preorder, thereby
        // the back edge or cross edge must occur at the last).
        // 2. A vertex `u` is on stack, if following `low_link[..low_link[u]]` until converges, getting `v`,
        // and `v` has not done visiting its adjacent nodes.
        for u in self.graph.successor_nodes(v) {
            if self.discovery_time[u.0] == INVALID_TIME {
                self.dfs_vertex(u);
                // self.stack
                self.low_link[v.0] = std::cmp::min(self.low_link[v.0], self.low_link[u.0]);
            } else if self.on_stack[u.0] {
                // it's a back edge or a cross edge
                self.low_link[v.0] = std::cmp::min(self.low_link[v.0], self.discovery_time[u.0]);
            }
        }

        // SCC found
        if self.discovery_time[v.0] == self.low_link[v.0] {
            while let Some(u) = self.stack.pop() {
                self.scc_indices[u.0] = SCCIndex(self.component_cnt);
                self.on_stack[u.0] = false;
                if u == v {
                    break;
                }
            }
            self.component_cnt += 1;
        }
    }

    fn run(&mut self) {
        for i in 0..self.graph.len_nodes() {
            let v = NodeIndex::new(i);
            if self.discovery_time[i] == INVALID_TIME {
                self.dfs_vertex(v);
            }
        }
    }

    pub fn construct(graph: &'g Graph) -> SCCs {
        let mut tarjan_scc = TarjanSCC::new(graph);
        tarjan_scc.run();
        SCCs {
            scc_indices: tarjan_scc.scc_indices,
            num_component: tarjan_scc.component_cnt,
        }
    }
}
*/

#[cfg(test)]
mod tests {

    use super::*;

    orc_common::macros::newtype_index! {
        pub struct Node {
            DEBUG_FORMAT = "{}"
        }
    }

    orc_common::macros::newtype_index! {
        pub struct Edge {
            DEBUG_FORMAT = "{}"
        }
    }

    #[test]
    #[should_panic]
    fn test_graph_nodes() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let node_num: usize = rng.gen_range(0..100);
        let g: Graph<Node, Edge> = Graph::new(node_num, [].into_iter());
        let _ = &g.nodes[Node::new(node_num)];
    }

    #[test]
    fn test_depth_first_search() {
        let mut g: Graph<Node, Edge> = Graph::new(6, [].into_iter());

        g.add_edge(Node::new(0), Node::new(1));
        g.add_edge(Node::new(0), Node::new(5));
        g.add_edge(Node::new(1), Node::new(2));
        g.add_edge(Node::new(5), Node::new(2));
        g.add_edge(Node::new(2), Node::new(3));
        g.add_edge(Node::new(2), Node::new(4));

        /*
                   1
                  / \
                 /   \
                2     6
                 \   /
                  \ /
                   3
                  / \
                 /   \
                4     5
        */

        assert_eq!(
            g.depth_traverse(Node::new(0), Direction::Outgoing)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5].map(|i| Node::new(i))
        );
    }
}
