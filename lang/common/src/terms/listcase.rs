use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct ListCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    nil_rhs: Box<T>,
    cons_fst: Var,
    cons_rst: Var,
    cons_rhs: Box<T>,
}

impl<T> Term for ListCase<T> where T: Term {}

impl<T> SubstTerm<T> for ListCase<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        let bound_subst = self.bound_term.subst(v, t);
        let nil_subst = self.nil_rhs.subst(v, t);
        if *v == self.cons_fst || *v == self.cons_rst {
            ListCase {
                bound_term: Box::new(bound_subst),
                nil_rhs: Box::new(nil_subst),
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: self.cons_rhs,
            }
            .into()
        } else {
            ListCase {
                bound_term: Box::new(bound_subst),
                nil_rhs: Box::new(nil_subst),
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: Box::new(self.cons_rhs.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T, Ty> SubstType<Ty> for ListCase<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        ListCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            nil_rhs: Box::new(self.nil_rhs.subst_type(v, ty)),
            cons_fst: self.cons_fst,
            cons_rst: self.cons_rst,
            cons_rhs: Box::new(self.cons_rhs.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for ListCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ nil => {} | cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}
