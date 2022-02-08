use rustc_index::vec::{Idx, IndexVec};

use rustc_data_structures::graph::{DirectedGraph, GraphSuccessors, WithNumEdges, WithNumNodes, WithSuccessors};

pub struct Graph {
    pub nodes: IndexVec<NodeIndex, Node>,
    pub edges: IndexVec<EdgeIndex, Edge>,
}

pub struct Node {
    pub first_edges: [EdgeIndex; 2],
}

rustc_index::newtype_index! {
    pub struct NodeIndex {
        DEBUG_FORMAT = "v_({})"
    }
}

pub struct Edge {
    pub next_edges: [EdgeIndex; 2],
    pub source: NodeIndex,
    pub target: NodeIndex,
}

rustc_index::newtype_index! {
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

/// This is to be compatible with rustc_index
pub const INVALID_EDGE_INDEX: u32 = 0xFFFF_FF00;

impl Graph {
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

    #[inline]
    pub fn len_nodes(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn next_node_index(&self) -> NodeIndex {
        NodeIndex::new(self.nodes.len())
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let idx = self.next_node_index();
        let node = Node {
            first_edges: [INVALID_EDGE_INDEX.into(); 2],
        };
        self.nodes.push(node);
        idx
    }

    pub fn next_edge_index(&self) -> EdgeIndex {
        EdgeIndex::new(self.edges.len())
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let idx = self.next_edge_index();
        let next_out = self.nodes[source].first_edges[Direction::Outgoing.index()];
        let next_in = self.nodes[target].first_edges[Direction::Incoming.index()];
        let edge = Edge {
            next_edges: [next_out, next_in],
            source,
            target,
        };
        self.nodes[source].first_edges[Direction::Outgoing.index()] = idx;
        self.nodes[target].first_edges[Direction::Incoming.index()] = idx;
        self.edges.push(edge);
        idx
    }

    pub fn outgoing_edges<'a>(&'a self, source: NodeIndex) -> AdjacentEdges<'a> {
        self.adjacent_edges(source, Direction::Outgoing)
    }

    pub fn incoming_edges<'a>(&'a self, source: NodeIndex) -> AdjacentEdges<'a> {
        self.adjacent_edges(source, Direction::Incoming)
    }

    pub fn adjacent_edges<'a>(
        &'a self,
        source: NodeIndex,
        direction: Direction,
    ) -> AdjacentEdges<'a> {
        AdjacentEdges {
            graph: self,
            direction,
            next_edge: self.nodes[source].first_edges[direction.index()],
        }
    }

    pub fn successor_nodes<'a>(&'a self, source: NodeIndex) -> SuccessorNodes<'a> {
        self.outgoing_edges(source).targets()
    }

    pub fn predecessor_nodes<'a>(&'a self, source: NodeIndex) -> PredecessorNodes<'a> {
        self.incoming_edges(source).sources()
    }

    pub fn depth_traverse<'a>(
        &'a self,
        start: NodeIndex,
        direction: Direction,
    ) -> DepthFirstTraversal<'a> {
        DepthFirstTraversal::with_start_node(self, start, direction)
    }
}

// # Iterators

pub struct SuccessorNodes<'g>(AdjacentEdges<'g>);

impl<'g> Iterator for SuccessorNodes<'g> {
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

pub struct PredecessorNodes<'g>(AdjacentEdges<'g>);

impl<'g> Iterator for PredecessorNodes<'g> {
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
pub struct AdjacentEdges<'g> {
    graph: &'g Graph,
    direction: Direction,
    next_edge: EdgeIndex,
}

impl<'g> AdjacentEdges<'g> {
    pub fn sources(self) -> PredecessorNodes<'g> {
        // self.map(|(_, e)| e.source)
        PredecessorNodes(self)
    }

    pub fn targets(self) -> SuccessorNodes<'g> {
        // self.map(|(_, e)| e.target)
        SuccessorNodes(self)
    }
}

impl<'g> Iterator for AdjacentEdges<'g> {
    type Item = (EdgeIndex, &'g Edge);

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
pub struct DepthFirstTraversal<'g> {
    graph: &'g Graph,
    direction: Direction,
    visited: Vec<bool>,
    stack: Vec<NodeIndex>,
}

impl<'g> DepthFirstTraversal<'g> {
    pub fn with_start_node(graph: &'g Graph, start: NodeIndex, direction: Direction) -> Self {
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

impl<'g> Iterator for DepthFirstTraversal<'g> {
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

impl Edge {
    pub fn target_of_direction(&self, direction: Direction) -> NodeIndex {
        match direction {
            Direction::Outgoing => self.target,
            Direction::Incoming => self.source,
        }
    }
}

impl DirectedGraph for Graph {
    type Node = NodeIndex;
}

impl WithNumNodes for Graph {
    fn num_nodes(&self) -> usize {
        self.len_nodes()
    }
}

impl WithNumEdges for Graph {
    fn num_edges(&self) -> usize {
        self.len_edges()
    }
}

impl WithSuccessors for Graph {
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter {
        self.successor_nodes(node)
    }
}

impl<'graph> GraphSuccessors<'graph> for Graph {
    type Item = NodeIndex;
    type Iter = SuccessorNodes<'graph>;
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

    #[test]
    #[should_panic]
    fn test_graph_nodes() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let node_num: usize = rng.gen_range(0..100);
        let mut g = Graph::new();
        for _ in 0..node_num {
            g.add_node();
        }
        let _ = &g.nodes[NodeIndex::new(node_num)];
    }

    #[test]
    fn test_depth_first_search() {
        let mut g = Graph::new();

        for _ in 0..6 {
            g.add_node();
        }

        g.add_edge(NodeIndex::new(0), NodeIndex::new(1));
        g.add_edge(NodeIndex::new(0), NodeIndex::new(5));
        g.add_edge(NodeIndex::new(1), NodeIndex::new(2));
        g.add_edge(NodeIndex::new(5), NodeIndex::new(2));
        g.add_edge(NodeIndex::new(2), NodeIndex::new(3));
        g.add_edge(NodeIndex::new(2), NodeIndex::new(4));

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
            g.depth_traverse(NodeIndex::new(0), Direction::Outgoing)
                .collect::<Vec<_>>(),
            [0, 1, 2, 3, 4, 5].map(|i| NodeIndex::new(i))
        );
    }
}
