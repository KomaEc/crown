use index::vec::{Idx, IndexVec};

use crate::{DirectedGraph, GraphSuccessors, WithNumEdges, WithNumNodes, WithSuccessors};

pub struct Graph<N, E> {
    pub nodes: IndexVec<NodeIndex, Node<N>>,
    pub edges: IndexVec<EdgeIndex, Edge<E>>,
}

pub struct Node<N> {
    pub first_edges: [EdgeIndex; 2],
    pub data: N,
}

index::newtype_index! {
    pub struct NodeIndex {
        DEBUG_FORMAT = "v_({})"
    }
}

pub struct Edge<E> {
    pub next_edges: [EdgeIndex; 2],
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub data: E,
}

index::newtype_index! {
    pub struct EdgeIndex {
        DEBUG_FORMAT = "v_({})"
    }
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

    pub fn index(self) -> usize {
        match self {
            Direction::Outgoing => 0,
            Direction::Incoming => 1,
        }
    }
}

pub const INVALID_EDGE_INDEX: u32 = u32::MAX;

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: IndexVec::new(),
            edges: IndexVec::new(),
        }
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: IndexVec::with_capacity(nodes),
            edges: IndexVec::with_capacity(edges),
        }
    }

    /*
    #[inline]
    pub fn all_nodes(&self) -> &[Node<N>] {
        &self.nodes
    }
    */

    #[inline]
    pub fn len_nodes(&self) -> usize {
        self.nodes.len()
    }

    /*
    #[inline]
    pub fn all_edges(&self) -> &[Edge<E>] {
        &self.edges
    }
    */

    #[inline]
    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn next_node_index(&self) -> NodeIndex {
        NodeIndex::new(self.nodes.len())
    }

    pub fn add_node(&mut self, node_data: N) -> NodeIndex {
        let idx = self.next_node_index();
        let node = Node {
            first_edges: [INVALID_EDGE_INDEX.into(); 2],
            data: node_data,
        };
        self.nodes.push(node);
        idx
    }

    pub fn node_data_mut(&mut self, idx: NodeIndex) -> &mut N {
        &mut self.nodes[idx].data
    }

    pub fn node_data(&mut self, idx: NodeIndex) -> &N {
        &self.nodes[idx].data
    }

    pub fn node(&self, idx: NodeIndex) -> &Node<N> {
        &self.nodes[idx]
    }

    pub fn node_indices(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.nodes_enumerated().map(|(node_id, __)| node_id)
    }

    pub fn next_edge_index(&self) -> EdgeIndex {
        EdgeIndex::new(self.edges.len())
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, edge_data: E) -> EdgeIndex {
        let idx = self.next_edge_index();
        let next_out = self.nodes[source].first_edges[Direction::Outgoing.index()];
        let next_in = self.nodes[target].first_edges[Direction::Incoming.index()];
        let edge = Edge {
            next_edges: [next_out, next_in],
            source,
            target,
            data: edge_data,
        };
        self.nodes[source].first_edges[Direction::Outgoing.index()] = idx;
        self.nodes[target].first_edges[Direction::Incoming.index()] = idx;
        self.edges.push(edge);
        idx
    }

    pub fn edge(&self, idx: EdgeIndex) -> &Edge<E> {
        &self.edges[idx]
    }

    pub fn nodes_enumerated(&self) -> impl Iterator<Item = (NodeIndex, &Node<N>)> {
        self.nodes.iter_enumerated()
    }

    pub fn edges_enumerated(&self) -> impl Iterator<Item = (EdgeIndex, &Edge<E>)> {
        self.edges.iter_enumerated()
    }

    pub fn each_node<'a>(&'a self, mut f: impl FnMut(NodeIndex, &'a Node<N>) -> bool) -> bool {
        self.nodes_enumerated().all(|(idx, n)| f(idx, n))
    }

    pub fn each_edge<'a>(&'a self, mut f: impl FnMut(EdgeIndex, &'a Edge<E>) -> bool) -> bool {
        self.edges_enumerated().all(|(idx, e)| f(idx, e))
    }

    pub fn outgoing_edges<'a>(&'a self, source: NodeIndex) -> AdjacentEdges<'a, N, E> {
        self.adjacent_edges(source, Direction::Outgoing)
    }

    pub fn incoming_edges<'a>(&'a self, source: NodeIndex) -> AdjacentEdges<'a, N, E> {
        self.adjacent_edges(source, Direction::Incoming)
    }

    pub fn adjacent_edges<'a>(
        &'a self,
        source: NodeIndex,
        direction: Direction,
    ) -> AdjacentEdges<'a, N, E> {
        AdjacentEdges {
            graph: self,
            direction,
            next_edge: self.nodes[source].first_edges[direction.index()],
        }
    }

    pub fn successor_nodes<'a>(
        &'a self,
        source: NodeIndex,
    ) -> impl Iterator<Item = NodeIndex> + 'a {
        self.outgoing_edges(source).targets()
    }

    pub fn predecessor_nodes<'a>(
        &'a self,
        source: NodeIndex,
    ) -> impl Iterator<Item = NodeIndex> + 'a {
        self.incoming_edges(source).sources()
    }

    pub fn depth_traverse<'a>(
        &'a self,
        start: NodeIndex,
        direction: Direction,
    ) -> DepthFirstTraversal<'a, N, E> {
        DepthFirstTraversal::with_start_node(self, start, direction)
    }
}

// # Iterators

pub struct SuccessorNodes<'g, N, E>(AdjacentEdges<'g, N, E>);

impl<'g, N, E> Iterator for SuccessorNodes<'g, N, E> {
    type Item = NodeIndex;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, e)| e.target)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

pub struct PredecessorNodes<'g, N, E>(AdjacentEdges<'g, N, E>);

impl<'g, N, E> Iterator for PredecessorNodes<'g, N, E> {
    type Item = NodeIndex;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(_, e)| e.source)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Iterator design pattern: laziness
pub struct AdjacentEdges<'g, N, E> {
    graph: &'g Graph<N, E>,
    direction: Direction,
    next_edge: EdgeIndex,
}

impl<'g, N, E> AdjacentEdges<'g, N, E> {
    pub fn sources(self) -> PredecessorNodes<'g, N, E> {
        // self.map(|(_, e)| e.source)
        PredecessorNodes(self)
    }

    pub fn targets(self) -> SuccessorNodes<'g, N, E> {
        // self.map(|(_, e)| e.target)
        SuccessorNodes(self)
    }
}

impl<'g, N, E> Iterator for AdjacentEdges<'g, N, E> {
    type Item = (EdgeIndex, &'g Edge<E>);

    fn next(&mut self) -> Option<Self::Item> {
        let edge_idx = self.next_edge;
        if edge_idx == INVALID_EDGE_INDEX.into() {
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
pub struct DepthFirstTraversal<'g, N, E> {
    graph: &'g Graph<N, E>,
    direction: Direction,
    visited: Vec<bool>,
    stack: Vec<NodeIndex>,
}

impl<'g, N, E> DepthFirstTraversal<'g, N, E> {
    pub fn with_start_node(graph: &'g Graph<N, E>, start: NodeIndex, direction: Direction) -> Self {
        let mut visited = vec![false; graph.len_nodes()];
        visited[start.index()] = true;
        DepthFirstTraversal {
            graph,
            direction,
            visited,
            stack: vec![start],
        }
    }

    pub fn visit(&mut self, node: NodeIndex) {
        if !self.visited[node.index()] {
            self.visited[node.index()] = true;
            self.stack.push(node)
        }
    }
}

impl<'g, N, E> Iterator for DepthFirstTraversal<'g, N, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            for (_, edge) in self.graph.adjacent_edges(node, self.direction) {
                self.visit(edge.target_of_direction(self.direction))
            }
            node
        })
    }
}

impl<E> Edge<E> {
    pub fn target_of_direction(&self, direction: Direction) -> NodeIndex {
        match direction {
            Direction::Outgoing => self.target,
            Direction::Incoming => self.source,
        }
    }
}

/*
impl<N, E> DirectedGraph for Graph<N, E> {
    type Node = NodeIndex;
}

impl<N, E> WithNumNodes for Graph<N, E> {
    fn num_nodes(&self) -> usize {
        self.len_nodes()
    }
}

impl<N, E> WithNumEdges for Graph<N, E> {
    fn num_edges(&self) -> usize {
        self.len_edges()
    }
}

impl<N, E> WithSuccessors for Graph<N, E> {
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter {
        todo!()
    }
}

impl<'graph, N, E> GraphSuccessors<'graph> for Graph<N, E> {
    type Item = NodeIndex;
    type Iter = SuccessorNodes<'graph, N, E>;
}
*/

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
            res[s.0].push(NodeIndex(i))
        }
        res
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time(usize);

const INVALID_TIME: Time = Time(usize::MAX);

impl AddAssign<usize> for Time {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

pub struct TarjanSCC<'g, N, E> {
    graph: &'g Graph<N, E>,
    low_link: Vec<Time>,
    timestamp: Time,
    discovery_time: Vec<Time>,
    stack: Vec<NodeIndex>,
    // visited: Vec<bool>,
    on_stack: Vec<bool>,

    component_cnt: usize,
    scc_indices: Vec<SCCIndex>,
}

impl<'g, N, E> TarjanSCC<'g, N, E> {
    fn new(graph: &'g Graph<N, E>) -> Self {
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
            let v = NodeIndex(i);
            if self.discovery_time[i] == INVALID_TIME {
                self.dfs_vertex(v);
            }
        }
    }

    pub fn construct(graph: &'g Graph<N, E>) -> SCCs {
        let mut tarjan_scc = TarjanSCC::new(graph);
        tarjan_scc.run();
        SCCs {
            scc_indices: tarjan_scc.scc_indices,
            num_component: tarjan_scc.component_cnt,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_depth_first_search() {
        let mut g = Graph::<usize, ()>::new();

        for i in [0, 1, 2, 3, 4, 5, 6] {
            g.add_node(i);
        }
        g.add_edge(NodeIndex(1), NodeIndex(2), ());
        g.add_edge(NodeIndex(1), NodeIndex(6), ());
        g.add_edge(NodeIndex(2), NodeIndex(3), ());
        g.add_edge(NodeIndex(6), NodeIndex(3), ());
        g.add_edge(NodeIndex(3), NodeIndex(4), ());
        g.add_edge(NodeIndex(3), NodeIndex(5), ());

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
            g.depth_traverse(NodeIndex(1), Direction::Outgoing)
                .collect::<Vec<_>>(),
            [1, 2, 3, 4, 5, 6].map(|i| NodeIndex(i))
        );
    }

    #[test]
    fn test_scc() {
        let mut g1: Graph<(), ()> = Graph::new();
        for _ in 0..5 {
            g1.add_node(());
        }
        g1.add_edge(NodeIndex(1), NodeIndex(0), ());
        g1.add_edge(NodeIndex(0), NodeIndex(2), ());
        g1.add_edge(NodeIndex(2), NodeIndex(1), ());
        g1.add_edge(NodeIndex(0), NodeIndex(3), ());
        g1.add_edge(NodeIndex(3), NodeIndex(4), ());
        let g1_result = g1.compute_scc().components();
        assert_eq!(
            g1_result,
            [vec![4], vec![3], vec![0, 1, 2]]
                .map(|v| v.into_iter().map(|i| i.into()).collect::<Vec<_>>())
        );

        /*
        let mut g2: Graph<(), ()> = Graph::new();
        for _ in 0..4 {
            g2.add_node(());
        }
        g2.add_edge(NodeIndex(0), NodeIndex(1), ());
        g2.add_edge(NodeIndex(1), NodeIndex(2), ());
        g2.add_edge(NodeIndex(2), NodeIndex(3), ());
        g2.compute_scc();

        let mut g3: Graph<(), ()> = Graph::new();
        for _ in 0..7 {
            g3.add_node(());
        }
        g3.add_edge(NodeIndex(0), NodeIndex(1), ());
        g3.add_edge(NodeIndex(1), NodeIndex(2), ());
        g3.add_edge(NodeIndex(2), NodeIndex(0), ());
        g3.add_edge(NodeIndex(1), NodeIndex(3), ());
        g3.add_edge(NodeIndex(1), NodeIndex(4), ());
        g3.add_edge(NodeIndex(1), NodeIndex(6), ());
        g3.add_edge(NodeIndex(3), NodeIndex(5), ());
        g3.add_edge(NodeIndex(4), NodeIndex(5), ());
        g3.compute_scc();

        let mut g4: Graph<(), ()> = Graph::new();
        for _ in 0..11 {
            g4.add_node(());
        }
        g4.add_edge(NodeIndex(0), NodeIndex(1), ());
        g4.add_edge(NodeIndex(0), NodeIndex(3), ());
        g4.add_edge(NodeIndex(1), NodeIndex(2), ());
        g4.add_edge(NodeIndex(1), NodeIndex(4), ());
        g4.add_edge(NodeIndex(2), NodeIndex(0), ());
        g4.add_edge(NodeIndex(2), NodeIndex(6), ());
        g4.add_edge(NodeIndex(3), NodeIndex(2), ());
        g4.add_edge(NodeIndex(4), NodeIndex(5), ());
        g4.add_edge(NodeIndex(4), NodeIndex(6), ());
        g4.add_edge(NodeIndex(5), NodeIndex(6), ());
        g4.add_edge(NodeIndex(5), NodeIndex(7), ());
        g4.add_edge(NodeIndex(5), NodeIndex(8), ());
        g4.add_edge(NodeIndex(5), NodeIndex(9), ());
        g4.add_edge(NodeIndex(6), NodeIndex(4), ());
        g4.add_edge(NodeIndex(7), NodeIndex(9), ());
        g4.add_edge(NodeIndex(8), NodeIndex(9), ());
        g4.add_edge(NodeIndex(9), NodeIndex(8), ());
        g4.compute_scc();

        let mut g5: Graph<(), ()> = Graph::new();
        for _ in 0..5 {
            g5.add_node(());
        }
        g5.add_edge(NodeIndex(0), NodeIndex(1), ());
        g5.add_edge(NodeIndex(1), NodeIndex(2), ());
        g5.add_edge(NodeIndex(2), NodeIndex(3), ());
        g5.add_edge(NodeIndex(2), NodeIndex(4), ());
        g5.add_edge(NodeIndex(3), NodeIndex(0), ());
        g5.add_edge(NodeIndex(4), NodeIndex(2), ());
        g5.compute_scc();
        */
    }
}
*/
