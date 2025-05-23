use super::Term;
use crate::{
    kinds::Kind,
    subst::{SubstTerm, SubstType},
    types::Mu,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fold<T>
where
    T: Term,
{
    term: Box<T>,
    ty: <T as Term>::Type,
}

impl<T> Fold<T>
where
    T: Term,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Fold<T>
    where
        T1: Into<T>,
        Typ: Into<<T as Term>::Type>,
    {
        Fold {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Fold<T> where T: Term {}

impl<T> SubstTerm<T> for Fold<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Fold {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Fold<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Fold {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Fold<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.term)
    }
}
