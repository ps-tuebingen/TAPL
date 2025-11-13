use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};

use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unpack<Lang>
where
    Lang: Language,
{
    pub ty_name: TypeVar,
    pub term_name: Var,
    pub bound_term: Rc<Lang::Term>,
    pub in_term: Rc<Lang::Term>,
}

impl<Lang> Unpack<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2>(tyn: &str, tn: &str, bound: T1, int: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            ty_name: tyn.to_owned(),
            term_name: tn.to_owned(),
            bound_term: Rc::new(bound.into()),
            in_term: Rc::new(int.into()),
        }
    }
}

impl<Lang> Term for Unpack<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Unpack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.term_name {
            Self {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: self.bound_term.subst(v, t),
                in_term: self.in_term,
            }
        } else {
            Self {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: self.bound_term.subst(v, t),
                in_term: self.in_term.subst(v, t),
            }
        }
    }
}

impl<Lang> SubstType for Unpack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        let bound_subst = self.bound_term.subst_type(v, ty);
        if *v == self.ty_name {
            Self {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: bound_subst,
                in_term: self.in_term,
            }
        } else {
            Self {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: bound_subst,
                in_term: self.in_term.subst_type(v, ty),
            }
        }
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
