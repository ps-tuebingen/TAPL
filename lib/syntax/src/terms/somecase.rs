use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SomeCase<Lang>
where
    Lang: Language,
{
    pub bound_term: Rc<Lang::Term>,
    pub none_term: Rc<Lang::Term>,
    pub some_var: Var,
    pub some_term: Rc<Lang::Term>,
}

impl<Lang> SomeCase<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2, T3>(bound: T1, none: T2, v: &str, some: T3) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        Self {
            bound_term: Rc::new(bound.into()),
            none_term: Rc::new(none.into()),
            some_var: v.to_owned(),
            some_term: Rc::new(some.into()),
        }
    }
}

impl<Lang> Term for SomeCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for SomeCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.some_var {
            Self {
                bound_term: self.bound_term.subst(v, t),
                none_term: self.none_term.subst(v, t),
                some_var: self.some_var,
                some_term: self.some_term,
            }
        } else {
            Self {
                bound_term: self.bound_term.subst(v, t),
                none_term: self.none_term.subst(v, t),
                some_var: self.some_var,
                some_term: self.some_term.subst(v, t),
            }
        }
    }
}

impl<Lang> SubstType for SomeCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            bound_term: self.bound_term.subst_type(v, ty),
            none_term: self.none_term.subst_type(v, ty),
            some_var: self.some_var,
            some_term: self.some_term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for SomeCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nothing => {} | Something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}
