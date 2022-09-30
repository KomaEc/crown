use std::ops::Range;

use rustc_data_structures::graph::scc::Sccs;
use rustc_index::vec::IndexVec;

use crate::{
    fat_thin_analysis::{Constraint, Lambda},
    utils::graph_ext::implementation::forward_star::Graph,
};

macro_rules! array_index {
    ($row: expr, $col: expr; $len: expr) => {{
        $row * $len + $col
    }};
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SolveSuccess {
    Unchanged,
    LocallyChanged,
    GloballyChanged,
}

pub fn solve(
    assumptions: &mut IndexVec<Lambda, Option<bool>>,
    globals: Range<usize>,
    locals: Range<usize>,
    constraints: &[Constraint],
    // boundary_constraints: impl Iterator<Item = Constraint>,
    boundary_constraints: &[Constraint],
) -> Result<SolveSuccess, ()> {
    assert_eq!(globals.start, 0);
    assert!(globals.end <= locals.start);
    assert!(locals.end <= assumptions.len());

    let globals_barrier = globals.end - globals.start;
    let num_nodes = globals_barrier + locals.end - locals.start + 2;
    let zero_idx = num_nodes - 2;
    let one_idx = num_nodes - 1;
    let mut relation = vec![false; num_nodes * num_nodes].into_boxed_slice();
    for idx in 0..num_nodes {
        relation[array_index!(idx, idx; num_nodes)] = true;
        relation[array_index!(idx, one_idx; num_nodes)] = true;
        relation[array_index!(zero_idx, idx; num_nodes)] = true;
    }
    constraints.iter().for_each(|c| {
        let lhs = locals
            .contains(&c.0.index())
            .then(|| c.0.index() - locals.start + globals_barrier)
            .unwrap_or(c.0.index());
        let rhs = locals
            .contains(&c.1.index())
            .then(|| c.1.index() - locals.start + globals_barrier)
            .unwrap_or(c.1.index());
        relation[array_index!(lhs, rhs; num_nodes)] = true;
    });

    for &Constraint(lhs, rhs) in boundary_constraints {
        if locals.contains(&lhs.as_usize()) {
            // return position
            if matches!(assumptions[rhs], Some(false)) {
                let idx = lhs.as_usize() - locals.start + globals_barrier;
                relation[array_index!(idx, zero_idx; num_nodes)] = true;
                relation[array_index!(zero_idx, idx; num_nodes)] = true;
            }
        } else {
            // argument position
            assert!(locals.contains(&rhs.as_usize()));
            if matches!(assumptions[lhs], Some(true)) {
                let idx = rhs.as_usize() - locals.start + globals_barrier;
                relation[array_index!(idx, one_idx; num_nodes)] = true;
                relation[array_index!(one_idx, idx; num_nodes)] = true;
            }
        }
    }

    for (idx, assumption) in assumptions.raw[globals.clone()]
        .iter()
        .chain(assumptions.raw[locals.clone()].iter())
        .enumerate()
    {
        match assumption {
            Some(true) => {
                relation[array_index!(one_idx, idx; num_nodes)] = true;
            }
            Some(false) => {
                relation[array_index!(idx, zero_idx; num_nodes)] = true;
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

    let mut solve_success = SolveSuccess::Unchanged;
    let mut idx = 0;
    for assumption in &mut assumptions.raw[globals] {
        if matches!(assumption, None) {
            if sccs.scc(idx) == sccs.scc(one_idx) {
                *assumption = Some(true);
                solve_success = std::cmp::max(solve_success, SolveSuccess::GloballyChanged)
            } else if sccs.scc(idx) == sccs.scc(zero_idx) {
                *assumption = Some(false);
                solve_success = std::cmp::max(solve_success, SolveSuccess::GloballyChanged)
            }
        } else {
            #[cfg(debug_assertions)]
            assumption
                .unwrap()
                .then(|| assert_eq!(sccs.scc(idx), sccs.scc(one_idx)))
                .unwrap_or_else(|| assert_eq!(sccs.scc(idx), sccs.scc(zero_idx)))
        }
        idx += 1;
    }
    for assumption in &mut assumptions.raw[locals.clone()] {
        if matches!(assumption, None) {
            if sccs.scc(idx) == sccs.scc(one_idx) {
                *assumption = Some(true);
                solve_success = std::cmp::max(solve_success, SolveSuccess::LocallyChanged)
            } else if sccs.scc(idx) == sccs.scc(zero_idx) {
                *assumption = Some(false);
                solve_success = std::cmp::max(solve_success, SolveSuccess::LocallyChanged)
            }
        } else {
            #[cfg(debug_assertions)]
            assumption
                .unwrap()
                .then(|| assert_eq!(sccs.scc(idx), sccs.scc(one_idx)))
                .unwrap_or_else(|| assert_eq!(sccs.scc(idx), sccs.scc(zero_idx)))
        }
        idx += 1;
    }

    /*
    for &Constraint(lhs, rhs) in boundary_constraints {
        if locals.contains(&lhs.as_usize()) {
            // return position
            if matches!(assumptions[lhs], Some(true)) {
                match assumptions[rhs] {
                    None => {
                        assumptions[rhs] = Some(true);
                        solve_success = std::cmp::max(solve_success, SolveSuccess::GloballyChanged)
                    }
                    Some(value) => assert!(value),
                }
            }
        } else {
            // argument position
            assert!(locals.contains(&rhs.as_usize()));
            if matches!(assumptions[rhs], Some(false)) {
                match assumptions[lhs] {
                    None => {
                        assumptions[lhs] = Some(false);
                        solve_success = std::cmp::max(solve_success, SolveSuccess::GloballyChanged)
                    }
                    Some(value) => assert!(!value),
                }
            }
        }
    }
    */

    Ok(solve_success)
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

    // static assertion?
    #[test]
    fn test_solve_success() {
        assert!(SolveSuccess::Unchanged < SolveSuccess::LocallyChanged);
        assert!(SolveSuccess::LocallyChanged < SolveSuccess::GloballyChanged);
    }

    fn assert_soundness(
        assumptions: IndexVec<Lambda, Option<bool>>,
        solutions: IndexVec<Lambda, Option<bool>>,
        constraints: &[Constraint],
    ) {
        // solutions must subsume assumptions
        for (&assumption, &solution) in std::iter::zip(assumptions.iter(), solutions.iter()) {
            if let Some(value) = assumption {
                assert!(solution.is_some() && solution.unwrap() == value)
            }
        }

        fn domain_le(lhs: Option<bool>, rhs: Option<bool>) -> bool {
            match lhs {
                Some(true) => rhs == Some(true),
                Some(false) => true,
                None => rhs == None || rhs == Some(true),
            }
        }

        // solutions must conform to constraints
        for &Constraint(lhs, rhs) in constraints {
            assert!(
                domain_le(solutions[lhs], solutions[rhs]),
                "{:?}: {:?}, {:?}: {:?}",
                lhs,
                solutions[lhs],
                rhs,
                solutions[rhs]
            )
        }
    }

    #[test]
    fn soundness_regression1() {
        crate::test::init_logger();
        let (globals, locals, assumptions, constraints) = (
            0..1,
            1..5,
            [None, None, Some(false), None, None]
                .into_iter()
                .collect::<IndexVec<_, _>>(),
            [
                Constraint(0u32.into(), 0u32.into()),
                Constraint(0u32.into(), 0u32.into()),
                Constraint(0u32.into(), 0u32.into()),
                Constraint(0u32.into(), 0u32.into()),
                Constraint(3u32.into(), 2u32.into()),
            ],
        );
        let mut solutions = assumptions.clone();
        assert!(solve(
            &mut solutions,
            globals,
            locals,
            &constraints,
            // std::iter::empty()
            &[]
        )
        .is_ok());

        assert_soundness(assumptions, solutions, &constraints)
    }

    use proptest::{collection::vec, prelude::*};

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
        fn constraints(n_lambdas: usize)
                      (n_constraints in 5..2*n_lambdas)
                      (v in vec((0..n_lambdas, 0..n_lambdas), n_constraints))
        -> Vec<Constraint>
        { v.into_iter().map(|(lhs, rhs)| Constraint(Lambda::from(lhs), Lambda::from(rhs))).collect() }
    }

    prop_compose! {
        fn instance()
                   (n_lambdas in 5usize..100)
                   ((assumptions, constraints, glob) in (assumptions(n_lambdas), constraints(n_lambdas), 0..n_lambdas))
        -> (Range<usize>, Range<usize>, IndexVec<Lambda, Option<bool>>, Vec<Constraint>)
        { (Range { start: 0, end: glob }, Range { start: glob, end: assumptions.len() }, assumptions, constraints) }
    }

    proptest! {
        #[test]
        fn test_soundness((globals, locals, assumptions, constraints) in instance()) {

            let mut solutions = assumptions.clone();
            prop_assume!(solve(&mut solutions, globals, locals, &constraints, &[] /*std::iter::empty()*/).is_ok());

            assert_soundness(assumptions, solutions, &constraints)
        }
    }
}
