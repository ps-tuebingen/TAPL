use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<Lang>
where
    Lang: Language,
{
    pub if_cond: Box<Lang::Term>,
    pub then_term: Box<Lang::Term>,
    pub else_term: Box<Lang::Term>,
}

impl<Lang> If<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2, T3>(cond: T1, th: T2, els: T3) -> If<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        If {
            if_cond: Box::new(cond.into()),
            then_term: Box::new(th.into()),
            else_term: Box::new(els.into()),
        }
    }
}

impl<Lang> Term for If<Lang> where Lang: Language {}

impl<Lang> SubstTerm for If<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst(v, t)),
            then_term: Box::new(self.then_term.subst(v, t)),
            else_term: Box::new(self.else_term.subst(v, t)),
        }
    }
}

impl<Lang> SubstType for If<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        If {
            if_cond: Box::new(self.if_cond.subst_type(v, ty)),
            then_term: Box::new(self.then_term.subst_type(v, ty)),
            else_term: Box::new(self.else_term.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for If<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if ({}) {{ {} }} else {{ {} }}",
            self.if_cond, self.then_term, self.else_term
        )
    }
}
