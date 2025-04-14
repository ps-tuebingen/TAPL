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

impl<T, Ty> Term for Nothing<T, Ty>
where
    Ty: Type,
    T: Term,
{
}

impl<T, Ty> SubstTerm<T> for Nothing<T, Ty>
where
    T: Term,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<T, Ty> SubstType<Ty> for Nothing<T, Ty>
where
    T: Term,
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
        write!(f, "nothing[{}]", self.ty)
    }
}
