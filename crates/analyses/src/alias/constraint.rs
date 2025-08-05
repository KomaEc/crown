pub type Constraint<Loc> = GenericConstraint<ConstraintKind, Loc>;
pub type BasicConstraint<Loc> = GenericConstraint<BasicConstraintKind, Loc>;
pub type ComplexConstraint<Loc> = GenericConstraint<ComplexConstraintKind, Loc>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GenericConstraint<K, Loc> {
    pub kind: K,
    pub lhs: Loc,
    pub rhs: Loc,
}

impl<K, Loc> GenericConstraint<K, Loc> {
    pub fn new(kind: K, lhs: Loc, rhs: Loc) -> Self {
        Self { kind, lhs, rhs }
    }
}

impl<Loc> Constraint<Loc> {
    pub fn addr(lhs: Loc, rhs: Loc) -> Self {
        Self::new(ConstraintKind::Basic(BasicConstraintKind::Addr), lhs, rhs)
    }

    pub fn assign(lhs: Loc, rhs: Loc) -> Self {
        Self::new(ConstraintKind::Basic(BasicConstraintKind::Assign), lhs, rhs)
    }

    pub fn store(lhs: Loc, rhs: Loc) -> Self {
        Self::new(
            ConstraintKind::Complex(ComplexConstraintKind::Store),
            lhs,
            rhs,
        )
    }

    pub fn load(lhs: Loc, rhs: Loc) -> Self {
        Self::new(
            ConstraintKind::Complex(ComplexConstraintKind::Load),
            lhs,
            rhs,
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ConstraintKind {
    Basic(BasicConstraintKind),
    Complex(ComplexConstraintKind),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BasicConstraintKind {
    Addr,
    Assign,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ComplexConstraintKind {
    Store,
    Load,
}

impl<Loc> From<GenericConstraint<BasicConstraintKind, Loc>>
    for GenericConstraint<ConstraintKind, Loc>
{
    fn from(value: GenericConstraint<BasicConstraintKind, Loc>) -> Self {
        Self {
            kind: ConstraintKind::Basic(value.kind),
            lhs: value.lhs,
            rhs: value.rhs,
        }
    }
}

impl<Loc> From<GenericConstraint<ComplexConstraintKind, Loc>>
    for GenericConstraint<ConstraintKind, Loc>
{
    fn from(value: GenericConstraint<ComplexConstraintKind, Loc>) -> Self {
        Self {
            kind: ConstraintKind::Complex(value.kind),
            lhs: value.lhs,
            rhs: value.rhs,
        }
    }
}

impl<K, Loc> GenericConstraint<K, Loc> {
    pub fn map_kind<J, F>(self, f: F) -> GenericConstraint<J, Loc>
    where
        F: FnOnce(K) -> J,
    {
        GenericConstraint {
            kind: f(self.kind),
            lhs: self.lhs,
            rhs: self.rhs,
        }
    }
}

impl<Loc: std::fmt::Debug> std::fmt::Display for GenericConstraint<BasicConstraintKind, Loc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            BasicConstraintKind::Addr => {
                write!(f, "{:?} = &{:?}", self.lhs, self.rhs)
            }
            BasicConstraintKind::Assign => {
                write!(f, "{:?} = {:?}", self.lhs, self.rhs)
            }
        }
    }
}

impl<Loc: std::fmt::Debug> std::fmt::Display for GenericConstraint<ComplexConstraintKind, Loc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ComplexConstraintKind::Store => {
                write!(f, "*{:?} = {:?}", self.lhs, self.rhs)
            }
            ComplexConstraintKind::Load => {
                write!(f, "{:?} = *{:?}", self.lhs, self.rhs)
            }
        }
    }
}

impl<Loc: std::fmt::Debug + Copy> std::fmt::Display for GenericConstraint<ConstraintKind, Loc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ConstraintKind::Basic(kind) => {
                write!(f, "{}", self.map_kind(|_| kind))
            }
            ConstraintKind::Complex(kind) => {
                write!(f, "{}", self.map_kind(|_| kind))
            }
        }
    }
}
