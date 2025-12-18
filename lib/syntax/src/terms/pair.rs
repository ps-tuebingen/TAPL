use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{fmt, rc::Rc};

/// Term representing a pair
#[derive(Clone, Debug, EqNoSpan)]
pub struct Pair<Lang>
where
    Lang: Language,
{
    /// First element
    pub fst: Rc<Lang::Term>,
    /// Second element
    pub snd: Rc<Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Pair<Lang>
where
    Lang: Language,
{
    //// Create a new pair with given first term, second term and span
    pub fn new<T1, T2>(fst: T1, snd: T2, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            fst: Rc::new(fst.into()),
            snd: Rc::new(snd.into()),
            span,
        }
    }
}

impl<Lang> Spanned for Pair<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for Pair<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Pair<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.fst = self.fst.subst(v, t);
        self.snd = self.snd.subst(v, t);
        self
    }
}

impl<Lang> SubstType for Pair<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.fst = self.fst.subst_type(v, ty);
        self.snd = self.snd.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for Pair<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
