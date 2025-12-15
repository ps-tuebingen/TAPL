use super::Value;
use crate::{
    Var,
    language::Language,
    span::{Span, Spanned},
    terms::UntypedLambda as UntypedLambdaT,
};
use std::fmt;

/// Untyped lambda value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UntypedLambda<Lang>
where
    Lang: Language,
{
    /// bound variable
    pub var: Var,
    /// body term
    pub body: Lang::Term,
    /// Source location
    pub span: Span,
}

impl<Lang> UntypedLambda<Lang>
where
    Lang: Language,
{
    /// Create a new untyped lambda value from bound variable, body and span
    pub fn new<T1>(v: &str, bd: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            body: bd.into(),
            span,
        }
    }
}

impl<Lang> Spanned for UntypedLambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for UntypedLambda<Lang>
where
    Lang: Language,
    UntypedLambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = UntypedLambdaT<Lang>;
}

impl<Lang> From<UntypedLambda<Lang>> for UntypedLambdaT<Lang>
where
    Lang: Language,
{
    fn from(lam: UntypedLambda<Lang>) -> Self {
        Self::new(&lam.var, lam.body, lam.span)
    }
}

impl<Lang> fmt::Display for UntypedLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}.{}", self.var, self.body)
    }
}
