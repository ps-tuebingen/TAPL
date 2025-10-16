use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListCase<Lang>
where
    Lang: Language,
{
    pub bound_term: Rc<Lang::Term>,
    pub nil_rhs: Rc<Lang::Term>,
    pub cons_fst: Var,
    pub cons_rst: Var,
    pub cons_rhs: Rc<Lang::Term>,
}

impl<Lang> ListCase<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2, T3>(bound: T1, nil: T2, hd: &str, tl: &str, cons: T3) -> ListCase<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
        T3: Into<Lang::Term>,
    {
        ListCase {
            bound_term: Rc::new(bound.into()),
            nil_rhs: Rc::new(nil.into()),
            cons_fst: hd.to_owned(),
            cons_rst: tl.to_owned(),
            cons_rhs: Rc::new(cons.into()),
        }
    }
}

impl<Lang> Term for ListCase<Lang> where Lang: Language {}

impl<Lang> SubstTerm for ListCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        let bound_subst = self.bound_term.subst(v, t);
        let nil_subst = self.nil_rhs.subst(v, t);
        if *v == self.cons_fst || *v == self.cons_rst {
            ListCase {
                bound_term: bound_subst,
                nil_rhs: nil_subst,
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: self.cons_rhs,
            }
        } else {
            ListCase {
                bound_term: bound_subst,
                nil_rhs: nil_subst,
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: self.cons_rhs.subst(v, t),
            }
        }
    }
}

impl<Lang> SubstType for ListCase<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        ListCase {
            bound_term: self.bound_term.subst_type(v, ty),
            nil_rhs: self.nil_rhs.subst_type(v, ty),
            cons_fst: self.cons_fst,
            cons_rst: self.cons_rst,
            cons_rhs: self.cons_rhs.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for ListCase<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nil => {} | Cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}
