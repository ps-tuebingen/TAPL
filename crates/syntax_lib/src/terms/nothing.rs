use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
{
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
{
    pub fn new<Typ>(ty: Typ) -> Nothing<T, Ty>
    where
        Typ: Into<Ty>,
    {
        Nothing {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Term for Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
{
}

impl<T, Ty> SubstTerm<T> for Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Nothing<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Nothing {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}
