use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assign<T>
where
    T: Term,
{
    lhs: Box<T>,
    rhs: Box<T>,
}

impl<T> Term for Assign<T> where T: Term {}

impl<T> SubstTerm<T> for Assign<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Assign {
            lhs: Box::new(self.lhs.subst(v, t)),
            rhs: Box::new(self.rhs.subst(v, t)),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Assign<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Assign {
            lhs: Box::new(self.lhs.subst_type(v, ty)),
            rhs: Box::new(self.rhs.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Assign<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} := {}", self.lhs, self.rhs)
    }
}
