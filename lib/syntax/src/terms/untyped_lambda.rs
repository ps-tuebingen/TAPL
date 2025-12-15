use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

/// Represents a lambda abstraction without type annotation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UntypedLambda<Lang>
where
    Lang: Language,
{
    /// Bound variable
    pub var: Var,
    /// Body term
    pub body: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> UntypedLambda<Lang>
where
    Lang: Language,
{
    /// Create a new untyped lambda with given variable, term and span
    pub fn new<T1>(v: &str, t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            body: Rc::new(t.into()),
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

impl<Lang> Term for UntypedLambda<Lang> where Lang: Language {}

impl<Lang> SubstTerm for UntypedLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v != self.var {
            self.body = self.body.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for UntypedLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.body = self.body.subst_type(v, ty);
        self
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
