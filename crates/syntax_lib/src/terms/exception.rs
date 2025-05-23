use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<T>
where
    T: Term,
{
    ty: <T as Term>::Type,
    phantom: PhantomData<T>,
}

impl<T> Exception<T>
where
    T: Term,
{
    pub fn new<Typ>(ty: Typ) -> Exception<T>
    where
        Typ: Into<<T as Term>::Type>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T> Term for Exception<T> where T: Term {}

impl<T> SubstTerm<T> for Exception<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T> SubstType<<T as Term>::Type> for Exception<T>
where
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as Term>::Type) -> Self::Target {
        Exception {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T> fmt::Display for Exception<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
