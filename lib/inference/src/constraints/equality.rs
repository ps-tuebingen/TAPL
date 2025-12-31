use super::Constraint;
use std::fmt;
use syntax::language::Language;

/// Equality Constraint between types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualityConstraint<Lang>
where
    Lang: Language,
{
    /// Left Type
    pub left: Lang::Type,
    /// Right Type
    pub right: Lang::Type,
}

impl<Lang> EqualityConstraint<Lang>
where
    Lang: Language,
{
    /// Create a new equality constraint from two given types
    pub fn new<Ty1, Ty2>(ty1: Ty1, ty2: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            left: ty1.into(),
            right: ty2.into(),
        }
    }
}

impl<Lang> From<EqualityConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(cns: EqualityConstraint<Lang>) -> Self {
        Self::Equality(cns)
    }
}

impl<Lang> fmt::Display for EqualityConstraint<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} == {}", self.left, self.right)
    }
}
