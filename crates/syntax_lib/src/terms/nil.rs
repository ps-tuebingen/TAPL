use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::List,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nil<T>
where
    T: Term,
{
    ty: <T as Term>::Type,
}

impl<T> Nil<T>
where
    T: Term,
{
    pub fn new<Typ>(ty: Typ) -> Nil<T>
    where
        Typ: Into<<T as Term>::Type>,
    {
        Nil { ty: ty.into() }
    }
}

impl<T> Term for Nil<T> where T: Term {}

impl<T> SubstTerm<T> for Nil<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Nil<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Nil {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Nil<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}
