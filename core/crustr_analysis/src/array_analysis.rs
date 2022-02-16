use rustc_middle::mir::Local;

pub mod intra;
#[cfg(test)]
mod test;

/// This structure should hold info about all struct definitions
/// and local nested pointers in the crate
pub struct CrateSummary;

pub enum Constraint {
    /// λ1 = λ2
    Eq(Lambda, Lambda),
    /// λ1 ≤ λ2
    LE(Lambda, Lambda),
}

/// The language constructs that a constraint variable λ corresponds to
pub enum LambdaData {
    /// A SSA scalar variable
    Local { base: Local, ssa_idx: usize },
    /// field definition
    FieldDef,
    /// A local nested pointer type.
    /// For example, if a local `_1` has type `*mut *mut *mut i32`, then
    /// we should have entries for `*_1` and `**_1`
    LocalNested { base: Local, nested_level: usize },
}

rustc_index::newtype_index! {
    pub struct ConstraintIdx {
        DEBUG_FORMAT = "constraint ({})"
    }
}

rustc_index::newtype_index! {
    /// Constraint variables for array analysis
    pub struct Lambda {
        DEBUG_FORMAT = "λ_({})"
    }
}
