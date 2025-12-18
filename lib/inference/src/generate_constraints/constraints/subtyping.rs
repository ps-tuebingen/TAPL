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

impl<Lang> From<SubtypeConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(cns: SubtypeConstraint<Lang>) -> Self {
        Self::Subtyping(cns)
    }
}
