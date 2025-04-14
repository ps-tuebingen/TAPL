use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct SomeCase<T>
where
    T: Term,
{
    bound_term: Box<T>,
    none_term: Box<T>,
    some_var: Var,
    some_term: Box<T>,
}

impl<T> Term for SomeCase<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for SomeCase<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        SomeCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            none_term: Box::new(self.none_term.subst_type(v, ty)),
            some_var: self.some_var,
            some_term: Box::new(self.some_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for SomeCase<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ nothing => {} | something({}) => {} }}",
            self.bound_term, self.none_term, self.some_var, self.some_term
        )
    }
}
