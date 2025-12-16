use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Tuple as TupleT,
};
use macros::EqNoSpan;
use std::fmt;

/// Tuple value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    /// Inner values
    pub vals: Vec<Lang::Value>,
    /// Source locations
    pub span: Span,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    /// Create a new tuple value with given inner values and span
    pub fn new<V1>(vals: Vec<V1>, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
    {
        Self {
            vals: vals.into_iter().map(std::convert::Into::into).collect(),
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

impl<Lang> Value for Tuple<Lang>
where
    Lang: Language,
    TupleT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TupleT<Lang>;
}

impl<Lang> From<Tuple<Lang>> for TupleT<Lang>
where
    Lang: Language,
{
    fn from(tup: Tuple<Lang>) -> Self {
        Self::new(tup.vals, tup.span)
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self
            .vals
            .iter()
            .map(std::string::ToString::to_string)
            .collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
