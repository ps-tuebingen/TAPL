use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If<Lang>
where
    Lang: Language,
{
    pub if_cond: Rc<Lang::Term>,
    pub then_term: Rc<Lang::Term>,
    pub else_term: Rc<Lang::Term>,
}

impl<Lang> If<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2, T3>(cond: T1, th: T2, els: T3) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            if_cond: Rc::new(cond.into()),
            then_term: Rc::new(th.into()),
            else_term: Rc::new(els.into()),
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
        Self {
            if_cond: self.if_cond.subst(v, t),
            then_term: self.then_term.subst(v, t),
            else_term: self.else_term.subst(v, t),
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
        Self {
            if_cond: self.if_cond.subst_type(v, ty),
            then_term: self.then_term.subst_type(v, ty),
            else_term: self.else_term.subst_type(v, ty),
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
