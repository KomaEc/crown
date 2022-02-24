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
) -> Result<IndexVec<Lambda, Option<bool>>, ()> {
    // nodes: [λ_0, λ_1, ..., 0, 1]
    let num_nodes = assumptions.len() + 2;
    let zero_idx = num_nodes - 2;
    let one_idx = num_nodes - 1;
    let mut relation = vec![false; num_nodes * num_nodes].into_boxed_slice();
    for idx in 0..num_nodes {
        relation[array_index!(idx, idx; num_nodes)] = true;
        relation[array_index!(idx, one_idx; num_nodes)] = true;
        relation[array_index!(zero_idx, idx; num_nodes)] = true;
    }
    constraints.iter().for_each(|c| {
        relation[array_index!(c.0.index(), c.1.index(); num_nodes)] = true;
    });

    for equality in equalities {
        assert!(equality.len() >= 2);
        let (&head, tail) = equality.split_first().unwrap();
        for &other in tail {
            relation[array_index!(head.index(), other.index(); num_nodes)] = true;
            relation[array_index!(other.index(), head.index(); num_nodes)] = true;
        }
    }

    for (lambda, &assumption) in assumptions.iter_enumerated() {
        match assumption {
            Some(true) => {
                relation[array_index!(one_idx, lambda.index(); num_nodes)] = true;
            }
            Some(false) => {
                relation[array_index!(lambda.index(), zero_idx; num_nodes)] = true;
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
        return Err(());
        // panic!("Constraints are not satisfied!")
    }

    for (lambda, assumption) in assumptions.iter_enumerated_mut() {
        if sccs.scc(lambda.index()) == sccs.scc(one_idx) {
            *assumption = Some(true);
        } else if sccs.scc(lambda.index()) == sccs.scc(zero_idx) {
            *assumption = Some(false)
        }
    }

    Ok(assumptions)
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

/// TODO: implement a naive worklist algorithm, then
/// 1. test for least model property
/// 2. test for performance
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

    macro_rules! le {
        ($lhs: expr, $rhs: expr) => {
            Constraint(Lambda::from($lhs), Lambda::from($rhs))
        };
    }

    #[test]
    fn soundness_regression1() {
        crate::test::init_logger();
        let assumptions = vec![Some(true), None, None]
            .into_iter()
            .collect::<IndexVec<_, _>>();
        let constraints = vec![le!(0u32, 2u32)];
        let solutions = solve(assumptions, [].to_vec(), &constraints).unwrap();
        assert_eq!(solutions[Lambda::from(2u32)], Some(true))
    }

    use proptest::collection::vec;
    use proptest::prelude::*;

    prop_compose! {
        fn domain()
                 (value in prop_oneof![
                     25 => Just(None),
                     1 => Just(Some(false)),
                     1 => Just(Some(true)),
                 ])
        -> Option<bool>
        { value }
    }

    prop_compose! {
        fn assumptions(n_lambdas: usize)
                      (v in vec(domain(), n_lambdas))
        -> IndexVec<Lambda, Option<bool>>
        {
            IndexVec::from_raw(v)
        }
    }

    prop_compose! {
        fn equality(n_lambdas: usize)
                   (length in 3usize..5)
                   (v in vec(0..n_lambdas, length))
        -> Vec<Lambda>
        {
            v.into_iter().map(Lambda::from).collect()
        }
    }

    prop_compose! {
        fn equalities(n_lambdas: usize)
                     (length in 0usize..5)
                     (v in vec(equality(n_lambdas), length))
        -> Vec<Vec<Lambda>>
        { v }
    }

    prop_compose! {
        fn constraints(n_lambdas: usize)
                      (n_constraints in 5..2*n_lambdas)
                      (v in vec((0..n_lambdas, 0..n_lambdas), n_constraints))
        -> Vec<Constraint>
        { v.into_iter().map(|(lhs, rhs)| Constraint(Lambda::from(lhs), Lambda::from(rhs))).collect() }
    }

    prop_compose! {
        fn instance()
                   (n_lambdas in 5usize..100)
                   (instance in (assumptions(n_lambdas), equalities(n_lambdas), constraints(n_lambdas)))
        -> (IndexVec<Lambda, Option<bool>>, Vec<Vec<Lambda>>, Vec<Constraint>)
        { instance }
    }

    proptest! {
        #[test]
        fn test_soundness((assumptions, equalities, constraints) in instance()) {
            let solutions = solve(assumptions.clone(), equalities.clone(), &constraints);
            prop_assume!(solutions.is_ok());
            let solutions = solutions.unwrap();

            // solutions must subsume assumptions
            for (&assumption, &solution) in std::iter::zip(assumptions.iter(), solutions.iter()) {
                if let Some(value) = assumption {
                    assert!(solution.is_some() && solution.unwrap() == value)
                }
            }

            // solutions must conform to equalities
            for equality in equalities {
                assert!(equality.len() >= 2);
                let (&head, tail) = equality.split_first().unwrap();
                for &other in tail {
                    assert_eq!(solutions[head], solutions[other])
                }
            }

            fn domain_le(lhs: Option<bool>, rhs: Option<bool>) -> bool {
                match lhs {
                    Some(true) => rhs == Some(true),
                    Some(false) => true,
                    None => rhs == None || rhs == Some(true)
                }
            }

            // solutions must conform to constraints
            for Constraint(lhs, rhs) in constraints {
                assert!(domain_le(solutions[lhs], solutions[rhs]), "{:?}: {:?}, {:?}: {:?}", lhs, solutions[lhs], rhs, solutions[rhs])
            }
        }
    }
}
