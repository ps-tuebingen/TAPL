use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ascribe<T>
where
    T: Term,
{
    pub term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Ascribe<T>
where
    T: Term,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Ascribe<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as Term>::Type>,
    {
        Ascribe {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Ascribe<T> where T: Term {}

impl<T> SubstTerm<T> for Ascribe<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Ascribe {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Ascribe<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Ascribe {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Ascribe<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} as {})", self.term, self.ty)
    }
}
