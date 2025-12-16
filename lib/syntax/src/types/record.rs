use super::Type;
use crate::{
    Label, TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use std::collections::HashMap;
use std::fmt;

/// Record type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<Lang>
where
    Lang: Language,
{
    /// Labeled inner types
    pub records: HashMap<Label, Lang::Type>,
    /// Source location
    pub span: Span,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    /// Create a new record type from records and span
    #[must_use]
    pub fn new<Ty1>(recs: HashMap<Label, Ty1>, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            records: recs.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
            span,
        }
    }
}

impl<Lang> Spanned for Record<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
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
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.records = self
            .records
            .into_iter()
            .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
            .collect();
        self
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
