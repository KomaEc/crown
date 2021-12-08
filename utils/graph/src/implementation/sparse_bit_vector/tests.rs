
use super::*;

fn cmp(x: &(usize, usize), y: &(usize, usize)) -> std::cmp::Ordering {
    if x.0 == y.0 {
        x.1.cmp(&y.1)
    } else {
        x.0.cmp(&y.0)
    }
}

#[test]
fn test_rev1() {
    let mut edges = [(0, 1), (0, 2), (1, 3), (2, 3)];
    let graph: SparseBitVectorGraph<usize> = SparseBitVectorGraph::new(4, edges.clone().into_iter());
    let graph = graph.reverse();
    edges.iter_mut().for_each(|(src, dest)| std::mem::swap(src, dest));
    edges.sort_by(cmp);
    assert_eq!(graph.collect_edges(), edges);
}

#[test]
fn test_rev2() {
    let mut edges = [(0, 1), (1, 2), (1, 3), (2, 0), (3, 2)];
    let graph: SparseBitVectorGraph<usize> = SparseBitVectorGraph::new(4, edges.clone().into_iter());
    let graph = graph.reverse();
    edges.iter_mut().for_each(|(src, dest)| std::mem::swap(src, dest));
    edges.sort_by(cmp);
    assert_eq!(graph.collect_edges(), edges)
}

#[test]
fn test_rev3() {
    let mut edges = [(0, 1), (0, 4), (1, 2), (1, 3), (2, 1), (3, 0), (4, 2)];
    let graph: SparseBitVectorGraph<usize> = SparseBitVectorGraph::new(5, edges.clone().into_iter());
    let graph = graph.reverse();
    edges.iter_mut().for_each(|(src, dest)| std::mem::swap(src, dest));
    edges.sort_by(cmp);
    assert_eq!(graph.collect_edges(), edges)
}