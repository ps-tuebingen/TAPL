use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;

use std::{fmt, rc::Rc};

/// Term representing unpacking an existential pack
#[derive(Clone, Debug, EqNoSpan)]
pub struct Unpack<Lang>
where
    Lang: Language,
{
    /// Variable for the bound type
    pub ty_name: TypeVar,
    /// Variable for the bound term
    pub term_name: Var,
    /// Bound term
    pub bound_term: Rc<Lang::Term>,
    /// Inner term
    pub in_term: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Unpack<Lang>
where
    Lang: Language,
{
    /// Create a new Unpack term from given type name, term name, bound term, in term and span
    pub fn new<T1, T2>(tyn: &str, tn: &str, bound: T1, int: T2, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            ty_name: tyn.to_owned(),
            term_name: tn.to_owned(),
            bound_term: Rc::new(bound.into()),
            in_term: Rc::new(int.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Unpack<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Unpack<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Unpack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.bound_term = self.bound_term.subst(v, t);
        if *v != self.term_name {
            self.in_term = self.in_term.subst(v, t);
        }
        self
    }
}

impl<Lang> SubstType for Unpack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.bound_term = self.bound_term.subst_type(v, ty);
        if *v != self.ty_name {
            self.in_term = self.in_term.subst_type(v, ty);
        }
        self
    }
}

impl<Lang> fmt::Display for Unpack<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
