use super::Type;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::fmt;

/// Tuple Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    /// Inner Types
    pub tys: Vec<Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    /// Create new Tuple with given inner types and span
    pub fn new<Ty1>(tys: Vec<Ty1>, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            tys: tys.into_iter().map(std::convert::Into::into).collect(),
            span,
        }
    }
}

impl<Lang> Spanned for Tuple<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Type for Tuple<Lang> where Lang: Language {}

impl<Lang> SubstType for Tuple<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Self;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.tys = self
            .tys
            .into_iter()
            .map(|ty1| ty1.subst_type(v, ty))
            .collect();
        self
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tys: Vec<String> = self
            .tys
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        tys.sort();
        write!(f, "({})", tys.join(", "))
    }
}
