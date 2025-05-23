use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::{Bool, List},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IsNil<T>
where
    T: Term,
{
    term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> IsNil<T>
where
    T: Term,
{
    pub fn new<T1, Ty>(t: T1, ty: Ty) -> IsNil<T>
    where
        T1: Into<T>,
        Ty: Into<<T as Term>::Type>,
    {
        IsNil {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for IsNil<T> where T: Term {}

impl<T> SubstTerm<T> for IsNil<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        IsNil {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for IsNil<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        IsNil {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for IsNil<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isnil[{}]({})", self.ty, self.term)
    }
}
