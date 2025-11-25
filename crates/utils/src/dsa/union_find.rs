use rustc_index::{Idx, IndexVec};
use std::collections::HashSet;

pub struct UnionFind<I: Idx> {
    parent: IndexVec<I, I>,
    size: IndexVec<I, usize>,
    members: IndexVec<I, HashSet<I>>,
}

impl<I: Idx + From<usize>> UnionFind<I> {
    pub fn new(n: usize) -> Self {
        let mut parent = IndexVec::with_capacity(n);
        let mut size = IndexVec::with_capacity(n);
        let mut members = IndexVec::with_capacity(n);

        for i in 0..n {
            let idx = I::from(i);
            parent.push(idx);
            size.push(1);

            let mut set = HashSet::new();
            set.insert(idx);
            members.push(set);
        }

        Self {
            parent,
            size,
            members,
        }
    }

    /// Path-compressed find.
    pub fn find(&mut self, x: I) -> I {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    /// Union by size + members merging.
    pub fn union(&mut self, a: I, b: I) -> I {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return ra;
        }

        // ensure ra is bigger
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        // attach rb → ra
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];

        // move rb members into ra
        let rb_set = std::mem::take(&mut self.members[rb]);
        self.members[ra].extend(rb_set);

        ra
    }

    /// Retrieve entire member set of x’s group.
    pub fn group(&mut self, x: I) -> HashSet<I> {
        let r = self.find(x);
        self.members[r].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    rustc_index::newtype_index! {
        pub struct IdxTy { }
    }

    type DisjointSet = UnionFind<IdxTy>;

    fn dsu(n: usize) -> DisjointSet {
        DisjointSet::new(n)
    }

    #[test]
    fn test_singletons() {
        let mut d = dsu(4);

        for i in 0..4 {
            let idx = IdxTy::from_usize(i);
            let g = d.group(idx);
            assert_eq!(g, HashSet::from([idx]));
        }
    }

    #[test]
    fn test_union_basic() {
        let mut d = dsu(3);
        let a = IdxTy::from_usize(0);
        let b = IdxTy::from_usize(1);

        d.union(a, b);

        let g = d.group(a);
        assert_eq!(g, HashSet::from([a, b]));
    }

    #[test]
    fn test_chain_union() {
        let mut d = dsu(5);

        for (x, y) in [(0, 1), (1, 2), (2, 3)] {
            d.union(IdxTy::from_usize(x), IdxTy::from_usize(y));
        }

        let g = d.group(IdxTy::from_usize(0));
        assert_eq!(g.len(), 4);
        for i in 0..4 {
            assert!(g.contains(&IdxTy::from_usize(i)));
        }
    }

    #[test]
    fn test_disjoint_groups() {
        let mut d = dsu(6);

        d.union(IdxTy::from_usize(0), IdxTy::from_usize(1));
        d.union(IdxTy::from_usize(2), IdxTy::from_usize(3));
        d.union(IdxTy::from_usize(4), IdxTy::from_usize(5));

        assert_eq!(
            d.group(IdxTy::from_usize(0)),
            HashSet::from([IdxTy::from_usize(0), IdxTy::from_usize(1)])
        );
        assert_eq!(
            d.group(IdxTy::from_usize(2)),
            HashSet::from([IdxTy::from_usize(2), IdxTy::from_usize(3)])
        );
        assert_eq!(
            d.group(IdxTy::from_usize(4)),
            HashSet::from([IdxTy::from_usize(4), IdxTy::from_usize(5)])
        );
    }

    #[test]
    fn test_redundant_union() {
        let mut d = dsu(3);

        let a = IdxTy::from_usize(0);
        let b = IdxTy::from_usize(1);
        let c = IdxTy::from_usize(2);

        d.union(a, b);
        d.union(b, c);

        let before = d.group(a);
        d.union(a, c); // redundant
        let after = d.group(a);

        assert_eq!(before, after);
    }

    #[test]
    fn test_group_members_consistent_across_all_nodes() {
        let mut d = dsu(5);

        d.union(IdxTy::from_usize(0), IdxTy::from_usize(1));
        d.union(IdxTy::from_usize(1), IdxTy::from_usize(2));

        let expected = HashSet::from([
            IdxTy::from_usize(0),
            IdxTy::from_usize(1),
            IdxTy::from_usize(2),
        ]);

        for i in 0..3 {
            assert_eq!(d.group(IdxTy::from_usize(i)), expected);
        }
    }

    #[test]
    fn test_members_are_not_left_in_old_group_after_merge() {
        let mut d = dsu(4);

        let a = IdxTy::from_usize(0);
        let b = IdxTy::from_usize(1);
        let c = IdxTy::from_usize(2);

        d.union(a, b); // {0,1}
        d.union(b, c); // {0,1,2}

        // Check internal invariant: old groups must be empty except the root
        let root = d.find(a);
        for i in 0..4 {
            let idx = IdxTy::from_usize(i);
            if d.find(idx) != root {
                assert!(
                    d.group(idx).len() <= 1,
                    "Non-root group contains leftover members"
                );
            }
        }
    }

    #[test]
    fn test_union_order_does_not_affect_members() {
        let mut d1 = dsu(4);
        let mut d2 = dsu(4);

        // Union in order 0-1, 1-2, 2-3
        d1.union(IdxTy::from_usize(0), IdxTy::from_usize(1));
        d1.union(IdxTy::from_usize(1), IdxTy::from_usize(2));
        d1.union(IdxTy::from_usize(2), IdxTy::from_usize(3));

        // Union in a different order 3-2, 2-1, 1-0
        d2.union(IdxTy::from_usize(3), IdxTy::from_usize(2));
        d2.union(IdxTy::from_usize(2), IdxTy::from_usize(1));
        d2.union(IdxTy::from_usize(1), IdxTy::from_usize(0));

        let expected = HashSet::from([
            IdxTy::from_usize(0),
            IdxTy::from_usize(1),
            IdxTy::from_usize(2),
            IdxTy::from_usize(3),
        ]);

        for i in 0..4 {
            assert_eq!(d1.group(IdxTy::from_usize(i)), expected);
            assert_eq!(d2.group(IdxTy::from_usize(i)), expected);
        }
    }

    #[test]
    fn test_large_group_merged_into_small_group() {
        let mut d = dsu(6);

        // Large group: 0,1,2,3
        d.union(IdxTy::from_usize(0), IdxTy::from_usize(1));
        d.union(IdxTy::from_usize(1), IdxTy::from_usize(2));
        d.union(IdxTy::from_usize(2), IdxTy::from_usize(3));

        // Small group: 4,5
        d.union(IdxTy::from_usize(4), IdxTy::from_usize(5));

        // Force merging small → large
        d.union(IdxTy::from_usize(3), IdxTy::from_usize(4));

        let expected: HashSet<_> = (0..6).map(IdxTy::from_usize).collect();

        for i in 0..6 {
            assert_eq!(d.group(IdxTy::from_usize(i)), expected);
        }
    }

    #[test]
    fn test_repeated_unions_do_not_duplicate_members() {
        let mut d = dsu(4);

        let a = IdxTy::from_usize(0);
        let b = IdxTy::from_usize(1);

        d.union(a, b);
        d.union(a, b);
        d.union(b, a);

        let g = d.group(a);
        assert_eq!(g.len(), 2);
        assert!(g.contains(&a));
        assert!(g.contains(&b));
    }

    #[test]
    fn test_members_reflect_path_compression() {
        let mut d = dsu(6);

        // Create a chain 0-1-2-3
        d.union(IdxTy::from_usize(0), IdxTy::from_usize(1));
        d.union(IdxTy::from_usize(1), IdxTy::from_usize(2));
        d.union(IdxTy::from_usize(2), IdxTy::from_usize(3));

        // Access far nodes to trigger compression
        d.find(IdxTy::from_usize(3));
        d.find(IdxTy::from_usize(2));

        let expected = HashSet::from([
            IdxTy::from_usize(0),
            IdxTy::from_usize(1),
            IdxTy::from_usize(2),
            IdxTy::from_usize(3),
        ]);

        for i in 0..4 {
            assert_eq!(d.group(IdxTy::from_usize(i)), expected);
        }
    }

    #[test]
    fn test_group_members_after_multiple_independent_unions() {
        let mut d = dsu(10);

        let group1 = [0, 3, 7];
        let group2 = [1, 4, 9];
        let group3 = [2, 5, 6, 8];

        // union group 1
        d.union(IdxTy::from_usize(0), IdxTy::from_usize(3));
        d.union(IdxTy::from_usize(3), IdxTy::from_usize(7));

        // union group 2
        d.union(IdxTy::from_usize(1), IdxTy::from_usize(4));
        d.union(IdxTy::from_usize(4), IdxTy::from_usize(9));

        // union group 3
        d.union(IdxTy::from_usize(2), IdxTy::from_usize(5));
        d.union(IdxTy::from_usize(5), IdxTy::from_usize(6));
        d.union(IdxTy::from_usize(6), IdxTy::from_usize(8));

        let set1: HashSet<_> = group1.iter().copied().map(IdxTy::from_usize).collect();
        let set2: HashSet<_> = group2.iter().copied().map(IdxTy::from_usize).collect();
        let set3: HashSet<_> = group3.iter().copied().map(IdxTy::from_usize).collect();

        for i in group1 {
            assert_eq!(d.group(IdxTy::from_usize(i)), set1);
        }
        for i in group2 {
            assert_eq!(d.group(IdxTy::from_usize(i)), set2);
        }
        for i in group3 {
            assert_eq!(d.group(IdxTy::from_usize(i)), set3);
        }
    }
}
