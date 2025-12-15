use super::Value;
use crate::{
    TypeVar,
    language::Language,
    span::{Span, Spanned},
    terms::LambdaSub as LambdaSubT,
};
use std::fmt;

/// Bounded Type abstraction value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LambdaSub<Lang>
where
    Lang: Language,
{
    /// Bound type variable
    pub var: TypeVar,
    /// super type annotation
    pub sup_ty: Lang::Type,
    /// body term
    pub term: Lang::Term,
    /// Source location
    pub span: Span,
}

impl<Lang> LambdaSub<Lang>
where
    Lang: Language,
{
    /// Create a new lambda sub value with given variable, super type body and span
    pub fn new<Ty, T>(v: &str, sup: Ty, t: T, span: Span) -> Self
    where
        Ty: Into<Lang::Type>,
        T: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            sup_ty: sup.into(),
            term: t.into(),
            span,
        }
    }
}

impl<Lang> Spanned for LambdaSub<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Value for LambdaSub<Lang>
where
    Lang: Language,
    LambdaSubT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LambdaSubT<Lang>;
}

impl<Lang> From<LambdaSub<Lang>> for LambdaSubT<Lang>
where
    Lang: Language,
{
    fn from(lam: LambdaSub<Lang>) -> Self {
        Self::new(&lam.var, lam.sup_ty, lam.term, lam.span)
    }
}

impl<Lang> fmt::Display for LambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:({}).{}", self.var, self.sup_ty, self.term)
    }
}
