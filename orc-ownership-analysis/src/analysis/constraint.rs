pub mod gathering;

orc_common::macros::orc_index!(OwnershipSig);

impl OwnershipSig {
    pub(crate) fn into_lit(self) -> i32 {
        self.as_u32() as i32
    }
}

/// 0 + 1 = 2
#[derive(Clone, Debug)]
pub(crate) struct OwnershipTransferConstraint(
    pub(crate) OwnershipSig,
    pub(crate) OwnershipSig,
    pub(crate) OwnershipSig,
);

#[derive(Clone, Debug)]
pub(crate) struct AssertionConstraint(pub(crate) OwnershipSig, pub(crate) bool);

#[derive(Clone, Debug)]
pub(crate) enum Constraint {
    Linear(OwnershipTransferConstraint),
    Const(AssertionConstraint),
}

// impl OwnershipTransferConstraint {

//     pub(crate) fn into_clauses(self) -> impl Iterator<Item = impl Iterator<Item = i32>> {
//         let OwnershipTransferConstraint(x, y, z) = self;
//         // [-x, y, z,
//         //  x, -y, z,
//         //  x, y, -z,
//         //  x, -y,
//         //  -y, -z,
//         //  x, -z]
//         let clauses = vec![
//             vec![-x.into_lit(), y.into_lit(), z.into_lit()],
//             vec![x.into_lit(), -y.into_lit()],
//             vec![]
//         ];
//         clauses.into_iter().map(|clause| clause.into_iter())
//     }
// }
