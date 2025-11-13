use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<Lang>
where
    Lang: Language,
{
    pub var: Var,
    pub bound_term: Rc<Lang::Term>,
    pub in_term: Rc<Lang::Term>,
}

impl<Lang> Let<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2>(v: &str, bound: T1, int: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            var: v.to_owned(),
            bound_term: Rc::new(bound.into()),
            in_term: Rc::new(int.into()),
        }
    }
}
impl<Lang> Term for Let<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Let<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            self
        } else {
            Self {
                var: self.var,
                bound_term: self.bound_term.subst(v, t),
                in_term: self.in_term.subst(v, t),
            }
        }
    }
}

impl<Lang> SubstType for Let<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            var: self.var,
            bound_term: self.bound_term.subst_type(v, ty),
            in_term: self.in_term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Let<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
