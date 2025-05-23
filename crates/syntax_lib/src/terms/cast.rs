use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cast<T>
where
    T: Term,
{
    term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Cast<T>
where
    T: Term,
{
    pub fn new<T1, Ty>(t: T1, ty: Ty) -> Cast<T>
    where
        T1: Into<T>,
        Ty: Into<<T as Term>::Type>,
    {
        Cast {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Cast<T> where T: Term {}

impl<T> SubstTerm<T> for Cast<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Cast {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Cast<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Cast {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Cast<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} as {})", self.term, self.ty)
    }
}
