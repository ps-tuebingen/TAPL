use super::Constraint;
use syntax::language::Language;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexConstraint<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub ind: usize,
    pub ind_ty: Lang::Type,
}

impl<Lang> IndexConstraint<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(ty: Ty1, ind: usize, ind_ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            ind,
            ind_ty: ind_ty.into(),
        }
    }
}

impl<Lang> From<IndexConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(c: IndexConstraint<Lang>) -> Self {
        Self::Indexing(c)
    }
}
