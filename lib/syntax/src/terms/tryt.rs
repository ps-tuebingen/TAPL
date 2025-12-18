use super::Term;
use crate::{
    TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, rc::Rc};

/// Term representing try-with
/// used with [`crate::terms::exception::Exception`]
#[derive(Clone, Debug, EqNoSpan)]
pub struct Try<Lang>
where
    Lang: Language,
{
    /// Term that may raise an exception
    pub term: Rc<Lang::Term>,
    /// Exception handler
    pub handler: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Try<Lang>
where
    Lang: Language,
{
    /// Create a new try term with given inner term, handler and span
    pub fn new<T1, T2>(t: T1, h: T2, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            handler: Rc::new(h.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Try<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}
impl<Lang> FreeVars for Try<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.term.free_vars(vars);
        self.handler.free_vars(vars);
    }
}

impl<Lang> FreeTypeVars for Try<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.term.free_type_vars(vars);
        self.handler.free_type_vars(vars);
    }
}

impl<Lang> Term for Try<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Try<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.term = self.term.subst(v, t);
        self.handler = self.handler.subst(v, t);
        self
    }
}

impl<Lang> SubstType for Try<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.term = self.term.subst_type(v, ty);
        self.handler = self.handler.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Try<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} with {{ {} }}", self.term, self.handler)
    }
}
