use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::fmt;

/// Term representing a tuple
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    /// Inner terms
    pub terms: Vec<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    /// Create a new tuple from given inner terms and span
    pub fn new<T1>(ts: Vec<T1>, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            terms: ts.into_iter().map(std::convert::Into::into).collect(),
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

impl<Lang> Term for Tuple<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Tuple<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.terms = self.terms.into_iter().map(|t1| t1.subst(v, t)).collect();
        self
    }
}

impl<Lang> SubstType for Tuple<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.terms = self
            .terms
            .into_iter()
            .map(|t| t.subst_type(v, ty))
            .collect();
        self
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self
            .terms
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
