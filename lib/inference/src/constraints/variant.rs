use super::Constraint;
use std::fmt;
use syntax::{Label, language::Language};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantConstraint<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub label: Label,
    pub label_ty: Lang::Type,
}

impl<Lang> VariantConstraint<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(ty: Ty1, label: &str, label_ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            label: label.to_string(),
            label_ty: label_ty.into(),
        }
    }
}

impl<Lang> From<VariantConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(v: VariantConstraint<Lang>) -> Self {
        Self::Variant(v)
    }
}

impl<Lang> fmt::Display for VariantConstraint<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} == <{}:{},...>", self.ty, self.label, self.label_ty)
    }
}
