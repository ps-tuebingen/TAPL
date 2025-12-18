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

/// Term representing a memory dereference
#[derive(Clone, Debug, EqNoSpan)]
pub struct Deref<Lang>
where
    Lang: Language,
{
    /// Term to be dereferenced (memory location)
    pub term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Deref<Lang>
where
    Lang: Language,
{
    /// Create a new deref from given term and span
    pub fn new<T1>(t: T1, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Deref<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for Deref<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.term.free_vars(vars);
    }
}

impl<Lang> FreeTypeVars for Deref<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.term.free_type_vars(vars);
    }
}

impl<Lang> Term for Deref<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Deref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            span: self.span,
        }
    }
}

impl<Lang> SubstType for Deref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for Deref<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!{}", self.term)
    }
}
