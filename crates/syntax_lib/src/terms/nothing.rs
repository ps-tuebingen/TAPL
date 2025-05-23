use super::Term;
use crate::{
    errors::Error,
    subst::{SubstTerm, SubstType},
    types::Optional,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nothing<T>
where
    T: Term,
{
    ty: <T as Term>::Type,
}

impl<T> Nothing<T>
where
    T: Term,
{
    pub fn new<Typ>(ty: Typ) -> Nothing<T>
    where
        Typ: Into<<T as Term>::Type>,
    {
        Nothing { ty: ty.into() }
    }
}

impl<T> Term for Nothing<T> where T: Term {}

impl<T> SubstTerm<T> for Nothing<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Nothing<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Nothing {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> fmt::Display for Nothing<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}
