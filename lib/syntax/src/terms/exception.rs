use super::Term;
use crate::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Typ>(ty: Typ) -> Exception<T, Ty>
    where
        Typ: Into<Ty>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Term for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstTerm<T> for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Exception<T, Ty>
where
    T: Term,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Exception {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
