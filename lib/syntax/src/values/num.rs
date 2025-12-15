use super::Value;
use crate::{
    language::Language,
    span::{Span, Spanned},
    terms::Num as NumT,
};
use std::{fmt, marker::PhantomData};

/// Number value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Num<Lang>
where
    Lang: Language,
{
    /// Number
    pub num: i64,
    /// Source location
    pub span: Span,
    /// Save the type parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> Num<Lang>
where
    Lang: Language,
{
    /// Create a new number value with given number and span
    #[must_use]
    pub const fn new(i: i64, span: Span) -> Self {
        Self {
            num: i,
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for Num<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Num<Lang>
where
    Lang: Language,
    NumT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NumT<Lang>;
}

impl<Lang> From<Num<Lang>> for NumT<Lang>
where
    Lang: Language,
{
    fn from(n: Num<Lang>) -> Self {
        Self::new(n.num, n.span)
    }
}

impl<Lang> fmt::Display for Num<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
