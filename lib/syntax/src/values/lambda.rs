use super::Value;
use crate::{
    Var,
    language::Language,
    span::{Span, Spanned},
    terms::Lambda as LambdaT,
};
use macros::EqNoSpan;
use std::fmt;

/// Lambda value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Lambda<Lang>
where
    Lang: Language,
{
    /// bound variable
    pub var: Var,
    /// Type annotation
    pub annot: Lang::Type,
    /// Body term
    pub body: Lang::Term,
    /// Source location
    pub span: Span,
}

impl<Lang> Lambda<Lang>
where
    Lang: Language,
{
    /// Create a new lambda value with given variable, type, body and span
    pub fn new<Ty, T>(v: &str, ty: Ty, bd: T, span: Span) -> Self
    where
        T: Into<Lang::Term>,
        Ty: Into<Lang::Type>,
        Lang: Language,
    {
        Self {
            var: v.to_owned(),
            annot: ty.into(),
            body: bd.into(),
            span,
        }
    }
}

impl<Lang> Spanned for Lambda<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for Lambda<Lang>
where
    Lang: Language,
    LambdaT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LambdaT<Lang>;
}

impl<Lang> From<Lambda<Lang>> for LambdaT<Lang>
where
    Lang: Language,
{
    fn from(lam: Lambda<Lang>) -> Self {
        Self::new(&lam.var, lam.annot, lam.body, lam.span)
    }
}

impl<Lang> fmt::Display for Lambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
