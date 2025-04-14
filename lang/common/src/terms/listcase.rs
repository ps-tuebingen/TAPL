use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
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
