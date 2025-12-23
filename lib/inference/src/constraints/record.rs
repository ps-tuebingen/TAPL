use super::Constraint;
use syntax::{Label, language::Language};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordConstraint<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub label: Label,
    pub label_ty: Lang::Type,
}

impl<Lang> RecordConstraint<Lang>
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

impl<Lang> From<RecordConstraint<Lang>> for Constraint<Lang>
where
    Lang: Language,
{
    fn from(rec: RecordConstraint<Lang>) -> Self {
        Self::Record(rec)
    }
}
