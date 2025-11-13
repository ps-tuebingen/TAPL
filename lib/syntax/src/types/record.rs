use super::Type;
use crate::{Label, TypeVar, language::Language, subst::SubstType};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<Lang>
where
    Lang: Language,
{
    pub records: HashMap<Label, Lang::Type>,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    #[must_use] pub fn new<Ty1>(recs: HashMap<Label, Ty1>) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            records: recs.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
        }
    }
}

impl<Lang> Type for Record<Lang> where Lang: Language {}

impl<Lang> SubstType for Record<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            records: self
                .records
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
    }
}

impl<Lang> fmt::Display for Record<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &<Lang as Language>::Type)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{{}}}",
            recs.iter()
                .map(|(lb, ty)| format!("{lb}:{ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
