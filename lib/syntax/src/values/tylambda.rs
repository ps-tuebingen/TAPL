use super::Value;
use crate::{
    TypeVar,
    kinds::Kind,
    language::Language,
    span::{Span, Spanned},
    terms::TyLambda as TyLambdaT,
};
use std::fmt;

/// Type abstraction value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyLambda<Lang>
where
    Lang: Language,
{
    /// Bound type variable
    pub var: TypeVar,
    /// Kind annotation
    pub annot: Kind,
    /// Body term
    pub term: Lang::Term,
    /// Source location
    pub span: Span,
}

impl<Lang> TyLambda<Lang>
where
    Lang: Language,
{
    /// Create a new ty lambda value with given variable, kind, body and span
    pub fn new<T1>(v: &str, knd: Kind, t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            annot: knd,
            term: t.into(),
            span,
        }
    }
}

impl<Lang> Spanned for TyLambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for TyLambda<Lang>
where
    Lang: Language,
    TyLambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TyLambdaT<Lang>;
}
impl<Lang> From<TyLambda<Lang>> for TyLambdaT<Lang>
where
    Lang: Language,
{
    fn from(tylam: TyLambda<Lang>) -> Self {
        Self::new(&tylam.var, tylam.annot, tylam.term, tylam.span)
    }
}

impl<Lang> fmt::Display for TyLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.term)
    }
}
