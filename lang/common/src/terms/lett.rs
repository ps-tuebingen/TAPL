use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar, Var};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Let<T>
where
    T: Term,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Term for Let<T> where T: Term {}

impl<T, Ty> SubstType<Ty> for Let<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Let {
            var: self.var,
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            in_term: Box::new(self.in_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Let<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let ({} = {}) in {}",
            self.var, self.bound_term, self.in_term
        )
    }
}
