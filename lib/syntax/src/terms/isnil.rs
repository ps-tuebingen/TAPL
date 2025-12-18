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

/// Term representing calling `isnil` on a list
#[derive(Clone, Debug, EqNoSpan)]
pub struct IsNil<Lang>
where
    Lang: Language,
{
    /// The list
    pub term: Rc<Lang::Term>,
    /// Annotated type
    pub ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> IsNil<Lang>
where
    Lang: Language,
{
    /// Create a new isnil term with given term, type and span
    pub fn new<T1, Typ>(t: T1, ty: Typ, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Self {
            term: Rc::new(t.into()),
            ty: ty.into(),
            span,
        }
    }
}

impl<Lang> Spanned for IsNil<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for IsNil<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.term.free_vars(vars);
    }
}

impl<Lang> FreeTypeVars for IsNil<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        self.term.free_type_vars(vars);
        self.ty.free_type_vars(vars);
    }
}

impl<Lang> Term for IsNil<Lang> where Lang: Language {}

impl<Lang> SubstTerm for IsNil<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            ty: self.ty,
            span: self.span,
        }
    }
}

impl<Lang> SubstType for IsNil<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
            span: self.span,
        }
    }
}

impl<Lang> fmt::Display for IsNil<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isnil[{}]({})", self.ty, self.term)
    }
}
