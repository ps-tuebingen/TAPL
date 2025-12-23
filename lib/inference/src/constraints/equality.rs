use super::Constraint;
use syntax::language::Language;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualityConstraint<Lang>
where
    Lang: Language,
{
    pub left: Lang::Type,
    pub right: Lang::Type,
}

impl<Lang> EqualityConstraint<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(ty1: Ty1, ty2: Ty2) -> EqualityConstraint<Lang>
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
