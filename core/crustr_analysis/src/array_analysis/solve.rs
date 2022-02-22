use rustc_index::vec::IndexVec;

use crate::array_analysis::{Constraint, Lambda};
use graph::implementation::forward_star::Graph;
use rustc_data_structures::graph::scc::Sccs;

macro_rules! array_index {
    ($row: expr, $col: expr; $len: expr) => {
        $row * $len + $col
    };
}

pub fn solve(
    mut assumptions: IndexVec<Lambda, Option<bool>>,
    equalities: Vec<Vec<Lambda>>,
    constraints: &[Constraint],
) -> IndexVec<Lambda, Option<bool>> {
    // nodes: [λ_0, λ_1, ..., 0, 1]
    let num_nodes = assumptions.len() + 2;
    let zero_idx = num_nodes - 2;
    let one_idx = num_nodes - 1;
    let mut relation = vec![false; num_nodes * num_nodes].into_boxed_slice();
    for idx in 0..num_nodes {
        relation[idx * idx] = true;
    }
    constraints.iter().for_each(|c| {
        relation[array_index!(c.0.index(), c.1.index(); num_nodes)] = true;
        // relation[c.0.index() * num_nodes + c.1.index()] = true;
    });

    for equality in equalities {
        assert!(equality.len() >= 2);
        let (&head, tail) = equality.split_first().unwrap();
        for &other in tail {
            // disjoint_sets.union(head.index(), other.index());
            relation[array_index!(head.index(), other.index(); num_nodes)] = true;
            //relation[head.index() * num_nodes + other.index()] = true;
            relation[array_index!(other.index(), head.index(); num_nodes)] = true;
            //relation[other.index() * num_nodes + head.index()] = true;
        }
    }

    for (lambda, &assumption) in assumptions.iter_enumerated() {
        match assumption {
            Some(true) => {
                relation[array_index!(lambda.index(), one_idx; num_nodes)] = true;
                // relation[lambda.index() * num_nodes + one_idx] = true;
                relation[array_index!(one_idx, lambda.index(); num_nodes)] = true;
                // relation[one_idx * num_nodes + lambda.index()] = true;
            }
            Some(false) => {
                relation[array_index!(lambda.index(), zero_idx; num_nodes)] = true;
                // relation[lambda.index() * num_nodes + zero_idx] = true;
                relation[array_index!(zero_idx, lambda.index(); num_nodes)] = true;
                // relation[zero_idx * num_nodes + lambda.index()] = true;
            }
            None => {}
        }
    }

    relation = transitive_closure(relation, num_nodes);

    let constraint_graph = Graph::<usize, usize>::new(
        num_nodes,
        relation.iter().enumerate().filter_map(|(idx, &related)| {
            related.then(|| {
                let row = idx / num_nodes;
                let col = idx % num_nodes;
                (row, col)
            })
        }),
    );
    let sccs = Sccs::<usize, usize>::new(&constraint_graph);

    // if 0 and 1 are unified
    if sccs.scc(one_idx) == sccs.scc(zero_idx) {
        panic!("Constraints are not satisfied!")
    }

    for (lambda, assumption) in assumptions.iter_enumerated_mut() {
        if sccs.scc(lambda.index()) == sccs.scc(one_idx) {
            *assumption = Some(true);
        } else if sccs.scc(lambda.index()) == sccs.scc(zero_idx) {
            *assumption = Some(false)
        }
    }

    assumptions
}

/// calculate the transitive closure of the constraint graph using Floyd-Warshall algorithm
pub fn transitive_closure(mut facts: Box<[bool]>, len: usize) -> Box<[bool]> {
    for k in 0..len {
        for i in 0..len {
            let i_base = i * len;
            let k_base = k * len;
            // Use assertions to help compiler with bound check elimination. Not sure if it works
            assert!(i_base + len <= facts.len() && k_base + len <= facts.len());
            // std::intrinsics::assume(i_base+len <= facts.len() && k_base+len <= facts.len());
            let facts_i_k = facts[i_base + k];
            for j in 0..len {
                facts[i_base + j] = facts[i_base + j] || (facts_i_k && facts[k_base + j])
            }
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::*;


    #[rustfmt::skip]
    #[test]
    fn test_floyd_warshall() {
        let facts = Box::new(
                 [true , true , true , false,
                  false, true , true , false,
                  true , false, true , true ,
                  false, false, false, true])
        ;
        let res = transitive_closure(facts, 4);
        assert_eq!(*res, [true , true , true , true,
                          true , true , true , true ,
                          true , true , true , true ,
                          false, false, false, true])
    }
}
