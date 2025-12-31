use super::Constraint;
use std::fmt;
use syntax::language::Language;

/// Subtyping Constraint
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtypeConstraint<Lang>
where
    Lang: Language,
{
    /// Subtype
    pub sub_type: Lang::Type,
    /// Super Type
    pub super_type: Lang::Type,
}

impl<Lang> SubtypeConstraint<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(sub_ty: Ty1, super_ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            sub_type: sub_ty.into(),
            super_type: super_ty.into(),
        }
    }
}

impl<Lang> From<SubtypeConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(cns: SubtypeConstraint<Lang>) -> Self {
        Self::Subtyping(cns)
    }
}

impl<Lang> fmt::Display for SubtypeConstraint<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} <: {}", self.sub_type, self.super_type)
    }
}
