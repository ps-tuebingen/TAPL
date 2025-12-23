use super::Constraint;
use syntax::language::Language;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtypeConstraint<Lang>
where
    Lang: Language,
{
    pub sub_type: Lang::Type,
    pub super_type: Lang::Type,
}

impl<Lang> SubtypeConstraint<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(sub_ty: Ty1, super_ty: Ty2) -> SubtypeConstraint<Lang>
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
