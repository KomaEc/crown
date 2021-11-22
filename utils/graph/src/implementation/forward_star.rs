use std::ops::{AddAssign, Range};

pub struct Graph<N, E> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<E>>,
}

pub struct Node<N> {
    pub first_edges: [EdgeIndex; 2],
    pub data: N,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeIndex(pub usize);

pub struct Edge<E> {
    pub next_edges: [EdgeIndex; 2],
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub data: E,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeIndex(pub usize);

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

pub const INVALID_EDGE_INDEX: EdgeIndex = EdgeIndex(usize::MAX);

impl<N, E> Graph<N, E> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(nodes),
            edges: Vec::with_capacity(edges),
        }
    }

    #[inline]
    pub fn all_nodes(&self) -> &[Node<N>] {
        &self.nodes
    }

    #[inline]
    pub fn len_nodes(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn all_edges(&self) -> &[Edge<E>] {
        &self.edges
    }

    #[inline]
    pub fn len_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn next_node_index(&self) -> NodeIndex {
        NodeIndex(self.nodes.len())
    }

    pub fn add_node(&mut self, node_data: N) -> NodeIndex {
        let idx = self.next_node_index();
        let node = Node {
            first_edges: [INVALID_EDGE_INDEX; 2],
            data: node_data,
        };
        self.nodes.push(node);
        idx
    }

    pub fn node_data_mut(&mut self, idx: NodeIndex) -> &mut N {
        &mut self.nodes[idx.0].data
    }

    pub fn node_data(&mut self, idx: NodeIndex) -> &N {
        &self.nodes[idx.0].data
    }

    pub fn node(&self, idx: NodeIndex) -> &Node<N> {
        &self.nodes[idx.0]
    }

    pub fn next_edge_index(&self) -> EdgeIndex {
        EdgeIndex(self.edges.len())
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, edge_data: E) -> EdgeIndex {
        let idx = self.next_edge_index();
        let next_out = self.nodes[source.0].first_edges[Direction::Outgoing.index()];
        let next_in = self.nodes[target.0].first_edges[Direction::Incoming.index()];
        let edge = Edge {
            next_edges: [next_out, next_in],
            source,
            target,
            data: edge_data,
        };
        self.nodes[source.0].first_edges[Direction::Outgoing.index()] = idx;
        self.nodes[target.0].first_edges[Direction::Incoming.index()] = idx;
        self.edges.push(edge);
        idx
    }

    pub fn edge(&self, idx: EdgeIndex) -> &Edge<E> {
        &self.edges[idx.0]
    }

    pub fn nodes_enumerated(&self) -> impl Iterator<Item = (NodeIndex, &Node<N>)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(idx, n)| (NodeIndex(idx), n))
    }

    pub fn edges_enumerated(&self) -> impl Iterator<Item = (EdgeIndex, &Edge<E>)> {
        self.edges
            .iter()
            .enumerate()
            .map(|(idx, n)| (EdgeIndex(idx), n))
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
            next_edge: self.nodes[source.0].first_edges[direction.index()],
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

    pub fn compute_scc(&self) {
        SCCsConstruction::construct(self)
    }
}

// # Iterators

/// Iterator design pattern: laziness
pub struct AdjacentEdges<'g, N, E> {
    graph: &'g Graph<N, E>,
    direction: Direction,
    next_edge: EdgeIndex,
}

impl<'g, N, E> AdjacentEdges<'g, N, E> {
    pub fn sources(self) -> impl Iterator<Item = NodeIndex> + 'g {
        self.map(|(_, e)| e.source)
    }

    pub fn targets(self) -> impl Iterator<Item = NodeIndex> + 'g {
        self.map(|(_, e)| e.target)
    }
}

impl<'g, N, E> Iterator for AdjacentEdges<'g, N, E> {
    type Item = (EdgeIndex, &'g Edge<E>);

    fn next(&mut self) -> Option<Self::Item> {
        let edge_idx = self.next_edge;
        if edge_idx == INVALID_EDGE_INDEX {
            return None;
        }
        let edge = &self.graph.edges[edge_idx.0];
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
        visited[start.0] = true;
        DepthFirstTraversal {
            graph,
            direction,
            visited,
            stack: vec![start],
        }
    }

    pub fn visit(&mut self, node: NodeIndex) {
        if !self.visited[node.0] {
            self.visited[node.0] = true;
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

/*

/// Visiting order: LIFO

pub struct DepthFirstTraversal<'g, N, E> {
    graph: &'g Graph<N, E>,
    direction: Direction,
    visited: Vec<bool>,
    next_node: Option<NodeIndex>,
    stack: Vec<AdjacentEdges<'g, N, E>>
}

impl<'g, N, E> DepthFirstTraversal<'g, N, E> {
    pub fn with_start_node(
        graph: &'g Graph<N, E>,
        start: NodeIndex,
        direction: Direction
    ) -> Self {
        let mut visited = vec![false; graph.len_nodes()];
        visited[start.0] = true;
        DepthFirstTraversal {
            graph,
            direction,
            visited,
            next_node: Some(start),
            stack: vec![graph.adjacent_edges(start, direction)],
        }
    }
}

impl<'g, N, E> Iterator for DepthFirstTraversal<'g, N, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {

        self.next_node.map(
            |next_node| {
                self.next_node = None;
                while let Some(adjacent_edges) = self.stack.last_mut() {
                    match adjacent_edges.next() {
                        None => {
                            self.stack.pop();
                        },
                        Some((_, next_edge)) => {
                            let next_node = next_edge.target_of_direction(self.direction);

                            if !self.visited[next_node.0] {
                                self.visited[next_node.0] = true;
                                self.next_node = Some(next_node);
                                self.stack.push(self.graph.adjacent_edges(next_node, self.direction));
                                break
                            }
                        }
                    }
                }
                next_node
            }
        )
    }
}

*/

impl<E> Edge<E> {
    pub fn target_of_direction(&self, direction: Direction) -> NodeIndex {
        match direction {
            Direction::Outgoing => self.target,
            Direction::Incoming => self.source,
        }
    }
}

// # SCC

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCCIndex(pub usize);

pub struct SCCs {
    scc_indices: Vec<SCCIndex>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Time(usize);

impl AddAssign<usize> for Time {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

pub struct SCCsConstruction<'g, N, E> {
    graph: &'g Graph<N, E>,
    low_link: Vec<Time>,
    cur_time: Time,
    discovery_time: Vec<Time>,
    stack: Vec<NodeIndex>,
    visited: Vec<bool>,
    on_stack: Vec<bool>,
}

impl<'g, N, E> SCCsConstruction<'g, N, E> {
    fn new(graph: &'g Graph<N, E>) -> Self {
        let num_nodes = graph.len_nodes();
        SCCsConstruction {
            graph,
            low_link: vec![Time(usize::MAX); num_nodes],
            cur_time: Time(0),
            discovery_time: vec![Time(usize::MAX); num_nodes],
            stack: Vec::with_capacity(num_nodes),
            visited: vec![false; num_nodes],
            on_stack: vec![false; num_nodes],
        }
    }

    /// Assuming `node` is not currently visited
    fn walk_vertex(&mut self, v: NodeIndex) {
        self.visited[v.0] = true;
        self.discovery_time[v.0] = self.cur_time;
        self.low_link[v.0] = self.cur_time;
        self.cur_time += 1;
        self.stack.push(v);
        self.on_stack[v.0] = true;

        for u in self.graph.successor_nodes(v) {
            if !self.visited[u.0] {
                self.walk_vertex(u);
                // self.stack
                self.low_link[v.0] = std::cmp::min(self.low_link[v.0], self.low_link[u.0]);
            } else if self.on_stack[u.0] {
                // it's a back edge
                // it's a back edge
                self.low_link[v.0] = std::cmp::min(self.low_link[v.0], self.discovery_time[u.0]);
            }
        }

        if self.discovery_time[v.0] == self.low_link[v.0] {
            // SCC found
            while let Some(u) = self.stack.pop() {
                print!("{} ", u.0);

                self.on_stack[u.0] = false;
                if u == v {
                    break;
                }
            }
            println!()
        }
    }

    fn run(&mut self) {
        for i in 0..self.graph.len_nodes() {
            let v = NodeIndex(i);
            if !self.visited[i] {
                self.walk_vertex(v);
            }
        }
    }

    pub fn construct(graph: &'g Graph<N, E>) {
        SCCsConstruction::new(graph).run()
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
        println!("SCCs in graph one:");
        g1.compute_scc();

        let mut g2: Graph<(), ()> = Graph::new();
        for _ in 0..4 {
            g2.add_node(());
        }
        g2.add_edge(NodeIndex(0), NodeIndex(1), ());
        g2.add_edge(NodeIndex(1), NodeIndex(2), ());
        g2.add_edge(NodeIndex(2), NodeIndex(3), ());
        println!("SCCs in graph two:");
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
        println!("SCCs in graph three:");
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
        println!("SCCs in graph four:");
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
        println!("SCCs in graph five:");
        g5.compute_scc();
    }
}
