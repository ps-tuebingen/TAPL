use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Let<T>
where
    T: Term,
{
    var: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Let<T>
where
    T: Term,
{
    pub fn new<T1, T2>(v: &str, bound: T1, int: T2) -> Let<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Let {
            var: v.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
        }
    }
}
impl<T> Term for Let<T> where T: Term {}

impl<T> SubstTerm<T> for Let<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Let {
                var: self.var,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

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
