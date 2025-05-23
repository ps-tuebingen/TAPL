use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head<T>
where
    T: Term,
{
    term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Head<T>
where
    T: Term,
{
    pub fn new<T1, Ty>(t: T1, ty: Ty) -> Head<T>
    where
        T1: Into<T>,
        Ty: Into<<T as Term>::Type>,
    {
        Head {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Head<T> where T: Term {}

impl<T> SubstTerm<T> for Head<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Head {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Head<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Head {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Head<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head[{}]({})", self.ty, self.term)
    }
}
